use clap::{Parser};

#[derive(Parser)]
pub enum Params {
    /// Generate 0ro tree file
    #[command(name = "tree")]
    Tree {
        /// Path of Source 0ro Code File
        #[arg()]
        source: String, 
        /// Path of Destination 0ro Tree File
        #[arg(long, short = 'o')]
        dest: String
    }
}
