use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, num::ParseIntError, time::Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RateLimiterError {
    #[error("failed to parse header due to an invalid format: `{0}`")]
    InvalidHeaderFormat(String),
    #[error("header contained invalid int characters")]
    HeaderIntParseFailed(#[from] ParseIntError),
    #[error("endpoint already exists: `{0}`")]
    DuplicateEndpoint(String),
    #[error("provided endpoint doesn't exist: `{0}`")]
    UnknownEndpoint(String),
    #[error("the rate limiter encountered an internal error")]
    InternalError,
}

#[derive(Clone, PartialEq)]
pub enum LimiterOutcome {
    Proceed,
    Retry { after: Duration },
}

impl Default for LimiterOutcome {
    fn default() -> Self {
        Self::Retry {
            after: Duration::from_secs(30),
        }
    }
}

/// RateLimiter implementations are responsible for determining when it is safe to make
/// requests to a given endpoint, and should only be used for in-memory implementations
#[async_trait]
pub trait RateLimiter {
    /// Check if we can make a request
    async fn check(&self, endpoint: &str) -> Result<LimiterOutcome, RateLimiterError>;
    /// Update the internal state of the rate limiter after our request
    async fn update(&mut self, endpoint: &str, policy: Policy) -> Result<(), RateLimiterError>;
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Policy {
    pub rules: Vec<Rule>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub enum RuleType {
    #[default]
    Ip,
    Client,
    Account,
}

impl From<&str> for RuleType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "ip" => RuleType::Ip,
            "client" => RuleType::Client,
            "account" => RuleType::Account,
            _ => RuleType::default(),
        }
    }
}

impl Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleType::Ip => write!(f, "ip"),
            RuleType::Client => write!(f, "client"),
            RuleType::Account => write!(f, "account"),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Rule {
    pub rtype: RuleType,
    pub ruleset: RuleSet,
    pub state: RuleState,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RuleSet {
    pub maximum_hits: i32,
    pub window: i32,
    pub time_restricted: i32,
}

impl TryFrom<&str> for RuleSet {
    type Error = RateLimiterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(':').collect::<Vec<&str>>();

        if parts.len() != 3 {
            return Err(RateLimiterError::InvalidHeaderFormat(value.to_owned()));
        }

        let ruleset = RuleSet {
            maximum_hits: header_part_to_i32(parts[0])?,
            window: header_part_to_i32(parts[1])?,
            time_restricted: header_part_to_i32(parts[2])?,
        };

        Ok(ruleset)
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RuleState {
    pub current_hits: i32,
    pub window: i32,
    pub active_time_restricted: i32,
}

impl TryFrom<&str> for RuleState {
    type Error = RateLimiterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(':').collect::<Vec<&str>>();

        if parts.len() != 3 {
            return Err(RateLimiterError::InvalidHeaderFormat(value.to_owned()));
        }

        let state = RuleState {
            current_hits: header_part_to_i32(parts[0])?,
            window: header_part_to_i32(parts[1])?,
            active_time_restricted: header_part_to_i32(parts[2])?,
        };

        Ok(state)
    }
}

fn header_part_to_i32(part: &str) -> Result<i32, RateLimiterError> {
    part.parse::<i32>()
        .map_err(RateLimiterError::HeaderIntParseFailed)
}
