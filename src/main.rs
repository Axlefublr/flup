use clap::Parser;

mod args;

fn main() {
	let args = args::Args::parse();
	let choice: bool = rand::random();
	match choice {
		true => println!("{}", args.first),
		false => println!("{}", args.second)
	}
}
