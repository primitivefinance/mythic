use anyhow::Result;
use simulation::settings::parameters::Multiple;
use simulation::settings::SimulationConfig;
use simulation::simulations::batch;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use toml;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let toml_str = String::from_utf8(whole_body.to_vec()).unwrap();
    let config: SimulationConfig<Multiple> = toml::from_str(&toml_str).unwrap();

    // Now you can use the deserialized TOML data
    println!("Received config: {:?}", config);

    Ok(Response::new(Body::from("Received TOML data")))
}

pub async fn run() {
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
