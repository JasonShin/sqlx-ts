mod import;
mod tag;

use std::{borrow::BorrowMut, fs, path::PathBuf};

use crate::parser::import::find_sqlx_import_alias;
use sqlx_ts_common::SQL;
use swc_common::{
    errors::{ColorConfig, Handler},
    input::StringInput,
    sync::Lrc,
    FileName, MultiSpan, SourceMap,
};
use swc_ecma_ast::{BlockStmt, ClassMember, Decl, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};
use tag::{get_sql_from_expr, get_sql_from_var_decl};

fn recurse_and_find_sql(
    mut sqls_container: &mut Vec<SQL>,
    stmt: &Stmt,
    import_alias: &String,
) -> Option<String> {
    match stmt {
        Stmt::Block(block) => {
            for stmt in &block.stmts {
                recurse_and_find_sql(&mut sqls_container, &stmt, &import_alias);
            }
            None
        }
        Stmt::With(with_stmt) => {
            let stmt = *with_stmt.body.clone();
            recurse_and_find_sql(&mut sqls_container, &stmt, &import_alias);
            None
        }
        Stmt::Return(rtn) => {
            if let Some(expr) = &rtn.arg {
                let span: MultiSpan = rtn.span.into();
                let mut sqls = get_sql_from_expr(*expr.clone(), span, import_alias);
                &sqls_container.append(&mut sqls);
            }
            None
        }
        Stmt::If(if_stmt) => {
            let stmt = *if_stmt.cons.clone();
            recurse_and_find_sql(&mut sqls_container, &stmt, import_alias);
            None
        }
        Stmt::Switch(switch_stmt) => {
            for case in &switch_stmt.cases {
                for stmt in &case.cons {
                    recurse_and_find_sql(&mut sqls_container, &stmt, &import_alias);
                }
            }
            None
        }
        Stmt::Throw(throw_stmt) => {
            let span: MultiSpan = throw_stmt.span.into();
            let expr = *throw_stmt.arg.clone();
            let mut result = get_sql_from_expr(expr, span, import_alias);
            &sqls_container.append(&mut result);
            None
        }
        Stmt::Try(try_stmt) => {
            // handles statements inside try {}
            for stmt in &try_stmt.block.stmts {
                recurse_and_find_sql(&mut sqls_container, &stmt, &import_alias);
            }

            // handles statements inside catch {}
            if let Some(stmt) = &try_stmt.handler {
                for stmt in &stmt.body.stmts {
                    recurse_and_find_sql(&mut sqls_container, &stmt, &import_alias);
                }
            }
            None
        }
        Stmt::While(while_stmt) => {
            let body_stmt = *while_stmt.body.clone();
            recurse_and_find_sql(&mut sqls_container, &body_stmt, &import_alias);
            None
        }
        Stmt::DoWhile(do_while_stmt) => {
            let body_stmt = *do_while_stmt.body.clone();
            recurse_and_find_sql(&mut sqls_container, &body_stmt, &import_alias);
            None
        }
        Stmt::For(for_stmt) => {
            let body_stmt = *for_stmt.body.clone();
            recurse_and_find_sql(&mut sqls_container, &body_stmt, &import_alias);
            None
        }
        Stmt::ForIn(for_in_stmt) => {
            let body_stmt = *for_in_stmt.body.clone();
            recurse_and_find_sql(&mut sqls_container, &body_stmt, &import_alias);
            None
        }
        Stmt::ForOf(for_of_stmt) => {
            let body_stmt = *for_of_stmt.body.clone();
            recurse_and_find_sql(&mut sqls_container, &body_stmt, &import_alias);
            None
        }
        Stmt::Decl(decl) => match decl {
            swc_ecma_ast::Decl::Class(class) => {
                let class_body = &class.class.body;
                for body_stmt in class_body {
                    match body_stmt {
                        ClassMember::Constructor(_) => {}
                        ClassMember::Method(class_method) => {
                            if let Some(body) = &class_method.function.body {
                                for stmt in &body.stmts {
                                    recurse_and_find_sql(&mut sqls_container, &stmt, import_alias);
                                }
                            }
                        }
                        ClassMember::PrivateMethod(private_method) => {
                            if let Some(body) = &private_method.function.body {
                                for stmt in &body.stmts {
                                    recurse_and_find_sql(&mut sqls_container, &stmt, import_alias);
                                }
                            }
                        }
                        ClassMember::StaticBlock(static_block) => {
                            for stmt in &static_block.body.stmts {
                                recurse_and_find_sql(&mut sqls_container, &stmt, import_alias);
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            swc_ecma_ast::Decl::Fn(fun) => {
                if let Some(body) = &fun.function.body {
                    for stmt in &body.stmts {
                        recurse_and_find_sql(&mut sqls_container, &stmt, import_alias);
                    }
                }
                None
            }
            swc_ecma_ast::Decl::Var(var) => {
                for var_decl in &var.decls {
                    let span: MultiSpan = var.span.into();
                    let mut sqls = get_sql_from_var_decl(var_decl, span, import_alias);
                    &sqls_container.append(sqls.borrow_mut());
                }

                None
            }
            _ => None,
        },
        Stmt::Expr(expr) => {
            let span: MultiSpan = expr.span.into();
            let expr = *expr.expr.clone();
            let mut result = get_sql_from_expr(expr, span, import_alias);
            &sqls_container.append(&mut result);
            None
        }
        _ => None,
    }
}

pub fn parse_source(path: &PathBuf) -> (Vec<SQL>, Handler) {
    let contents = fs::read_to_string(path).unwrap();

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let file_path = path.as_os_str().to_str().unwrap().to_string();
    let fm = cm.new_source_file(FileName::Custom(file_path), contents.into());
    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let _module = parser
        .parse_module()
        .map_err(|mut e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parse module");

    let mut sqls: Vec<SQL> = vec![];

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
        .find_map(|import_decl| find_sqlx_import_alias(import_decl))
        .unwrap_or_else(|| "sql".to_string());

    for item in &_module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                // TODO: maybe have a main mutable array and pass it to the recurse method
                recurse_and_find_sql(&mut sqls, &stmt, &import_alias);
            }
            _ => {}
        }
    }

    (sqls, handler)
}
