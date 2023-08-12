use crate::common::SQL;
use swc_common::MultiSpan;
use swc_ecma_ast::{BlockStmt, ClassMember, Expr, Pat, Prop, PropOrSpread, SuperProp, VarDeclarator};

use super::recurse_and_find_sql;

/// The method process block statement as expression
/// It receives a block statement object from Class expression
/// inserts the sqls into the sqls vector
pub fn process_block_stmt_as_expr(
    block_stmt: &Option<BlockStmt>,
    sqls: &mut Vec<SQL>,
    var_decl_name: &Option<String>,
    span: &MultiSpan,
    import_alias: &String,
) {
    if let Some(body) = block_stmt {
        for stmt in &body.stmts {
            let expr = stmt.as_expr();
            if let Some(expr) = expr {
                let expr = &expr.expr;
                get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
            } else {
                recurse_and_find_sql(sqls, stmt, import_alias);
            }
        }
    }
}

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
            let num_args = &call_expr.args.len();

            if let Some(callee_expr) = &call_expr.callee.as_expr() {
                if num_args == &0 {
                    get_sql_from_expr(sqls, var_decl_name, callee_expr, span, import_alias);
                }
            }
            for arg in &call_expr.args {
                get_sql_from_expr(sqls, var_decl_name, &arg.expr, span, import_alias)
            }
        }
        Expr::This(_) => {}
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
                    PropOrSpread::Spread(_) => {}
                    PropOrSpread::Prop(prop) => match *prop.clone() {
                        Prop::Shorthand(_) => {}
                        Prop::KeyValue(key_val) => {
                            let value = &key_val.value;
                            get_sql_from_expr(sqls, var_decl_name, value, span, import_alias)
                        }
                        Prop::Assign(assign) => {
                            let value = &assign.value;
                            get_sql_from_expr(sqls, var_decl_name, value, span, import_alias)
                        }
                        Prop::Getter(getter) => {
                            let body = &getter.body;
                            process_block_stmt_as_expr(body, sqls, var_decl_name, span, import_alias)
                        }
                        // TODO: add test
                        Prop::Setter(setter) => {
                            let body = &setter.body;
                            process_block_stmt_as_expr(body, sqls, var_decl_name, span, import_alias)
                        }
                        Prop::Method(method) => {
                            let body = &method.function.body;
                            process_block_stmt_as_expr(body, sqls, var_decl_name, span, import_alias)
                        }
                    },
                }
            }
        }
        Expr::Fn(_) => {}
        Expr::Unary(unary) => {
            let expr = &unary.arg;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Update(update) => {
            let expr = &update.arg;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Bin(bin) => {
            let left = &bin.left;
            let right = &bin.right;
            get_sql_from_expr(sqls, var_decl_name, left, span, import_alias);
            get_sql_from_expr(sqls, var_decl_name, right, span, import_alias);
        }
        Expr::Assign(assign) => {
            let expr = &assign.right;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Member(member) => {
            let obj = &member.obj;
            return get_sql_from_expr(sqls, var_decl_name, obj, span, import_alias);
        }
        Expr::SuperProp(s) => {
            let super_prop = &s.prop;
            match &super_prop {
                SuperProp::Ident(_) => {}
                SuperProp::Computed(comp) => {
                    let expr = &comp.expr;
                    return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
                }
            }
        }
        Expr::Cond(_) => {}
        Expr::New(expr) => {
            let expr = &expr.callee;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Seq(seq) => {
            let exprs = &seq.exprs;
            for expr in exprs {
                get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias)
            }
        }
        Expr::Ident(ident) => {}
        Expr::Lit(lit) => {}
        Expr::Tpl(tpl) => {
            for expr in &tpl.exprs {
                get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias)
            }
        }
        Expr::Arrow(arrow) => {
            let expr = &arrow.clone().body.expr();
            let block_stmt = &arrow.clone().body.block_stmt();
            process_block_stmt_as_expr(&block_stmt, sqls, var_decl_name, span, import_alias);

            if let Some(expr) = expr {
                return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
            }
        }
        Expr::Class(class) => {
            let class_body = &class.class.body;
            for body_stmt in class_body {
                match body_stmt {
                    ClassMember::Constructor(constructor) => {
                        if let Some(body) = &constructor.body {
                            for stmt in &body.stmts {
                                let expr = stmt.as_expr();
                                if let Some(expr) = expr {
                                    let expr = &expr.expr;
                                    return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
                                }
                            }
                        }
                    }
                    ClassMember::Method(method) => {
                        let body = &method.function.body;
                        process_block_stmt_as_expr(body, sqls, var_decl_name, span, import_alias)
                    }
                    ClassMember::PrivateMethod(private_method) => {
                        let body = &private_method.function.body;
                        process_block_stmt_as_expr(body, sqls, var_decl_name, span, import_alias)
                    }
                    ClassMember::ClassProp(class_prop) => {
                        let body = &class_prop.value;
                        if let Some(body) = body {
                            return get_sql_from_expr(sqls, var_decl_name, body, span, import_alias);
                        }
                    }
                    ClassMember::PrivateProp(private_prop) => {
                        let body = &private_prop.value;
                        if let Some(body) = body {
                            return get_sql_from_expr(sqls, var_decl_name, body, span, import_alias);
                        }
                    }
                    ClassMember::TsIndexSignature(_) => {}
                    ClassMember::Empty(_) => {}
                    ClassMember::StaticBlock(static_block) => {
                        let body = &static_block.body;
                        process_block_stmt_as_expr(&Some(body.clone()), sqls, var_decl_name, span, import_alias)
                    }
                    ClassMember::AutoAccessor(auto_accessor) => {
                        let value = &auto_accessor.value;
                        if let Some(expr) = &value {
                            get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias)
                        }
                    }
                }
            }
        }
        Expr::Yield(yield_expr) => {
            let expr = &yield_expr.arg;
            if let Some(expr) = expr {
                return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
            }
        }
        Expr::MetaProp(_) => {}
        Expr::Await(await_expr) => {
            let expr = &await_expr.arg;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::Paren(paren) => {
            let expr = &paren.expr;
            return get_sql_from_expr(sqls, var_decl_name, expr, span, import_alias);
        }
        Expr::JSXMember(_) => {}
        Expr::JSXNamespacedName(_) => {}
        Expr::JSXEmpty(_) => {}
        Expr::JSXElement(_) => {}
        Expr::JSXFragment(_) => {}
        Expr::TsTypeAssertion(_) => {}
        Expr::TsConstAssertion(_) => {}
        Expr::TsAs(_) => {}
        Expr::TsInstantiation(_) => {}
        Expr::PrivateName(_) => {}
        Expr::OptChain(_) => {}
        Expr::Invalid(_) => {}
        Expr::TsSatisfies(_) => {}
    }
}

/// you would normally pass in any var declarator such as
/// const sql = sql`SELECT * FROM xxx;`
pub fn get_sql_from_var_decl(var_declarator: &VarDeclarator, span: &MultiSpan, import_alias: &String) -> Vec<SQL> {
    let mut bag_of_sqls: Vec<SQL> = vec![];
    let var_decl_name = get_var_decl_name(var_declarator);

    // We should skip if we fail to
    if var_decl_name.is_none() {
        return bag_of_sqls;
    }

    if let Some(init) = &var_declarator.init {
        // TODO: make it understand `const someQuery = SQLX.sql`SELECT * FROM lazy_unknown2`;` in js_failure_path1/lazy-loaded.js
        get_sql_from_expr(&mut bag_of_sqls, &var_decl_name, &init.clone(), &span, import_alias);
    }

    bag_of_sqls
}
