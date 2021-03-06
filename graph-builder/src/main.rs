// Copyright 2018 Alex Crawford
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate actix_web;
extern crate failure;
extern crate graph_builder;
extern crate log;
extern crate structopt;

use graph_builder::{config, graph};

use actix_web::{http::Method, middleware::Logger, server, App};
use failure::Error;
use log::LevelFilter;
use std::thread;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
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

    let state = graph::State::new(opts.mandatory_client_parameters.clone());
    let addr = (opts.address, opts.port);
    let app_prefix = opts.path_prefix.clone();

    {
        let state = state.clone();
        thread::spawn(move || graph::run(&opts, &state));
    }

    server::new(move || {
        let app_prefix = app_prefix.clone();
        let state = state.clone();
        App::with_state(state)
            .middleware(Logger::default())
            .prefix(app_prefix)
            .route("/v1/graph", Method::GET, graph::index)
    })
    .bind(addr)?
    .run();
    Ok(())
}
