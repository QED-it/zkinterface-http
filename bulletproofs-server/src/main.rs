use bulletproofs::r1cs::{R1CSProof, zkinterface_backend};
use failure::{Error, format_err};
use std::convert::TryInto;
use std::io::Read;
use std::thread::sleep;
use std::time;
use std::time::Duration;
use zkinterface::reading::Messages;

fn main() -> Result<(), Error> {
    // Use a system assigned port.
    let addr = "0.0.0.0:40001";
    println!("Listening on {:?}", addr);

    rouille::start_server(addr, move |request| {
        match handle(request) {
            Ok(res) => rouille::Response::text(res),
            Err(err) => {
                println!("Error: {}", err);
                rouille::Response::text(err.to_string()).with_status_code(500)
            }
        }
    });
}

fn handle(request: &rouille::Request) -> Result<String, Error> {
    //println!("URL = {}", request.url());
    match request.url().as_ref() {
        "/status" => {
            Ok("ready".into())
        }

        "/prove" => {
            let mut data = request.data().unwrap();
            let mut msg = Vec::<u8>::new();
            data.read_to_end(&mut msg)?;

            let now = time::Instant::now();

            handle_prove(msg)?;

            //println!("DONE {:?} ", now.elapsed());
            Ok("ok".into())
        }

        _ => Err(format_err!("Unknown endpoint {}", request.url()))
    }
}

fn handle_prove(msg: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut reader = Messages::new(1);
    reader.push_message(msg).unwrap();

    let proof = zkinterface_backend::prove(&reader).unwrap();

    let proof_ser = bincode::serialize(&proof).unwrap();

    Ok(proof_ser)
}