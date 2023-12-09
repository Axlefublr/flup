use clap::Parser;

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Choose and print a string out of those specified")]
pub struct Args {
	pub first: String,
	pub second: String
}