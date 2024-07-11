use discord_rpc_client::{Client, Event};
use std::{env, thread, time};

fn main() {
    // Create the client
    let mut drpc = Client::new(69420);

    // Register event handlers with the corresponding methods
    drpc.on_ready(|_ctx| {
        println!("ready?");
    });

    // or

    drpc.on_event(Event::Ready, |_ctx| {
        println!("READY!");
    });

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Set the activity
    drpc.set_activity(|act| act.state(String::from("amogus")))
        .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(10));
}
