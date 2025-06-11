// mariadb.rs
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

use sqlx::MySqlPool;
use tokio::time::timeout;
use tracing::info;

/// acquire a connection pool to a MariaDB database
pub async fn get_db() -> anyhow::Result<MySqlPool> {
    // source connection information from the environment
    let maria_url = std::env::var("DATABASE_URL")?;
    info!("Connecting to MariaDB: {}", maria_url);

    // within 1 second
    let pool = timeout(Duration::from_secs(1), async {
        // create a MySqlPool connection to the MariaDB database
        MySqlPool::connect(&maria_url).await
    })
    .await
    .expect("Timed out waiting for connection to MariaDB")
    .expect("Error while creating MySqlPool");

    // return the pool to the caller
    Ok(pool)
}
