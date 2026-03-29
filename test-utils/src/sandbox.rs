use std::path::PathBuf;

use serde;
use serde::{Deserialize, Serialize};

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
    pub file_extension: String,
    pub db_host: String,
    pub db_port: i32,
    pub db_user: String,
    pub db_pass: Option<String>,
    pub db_name: String,
    pub generate_path: Option<PathBuf>,
    pub generate_types: bool,
    pub config_file_name: Option<String>,
}

impl TestConfig {
    pub fn new(db_type: &str, generate_types: bool, generate_path: Option<PathBuf>, config_file_name: Option<String>) -> Self {
        let generate_path = generate_path.clone();
        println!("checking db_type: {db_type}");
        if db_type == "mysql" {
            return TestConfig {
                db_type: "mysql".into(),
                file_extension: "ts".to_string(),
                db_host: "127.0.0.1".to_string(),
                db_port: 33306,
                db_user: "root".to_string(),
                db_pass: None,
                db_name: "sqlx-ts".to_string(),
                generate_path,
                generate_types,
                config_file_name,
            }
        }
        if db_type == "sqlite" {
            return TestConfig {
                db_type: "sqlite".into(),
                file_extension: "ts".to_string(),
                db_host: String::new(),
                db_port: 0,
                db_user: String::new(),
                db_pass: None,
                // db_name will be overridden per-test with the actual temp SQLite file path
                db_name: ":memory:".to_string(),
                generate_path,
                generate_types,
                config_file_name,
            }
        }
        TestConfig {
            db_type: "postgres".into(),
            file_extension: "ts".to_string(),
            db_host: "127.0.0.1".to_string(),
            db_port: 54321,
            db_user: "postgres".to_string(),
            db_pass: Some("postgres".to_string()),
            db_name: "postgres".to_string(),
            generate_path,
            generate_types,
            config_file_name,
        }
    }

    pub fn set_db_type(&mut self, db_type: String) -> Self {
        self.db_type = db_type;
        self.clone()
    }

    pub fn set_file_extension(&mut self, file_extension: String) -> Self {
        self.file_extension = file_extension;
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

/// Checks if the MySQL server at the given host:port is above the specified major.minor version.
/// Uses `docker exec` to query the version. Returns true if version cannot be determined.
pub fn is_mysql_version_above(db_host: &str, db_port: i32, major: u32, minor: u32) -> bool {
  let output = std::process::Command::new("docker")
    .args(["exec", "sqlx-ts-mysql-1", "mysql", "-u", "root", "-N", "-e", "SELECT VERSION();"])
    .output();
  match output {
    Ok(out) => {
      let version = String::from_utf8_lossy(&out.stdout);
      let version = version.trim();
      let parts: Vec<&str> = version.split('.').collect();
      if parts.len() >= 2 {
        let srv_major: u32 = parts[0].parse().unwrap_or(0);
        let srv_minor: u32 = parts[1].parse().unwrap_or(0);
        srv_major > major || (srv_major == major && srv_minor > minor)
      } else {
        true
      }
    }
    Err(_) => true,
  }
}

#[macro_export]
macro_rules! run_test {
// Arm with minimum MySQL version requirement: (major, minor)
($($name: ident, $test_config: expr, $ts_content: expr, $generated_types: expr, min_mysql: ($maj:expr, $min:expr))*) => {
$(
    #[test]
    fn $name() -> Result<(), Box<dyn std::error::Error>> {
      use assert_cmd::cargo::cargo_bin_cmd;
      let ts_content = $ts_content;
      let test_config: TestConfig = $test_config;

      // Check minimum MySQL version requirement
      if test_config.db_type == "mysql" {
        if !test_utils::sandbox::is_mysql_version_above(&test_config.db_host, test_config.db_port, $maj, $min) {
          eprintln!("Skipping test {}: requires MySQL > {}.{}", stringify!($name), $maj, $min);
          return Ok(());
        }
      }

      println!("checking test config {:?}", test_config);
      let file_extension = test_config.file_extension;
      let db_type = test_config.db_type;
      let db_host = test_config.db_host;
      let db_port = test_config.db_port;
      let db_user = test_config.db_user;
      let db_pass = test_config.db_pass;
      let db_name = test_config.db_name;
      let config_file_name = test_config.config_file_name;
      let generate_path = test_config.generate_path;
      let is_sqlite = db_type == "sqlite";

      // SETUP
      let dir = tempdir()?;
      let parent_path = dir.path();
      let file_path = parent_path.join(format!("index.{file_extension}"));

      let mut temp_file = fs::File::create(&file_path)?;
      writeln!(temp_file, "{}", ts_content)?;
      let file_result = fs::read_to_string(&file_path)?;

      // EXECUTE
      let mut cmd = cargo_bin_cmd!("sqlx-ts");

      cmd.arg(parent_path.to_str().unwrap())
          .arg(format!("--ext={file_extension}"))
          .arg(format!("--db-type={db_type}"))
          .arg(format!("--db-name={db_name}"));

      if !is_sqlite {
        cmd.arg(format!("--db-host={db_host}"))
            .arg(format!("--db-port={db_port}"))
            .arg(format!("--db-user={db_user}"));
      }

      if &generate_path.is_some() == &true {
        let generate_path = generate_path.clone();
        let generate_path = generate_path.unwrap();
        let generate_path = generate_path.as_path();
        let generate_path = parent_path.join(generate_path);
        let generate_path = generate_path.display();
        cmd.arg(format!("--generate-path={generate_path}"));
      }

      if (test_config.generate_types) {
        cmd.arg("-g");
      }

      if (config_file_name.is_some()) {
        let cwd = env::current_dir()?;
        let config_file_name = format!("{}", config_file_name.unwrap());
        let config_path = cwd.join(format!("tests/configs/{config_file_name}"));
        let config_path = config_path.display();
        cmd.arg(format!("--config={config_path}"));
      }

      if !is_sqlite {
        if (db_pass.is_some()) {
          let db_pass = db_pass.unwrap();
          cmd.arg(format!("--db-pass={db_pass}"));
        } else {
          cmd.arg("--db-pass=");
        }
      }

      cmd.assert()
         .success()
         .stdout(predicates::str::contains("No SQL errors detected!"));

      let generated_types: &str = $generated_types.clone();

      if generate_path.is_some() {
        let generate_path = parent_path.join(generate_path.unwrap().as_path());
        let type_file = fs::read_to_string(generate_path);
        let type_file = type_file.unwrap();

        assert_eq!(
            generated_types.trim().to_string().flatten(),
            type_file.trim().to_string().flatten()
        );
        return Ok(());
      }

      let type_file = fs::read_to_string(parent_path.join("index.queries.ts"));
      if type_file.is_ok() {
        let type_file = type_file.unwrap().clone();
        let type_file = type_file.trim();
        assert_eq!(
            generated_types.trim().to_string().flatten(),
            type_file.to_string().flatten()
        );
      }
      Ok(())
    }
)*};

// Original arm without version requirement
($($name: ident, $test_config: expr, $ts_content: expr, $generated_types: expr)*) => {
$(
// MACRO STARTS

    #[test]
    fn $name() -> Result<(), Box<dyn std::error::Error>> {
      use assert_cmd::cargo::cargo_bin_cmd;
      let ts_content = $ts_content;
      let test_config: TestConfig = $test_config;
      println!("checking test config {:?}", test_config);
      let file_extension = test_config.file_extension;
      let db_type = test_config.db_type;
      let db_host = test_config.db_host;
      let db_port = test_config.db_port;
      let db_user = test_config.db_user;
      let db_pass = test_config.db_pass;
      let db_name = test_config.db_name;
      let config_file_name = test_config.config_file_name;
      let generate_path = test_config.generate_path;
      let is_sqlite = db_type == "sqlite";

      // SETUP
      let dir = tempdir()?;
      let parent_path = dir.path();
      let file_path = parent_path.join(format!("index.{file_extension}"));

      let mut temp_file = fs::File::create(&file_path)?;
      writeln!(temp_file, "{}", ts_content)?;
      let file_result = fs::read_to_string(&file_path)?;

      // EXECUTE
      let mut cmd = cargo_bin_cmd!("sqlx-ts");

      cmd.arg(parent_path.to_str().unwrap())
          .arg(format!("--ext={file_extension}"))
          .arg(format!("--db-type={db_type}"))
          .arg(format!("--db-name={db_name}"));

      if !is_sqlite {
        cmd.arg(format!("--db-host={db_host}"))
            .arg(format!("--db-port={db_port}"))
            .arg(format!("--db-user={db_user}"));
      }

      if &generate_path.is_some() == &true {
        let generate_path = generate_path.clone();
        let generate_path = generate_path.unwrap();
        let generate_path = generate_path.as_path();
        let generate_path = parent_path.join(generate_path);
        let generate_path = generate_path.display();
        cmd.arg(format!("--generate-path={generate_path}"));
      }

      if (test_config.generate_types) {
        cmd.arg("-g");
      }

      if (config_file_name.is_some()) {
        let cwd = env::current_dir()?;
        let config_file_name = format!("{}", config_file_name.unwrap());
        let config_path = cwd.join(format!("tests/configs/{config_file_name}"));
        let config_path = config_path.display();
        cmd.arg(format!("--config={config_path}"));
      }

      if !is_sqlite {
        if (db_pass.is_some()) {
          let db_pass = db_pass.unwrap();
          cmd.arg(format!("--db-pass={db_pass}"));
        } else {
          cmd.arg("--db-pass=");
        }
      }

      cmd.assert()
         .success()
         .stdout(predicates::str::contains("No SQL errors detected!"));

      let generated_types: &str = $generated_types.clone();

      if generate_path.is_some() {
        let generate_path = parent_path.join(generate_path.unwrap().as_path());
        let type_file = fs::read_to_string(generate_path);
        let type_file = type_file.unwrap();

        assert_eq!(
            generated_types.trim().to_string().flatten(),
            type_file.trim().to_string().flatten()
        );
        return Ok(());
      }

      let type_file = fs::read_to_string(parent_path.join("index.queries.ts"));
      if type_file.is_ok() {
        let type_file = type_file.unwrap().clone();
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
