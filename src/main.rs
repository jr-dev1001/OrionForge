mod errors;
mod vm;
mod loader;
use errors::VMResult;
use vm::VM;
use vm::instruction::Instruction as I;
fn main() -> VMResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let program = if args.len() > 1 {
        loader::load_program_from_json(&args[1])?
    } else {
        vec![I::PUSH(5), I::PUSH(10), I::ADD, I::PRINT, I::HALT]
    };

    VM::with_gas_limit(program, 10_000_000).run()
}
