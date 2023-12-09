use clap::Parser;
use rand::seq::IteratorRandom;
use std::error::Error;

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    let mut rng = rand::thread_rng();
    let result = args
        .into_iter()
        .choose(&mut rng)
        .ok_or("Specify at least one option")?;
    println!("{}", result);
    Ok(())
}
