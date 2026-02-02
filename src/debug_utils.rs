use std::io::Write;
use std::{fs, path::PathBuf};
use steel::steel_vm::engine::Engine;

pub trait Runner {
    fn run_or_print_error(&mut self, path: PathBuf) -> std::io::Result<()>;
}

impl Runner for Engine {
    fn run_or_print_error(&mut self, path: PathBuf) -> std::io::Result<()>{
        let file_contents = fs::read_to_string(path.clone())?;
                
        let res =
            self.compile_and_run_raw_program_with_path(file_contents, path);
        match res {
            Ok(_) => (),
            Err(e) => {
                self.raise_error(e);
            }
        };
        let _ = std::io::stdout().flush();

        Ok(())
    }
}
