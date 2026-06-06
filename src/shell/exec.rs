use anyhow::anyhow;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use crate::shell::output::Output;

pub trait Exec {
    fn exec(&self, command: Command, stdin: Vec<u8>) -> anyhow::Result<Output>;
}

pub struct SystemExec;

impl Exec for SystemExec {
    fn exec(&self, mut command: Command, stdin: Vec<u8>) -> anyhow::Result<Output> {
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
                    Ok(Output::Ok(output.stdout))
                } else {
                    // failed successfully!
                    Ok(Output::Err(output.stderr, output.status.code()))
                }
            }
            Err(e) => Err(anyhow!("Failed to execute command '{command:?}': {e}")),
        }
    }
}

#[cfg(test)]
pub struct MockExec {
    pub calls: std::rc::Rc<std::cell::RefCell<Vec<(String, String)>>>,
}

#[cfg(test)]
impl Exec for MockExec {
    fn exec(&self, command: Command, stdin: Vec<u8>) -> anyhow::Result<Output> {
        use itertools::Itertools;
        let program = command.get_program().to_string_lossy().into_owned();
        self.calls.borrow_mut().push((
            program.clone(),
            String::from_utf8_lossy(stdin.as_slice()).into(),
        ));
        if program.ends_with("err") {
            Ok(Output::Err(
                format!("{}-output", program).bytes().collect_vec(),
                Some(1),
            ))
        } else {
            Ok(Output::Ok(
                format!("{}-output", program).bytes().collect_vec(),
            ))
        }
    }
}
