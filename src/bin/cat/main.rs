use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
struct Args {
    args: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args: Args = Parser::parse();

    for arg in &args.args {
        let file = File::open(arg);
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                print!("{}", contents);
            }
            Err(e) => println!("cat: {arg}: {}", e.to_string().replace("(os error 2)", "")),
        }
    }

    Ok(())
}
