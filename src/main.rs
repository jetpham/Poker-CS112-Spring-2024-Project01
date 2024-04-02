use std::io::prelude::*;
use std::net::TcpStream;
use std::process::exit;
use std::str::from_utf8;

use anyhow::Context;
use clap::Parser;

use three_claw_stud::*;

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
        let Ok(resp) = from_utf8(&buffer) else {
            buffer = [0; 1500];
            continue;
        };

        match parse_response(resp) {
            Ok((
                i,
                ServerResponse {
                    variant: ResponseVariant::Login,
                    contents,
                },
            )) => {
                stream
                    .write_all("jetpham:Crusty".as_bytes())
                    .context("could not write to socket!")?;
            }
            Ok((
                i,
                ServerResponse {
                    variant: ResponseVariant::Bet,
                    contents,
                },
            )) => {}
            Ok((
                i,
                ServerResponse {
                    variant: ResponseVariant::Status,
                    contents,
                },
            )) => {}
            Ok((
                i,
                ServerResponse {
                    variant: ResponseVariant::Done,
                    contents,
                },
            )) => {
                println!("done{}", contents);
                break;
            }
            Err(e) => {
                // eprintln!("{}", e);
            }
        };

        buffer = [0; 1500];
    }

    Ok(())
}
