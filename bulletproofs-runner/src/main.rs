use bulletproofs::r1cs::{R1CSProof, zkinterface_backend};
use failure::Error;
use libzmq::{*, prelude::*};
use std::convert::TryInto;
use std::thread::sleep;
use std::time;
use std::time::Duration;
use zkinterface::reading::Messages;

fn main() -> Result<(), Error> {
    // Use a system assigned port.
    let addr: TcpAddr = "127.0.0.1:40001".try_into()?;

    let server = ServerBuilder::new()
        .bind(addr)
        .build()?;

    // Retrieve the addr that was assigned.
    let bound = server.last_endpoint()?;
    println!("Listening on {:?}", bound);

    loop {
        // Receive the client request.
        let msg = server.recv_msg()?;
        let id = msg.routing_id().unwrap();

        let mut reader = Messages::new(1);
        reader.push_message(msg.as_bytes().into()).unwrap();
        let num_constraints = reader.iter_constraints().count();

        let now = time::Instant::now();

        let proof = zkinterface_backend::prove(&reader).unwrap();

        println!("{:?}", now.elapsed());

        bincode::serialize(&proof).unwrap();

        // Reply to the client.
        server.route("ok", id)?;
    }

    //Ok(())
}
