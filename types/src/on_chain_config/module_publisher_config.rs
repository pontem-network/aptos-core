use crate::{account_address::AccountAddress, on_chain_config::OnChainConfig};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModulePublisherConfig {
    publisher_allowlist: Vec<AccountAddress>,
}

impl ModulePublisherConfig {
    pub fn publisher_allowlist(&self) -> &Vec<AccountAddress> {
        &self.publisher_allowlist
    }
}

impl From<Vec<AccountAddress>> for ModulePublisherConfig {
    fn from(allowlist: Vec<AccountAddress>) -> Self {
        Self {
            publisher_allowlist: allowlist,
        }
    }
}

impl OnChainConfig for ModulePublisherConfig {
    const IDENTIFIER: &'static str = "ModulePublisherConfig";
}

impl Default for ModulePublisherConfig {
    fn default() -> Self {
        Self {
            publisher_allowlist: Vec::new(),
        }
    }
}
