//use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
//use std::process::Command; // Run programs
//use cargo_binutils::Command;
mod tests {
    use std::process::Command;

    use predicates::prelude::predicate;

    #[test]
    fn test_invalid_input() {
        let mut cmd = Command::cargo_bin("grrs").unwrap(); 
        cmd.arg("-1.0").arg("/target/debug/rust-lambert_w").arg("--x");
        cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Input must be greater than or equal to -1/e"));
    }
    fn test_valid_input() {
        let mut cmd = Command::cargo_bin("grrs").unwrap();
        cmd.arg("-0.3").arg("/target/debug/rust-lambert_w").arg("--x=");
        cmd.assert()
        .success()
    }

    #[test]
    fn test_outpu() {
        let mut cmd = Command::cargo_bin("grrs").uwrap(); 
        cmd.arg("1.0").arg("/target/debug/rust-lambert_w").arg("--x");
        cmd.assert()
        .failure()
        .stdout(predicate::str::contains("lambert W(1.0)"));

    }
}
