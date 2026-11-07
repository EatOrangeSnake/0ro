pub mod cmd_matcher;
pub mod tree;

use clap::Parser;
use cmd_matcher::Params;
use tree::Tree;

fn main () -> Result<(), std::io::Error> {
    match Params::parse() {
        Params::Tree { source, dest } => {
        }
    };
    Ok(())
}
