use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self, Read};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read from stdin
    stdin: bool
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog!");
    }

    let eye = if options.dead { "x" } else { "0" };

    println!("{}", message.trim().bright_yellow().underline().on_purple());

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", cat_picture);
        },
        None => {
            println!(" \\ ");
            println!("  \\ ");
            println!("    /\\_/\\ ");
            println!("   ( {eye} {eye} )", eye=eye.red().bold());
            println!("   =( I )=");        
        }
    }
    Ok(())
}
