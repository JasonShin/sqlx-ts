use sqlx_ts_common::SQL;
use swc_common::MultiSpan;
use swc_ecma_ast::{Expr, VarDeclarator};

pub fn get_sql_from_expr<'a>(expr: Expr, span: MultiSpan, import_alias: &String) -> Vec<SQL> {
    let mut sqls: Vec<SQL> = vec![];
    match expr {
        Expr::TaggedTpl(tagged_tpl) => {
            let tag = *tagged_tpl.tag;
            if let Expr::Ident(ident) = tag {
                let ident = ident.to_string();

                if ident.contains(import_alias) {
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
pub fn get_sql_from_var_decl(
    var_declarator: &VarDeclarator,
    span: MultiSpan,
    import_alias: &String,
) -> Vec<SQL> {
    let mut bag_of_sqls: Vec<SQL> = vec![];

    if let Some(init) = &var_declarator.init {
        let mut result = get_sql_from_expr(*init.clone(), span, import_alias);
        bag_of_sqls.append(&mut result);
    }

    bag_of_sqls
}
