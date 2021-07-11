//#![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::http;
use hyper::{Body, Request, Response, server};

async fn hello(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Request: {:?}", request.uri());
    if request.uri() == "/" {
        let mut builder = Response::builder()
        .status(http::StatusCode::OK);
        Ok(builder.body(Body::from("<h1>g</h1>")).unwrap())
        //Ok(Response::new(Body::from("Hello World!")))
    } else {
        let mut builder = Response::builder()
            .status(http::StatusCode::NOT_FOUND);
        Ok(builder.body(Body::from("Not found.")).unwrap())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = server::Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}