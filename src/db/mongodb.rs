// mongodb.rs
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

use std::time::Duration;

use mongodb::{bson::Document, Client};
use tokio::time::timeout;
use tracing::info;

/// obtain a MongoDB Client to access a MongoDB database
pub async fn get_db() -> anyhow::Result<Client> {
    // source connection information from the environment
    let mongo_url = std::env::var("MONGODB_URI")?;
    info!("Connecting to MongoDB: {}", mongo_url);

    // create the MongoDB client
    let mut opts = mongodb::options::ClientOptions::parse(&mongo_url).await?;
    opts.app_name = Some("gengo-sensei".to_string());
    let mongo = Client::with_options(opts)?;

    // within 1 second
    let _result = timeout(Duration::from_secs(1), async {
        // run a "ping" command on the database
        let database = mongo.database("perapera");
        let mut doc = Document::new();
        doc.insert("ping", 1);
        database.run_command(doc).await
    })
    .await
    .expect("Timed out waiting to ping MongoDB")
    .expect("Error while running ping on MongoDB");

    // return the database client to the caller
    Ok(mongo)
}
