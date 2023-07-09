use crate::common::SQL;
use swc_common::MultiSpan;
use swc_ecma_ast::{Expr, Pat, Prop, PropOrSpread, VarDeclarator};

/// The method grabs the name of the variable if it exists
pub fn get_var_decl_name(var_declarator: &VarDeclarator) -> Option<String> {
    match &var_declarator.name {
        // ident is a valid pattern to figure out var_decl_name `const someQuery = foo`
        Pat::Ident(ident) => Some(ident.id.sym.to_string()),
        // `const [foo, bar]` = foo is not a valid pattern to figure out var_decl_name
        Pat::Array(_) => None,
        Pat::Rest(_) => None,
        // `const { something } = foo` is not a valid pattern to figure out var_decl_name
        Pat::Object(_) => None,
        Pat::Assign(_) => None,
        Pat::Invalid(_) => None,
        Pat::Expr(_) => None,
    }
}

pub fn get_sql_from_expr<'a>(
    sqls: &mut Vec<SQL>,
    var_decl_name: &Option<String>,
    expr: &Expr,
    span: &MultiSpan,
    import_alias: &String,
) {
    match &expr {
        Expr::TaggedTpl(tagged_tpl) => {
            let tag = &*tagged_tpl.tag;
            if let Expr::Ident(ident) = tag {
                let ident = ident.to_string();

                if ident.contains(import_alias) {
                    let new_sqls: Vec<SQL> = tagged_tpl
                        .tpl
                        .quasis
                        .iter()
                        .map(|tpl_element| SQL {
                            var_decl_name: var_decl_name.to_owned(),
                            query: tpl_element.raw.to_string(),
                            span: span.clone(),
                        })
                        .collect();

                    sqls.extend(new_sqls.clone());
                }
            }
        }
        Expr::TsNonNull(expr) => {
            get_sql_from_expr(sqls, var_decl_name, &expr.expr, span, import_alias);
        }
        Expr::Call(call_expr) => {
            for arg in &call_expr.args {
                get_sql_from_expr(sqls, var_decl_name, &arg.expr, span, import_alias)
            }
            /*let new_sqls: Vec<SQL> = call_expr
            .args
            .clone()
            .into_iter()
            .flat_map(|arg| get_sql_from_expr(sqls, var_decl_name, &arg.expr, span, import_alias))
            .collect();*/
        }
        Expr::This(_) => todo!(),
        Expr::Array(a) => {
            for elem in &a.elems {
                match elem {
                    Some(expr) => get_sql_from_expr(sqls, var_decl_name, &expr.expr, span, import_alias),
                    None => {}
                }
            }
        }
        Expr::Object(object) => {
            for prop in &object.props {
                match prop {
                    PropOrSpread::Spread(_) => todo!(),
                    PropOrSpread::Prop(prop) => match *prop.clone() {
                        Prop::Shorthand(_) => todo!(),
                        Prop::KeyValue(key_val) => {
                            let value = &key_val.value;
                            get_sql_from_expr(sqls, var_decl_name, value, span, import_alias)
                        }
                        Prop::Assign(_) => todo!(),
                        Prop::Getter(_) => todo!(),
                        Prop::Setter(_) => todo!(),
                        Prop::Method(_) => todo!(),
                    },
                }
            }
        }
        Expr::Fn(_) => todo!(),
        Expr::Unary(_) => todo!(),
        Expr::Update(_) => todo!(),
        Expr::Bin(_) => todo!(),
        Expr::Assign(_) => todo!(),
        Expr::Member(member) => {
            let obj = &member.obj;
            return get_sql_from_expr(sqls, var_decl_name, obj, span, import_alias);
        }
        Expr::SuperProp(_) => todo!(),
        Expr::Cond(_) => todo!(),
        Expr::New(expr) => {
            let expr = &expr.callee;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Seq(_) => todo!(),
        Expr::Ident(ident) => {}
        Expr::Lit(_) => todo!(),
        Expr::Tpl(_) => todo!(),
        Expr::Arrow(_) => todo!(),
        Expr::Class(_) => todo!(),
        Expr::Yield(_) => todo!(),
        Expr::MetaProp(_) => todo!(),
        Expr::Await(await_expr) => {
            let expr = &await_expr.arg;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Paren(_) => todo!(),
        Expr::JSXMember(_) => todo!(),
        Expr::JSXNamespacedName(_) => todo!(),
        Expr::JSXEmpty(_) => todo!(),
        Expr::JSXElement(_) => todo!(),
        Expr::JSXFragment(_) => todo!(),
        Expr::TsTypeAssertion(_) => todo!(),
        Expr::TsConstAssertion(_) => todo!(),
        Expr::TsAs(_) => todo!(),
        Expr::TsInstantiation(_) => todo!(),
        Expr::PrivateName(_) => todo!(),
        Expr::OptChain(_) => todo!(),
        Expr::Invalid(_) => todo!(),
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
        let mut sqls = vec![];
        // TODO: make it understand `const someQuery = SQLX.sql`SELECT * FROM lazy_unknown2`;` in js_failure_path1/lazy-loaded.js
        get_sql_from_expr(&mut sqls, &var_decl_name, &init.clone(), &span, import_alias);
        bag_of_sqls.extend(sqls);
    }

    bag_of_sqls
}
