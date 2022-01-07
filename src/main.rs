extern crate hyper;
extern crate futures;
extern crate tokio;

mod pages;
mod handler;
mod data;
mod config;

use crate::{handler::uri_handler::default_handler, data::user::InMemoryDataSource};
use crate::config::server_config::DataBaseServer;


use std::collections::HashMap;
use std::{sync::{Arc, Mutex}, io::Write};
use clap::{App, crate_authors, crate_name, crate_version, crate_description, Arg};
use hyper::{Server, service::{service_fn, make_service_fn}, http::Error, server::conn::AddrIncoming};
use slab::Slab;
use log::{debug, info, warn, error};
use pretty_env_logger::formatted_builder;
use chrono::Local;

async fn start_server(server_config: &DataBaseServer) {
    info!("Server starting ...");

    let builder = Server::bind(&server_config.addr);

    let server: Server<AddrIncoming, _> = builder.serve(make_service_fn(move |_| {

        let in_mem_ds = InMemoryDataSource {
            data_source: server_config.user_db.clone(),
        };

        async move {
            Ok::<_, Error>(service_fn(move |req| default_handler(req, &in_mem_ds)))
        }
        
    }));


    info!("Server is running ...");
    info!("http://{}", server_config.addr);

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}

fn init_logger() {
    formatted_builder().format(|buf, record| {
        let now = Local::now().format("%d.%m.%Y %H:%M:%S");
            writeln!(buf, "{} [{}] - {}", now, record.level(), record.args())
        })
        .filter(None, log::LevelFilter::Info)
        .init();
}

fn parse_args() -> HashMap<String, String>{
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .takes_value(true))
        .get_matches();

    let mut params_map = HashMap::new();

    let p = matches.value_of("port").map(|p| p.to_owned()).unwrap_or_else(|| "5050".to_owned());

    params_map.insert("port".to_owned(), p);

    params_map
}

#[tokio::main]
async fn main() {
    let arg_map = parse_args();
    init_logger();

    let port: u16 = arg_map.get("port").unwrap()
        .parse()
        .expect("Konnte Port nicht parsen");

    info!("Port-Settings: {}", port);
    let server_config = DataBaseServer {
        addr: ([127, 0, 0, 1], port).into(),
        user_db: Arc::new(Mutex::new(Slab::new())),
    };

    start_server(&server_config).await

}