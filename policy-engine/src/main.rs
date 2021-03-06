//! Cincinnati backend: policy-engine server.

extern crate actix;
extern crate actix_web;
extern crate cincinnati;
extern crate commons;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prometheus;
extern crate semver;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate openapiv3;
extern crate url;

mod config;
mod graph;
mod metrics;
mod openapi;

use actix_web::{http::Method, middleware::Logger, server, App};
use failure::Error;
use log::LevelFilter;
use std::collections::HashSet;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let sys = actix::System::new("policy-engine");
    let opts = config::Options::from_args();

    env_logger::Builder::from_default_env()
        .filter(
            Some(module_path!()),
            match opts.verbosity {
                0 => LevelFilter::Warn,
                1 => LevelFilter::Info,
                2 => LevelFilter::Debug,
                _ => LevelFilter::Trace,
            },
        )
        .init();

    // Metrics service.
    server::new(|| {
        App::new()
            .middleware(Logger::default())
            .route("/metrics", Method::GET, metrics::serve)
    })
    .bind((opts.metrics_address, opts.metrics_port))?
    .start();

    // Main service.
    let state = AppState {
        mandatory_params: opts.mandatory_client_parameters.clone(),
        upstream: opts.upstream.clone(),
        path_prefix: opts.path_prefix.clone(),
    };

    server::new(move || {
        let app_prefix = state.path_prefix.clone();
        App::with_state(state.clone())
            .middleware(Logger::default())
            .prefix(app_prefix)
            .route("/v1/graph", Method::GET, graph::index)
            .route("/v1/openapi", Method::GET, openapi::index)
    })
    .bind((opts.address, opts.port))?
    .start();

    sys.run();
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {
    /// Query parameters that must be present in all client requests.
    pub mandatory_params: HashSet<String>,
    pub upstream: hyper::Uri,
    pub path_prefix: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mandatory_params: HashSet::new(),
            upstream: hyper::Uri::from_static(config::DEFAULT_UPSTREAM_URL),
            path_prefix: String::new(),
        }
    }
}
