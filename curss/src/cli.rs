use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;
use curss_server::Config;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(author, version, about)]
pub struct Cli {
    /// The IP address and port at which to bind curss
    #[arg(long, default_value = "0.0.0.0:8000")]
    address: SocketAddr,

    /// The directory where the web client's files are located
    #[arg(short, long, default_value = "dist")]
    client_dir: PathBuf,
}

impl From<Cli> for Config {
    fn from(value: Cli) -> Self {
        Self {
            address: value.address,
            client_dir: value.client_dir,
        }
    }
}
