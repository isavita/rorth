use std::{env, process};

#[derive(Debug)]
enum Op {
    Push(i64),
    Plus,
    Minus,
    Dump,
}

fn main() {
    let output = process::Command::new("echo")
                     .arg("\"Hello!!!\"")
                     .output()
                     .expect("failed to execute process");
    /*
    println!("{:?}", output);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("__END__");
    */

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
        "com" => compile(&vec![Op::Push(1)]),
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
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("output.asm").unwrap();
    writeln!(file, ".global _main").unwrap();
    writeln!(file, ".align 4\n").unwrap();
    writeln!(file, "_main:").unwrap();
    for op in program {
        match *op {
            Push(x) => {
                writeln!(file, "  mov X0, #0").unwrap();
                writeln!(file, "  mov X16, #1").unwrap();
                writeln!(file, "  svc 0").unwrap();
            },
            Plus => {
            },
            Minus => {
            },

            Dump => {
            },
        }
    }
   drop(file);

    build_assembly();
}

fn build_assembly() {
    use std::str;

    process::Command::new("as")
        .args(["output.asm", "-o", "output.o"])
        .output()
        .expect("failed to execute process");

    let mac_sdk_path = process::Command::new("xcrun")
        .args(["-sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("failed to execute process").stdout;
    let mac_sdk_path = str::from_utf8(&mac_sdk_path).unwrap().trim();

    process::Command::new("ld")
        .args(["output.o", "-o", "output", "-lSystem", "-syslibroot", mac_sdk_path])
        .output()
        .expect("failed to execute process");

}
