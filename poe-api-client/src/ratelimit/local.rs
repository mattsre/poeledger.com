use std::{
    collections::HashMap,
    sync::{Arc, RwLock, TryLockError},
};

use async_trait::async_trait;

use super::limiter::{LimiterOutcome, Policy, RateLimiter, RateLimiterError};

#[derive(Default)]
pub struct LocalRateLimiter {
    endpoints: Arc<RwLock<HashMap<String, Policy>>>,
}

impl LocalRateLimiter {
    pub fn new() -> Self {
        Self {
            endpoints: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl RateLimiter for LocalRateLimiter {
    async fn check(&self, endpoint: &str) -> Result<LimiterOutcome, RateLimiterError> {
        let _endpoints = match self.endpoints.try_read() {
            Ok(p) => p,
            Err(e) => match e {
                TryLockError::Poisoned(_) => return Err(RateLimiterError::InternalError),
                TryLockError::WouldBlock => {
                    unimplemented!();
                }
            },
        };

        if let Some(p) = _endpoints.get(endpoint) {
            let mut decision = LimiterOutcome::default();

            for rule in &p.rules {
                if rule.state.current_hits + 1 < rule.ruleset.maximum_hits {
                    decision = LimiterOutcome::Proceed
                }
            }

            return Ok(decision);
        }

        Ok(LimiterOutcome::default())
    }

    async fn update(&mut self, endpoint: &str, policy: Policy) -> Result<(), RateLimiterError> {
        let mut _endpoints = match self.endpoints.try_write() {
            Ok(e) => e,
            Err(e) => match e {
                TryLockError::Poisoned(_) => return Err(RateLimiterError::InternalError),
                TryLockError::WouldBlock => {
                    unimplemented!();
                }
            },
        };

        _endpoints.insert(endpoint.to_owned(), policy);

        Ok(())
    }
}
