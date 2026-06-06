use anyhow::anyhow;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;

pub trait Exec {
    fn exec(&self, command: Command, stdin: Vec<u8>) -> anyhow::Result<CommandOutput>;
}

pub struct SystemExec;

impl Exec for SystemExec {
    fn exec(&self, mut command: Command, stdin: Vec<u8>) -> anyhow::Result<CommandOutput> {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn command [{command:?}]: {e}"))?;

        let mut child_stdin = child
            .stdin
            .take()
            .ok_or(anyhow!("Failed to take stdin handle"))?;

        thread::spawn(move || {
            let _ = child_stdin.write_all(&stdin);
        });

        match child.wait_with_output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(CommandOutput::Stdout(output.stdout))
                } else {
                    Ok(CommandOutput::Stderr(output.stderr, output.status.code()))
                }
            }
            Err(e) => Err(anyhow!("Failed to execute command '{command:?}': {e}")),
        }
    }
}

pub enum CommandOutput {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>, Option<i32>),
}

#[cfg(test)]
pub struct MockExec {
    pub calls: std::rc::Rc<std::cell::RefCell<Vec<(String, String)>>>,
}

#[cfg(test)]
impl Exec for MockExec {
    fn exec(&self, command: Command, stdin: Vec<u8>) -> anyhow::Result<CommandOutput> {
        use itertools::Itertools;
        let program = command.get_program().to_string_lossy().into_owned();
        self.calls.borrow_mut().push((
            program.clone(),
            String::from_utf8_lossy(stdin.as_slice()).into(),
        ));
        if program.ends_with("err") {
            Ok(CommandOutput::Stderr(
                format!("{}-output", program).bytes().collect_vec(),
                Some(1),
            ))
        } else {
            Ok(CommandOutput::Stdout(
                format!("{}-output", program).bytes().collect_vec(),
            ))
        }
    }
}
