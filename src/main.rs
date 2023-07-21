use args::Flavor;
use clap::Parser;

mod args;

fn main() {
	let args = args::Args::parse();
	let choice: bool = rand::random();
	match choice {
		true => {
			match args.flavor {
				Flavor::Bool => println!("True!"),
				Flavor::Coin => println!("Heads!")
			}
		}
		false => {
			match args.flavor {
				Flavor::Bool => println!("False!"),
				Flavor::Coin => println!("Tails!")
			}
		}
	}
}
