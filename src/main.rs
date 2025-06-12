// main.rs
// Copyright 2025 Patrick Meade
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod config;
pub mod db;
pub mod middle;
pub mod route;
pub mod state;
pub mod utils;

use std::sync::Arc;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::config::get_config;
use crate::state::AppState;
use crate::utils::logging::setup_logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup logging and log something nice
    setup_logging();
    info!("Gengo-sensei starting up...");

    // read the application configuration
    let config = get_config()?;

    // create application state
    let maria = db::mariadb::get_db(&config).await?;
    let mongo = Arc::new(db::mongodb::get_db(&config).await?);
    let state = AppState { maria, mongo };

    // Build routes and attach state
    let app = Router::new()
        .nest("/v1", route::v1::build(&state))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // bind the port we want to listen on for requests
    let addr = std::env::var("BIND_ADDRESS_AND_PORT").expect("BIND_ADDRESS_AND_PORT must be set");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let socket_addr = listener.local_addr().unwrap();
    info!("Listening for login requests on {}", socket_addr);

    // start the gengo-sensei service
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

//---------------------------------------------------------------------------
//---------------------------------------------------------------------------
//---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_always_succeed() {
        assert!(true);
    }
}
