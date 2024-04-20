use serde;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SqlxConfigFile {
    pub generate_types: Option<GenerateTypesConfig>,
    pub connections: HashMap<String, DbConnectionConfig>
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenerateTypesConfig {
    pub enabled: bool,
    pub convert_to_camel_case_column_names: bool,
    pub generate_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    Postgres,
    Mysql,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbConnectionConfig {
    pub db_type: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: Option<String>,
    pub db_name: Option<String>,
    pub pg_search_path: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TestConfig {
    pub db_type: String,
    pub js_extension: String,
    pub db_host: String,
    pub db_port: i32,
    pub db_user: String,
    pub db_pass: Option<String>,
    pub db_name: String,
    pub config_file: Option<SqlxConfig>,
}

impl TestConfig {
    pub fn new(db_type: &str, config_file: Option<SqlxConfig>) -> Self {
        if db_type == "mysql" {
            return TestConfig {
                db_type: "mysql".into(),
                js_extension: "ts".to_string(),
                db_host: "127.0.0.1".to_string(),
                db_port: 33306,
                db_user: "root".to_string(),
                db_pass: None,
                db_name: "sqlx-ts".to_string(),
                config_file,
            }
        }
        TestConfig {
            db_type: "postgres".into(),
            js_extension: "ts".to_string(),
            db_host: "127.0.0.1".to_string(),
            db_port: 54321,
            db_user: "postgres".to_string(),
            db_pass: Some("postgres".to_string()),
            db_name: "postgres".to_string(),
            config_file,
        }
    }

    pub fn set_db_type(&mut self, db_type: String) -> Self {
        self.db_type = db_type;
        self.clone()
    }

    pub fn set_js_extension(&mut self, js_extension: String) -> Self {
        self.js_extension = js_extension;
        self.clone()
    }

    pub fn set_db_host(&mut self, db_host: String) -> Self {
        self.db_host = db_host;
        self.clone()
    }

    pub fn set_db_port(&mut self, db_port: i32) -> Self {
        self.db_port = db_port;
        self.clone()
    }

    pub fn set_db_user(&mut self, db_user: String) -> Self {
        self.db_user = db_user;
        self.clone()
    }
    
    pub fn set_db_pass(&mut self, db_pass: String) -> Self {
        self.db_pass = Some(db_pass);
        self.clone()
    }

    pub fn set_db_name(&mut self, db_name: String) -> Self {
        self.db_name = db_name;
        self.clone()
    }
}

#[macro_export]
macro_rules! run_test {
($($name: ident, $test_config: expr, $ts_content: expr, $generated_types: expr)*) => {
$(
// MACRO STARTS

    #[test]
    fn $name() -> Result<(), Box<dyn std::error::Error>> {
      let ts_content = $ts_content;
      let test_config: TestConfig = $test_config;
      let js_extension = test_config.js_extension;
      let db_type = test_config.db_type;
      let db_host = test_config.db_host;
      let db_port = test_config.db_port;
      let db_user = test_config.db_user;
      let db_pass = test_config.db_pass;
      let db_name = test_config.db_name;
      
      // SETUP
      let dir = tempdir()?;
      let parent_path = dir.path();
      let file_path = parent_path.join(format!("index.{js_extension}"));

      let mut temp_file = fs::File::create(&file_path)?;
      writeln!(temp_file, "{}", ts_content)?;
      let file_result = fs::read_to_string(&file_path)?;
      
      // EXECUTE
      let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();

      cmd.arg(parent_path.to_str().unwrap())
          .arg(format!("--ext={js_extension}"))
          .arg(format!("--db-type={db_type}"))
          .arg(format!("--db-host={db_host}"))
          .arg(format!("--db-port={db_port}"))
          .arg(format!("--db-user={db_user}"))
          .arg(format!("--db-name={db_name}"))
          .arg("-g");

      if (db_pass.is_some()) {
        let db_pass = db_pass.unwrap();
        cmd.arg(format!("--db-pass={db_pass}"));
      }

      if &test_config.config_file.is_some() == &true {
        let config_file = test_config.config_file.unwrap();
        let config_file_path = parent_path.join("sqlx-ts.json");
        let mut config_file = fs::File::create(&config_file_path)?;
        let config_file_content = serde_json::to_string(&config_file)?;
        writeln!(config_file, "{}", config_file_content)?;
        cmd.arg(format!("--config={config_file_path}"));
      }

      cmd.assert()
         .success()
         .stdout(predicates::str::contains("No SQL errors detected!"));

      let generated_types: &str = $generated_types;

      if generated_types != "" {
        let type_file = fs::read_to_string(parent_path.join("index.queries.ts"))?;
        let type_file = type_file.trim();
        assert_eq!(
            generated_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
      }
      Ok(())
    }

// MACRO ENDS
)*};}