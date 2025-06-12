// middle.rs
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

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_auth::AuthBearer;
use tracing::error;

use crate::db::mongodb::load_account;
use crate::state::AppState;

/// Convert an `Authorization: Bearer ${token}` header into an Account struct
/// that is added as an Extension on the request. Downstream routes can rely
/// on the Account struct to contain the account information of the user
/// calling the route.
pub async fn auth_middleware(
    State(state): State<AppState>,
    AuthBearer(token): AuthBearer,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    // query MongoDB for the user's account
    let client = Arc::clone(&state.mongo);
    let account = match load_account(&client, &token).await {
        // if we got a result back
        Ok(maybe_account) => {
            // if the result was empty, we didn't find the account
            if maybe_account.is_none() {
                return Err((StatusCode::UNAUTHORIZED, "Account not found"));
            }
            // otherwise, we did find the account
            maybe_account.unwrap()
        }
        // oops, we got an error back
        Err(e) => {
            // log about that, and return an error
            error!("Error during load_account: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error"));
        }
    };
    // put the Account object in extensions, so downstream routes can use it
    req.extensions_mut().insert(account);
    // call the next thing in line for handling this request
    Ok(next.run(req).await)
}
