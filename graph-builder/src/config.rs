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

use cincinnati::plugins::internal::metadata_fetch_quay::{
    DEFAULT_QUAY_LABEL_FILTER, DEFAULT_QUAY_MANIFESTREF_KEY,
};
use commons::{parse_params_set, parse_path_prefix};
use quay::v1::DEFAULT_API_BASE;
use std::collections::HashSet;
use std::net::IpAddr;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// Verbosity level
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u64,

    /// URL for the container image registry
    #[structopt(long = "registry", default_value = "http://localhost:5000")]
    pub registry: String,

    /// Name of the container image repository
    #[structopt(long = "repository", default_value = "openshift")]
    pub repository: String,

    /// Duration of the pause (in seconds) between scans of the registry
    #[structopt(
        long = "period",
        default_value = "30",
        parse(try_from_str = "parse_duration")
    )]
    pub period: Duration,

    /// Address on which the server will listen
    #[structopt(long = "address", default_value = "127.0.0.1")]
    pub address: IpAddr,

    /// Port to which the server will bind
    #[structopt(long = "port", default_value = "8080")]
    pub port: u16,

    /// Credentials file for authentication against the image registry
    #[structopt(long = "credentials-file", parse(from_os_str))]
    pub credentials_path: Option<PathBuf>,

    /// Path prefix for all paths.
    #[structopt(
        long = "path-prefix",
        default_value = "",
        parse(from_str = "parse_path_prefix")
    )]
    pub path_prefix: String,

    /// Comma-separated set of mandatory client parameters.
    #[structopt(
        long = "mandatory-client-parameters",
        default_value = "",
        parse(from_str = "parse_params_set")
    )]
    pub mandatory_client_parameters: HashSet<String>,

    /// Whether to disable the fetching and processing metadata from the quay API
    #[structopt(long = "disable-quay-api-metadata")]
    pub disable_quay_api_metadata: bool,

    /// Base URL to the quay API host
    #[structopt(
        long = "quay-api-base",
        long_help = "API base URL",
        raw(default_value = "DEFAULT_API_BASE")
    )]
    pub quay_api_base: String,

    /// Filter for receiving quay labels
    #[structopt(
        long = "quay-label-filter",
        raw(default_value = "DEFAULT_QUAY_LABEL_FILTER")
    )]
    pub quay_label_filter: String,

    /// Metadata key where the quay fetcher expects the manifestref
    #[structopt(
        long = "quay-manifestref-key",
        raw(default_value = "DEFAULT_QUAY_MANIFESTREF_KEY")
    )]
    pub quay_manifestref_key: String,

    /// Credentials file for authentication against API described at https://docs.quay.io/api/
    #[structopt(long = "quay-api-credentials-path", parse(from_os_str))]
    pub quay_api_credentials_path: Option<PathBuf>,
}

fn parse_duration(src: &str) -> Result<Duration, ParseIntError> {
    Ok(Duration::from_secs(u64::from_str(src)?))
}
