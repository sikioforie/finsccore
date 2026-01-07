use crate::credit_scoring::ScoringConfig;
use crate::openbank::Config as OpenBankConfig;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub scoring_config: ScoringConfig,
    pub openbank_config: OpenBankConfig,
}

impl AppState {
    pub fn new() -> SharedAppState {
        RwLock::new(Self {
            scoring_config: ScoringConfig::default(),
            openbank_config: OpenBankConfig::default(),
        })
    }
}

pub type SharedAppState = RwLock<AppState>;
