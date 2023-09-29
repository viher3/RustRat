mod server;
mod client;

use std::collections::HashSet;
use clap::Parser;

// @doc https://rust-cli.github.io/book/tutorial/cli-args.html

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    mode: String,
}

const MODE_SERVER: &str = "server";
const MODE_CLIENT: &str = "client";

fn main() {
    let args = CliArgs::parse();
    println!("{:?}", args);
    println!("{}", args.mode);

    // validate mode
    let mut valid_modes: HashSet<String> = HashSet::new();
    valid_modes.insert(String::from(MODE_SERVER));
    valid_modes.insert(String::from(MODE_CLIENT));

    if valid_modes.contains(&args.mode) {
        load_mode_module(args.mode);
    } else {
        panic!("Invalid mode");
    }
}

fn load_mode_module(mode : String) {
    if mode.eq(MODE_CLIENT) {
        client::main()
    }else if mode.eq(MODE_SERVER){
        server::main()
    }
}
