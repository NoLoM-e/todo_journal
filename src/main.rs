use cli::CliArgs;
use structopt::StructOpt;

mod cli;

fn main() {
    let app_args = CliArgs::from_args();
    println!("{:?}", app_args);
}
