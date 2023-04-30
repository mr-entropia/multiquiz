use std::env;

use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
};

fn main() -> mongodb::error::Result<()> {
    let mut client_options =
        ClientOptions::parse(env::var("multiquiz").unwrap())?;
    
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("multiquiz")
        .run_command(doc! {"ping": 1}, None)?;

    let questions = client.database("multiquiz").collection::<Document>("questions").find(doc! { "iso639-1": "fi" }, None).expect("Not found");

    dbg!(questions);

    /*
    for name in client.list_database_names(None, None)? {
        println!("- {}", name);
    }
    */

    Ok(())
}