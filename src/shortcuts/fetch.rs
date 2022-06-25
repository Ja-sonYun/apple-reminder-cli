use std::process::Command;

pub fn fetch_todos() -> Result<String, String> {
    let raw_output = Command::new("shortcuts")
        .arg("run")
        .arg("GetTodos")
        .output();

    match raw_output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(error) => Err(error.to_string()),
    }
}
