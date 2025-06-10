// logging.rs
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

use std::io::IsTerminal;

use tracing_subscriber::{
    EnvFilter, fmt, fmt::format::FmtSpan, prelude::__tracing_subscriber_SubscriberExt,
    registry::Registry,
};

/// configure logging for the application
pub fn setup_logging() {
    // figure out what things we want to look by looking at
    // RUST_LOG from the environment or default to "info"
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // figure out how we want those logging messages to be formatted
    // and where the logging messages should be sent
    let fmt_layer = fmt::layer()
        // produce JSON‐formatted logs
        // .json()
        .with_ansi(std::io::stdout().is_terminal())
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(std::io::stdout);

    // build the logging subscriber (filter + formatting)
    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    // install our subscriber as the global default
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");
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
