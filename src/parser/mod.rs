mod import;
mod tag;

use std::collections::HashMap;
use std::{fs, path::PathBuf};

use crate::common::SQL;
use crate::parser::import::find_sqlx_import_alias;
use color_eyre::eyre::Result;
use swc_common::{
    errors::{ColorConfig, Handler},
    input::StringInput,
    sync::Lrc,
    FileName, MultiSpan, SourceMap,
};
use swc_ecma_ast::{ClassMember, Decl, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};
use tag::{get_sql_from_expr, get_sql_from_var_decl};

fn insert_or_append_sqls(sqls_container: &mut HashMap<PathBuf, Vec<SQL>>, sqls: &Vec<SQL>, file_path: &PathBuf) {
    if sqls_container.contains_key(&*file_path.clone()) {
        let mut value = sqls_container.get(file_path).unwrap().clone();
        value.extend(sqls.clone());
        sqls_container.insert(file_path.clone(), (*value.to_owned()).to_owned());
    } else {
        sqls_container.insert(file_path.clone(), sqls.clone());
    }
}

fn recurse_and_find_sql(mut sqls: &mut Vec<SQL>, stmt: &Stmt, import_alias: &String) -> Result<()> {
    match stmt {
        Stmt::Block(block) => {
            for stmt in &block.stmts {
                recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
            }
        }
        Stmt::With(with_stmt) => {
            let stmt = *with_stmt.body.clone();
            recurse_and_find_sql(&mut sqls, &stmt, import_alias)?;
        }
        Stmt::Return(rtn) => {
            if let Some(expr) = &rtn.arg {
                let span: MultiSpan = rtn.span.into();
                get_sql_from_expr(&mut sqls, &None, &expr.clone(), &span, import_alias);
            }
        }
        Stmt::If(if_stmt) => {
            let stmt = *if_stmt.cons.clone();
            recurse_and_find_sql(&mut sqls, &stmt, import_alias)?;
        }
        Stmt::Switch(switch_stmt) => {
            for case in &switch_stmt.cases {
                for stmt in &case.cons {
                    recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                }
            }
        }
        Stmt::Throw(throw_stmt) => {
            let span: MultiSpan = throw_stmt.span.into();
            let expr = *throw_stmt.arg.clone();
            get_sql_from_expr(&mut sqls, &None, &expr, &span, import_alias);
        }
        Stmt::Try(try_stmt) => {
            // handles statements inside try {}
            for stmt in &try_stmt.block.stmts {
                recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
            }

            // handles statements inside catch {}
            if let Some(stmt) = &try_stmt.handler {
                for stmt in &stmt.body.stmts {
                    recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                }
            }
        }
        Stmt::While(while_stmt) => {
            let body_stmt = *while_stmt.body.clone();
            recurse_and_find_sql(&mut sqls, &body_stmt, import_alias)?;
        }
        Stmt::DoWhile(do_while_stmt) => {
            let body_stmt = *do_while_stmt.body.clone();
            recurse_and_find_sql(&mut sqls, &body_stmt, import_alias)?;
        }
        Stmt::For(for_stmt) => {
            let body_stmt = *for_stmt.body.clone();
            recurse_and_find_sql(&mut sqls, &body_stmt, import_alias)?;
        }
        Stmt::ForIn(for_in_stmt) => {
            let body_stmt = *for_in_stmt.body.clone();
            recurse_and_find_sql(&mut sqls, &body_stmt, import_alias)?;
        }
        Stmt::ForOf(for_of_stmt) => {
            let body_stmt = *for_of_stmt.body.clone();
            recurse_and_find_sql(&mut sqls, &body_stmt, import_alias)?;
        }
        Stmt::Decl(decl) => match decl {
            Decl::Class(class) => {
                let class_body = &class.class.body;
                for body_stmt in class_body {
                    match body_stmt {
                        ClassMember::Constructor(constructor) => {
                            if let Some(body) = &constructor.body {
                                for stmt in &body.stmts {
                                    recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                                }
                            }
                        }
                        ClassMember::Method(class_method) => {
                            if let Some(body) = &class_method.function.body {
                                for stmt in &body.stmts {
                                    recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                                }
                            }
                        }
                        ClassMember::PrivateMethod(private_method) => {
                            if let Some(body) = &private_method.function.body {
                                for stmt in &body.stmts {
                                    recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                                }
                            }
                        }
                        ClassMember::StaticBlock(static_block) => {
                            for stmt in &static_block.body.stmts {
                                recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                            }
                        }
                        ClassMember::PrivateProp(private_prop) => {
                            if let Some(expr) = &private_prop.value {
                                let span: MultiSpan = private_prop.span.into();
                                get_sql_from_expr(&mut sqls, &None, &expr.clone(), &span, import_alias);
                            }
                        }
                        ClassMember::ClassProp(class_prop) => {
                            if let Some(expr) = &class_prop.value {
                                let span: MultiSpan = class_prop.span.into();
                                get_sql_from_expr(&mut sqls, &None, &expr.clone(), &span, import_alias);
                            }
                        }
                        ClassMember::TsIndexSignature(_) => {}
                        ClassMember::Empty(_) => {}
                    }
                }
            }
            Decl::Fn(fun) => {
                if let Some(body) = &fun.function.body {
                    for stmt in &body.stmts {
                        recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                    }
                }
            }
            Decl::Var(var) => {
                for var_decl in &var.decls {
                    let span: MultiSpan = var.span.into();
                    let new_sqls = get_sql_from_var_decl(var_decl, &span, import_alias);
                    sqls.extend(new_sqls);

                    if let Some(init) = &var_decl.init {
                        let expr = *init.clone();
                        get_sql_from_expr(&mut sqls, &None, &expr, &span, import_alias);
                    }
                }
            }
            Decl::TsInterface(_) => {}
            Decl::TsTypeAlias(_) => {}
            Decl::TsEnum(_) => {}
            Decl::TsModule(module) => {
                for stmt in &module.body {
                    for block in &stmt.as_ts_module_block() {
                        for body in &block.body {
                            let stmt = &body.clone().stmt();
                            if let Some(stmt) = stmt {
                                recurse_and_find_sql(&mut sqls, stmt, import_alias)?;
                            }
                        }
                    }
                }
            }
        },
        Stmt::Expr(expr) => {
            let span: MultiSpan = expr.span.into();
            let expr = *expr.expr.clone();
            get_sql_from_expr(&mut sqls, &None, &expr, &span, import_alias);
        }
        Stmt::Empty(_) => {}
        Stmt::Debugger(_) => {}
        Stmt::Labeled(labeled) => {
            let body_stmt = *labeled.body.clone();
            recurse_and_find_sql(&mut sqls, &body_stmt, import_alias)?;
        }
        Stmt::Break(_) => {}
        Stmt::Continue(_) => {}
    }
    Ok(())
}

pub fn parse_source(path: &PathBuf) -> Result<(HashMap<PathBuf, Vec<SQL>>, Handler)> {
    let contents = fs::read_to_string(path).unwrap();

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let file_path = path.as_os_str().to_str().unwrap().to_string();
    let fm = cm.new_source_file(FileName::Custom(file_path), contents);
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let _module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parse module");

    let mut sqls_map: HashMap<PathBuf, Vec<SQL>> = HashMap::new();

    let import_alias = _module
        .body
        .iter()
        .filter_map(|line| match line {
            ModuleItem::ModuleDecl(module_decl) => match module_decl {
                ModuleDecl::Import(module_import_decl) => Some(module_import_decl),
                _ => None,
            },
            _ => None,
        })
        .find_map(find_sqlx_import_alias)
        .unwrap_or_else(|| "sql".to_string());

    for item in &_module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                let mut sqls = vec![];
                recurse_and_find_sql(&mut sqls, stmt, &import_alias).unwrap();
                insert_or_append_sqls(&mut sqls_map, &sqls, path);
            }
            _ => {}
        }
    }

    Ok((sqls_map, handler))
}
