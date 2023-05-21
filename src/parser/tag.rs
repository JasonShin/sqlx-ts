use crate::common::SQL;
use swc_common::MultiSpan;
use swc_ecma_ast::{Expr, Pat, VarDeclarator};

pub fn get_var_decl_name(var_declarator: &VarDeclarator) -> Option<String> {
    match &var_declarator.name {
        // ident is a valid pattern to figure out var_decl_name `const someQuery = foo`
        Pat::Ident(ident) => Some(ident.id.sym.to_string()),
        // `const [foo, bar]` = foo is not a valid pattern to figure out var_decl_name
        Pat::Array(_) => None,
        Pat::Rest(_) => todo!(),
        // `const { something } = foo` is not a valid pattern to figure out var_decl_name
        Pat::Object(_object_pat) => None,
        Pat::Assign(_) => todo!(),
        Pat::Invalid(_) => todo!(),
        Pat::Expr(_) => todo!(),
    }
}

pub fn get_sql_from_expr<'a>(
    var_decl_name: &Option<String>,
    expr: &Expr,
    span: &MultiSpan,
    import_alias: &String,
) -> Vec<SQL> {
    match &expr {
        Expr::TaggedTpl(tagged_tpl) => {
            let tag = &*tagged_tpl.tag;
            if let Expr::Ident(ident) = tag {
                let ident = ident.to_string();

                if ident.contains(import_alias) {
                    return tagged_tpl
                        .tpl
                        .quasis
                        .iter()
                        .map(|tpl_element| SQL {
                            var_decl_name: var_decl_name.to_owned(),
                            query: tpl_element.raw.to_string(),
                            span: span.clone(),
                        })
                        .collect();
                }
            }

            vec![]
        }
        Expr::TsNonNull(expr) => get_sql_from_expr(var_decl_name, &expr.expr, span, import_alias),
        Expr::Call(call_expr) => call_expr
            .args
            .clone()
            .into_iter()
            .flat_map(|arg| get_sql_from_expr(var_decl_name, &arg.expr, span, import_alias))
            .collect(),
        _ => vec![],
    }
}

/// you would normally pass in any var declarator such as
/// const sql = sql`SELECT * FROM xxx;`
pub fn get_sql_from_var_decl(var_declarator: &VarDeclarator, span: MultiSpan, import_alias: &String) -> Vec<SQL> {
    let mut bag_of_sqls: Vec<SQL> = vec![];
    let var_decl_name = get_var_decl_name(var_declarator);

    // We should skip if we fail to
    if var_decl_name.is_none() {
        return bag_of_sqls;
    }

    if let Some(init) = &var_declarator.init {
        // TODO: make it understand `const someQuery = SQLX.sql`SELECT * FROM lazy_unknown2`;` in js_failure_path1/lazy-loaded.js
        let mut result = get_sql_from_expr(&Some(var_decl_name.unwrap()), &init.clone(), &span, import_alias);
        bag_of_sqls.append(&mut result);
    }

    bag_of_sqls
}
