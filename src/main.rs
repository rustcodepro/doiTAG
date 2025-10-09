mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod doigen;
use crate::doigen::generatetag;

/*
 Author Gaurav Sablok,
 Email: codeprog@icloud.com
 Date: 2025-22-8
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Doigen { pathfile } => {
            let command = generatetag(pathfile).unwrap();
            println!("The command has finished:{}", command);
        }
    }
}
