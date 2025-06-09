// v1.rs
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

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::AppState;

pub fn build() -> Router<AppState> {
    Router::new().route("/@me/consent", post(update_consent))
    // .route("/v1/@me/progress", post(submit_progress))
}

// TODO: Refactor below into a submodule of v1

#[derive(Clone, Deserialize, Serialize)]
pub struct ConsentUpdate {
    pub data_science: bool,
}

async fn update_consent(
    State(_app): State<AppState>,
    Json(_body): Json<ConsentUpdate>,
) -> impl IntoResponse {
    info!("Updating consent for user XYZ");
    // use `app.sql` for MariaDB…
    // use `app.mongo` for MongoDB…
    // e.g. app.sql.execute("…").await;
    //      app.mongo.database("perapera").collection("logs").insert_one(doc!{…}, None).await;
    StatusCode::NO_CONTENT
}
