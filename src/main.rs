mod process;
mod vm;
use vm::engine;

fn main() {
    let engine = engine();

    steel_repl::run_repl(engine).unwrap();

}
