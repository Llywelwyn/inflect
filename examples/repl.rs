use std::io::{self, BufRead, IsTerminal, Write};

use inflect_rs::Engine;

fn main() {
    let mut engine = Engine::new();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        for arg in args {
            evaluate(&mut engine, &arg);
        }
        return;
    }

    let interactive = io::stdin().is_terminal();
    if interactive {
        println!("Type text containing calls like plural('cat') or a('hour'). Ctrl-D to exit.");
    }
    loop {
        if interactive {
            print!("> ");
            io::stdout().flush().ok();
        }
        let mut line = String::new();
        if io::stdin().lock().read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        evaluate(&mut engine, line.trim_end_matches('\n'));
    }
}

fn evaluate(engine: &mut Engine, text: &str) {
    match engine.inflect(text) {
        Ok(output) => println!("{output}"),
        Err(err) => eprintln!("error: {err}"),
    }
}
