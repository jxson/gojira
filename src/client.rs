use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use errors::*;
use serde_json;
use serde_json::Value;

pub fn projects() -> Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);
    let uri = "https://httpbin.org/ip".parse()?;

    println!("trying a thing...");

    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    e
                )
            })?;
            println!("current IP address is {}", v["origin"]);
            Ok(())
        })
    });

    core.run(work).chain_err(|| "Request failed")
}
