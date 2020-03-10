/*
 * Copyright 2020 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::path::PathBuf;

use log::{debug, info};
use structopt::StructOpt;

use chat::prelude::*;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Server {
        #[structopt(short, long, parse(from_os_str))]
        /// Database directory path
        database_dir: Option<PathBuf>,

        #[structopt(short, long)]
        /// Server address
        address: String,

        #[structopt(short, long)]
        /// Hostname to use json result
        hostname: String,
    },
    Migration {
        /// Database directory path
        database_dir: Option<PathBuf>,
    },
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    print_env();

    let opt: Opt = Opt::from_args();

    match opt.cmd {
        Command::Server {
            database_dir,
            address,
            hostname,
        } => chat::server::server(database_dir, address, hostname)?,
        Command::Migration { database_dir } => chat::feature::migration::migration(database_dir)?,
    }

    info!("Bye");

    Ok(())
}

fn print_env() {
    debug!("CARGO: {}", env!("CARGO"));
    debug!("CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));
    debug!("CARGO_PKG_VERSION: {}", env!("CARGO_PKG_VERSION"));
    debug!(
        "CARGO_PKG_VERSION_MAJOR: {}",
        env!("CARGO_PKG_VERSION_MAJOR")
    );
    debug!(
        "CARGO_PKG_VERSION_MINOR: {}",
        env!("CARGO_PKG_VERSION_MINOR")
    );
    debug!(
        "CARGO_PKG_VERSION_PATCH: {}",
        env!("CARGO_PKG_VERSION_PATCH")
    );
    debug!("CARGO_PKG_VERSION_PRE: {}", env!("CARGO_PKG_VERSION_PRE"));
    debug!("CARGO_PKG_AUTHORS: {}", env!("CARGO_PKG_AUTHORS"));
    debug!("CARGO_PKG_NAME: {}", env!("CARGO_PKG_NAME"));
    debug!("CARGO_PKG_DESCRIPTION: {}", env!("CARGO_PKG_DESCRIPTION"));
    debug!("CARGO_PKG_HOMEPAGE: {}", env!("CARGO_PKG_HOMEPAGE"));
    debug!("CARGO_PKG_REPOSITORY: {}", env!("CARGO_PKG_REPOSITORY"));
}
