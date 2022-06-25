extern crate clap;
extern crate derive_builder;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(name = "dump-dvb sink")]
#[clap(author = "dvb-dump@protonmail.com")]
#[clap(version = "0.1.0")]
#[clap(about = "data collection server", long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,

    #[clap(short, long, default_value_t = String::from("http://[::1]:50051"))]
    pub grpc_host: String,
}
