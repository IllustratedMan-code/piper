use std::{
    borrow::BorrowMut,
    process::{Command, ExitStatus},
};

use enum_dispatch::enum_dispatch;
pub fn run_derivation(
    derivation: &super::Derivation,
    work_dir: String,
    edge_hashes: Vec<String>,
) -> HPCRuntime {
    let mut cmd = derivation.script.to_string();
    let c_r = NoContainerRuntime::new();
    let mut hpc_r = NoHPCRuntime::new();
    cmd = c_r.cmd(cmd);
    hpc_r.submit_job(cmd, work_dir);
    HPCRuntime::from(hpc_r)
}

#[enum_dispatch(HPCRuntimeFunctions)]
pub enum HPCRuntime {
    NoHPCRuntime,
}



#[enum_dispatch]
pub trait HPCRuntimeFunctions {
    fn submit_job(&mut self, cmd: String, work_dir: String);
    fn cmd(&self, cmd: String) -> String;
    fn wait(&mut self) -> Option<ExitStatus>;
    fn finished(&mut self) -> bool;
}

pub struct NoHPCRuntime {
    childprocess: Option<std::process::Child>,
}


impl NoHPCRuntime {
    fn new() -> Self {
        Self { childprocess: None }
    }
}

impl HPCRuntimeFunctions for NoHPCRuntime {

    fn submit_job(&mut self, cmd: String, work_dir: String) {
        let cmd = self.cmd(cmd);
        let mut child = Command::new(cmd);
        self.childprocess = Some(
            child
                .current_dir(work_dir)
                .spawn()
                .expect("failed to start job"),
        );
    }
    fn cmd(&self, cmd: String) -> String {
        cmd
    }
    fn wait(&mut self) -> Option<ExitStatus> {
        Some(
            self.childprocess
                .take()?
                .wait()
                .expect("failed to wait for job"),
        )
    }
    fn finished(&mut self) -> bool {
        if let Some(c) = self.childprocess.borrow_mut() {
            match c.try_wait() {
                Ok(Some(status)) => true,
                Ok(None) => false,
                Err(e) => panic!("failed to check job status"),
            }
        } else {
            false // hasn't started yet
        }
    }
}

pub struct LsfHPCRuntime {}

pub enum ContainerRuntime {
    None(NoContainerRuntime),
}

pub trait ContainerRuntimeFunctions {
    fn cmd(&self, cmd: String) -> String;
    fn new() -> Self;
}

pub struct NoContainerRuntime {}

impl ContainerRuntimeFunctions for NoContainerRuntime {
    fn cmd(&self, cmd: String) -> String {
        cmd
    }
    fn new() -> Self {
        Self {}
    }
}
