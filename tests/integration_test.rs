#[cfg(test)]
mod tests {
    #[test]
    fn integration_test() {
        let output = std::process::Command::new("cargo")
            .args(["run", "--", "--category", "pokemon", "--key", "1"])
            .output()
            .expect("Failed to run command");
        assert!(output.status.success());
    }
}
