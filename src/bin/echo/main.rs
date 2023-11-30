use clap::Parser;
use std::io::{self, Result, Write};

#[derive(Parser)]
struct Args {
    args: Vec<String>,
}

fn main() {
    let args: Args = Parser::parse();
    let _ = echo(&args.args);
}

fn echo(free: &[String]) -> Result<()> {
    let stdout = io::stdout();
    let mut output = stdout;

    for input in free.iter() {
        write!(output, "{} ", input)?;
    }
    writeln!(output)?;
    Ok(())
}
