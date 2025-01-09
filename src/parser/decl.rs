use color_eyre::eyre::Result;
use swc_common::MultiSpan;
use swc_ecma_ast::{ClassDecl, ClassMember, Decl, DefaultDecl};

use crate::common::SQL;

use super::{
  get_var_decl_name_from_key, recurse_and_find_sql,
  tag::{get_sql_from_expr, get_sql_from_var_decl},
};

fn process_class_member(sqls: &mut Vec<SQL>, body_stmt: &ClassMember, import_alias: &String) -> Result<()> {
  match body_stmt {
    ClassMember::Constructor(constructor) => {
      if let Some(body) = &constructor.body {
        for stmt in &body.stmts {
          recurse_and_find_sql(sqls, stmt, import_alias)?;
        }
      }
    }
    ClassMember::Method(class_method) => {
      if let Some(body) = &class_method.function.body {
        for stmt in &body.stmts {
          recurse_and_find_sql(sqls, stmt, import_alias)?;
        }
      }
    }
    ClassMember::PrivateMethod(private_method) => {
      if let Some(body) = &private_method.function.body {
        for stmt in &body.stmts {
          recurse_and_find_sql(sqls, stmt, import_alias)?;
        }
      }
    }
    ClassMember::StaticBlock(static_block) => {
      for stmt in &static_block.body.stmts {
        recurse_and_find_sql(sqls, stmt, import_alias)?;
      }
    }
    ClassMember::PrivateProp(private_prop) => {
      let name = &private_prop.key;
      let name = name.clone().name.to_string();
      if let Some(expr) = &private_prop.value {
        let span: MultiSpan = private_prop.span.into();
        get_sql_from_expr(sqls, &Some(name), &expr.clone(), &span, import_alias);
      }
    }
    ClassMember::ClassProp(class_prop) => {
      let name = &class_prop.key;
      let name = name.clone().ident().map(|x| x.to_string());
      if let Some(expr) = &class_prop.value {
        let span: MultiSpan = class_prop.span.into();
        get_sql_from_expr(sqls, &name, &expr.clone(), &span, import_alias);
      }
    }
    ClassMember::AutoAccessor(auto_accessor) => {
      let value = &auto_accessor.value;
      let key = &auto_accessor.key;

      if let Some(expr) = &value {
        let span: MultiSpan = auto_accessor.span.into();
        let var_decl_name = get_var_decl_name_from_key(key);
        get_sql_from_expr(sqls, &var_decl_name, expr, &span, import_alias);
      }
    }
    ClassMember::TsIndexSignature(_) => {}
    ClassMember::Empty(_) => {}
  }
  Ok(())
}

pub fn process_default_decl(sqls: &mut Vec<SQL>, default_decl: &DefaultDecl, import_alias: &String) -> Result<()> {
  match default_decl {
    DefaultDecl::Class(class) => {
      let class_body = &class.class.body;
      for body_stmt in class_body {
        process_class_member(sqls, body_stmt, import_alias)?;
      }
    }
    DefaultDecl::Fn(func) => {
      let body = &func.function.body;

      if let Some(body) = body {
        for stmt in &body.stmts {
          recurse_and_find_sql(sqls, stmt, import_alias)?;
        }
      }
    }
    DefaultDecl::TsInterfaceDecl(_) => {}
  }
  Ok(())
}

pub fn process_class_decl(sqls: &mut Vec<SQL>, class: &ClassDecl, import_alias: &String) -> Result<()> {
  let class_body = &class.class.body;
  let class_decorators = &class.class.decorators;

  for decorator in class_decorators {
    let expr = &decorator.expr;
    let span: MultiSpan = decorator.span.into();
    get_sql_from_expr(sqls, &None, expr, &span, import_alias);
  }

  for body_stmt in class_body {
    process_class_member(sqls, body_stmt, import_alias)?;
  }
  Ok(())
}

pub fn process_decl(sqls: &mut Vec<SQL>, decl: &Decl, import_alias: &String) -> Result<()> {
  match decl {
    Decl::Class(class) => {
      process_class_decl(sqls, class, import_alias)?;
    }
    Decl::Fn(fun) => {
      if let Some(body) = &fun.function.body {
        for stmt in &body.stmts {
          recurse_and_find_sql(sqls, stmt, import_alias)?;
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
          get_sql_from_expr(sqls, &name, &expr, &span, import_alias);
        }
      }
    }
    Decl::TsInterface(_) => {}
    Decl::TsTypeAlias(_) => {}
    Decl::TsEnum(_) => {}
    Decl::TsModule(module) => {
      while let Some(stmt) = &module.body {
        while let Some(block) = &stmt.as_ts_module_block() {
          for body in &block.body {
            let stmt = &body.clone().stmt();
            if let Some(stmt) = stmt {
              recurse_and_find_sql(sqls, stmt, import_alias)?;
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
