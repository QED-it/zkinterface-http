use failure::{Error, format_err};
use std::fs::read_to_string;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use test::test::Bencher;

pub fn run(b: &mut Bencher, command: &str, size: u64) -> Result<(), Error> {
    let buf = crate::circuit::make_benchmark_circuit(10, size)?;

    let mut backend_process = Command::new("bash").arg("-c")
        .arg(command)
        .spawn()?;

    let addr = "127.0.0.1:40001";
    let client = reqwest::Client::new();

    // Wait for the backend to accept connections.
    for _ in 0..30 {
        let response = client.get(&format!("http://{}/status", addr)).send();
        if response.is_ok() {
            break;
        } else {
            sleep(Duration::from_millis(1000));
        }
    }

    // Setup.
    let mut response = client
        .post(&format!("http://{}/setup", addr))
        .body(buf.clone())
        .send()?;
    if response.status() != reqwest::StatusCode::OK {
        let body = response.text().unwrap_or("".to_string());
        println!("WARNING during setup: {:?} {:?}", response, body);
        // Setup is optional, continue anyway.
    }

    // A function that requests one proof.
    let iter = || -> Result<(), Error> {
        let mut response = client
            .post(&format!("http://{}/prove", addr))
            .body(buf.clone())
            .send()?;

        if response.status() != reqwest::StatusCode::OK {
            let body = response.text().unwrap_or("".to_string());
            return Err(format_err!("{:?} {:?}", response, body));
        }
        Ok(())
    };

    // Measure average run time to make one proof.
    b.iter(|| {
        match iter() {
            Ok(()) => {}
            Err(err) => {
                println!("ERROR during proving: {}", err);
                backend_process.kill();
                panic!("Stopping benchmark due to previous error");
            }
        }
    });

    backend_process.kill()?;

    // Wait for shutdown.
    for _ in 0..30 {
        let response = client.get(&format!("http://{}/status", addr)).send();
        if response.is_err() {
            break;
        } else {
            sleep(Duration::from_millis(1000));
        }
    }

    println!("Done.");

    Ok(())
}
