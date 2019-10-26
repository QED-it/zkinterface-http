#![feature(test)]
extern crate test;

use failure::Error;
use libzmq::{*, prelude::*};
use std::fs::read_to_string;
use std::process::Command;
use test::test::Bencher;

mod circuit;


#[bench]
fn bench_1_bulletproofs(b: &mut Bencher) {
    run(b, "cd ../bulletproofs-runner && cargo run", 1).unwrap();
}

#[bench]
fn bench_10_bulletproofs(b: &mut Bencher) {
    run(b, "cd ../bulletproofs-runner && cargo run", 10).unwrap();
}

/*#[bench]
fn bench_100_bulletproofs(b: &mut Bencher) {
    run(b, "cd ../bulletproofs-runner && cargo run", 100).unwrap();
}*/

fn run(b: &mut Bencher, command: &str, size: u64) -> Result<(), Error> {
    let buf = circuit::make_benchmark_circuit(10, size)?;

    let mut proc = Command::new("bash").arg("-c")
        .arg(command)
        .spawn()?;

    let addr: TcpAddr = "127.0.0.1:40001".try_into()?;

    let client = ClientBuilder::new().connect(addr).build()?;

    b.iter(|| -> Result<(), Error> {
        // Request.
        client.send(&buf)?;
        // Retrieve the reply.
        let msg = client.recv_msg()?;
        assert_eq!(msg.to_str()?, "ok");
        Ok(())
    });

    proc.kill()?;

    println!("Done.");

    Ok(())
}


