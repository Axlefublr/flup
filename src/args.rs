use clap::Parser;
use std::vec;

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Choose and print a string out of those specified")]
pub struct Args {
    options: Vec<String>,
}

impl IntoIterator for Args {
    type Item = String;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.options.into_iter()
    }
}
