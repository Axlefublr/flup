use clap::Parser;

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
	pub first: String,
	pub second: String
}