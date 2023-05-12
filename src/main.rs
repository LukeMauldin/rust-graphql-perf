mod controllers;

use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Method, Response, StatusCode,
};
use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub struct Context {}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let db = Arc::new(());
    let root_node = Arc::new(RootNode::new(
        controllers::QueryRoot {},
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));

    let new_service = make_service_fn(move |_| {
        let root_node = root_node.clone();
        let ctx = db.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let root_node = root_node.clone();
                let ctx = ctx.clone();
                async {
                    Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(root_node, ctx, req).await
                        }
                        _ => {
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            response
                        }
                    })
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{addr}");

    if let Err(e) = server.await {
        eprintln!("server error: {e}")
    }
}
