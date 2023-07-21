use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
	#[arg(value_enum, default_value_t = Flavor::Coin)]
	pub flavor: Flavor,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Flavor {
	Coin,
	Bool,
}