use std::process::Command;
use std::str;

pub fn get_hostname() -> Result<String, String> {
    let status = Command::new("sudo")
        .arg("hostnamectl")
        .arg("status")
        .output();

        match status {
            Ok(v) => {
                let hostname_str = str::from_utf8(&v.stdout).unwrap();
                Ok(hostname_str.to_string())
            },
            Err(e) => Err(format!("failed to get hostnamea: {}", e))
        }
}