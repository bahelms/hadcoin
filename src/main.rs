mod blockchain;

use blockchain::Blockchain;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde_json::json;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

async fn handle_request(
    chain: Arc<Mutex<Blockchain>>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let mut chain = chain.lock().unwrap();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/chain") => {
            let mut data = json!(*chain);
            data["length"] = json!((*chain).len());
            let json = serde_json::to_string(&data).expect("Error serializing");
            *response.body_mut() = Body::from(json);
        }
        (&Method::POST, "/mine") => {
            let block = chain.mine_block().expect("Mining failed");
            let mut data = json!(block);
            data["message"] = json!("Successfully mined a block!");
            let json = serde_json::to_string(&data).expect("Error serializing");
            *response.body_mut() = Body::from(json);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

#[tokio::main]
async fn main() {
    let chain = Arc::new(Mutex::new(Blockchain::new()));

    let service = make_service_fn(move |_conn| {
        let chain = chain.clone();
        let service = service_fn(move |req| handle_request(chain.clone(), req));
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}
