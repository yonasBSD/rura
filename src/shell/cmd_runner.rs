use crate::rura::RuraCommand;
use crate::shell::cached_runner::CachedCmdRunner;
use crate::shell::output::Output;
use crate::shell::split_runner::SplitCmdRunner;
use anyhow::Result;
use itertools::Itertools;
use std::sync::Arc;

pub trait CmdRunner {
    fn run(&self, command: &RuraCommand) -> Result<CmdResult>;
}

pub struct CmdRunners;
impl CmdRunners {
    #[cfg(unix)]
    pub fn new(shell: &str, stdin: Arc<[u8]>, no_cache: bool) -> Box<dyn CmdRunner> {
        if no_cache {
            Box::new(SplitCmdRunner::new(shell, stdin))
        } else {
            Box::new(CachedCmdRunner::new(shell, stdin))
        }
    }

    #[cfg(windows)]
    pub fn new(shell: &str, stdin: Arc<[u8]>, _no_cache: bool) -> Box<dyn CmdRunner> {
        use crate::shell::simple_runner::SimpleCmdRunner;

        Box::new(SimpleCmdRunner::new(shell, stdin))
    }
}

#[derive(Clone, Debug)]
pub struct CmdResult {
    pub stdin: Arc<[u8]>,
    pub outputs: Vec<Output>,
}

impl CmdResult {
    pub fn ok_bytes(&self) -> Vec<Arc<[u8]>> {
        self.outputs
            .clone()
            .into_iter()
            .map_while(|output| match output {
                Output::Ok(bytes) => Some(bytes),
                Output::Err(_, _) => None,
            })
            .collect_vec()
    }

    // todo if there's any Err output then it's always the last one
    pub fn failed_subcommand(&self) -> Option<usize> {
        self.outputs
            .iter()
            .find_position(|output| match output {
                Output::Err { .. } => true,
                _ => false,
            })
            .map(|(index, _)| index)
    }
}
