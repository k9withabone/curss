#![warn(clippy::pedantic)]

mod cli;

use clap::Parser;

use cli::Cli;

#[tokio::main]
async fn main() {
    curss_server::run(Cli::parse().into()).await;
}
