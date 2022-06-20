use std::env;
use std::process;

#[derive(Debug)]
enum Op {
    Push(i64),
    Plus,
    Minus,
    Dump,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rorth <SUBCOMMAND> [ARGS]");
        println!("SUBCOMMANDS:");
        println!("    sim        Simulate the program");
        println!("    com        Compile the program");
        process::exit(1);
    }

    let sub_cmd = args.remove(1);
    match sub_cmd.as_str() {
        "sim" => demo(),
        "com" => todo!(),
        _ => panic!("unkown subcommand: {}", sub_cmd),
    }
}

fn demo() {
    let mut program: Vec<Op> = Vec::new();
    program.push(Op::Push(1));
    program.push(Op::Push(41));
    program.push(Op::Plus);
    program.push(Op::Push(2));
    program.push(Op::Minus);
    program.push(Op::Dump);
    simulate(&program);
}

fn simulate(program: &Vec<Op>) {
    use Op::*;

    let mut stack: Vec<i64> = Vec::new();
    for op in program {
        match *op {
            Push(x) => stack.push(x),
            Plus => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y + x);
            },
            Minus => {
                let x = stack.pop().unwrap();
                if let Some(y) = stack.pop() {
                    stack.push(y - x);
                } else {
                    stack.push(-x);
                }
            },

            Dump => {
                println!("{:?}", stack.remove(0));
            },
        };

        println!("{:?}", stack);
    }
}

fn compile(program: &Vec<Op>) {
}
