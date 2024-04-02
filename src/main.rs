use std::io::prelude::*;
use std::net::TcpStream;

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    address_pair: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut stream =
        TcpStream::connect(args.address_pair).context("could not connect to socket")?;

    let mut buffer = [0; 1500];
    while let Ok(bytes) = stream.read(&mut buffer) {
        // let actual = buffer[0..bytes].clone();
        match todo!() {}
    }

    // while true {

    // }

    Ok(())
}
