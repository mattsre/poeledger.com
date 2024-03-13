use std::thread;

use reqwest::{RequestBuilder, Response};

use crate::{
    ratelimit::limiter::{
        LimiterOutcome, Policy, RateLimiter, RateLimiterError, Rule, RuleSet, RuleState, RuleType,
    },
    Client, ClientError,
};

impl<L: RateLimiter> Client<L> {
    pub(crate) async fn fetch_api_response(
        &mut self,
        endpoint: &str,
        request: RequestBuilder,
    ) -> Result<Response, ClientError> {
        tracing::debug!("recieved request for endpoint: {endpoint}");

        let limiter_outcome = match self.limiter.check(endpoint).await {
            Ok(o) => o,
            Err(e) => match e {
                RateLimiterError::UnknownEndpoint(_) => LimiterOutcome::Proceed,
                _ => return Err(ClientError::UnknownError),
            },
        };

        match limiter_outcome {
            LimiterOutcome::Proceed => {
                tracing::debug!("rate limiter decided to proceed");
            }
            LimiterOutcome::Retry { after } => {
                tracing::debug!(
                    "rate limiter decided to wait for {} seconds, sleeping!",
                    after.as_secs()
                );
                thread::sleep(after);
            }
        };

        let response = request.send().await.map_err(ClientError::SendFailed)?;

        let headers = response.headers();
        if headers.get("x-rate-limit-policy").is_some() {
            tracing::debug!(
                "rate limit headers detected, updating rate limit policy for endpoint: {endpoint}"
            );
            let rules_raw = headers
                .get("x-rate-limit-rules")
                .unwrap()
                .to_str()
                .unwrap()
                .split(',')
                .map(RuleType::from)
                .collect::<Vec<RuleType>>();

            let mut rules: Vec<Rule> = Vec::new();
            for rr in rules_raw {
                if let (Some(rset), Some(rstate)) = (
                    headers.get(format!("x-rate-limit-{rr}")),
                    headers.get(format!("x-rate-limit-{rr}-state")),
                ) {
                    let ruleset = RuleSet::try_from(rset.to_str().unwrap())
                        .map_err(ClientError::RateLimiterRuleError)?;

                    let rulestate = RuleState::try_from(rstate.to_str().unwrap())
                        .map_err(ClientError::RateLimiterRuleError)?;

                    let rule = Rule {
                        rtype: rr,
                        ruleset,
                        state: rulestate,
                    };

                    rules.push(rule);
                }
            }

            let policy = Policy { rules };

            self.limiter
                .update(endpoint, policy)
                .await
                .map_err(ClientError::RateLimiterRuleError)?;
        };

        Ok(response)
    }
}
