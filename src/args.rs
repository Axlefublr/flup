use clap::Parser;

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "A tool that randomly picks between two options you specify")]
pub struct Args {
	pub first: String,
	pub second: String
}