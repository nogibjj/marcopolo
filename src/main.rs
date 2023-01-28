//marco polo tool
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Liam", about = "a marco polo generator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "0.1", author = "Liam")]
    Speak {
        #[clap(short, long)]
        name: String,
    },
}

/* use marco polo lib.rs */
//create main function
fn main() {
    //create a variable called args that is a struct of the Cli struct
    let args = Cli::parse();
    //create a match statement that matches the command variable in the args struct
    match args.command {
        //if the command is speak
        Some(Commands::Speak { name }) => {
            //create a variable called result that is the result of the marco_polo function in lib.rs
            let result = marcopolo::marco_polo(&name);
            //print the result
            println!("{}", result);
        }
        //if the command is not speak
        None => {
            //print the help message
            println!("Please enter a command");
        }
    }
}
