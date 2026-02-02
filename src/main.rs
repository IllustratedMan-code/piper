#![warn(clippy::unwrap_used)]

mod process;
mod vm;
mod config;
mod debug_utils;
use vm::engine;
use clap::Parser;
use std::fs;
use steel_repl::colored::{ColoredString, Colorize};
use crate::debug_utils::Runner;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    repl: bool,

    #[arg(short, long, default_value=".piperConfig.scm")]
    config: std::path::PathBuf,
}



fn main() {
    let args = Cli::parse();
    let config_string = fs::read_to_string(args.config).unwrap_or("".to_string());
    let mut engine = engine(config_string);
    engine
        .run(include_str!("steel-modules/main.scm"))
        .expect("Couldn't run main!");
    engine.run_or_print_error(std::path::PathBuf::from("examples/hello_pipeline/main.scm"));
    if args.repl{
        
        let repl = steel_repl::Repl::new(engine).with_startup(":? for help".bright_yellow().bold());
//https://github.com/mattwparas/steel/blob/b77360e462bd43992a497ab93ee081455cd61fd9/crates/steel-repl/src/repl.rs#L77 for examples on error handling
        repl.run().expect("couldn't load repl");
        //steel_repl::repl::repl::newrun_repl(engine).expect("Couldn't run repl!");
    }
}
