mod auth;
mod scroll;

use std::error::Error;
use std::fs;

use clap::Parser;

use crate::auth::{parse_auth_string_arg, AuthString};
use crate::scroll::scroll;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_parser = parse_auth_string_arg)]
    auth: Option<AuthString>,

    #[arg(long)]
    host: String,

    #[arg(long)]
    index: String,

    #[arg(short, long)]
    query_file: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let host = args.host;
    let index = args.index;
    let auth = args.auth;
    let query_file = args.query_file;

    let query = fs::read_to_string(query_file).expect("Unable to read the file");

    for doc in scroll(host, index, auth, query) {
        print!("{doc}");
    }

    Ok(())
}
