use std::time::Duration;

use async_nats::jetstream;
use async_trait::async_trait;
use poe_api_client::ratelimit::limiter::{
    LimiterOutcome, Policy, RateLimiter, RateLimiterError, Rule, RuleType,
};

pub struct NatsRateLimiter {
    bucket: jetstream::kv::Store,
    ip: String,
}

impl NatsRateLimiter {
    pub async fn new(nats: async_nats::Client) -> anyhow::Result<Self> {
        let jetstream = jetstream::new(nats);
        let bucket = jetstream.get_key_value("ratelimiter").await?;

        let ip = reqwest::get("https://api.ipify.org/?format=text")
            .await?
            .text()
            .await?;

        Ok(Self { bucket, ip })
    }

    async fn kv_insert_rule(&self, key: String, rule: Rule) -> anyhow::Result<()> {
        self.bucket
            .put(key, serde_json::to_string(&rule)?.into())
            .await?;

        Ok(())
    }

    async fn kv_get_rule(&self, key: String) -> anyhow::Result<Option<Rule>> {
        let val = self.bucket.get(key).await?;
        if let Some(rb) = val {
            let rule: Rule = serde_json::from_slice(&rb)?;

            return Ok(Some(rule));
        }

        Ok(None)
    }

    async fn kv_set_endpoint_rtypes(
        &self,
        endpoint: &str,
        rtypes: Vec<RuleType>,
    ) -> anyhow::Result<()> {
        let key = format!("{}_{}_policy", self.ip, endpoint);
        self.bucket
            .put(key, serde_json::to_string(&rtypes)?.into())
            .await?;

        Ok(())
    }

    async fn kv_get_endpoint_rtypes(
        &self,
        endpoint: &str,
    ) -> anyhow::Result<Option<Vec<RuleType>>> {
        let key = format!("{}_{}_policy", self.ip, endpoint);

        let val = self.bucket.get(key).await?;
        if let Some(rb) = val {
            let rtypes: Vec<RuleType> = serde_json::from_slice(&rb)?;

            return Ok(Some(rtypes));
        }

        Ok(None)
    }

    fn generate_remote_key(&self, rtype: &RuleType, endpoint: &str) -> String {
        match rtype {
            RuleType::Ip => format!("{}_{}_{}", &self.ip, rtype, endpoint),
            RuleType::Client => format!("{}_{}", rtype, endpoint),
            RuleType::Account => format!("{}_{}", rtype, endpoint),
        }
    }
}

#[async_trait]
impl RateLimiter for NatsRateLimiter {
    async fn check(&self, endpoint: &str) -> Result<LimiterOutcome, RateLimiterError> {
        let local_rtypes = self.kv_get_endpoint_rtypes(endpoint).await.unwrap();

        let mut outcome = LimiterOutcome::Proceed;
        match local_rtypes {
            Some(rtypes) => {
                for rtype in rtypes {
                    let key = self.generate_remote_key(&rtype, endpoint);

                    if let Some(rule) = self.kv_get_rule(key).await.unwrap() {
                        if rule.state.current_hits + 1 > rule.ruleset.maximum_hits {
                            outcome = LimiterOutcome::Retry {
                                after: Duration::from_secs(rule.ruleset.window as u64),
                            }
                        }
                    }
                }
            }
            None => tracing::info!("no rtypes found for endpoint: {endpoint}"),
        }

        Ok(outcome)
    }

    async fn update(&mut self, endpoint: &str, policy: Policy) -> Result<(), RateLimiterError> {
        let mut rtypes = Vec::new();

        for rule in policy.rules {
            rtypes.push(rule.rtype.clone());

            let key = self.generate_remote_key(&rule.rtype, endpoint);
            if let Err(e) = self.kv_insert_rule(key, rule).await {
                tracing::error!("failed updating KV backend for rate limiter: {e}");
            }
        }

        if let Err(e) = self.kv_set_endpoint_rtypes(endpoint, rtypes).await {
            tracing::error!("failed to update rtypes for endpoint: {endpoint} with error: {e}");
        }

        Ok(())
    }
}
