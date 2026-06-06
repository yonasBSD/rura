use std::process::Command;

pub trait CommandBuilder {
    fn build(&self, command: &str) -> Command;
}

pub struct UsrBinEnvCommandBuilder {
    pub shell: String,
}

impl CommandBuilder for UsrBinEnvCommandBuilder {
    fn build(&self, command: &str) -> Command {
        let mut cmd = Command::new("/usr/bin/env");
        cmd.args([&self.shell, "-c", command]);
        cmd
    }
}

#[cfg(windows)]
pub struct PwshCommandBuilder {
    pub shell: String,
}

#[cfg(windows)]
impl CommandBuilder for PwshCommandBuilder {
    fn build(&self, command: &str) -> Command {
        let mut cmd = Command::new(&self.shell);
        cmd.env("NO_COLOR", "1");
        cmd.arg("-NonInteractive");
        cmd.arg("-NoProfile");
        cmd.arg("-NoLogo");
        cmd.args(["/C", &command]);
        cmd
    }
}

#[cfg(test)]
pub struct TestBuilder;

#[cfg(test)]
impl CommandBuilder for TestBuilder {
    fn build(&self, command: &str) -> Command {
        Command::new(command)
    }
}
