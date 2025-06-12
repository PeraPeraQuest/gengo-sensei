// config.rs
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

/// Configuration for the application, populated from the environment
pub struct AppConfig {
    /// BIND_ADDRESS_AND_PORT
    pub bind_addr: String,
    /// MARIADB_URL
    pub mariadb_url: String,
    /// MONGODB_URL
    pub mongodb_url: String,
}

/// Obtain the application configuration from the environment
pub fn get_config() -> anyhow::Result<AppConfig> {
    // read configuration from the environment
    let bind_addr =
        std::env::var("BIND_ADDRESS_AND_PORT").expect("BIND_ADDRESS_AND_PORT must be set");
    let mariadb_url = std::env::var("MARIADB_URL").expect("MARIADB_URL must be set");
    let mongodb_url = std::env::var("MONGODB_URL").expect("MONGODB_URL must be set");

    // create the application configuration object
    let config = AppConfig {
        bind_addr,
        mariadb_url,
        mongodb_url,
    };

    // return the configuration to the caller
    Ok(config)
}
