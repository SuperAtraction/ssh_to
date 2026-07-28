use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct Machine {
    pub name: String,
    pub user: String,
    pub ip: String,
}

impl Machine {
    pub fn connect(&self) -> Result<i32, std::io::Error> {
        let status = Command::new("ssh").arg(format!("{}@{}", self.user, self.ip)).status()?;

        Ok(status.code().unwrap_or(-1))
    }
}
