use std::{
    borrow::BorrowMut,
    fs,
    path::{Path, PathBuf},
};

use sqlx_ts_common::SQL;
use swc_common::{
    errors::{ColorConfig, Handler},
    input::StringInput,
    sync::Lrc,
    FileName, MultiSpan, SourceMap,
};
use swc_ecma_ast::{Expr, ModuleItem, Stmt, VarDeclarator};
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};

fn get_sql_from_expr<'a>(expr: Expr, span: MultiSpan) -> Vec<SQL> {
    let mut sqls: Vec<SQL> = vec![];
    match expr {
        Expr::TaggedTpl(tagged_tpl) => {
            let tag = *tagged_tpl.tag;
            if let Expr::Ident(ident) = tag {
                let ident = ident.to_string();

                if ident.contains("sql") {
                    let mut sql_statements: Vec<SQL> = tagged_tpl
                        .tpl
                        .quasis
                        .iter()
                        .map(|tpl_element| SQL {
                            query: tpl_element.raw.to_string(),
                            span: span.clone(),
                        })
                        .collect();

                    sqls.append(&mut sql_statements)
                }
            }
        }
        _ => {}
    }

    sqls
}

/// you would normally pass in any var declarator such as
/// const sql = sql`SELECT * FROM xxx;`
fn get_sql_from_var_decl(var_declarator: VarDeclarator, span: MultiSpan) -> Vec<SQL> {
    let mut bag_of_sqls: Vec<SQL> = vec![];

    if let Some(init) = var_declarator.init {
        let mut result = get_sql_from_expr(*init, span);
        bag_of_sqls.append(&mut result);
    }

    bag_of_sqls
}

fn recurse_and_find_sql(mut sqls_container: &mut Vec<SQL>, stmt: Stmt) -> Option<String> {
    match stmt {
        Stmt::Block(_) => todo!(),
        Stmt::Empty(_) => todo!(),
        Stmt::Debugger(_) => todo!(),
        Stmt::With(_) => todo!(),
        Stmt::Return(rtn) => {
            if let Some(expr) = rtn.arg {
                let span: MultiSpan = rtn.span.into();
                let mut sqls = get_sql_from_expr(*expr, span);
                &sqls_container.append(&mut sqls);
            }
            None
        }
        Stmt::Labeled(_) => todo!(),
        Stmt::Break(_) => todo!(),
        Stmt::Continue(_) => todo!(),
        Stmt::If(_) => todo!(),
        Stmt::Switch(_) => todo!(),
        Stmt::Throw(_) => todo!(),
        Stmt::Try(_) => todo!(),
        Stmt::While(_) => todo!(),
        Stmt::DoWhile(_) => todo!(),
        Stmt::For(_) => todo!(),
        Stmt::ForIn(_) => todo!(),
        Stmt::ForOf(_) => todo!(),
        Stmt::Decl(decl) => {
            match decl {
                swc_ecma_ast::Decl::Class(_) => todo!(),
                swc_ecma_ast::Decl::Fn(fun) => {
                    if let Some(body) = fun.function.body {
                        for stmt in body.stmts {
                            recurse_and_find_sql(&mut sqls_container, stmt);
                        }
                    }
                    None
                }
                swc_ecma_ast::Decl::Var(var) => {
                    for var_decl in var.decls {
                        let span: MultiSpan = var.span.into();
                        let mut sqls = get_sql_from_var_decl(var_decl, span);
                        &sqls_container.append(sqls.borrow_mut());
                    }
                    // println!("checking var decl {:?}", var.decls);

                    None
                }
                swc_ecma_ast::Decl::TsInterface(_) => todo!(),
                swc_ecma_ast::Decl::TsTypeAlias(_) => todo!(),
                swc_ecma_ast::Decl::TsEnum(_) => todo!(),
                swc_ecma_ast::Decl::TsModule(_) => todo!(),
            }
        }
        Stmt::Expr(expr) => {
            let span: MultiSpan = expr.span.into();
            let expr = *expr.expr;
            let mut result = get_sql_from_expr(expr, span);
            &sqls_container.append(&mut result);
            None
        }
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
        .expect("failed to parser module");

    let mut sqls: Vec<SQL> = vec![];

    for item in _module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                // TODO: maybe have a main mutable array and pass it to the recurse method
                recurse_and_find_sql(&mut sqls, stmt);
            }
            ModuleItem::ModuleDecl(decl) => {
                println!("decl?");
            }
        }
    }

    (sqls, handler)
}
