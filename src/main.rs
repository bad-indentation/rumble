use clap::Parser;
use rumble_cli::{Config, run};

fn main() {
    let args = Config::parse();
    run(args);
}
