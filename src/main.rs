extern crate iron;
extern crate router;
extern crate url;
#[macro_use]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate lazy_static;
extern crate clap;

use iron::prelude::*;
use router::Router;
use clap::{Arg, App, ArgMatches};

mod handler;
mod middleware;
mod error;
use middleware::{BasicAuth, LogAfterRequest};

fn get_logger() -> &'static slog::Logger {
    use slog::*;
    use std::io::stdout;
    lazy_static! {
        static ref ROOT: slog::Logger = slog::Logger::root(
            slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(stdout()))
            .build().fuse(), o!()
        );
    }
    &ROOT
}

fn get_args() -> &'static ArgMatches<'static> {
    lazy_static! {
        static ref MATCHES: ArgMatches<'static> = App::new("MediaServer")
            .version("0.1")
            .author("Frederick Zhang <frederick888@tsundere.moe>")
            .arg(
                Arg::with_name("host")
                    .long("host")
                    .short("H")
                    .help("The IP address that the server listens on, default: 0.0.0.0")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .long("port")
                    .short("P")
                    .help("The port that the server listens on, default: 8000")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("username")
                    .long("user")
                    .short("u")
                    .help("Username of the server, default: user")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("password")
                    .long("pass")
                    .short("p")
                    .help("Password of the server, default: comicglass")
                    .takes_value(true),
            )
            .get_matches();
    }
    &MATCHES
}

fn main() {
    let listen_host = get_args().value_of("host").unwrap_or_else(|| "0.0.0.0");
    let listen_port = get_args().value_of("port").unwrap_or_else(|| "8000");
    let listen_address = format!("{}:{}", listen_host, listen_port);

    let mut router = Router::new();
    router.get("/", handler::server::server_handler, "index");
    router.get("/:query_path", handler::server::server_handler, "main");
    let mut chain = Chain::new(router);
    chain.link_before(BasicAuth);
    chain.link_after(LogAfterRequest);
    Iron::new(chain).http(listen_address).unwrap();;
}
