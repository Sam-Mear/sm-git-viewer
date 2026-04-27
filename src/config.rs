// SPDX-License-Identifier: EUPL-1.2

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 2]
pub struct Config {
    demo: String,
    /// Optional repository path to load for the application UI.
    pub repo_path: Option<String>,
}
