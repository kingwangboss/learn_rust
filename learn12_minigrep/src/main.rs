use std::{env, process};
// use learn12_minigrep::{run, Config};
use minigrep::{run, Config};

fn main() {
    // 读取参数
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    println!("{:?}", args);
}

