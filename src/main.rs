use clap::Parser;
use rumble::{Config, run};

fn main() {
    let args = Config::parse();
    run(args);
}
