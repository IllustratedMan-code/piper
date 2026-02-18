#![warn(clippy::unwrap_used)]

mod bindings;
mod config;
mod debug_utils;
mod process;
mod vm;

use crate::debug_utils::Runner;
use clap::Parser;
use steel_repl::colored::Colorize;
use vm::engine;
mod derivation_runner;

/// The command line interface for piper
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// --repl is a boolean flag that prevents the pipeline from being
    /// run and loads the user into an interactive repl with access to the
    /// constructed pipeline
    #[arg(short, long)]
    repl: bool,

    /// --config is the path to the piper config, a scheme file
    #[arg(short, long, default_value = ".piperConfig.scm")]
    config: std::path::PathBuf,
}

/// The entrypoint function for piper
fn main() {
    let args = Cli::parse();
    let mut engine = engine(Some(args.config));
    engine
        .run_builtin_or_print_error(
            include_str!("steel-modules/main.scm"),
            "builtin/main.scm",
        )
        .expect("Couldn't run main!");
    if args.repl {
        let repl = steel_repl::Repl::new(engine)
            .with_startup(":? for help".bright_yellow().bold());
        repl.run().expect("couldn't load repl");
        //steel_repl::repl::repl::newrun_repl(engine).expect("Couldn't run repl!");
    } else {
        let runner = derivation_runner::DerivationRunner {
            graph: process::extract_graph(&mut engine),
        };
        runner.run_derivations();
    }
}
