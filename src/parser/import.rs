use swc_ecma_ast::{ImportDecl, ImportSpecifier, ModuleExportName};

pub fn find_sqlx_import_alias(import_decl: &ImportDecl) -> Option<String> {
  let mut name: Option<String> = None;
  let src = import_decl.src.value.to_string_lossy().to_string();

  if src == "sqlx-ts" {
    for specifier in &import_decl.specifiers {
      if let ImportSpecifier::Named(import_named_specifier) = specifier {
        // for exampleL
        // import { sql: aliased } from 'sqlx-ts' <<< should satisfy following
        if let Some(imported) = &import_named_specifier.imported {
          match imported {
            ModuleExportName::Ident(ident) => {
              if ident.sym == "sql" {
                name = Some(import_named_specifier.local.sym.to_string())
              }
            }
            _ => continue,
          }
          // for example:
          // import { sql } from 'sqlx-ts' <<< should satisfy following
        } else if import_named_specifier.local.sym == "sql" {
          name = Some("sql".to_string())
        }
      }
    }
  }

  name
}
