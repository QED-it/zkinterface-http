use bellman::{
    Circuit,
    ConstraintSystem,
    groth16::{
        create_random_proof,
        generate_random_parameters,
        Parameters,
    },
    SynthesisError,
    Variable,
};
use failure::{Error, format_err};
use pairing::{bls12_381::Bls12, Engine};
use rand::{OsRng, Rng, SeedableRng, StdRng};
use std::convert::TryInto;
use std::env;
use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time;
use std::time::Duration;
use zkinterface_bellman::zkif_backend::{Messages, zkif_backend, ZKIFCircuit};


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

        "/setup" => {
            let mut data = request.data().unwrap();
            let mut msg = Vec::<u8>::new();
            data.read_to_end(&mut msg)?;

            handle_setup(msg)?;

            Ok("ok".into())
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

static mut PARAMS: Option<Parameters<Bls12>> = None;

fn handle_setup(msg: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut reader = Messages::new(1);
    reader.push_message(msg).unwrap();
    let circuit = ZKIFCircuit { messages: &reader };

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    unsafe {
        PARAMS = Some(
            generate_random_parameters::<Bls12, _, _>(
                circuit.clone(),
                &mut rng)?);
    }

    Ok(vec![])
}

fn handle_prove(msg: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut reader = Messages::new(1);
    reader.push_message(msg).unwrap();
    let circuit = ZKIFCircuit { messages: &reader };

    let params = unsafe { PARAMS.as_ref().unwrap() };
    let mut rng = OsRng::new()?;

    let proof = create_random_proof(
        circuit,
        params,
        &mut rng,
    )?;

    let mut proof_ser = Vec::<u8>::new();
    proof.write(&mut proof_ser)?;

    Ok(proof_ser)
}