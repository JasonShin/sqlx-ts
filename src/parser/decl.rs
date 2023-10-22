use swc_common::MultiSpan;
use swc_ecma_ast::{Decl, ClassMember};
use color_eyre::eyre::Result;

use crate::common::SQL;

use super::{recurse_and_find_sql, tag::{get_sql_from_expr, get_sql_from_var_decl}, get_var_decl_name_from_key};

pub fn process_decl(mut sqls: &mut Vec<SQL>, decl: &Decl, import_alias: &String) -> Result<()> {
    match decl {
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
                    ClassMember::AutoAccessor(auto_accessor) => {
                        let value = &auto_accessor.value;
                        let key = &auto_accessor.key;

                        if let Some(expr) = &value {
                            let span: MultiSpan = auto_accessor.span.into();
                            let var_decl_name = get_var_decl_name_from_key(&key);
                            get_sql_from_expr(&mut sqls, &var_decl_name, expr, &span, import_alias);
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
                let num_new_sqls = new_sqls.len();

                sqls.extend(new_sqls);

                // We've already found the sqls based on the variable name, we should skip processing further
                if num_new_sqls > 0 {
                    continue;
                }
                // Try to retrieve name of the variable
                let name = var_decl.name.as_ident().map(|ident| ident.sym.to_string());
                // this is when the variable name is not found due to syntax like
                // const [rows, i] = await connection.execute....
                if let Some(init) = &var_decl.init {
                    let expr = *init.clone();
                    get_sql_from_expr(&mut sqls, &name, &expr, &span, import_alias);
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
        Decl::Using(using) => {
            for decl in &using.decls {
                let init = &decl.init;
                if let Some(expr) = init {
                    let span: &MultiSpan = &using.span.into();
                    get_sql_from_expr(sqls, &None, expr, span, import_alias);
                }
            }
        }
    }
    Ok(())
}
