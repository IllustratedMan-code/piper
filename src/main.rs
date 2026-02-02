#![warn(clippy::unwrap_used)]

mod config;
mod debug_utils;
mod process;
mod vm;
use crate::debug_utils::Runner;
use clap::Parser;
use std::fs;
use steel_repl::colored::Colorize;
use vm::engine;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    repl: bool,

    #[arg(short, long, default_value = ".piperConfig.scm")]
    config: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut engine = engine(Some(args.config));
    engine
        .run(include_str!("steel-modules/main.scm"))
        .expect("Couldn't run main!");
    if args.repl {
        let repl = steel_repl::Repl::new(engine)
            .with_startup(":? for help".bright_yellow().bold());
        repl.run().expect("couldn't load repl");
        //steel_repl::repl::repl::newrun_repl(engine).expect("Couldn't run repl!");
    }
}
