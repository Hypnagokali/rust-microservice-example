extern crate hyper;
extern crate futures;
extern crate tokio;

mod pages;
mod handler;
mod data;
mod config;

use crate::{handler::uri_handler::default_handler, data::user::InMemoryDataSource};
use crate::config::server_config::DataBaseServer;

use std::{sync::{Arc, Mutex}};
use hyper::{Server, service::{service_fn, make_service_fn}, http::Error, server::conn::AddrIncoming};
use slab::Slab;


async fn start_server(server_config: &DataBaseServer) {
    println!("Server starting ...");

    let builder = Server::bind(&server_config.addr);

    let server: Server<AddrIncoming, _> = builder.serve(make_service_fn(move |_| {

        let in_mem_ds = InMemoryDataSource {
            data_source: server_config.user_db.clone(),
        };

        async move {
            Ok::<_, Error>(service_fn(move |req| default_handler(req, &in_mem_ds)))
        }
        
    }));


    println!("Server is running ...");
    println!("http://{}", server_config.addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

#[tokio::main]
async fn main() {    
    let server_config = DataBaseServer {
        addr: ([127, 0, 0, 1], 5051).into(),
        user_db: Arc::new(Mutex::new(Slab::new())),
    };

    start_server(&server_config).await

}