 // Run programs
mod tests {
    use anyhow::Ok;
    use assert_cmd::Command;

    use predicates::prelude::predicate;

   

   

    #[test]
    fn test_invalid_input()  -> Result<(), anyhow::Error> {
        let mut cmd = Command::cargo_bin("rust_lambert_w").unwrap(); 
        cmd.arg("--x=-1");
        cmd.assert()
        .success()
        .stdout(predicate::str::contains("Lambert W(-1) = Err(Input must be greater than or equal to -1/e)"));
    Ok(())
    }
    #[test]
    fn test_valid_input()  -> Result<(), anyhow::Error>{
        let mut cmd = Command::cargo_bin("rust_lambert_w").unwrap();
        cmd.arg("--x=-0.3");
        cmd.assert()
        .success()
        .stdout(predicate::str::contains("Lambert W(-0.3) = Ok(-0.48940222718021487)"));
          Ok(())
    }


    #[test]
    fn test_output()  -> Result<(), anyhow::Error> {
        let mut cmd = Command::cargo_bin("rust_lambert_w")?; 
        cmd.arg("--x").arg("1.0");
        cmd.assert()
        .success()
        .stdout(predicate::str::contains("Lambert W(1) = Ok(0.5671432904097838)"));
 Ok(())

    }
}
