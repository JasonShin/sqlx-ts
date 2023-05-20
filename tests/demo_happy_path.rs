#[cfg(test)]
mod demo_happy_path_tests {
    use assert_cmd::prelude::*;
    use std::process::Command;
    use std::env::current_dir;

    #[test]
    fn all_demo_should_pass() -> Result<(), Box<dyn std::error::Error>> {
        // SETUP
        let root_path = current_dir().unwrap();
        let demo_path = root_path.join("tests/demo");

        // EXECUTE
        let mut cmd = Command::cargo_bin("sqlx-ts").unwrap();
        cmd.arg(demo_path.to_str().unwrap())
          .arg(format!("--ext=ts"))
          .arg("--config=.sqlxrc.sample.json")
          .arg("-g");

        // ASSERT
      cmd.assert()
        .success()
        .stdout(predicates::str::contains("No SQL errors detected!"));

      Ok(())
    }
}