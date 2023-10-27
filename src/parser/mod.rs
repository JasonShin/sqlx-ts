mod decl;
mod import;
mod tag;

use std::collections::HashMap;
use std::{fs, path::PathBuf};

use crate::common::SQL;
use crate::parser::decl::{process_decl, process_default_decl};
use crate::parser::import::find_sqlx_import_alias;
use color_eyre::eyre::Result;
use swc_common::{
    errors::{ColorConfig, Handler},
    input::StringInput,
    sync::Lrc,
    FileName, MultiSpan, SourceMap,
};
use swc_ecma_ast::{Key, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_parser::TsConfig;
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};
use tag::get_sql_from_expr;

fn insert_or_append_sqls(sqls_container: &mut HashMap<PathBuf, Vec<SQL>>, sqls: &Vec<SQL>, file_path: &PathBuf) {
    if sqls_container.contains_key(&*file_path.clone()) {
        let mut value = sqls_container.get(file_path).unwrap().clone();
        value.extend(sqls.clone());
        sqls_container.insert(file_path.clone(), (*value.to_owned()).to_owned());
    } else {
        sqls_container.insert(file_path.clone(), sqls.clone());
    }
}

fn get_var_decl_name_from_key(key: &Key) -> Option<String> {
    match &key {
        swc_ecma_ast::Key::Private(private) => Some(private.id.sym.to_string()),
        swc_ecma_ast::Key::Public(public) => match &public {
            swc_ecma_ast::PropName::Ident(ident) => Some(ident.sym.to_string()),
            swc_ecma_ast::PropName::Str(val) => Some(val.value.to_string()),
            swc_ecma_ast::PropName::Num(_) => None,
            swc_ecma_ast::PropName::Computed(_) => None,
            swc_ecma_ast::PropName::BigInt(_) => None,
        },
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
        Stmt::Decl(decl) => {
            process_decl(&mut sqls, decl, import_alias)?;
        }
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
    let ts_config: TsConfig = TsConfig {
        tsx: false,
        decorators: true,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: false,
    };
    let lexer = Lexer::new(
        Syntax::Typescript(ts_config),
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
        let mut sqls = vec![];

        match item {
            ModuleItem::Stmt(stmt) => {
                recurse_and_find_sql(&mut sqls, stmt, &import_alias)?;
            }
            ModuleItem::ModuleDecl(decl) => match decl {
                ModuleDecl::Import(import_decl) => {
                    let specifiers = &import_decl.specifiers;
                    for specifier in specifiers {
                        match specifier {
                            swc_ecma_ast::ImportSpecifier::Named(named) => {}
                            swc_ecma_ast::ImportSpecifier::Default(_) => todo!(),
                            swc_ecma_ast::ImportSpecifier::Namespace(_) => todo!(),
                        }
                    }
                }
                ModuleDecl::ExportDecl(export_decl) => {
                    let decl = export_decl.decl.clone();
                    process_decl(&mut sqls, &decl, &import_alias)?;
                }
                ModuleDecl::ExportNamed(export_named) => {}
                ModuleDecl::ExportDefaultDecl(export_default_decl) => {
                    let decl = export_default_decl.decl.clone();
                    process_default_decl(&mut sqls, &decl, &import_alias)?;
                }
                ModuleDecl::ExportDefaultExpr(_) => todo!(),
                ModuleDecl::ExportAll(_) => todo!(),
                ModuleDecl::TsImportEquals(_) => todo!(),
                ModuleDecl::TsExportAssignment(_) => todo!(),
                ModuleDecl::TsNamespaceExport(_) => todo!(),
            },
        }

        // This is to prevent any emptry string queries being inserted into sqls_map
        // which will be used to run `PREPARE` step and SQL parser logic
        let sqls: Vec<SQL> = sqls.into_iter().filter(|sql| !sql.query.is_empty()).collect();
        insert_or_append_sqls(&mut sqls_map, &sqls, path);
    }

    Ok((sqls_map, handler))
}
