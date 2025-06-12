// me.rs
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

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::db::mongodb::Account;
use crate::middle::auth_middleware;
use crate::state::AppState;

pub fn build(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/account", get(get_account))
        .route("/consent", post(update_consent))
        // .route("/v1/@me/progress", post(submit_progress))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
}

async fn get_account(Extension(user): Extension<Account>) -> impl IntoResponse {
    Json(user)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Consent {
    /// Indicates if the user has given explicit consent for their PeraPera
    /// Quest usage data to be included in aggregate data provided to data
    /// scientists. Here "data scientists" stands in for people who want to
    /// study the way people learn languages; answering questions like how
    /// long do they take, what parts do they find easy or difficult, etc.
    pub data_science: bool,

    /// Indicates if the user has given explicit consent to participate in
    /// testing of new PeraPera Quest features. For example, if a new learning
    /// algorithm is designed, this user has given explicit consent to be
    /// included in an A/B trial to test the efficacy of the new algorithm.
    pub ppq_test: bool,
}

async fn update_consent(
    State(_app): State<AppState>,
    Extension(_user): Extension<Account>,
    Json(_body): Json<Consent>,
) -> impl IntoResponse {
    info!("Updating consent for user XYZ");
    // use `app.sql` for MariaDB…
    // use `app.mongo` for MongoDB…
    // e.g. app.sql.execute("…").await;
    //      app.mongo.database("perapera").collection("logs").insert_one(doc!{…}, None).await;
    StatusCode::NO_CONTENT
}
