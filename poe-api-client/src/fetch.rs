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

        let response = request.send().await.map_err(ClientError::ReqwestError)?;

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
                let ruleset = RuleSet::try_from(
                    headers
                        .get(format!("x-rate-limit-{rr}"))
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

                let rulestate = RuleState::try_from(
                    headers
                        .get(format!("x-rate-limit-{rr}-state"))
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
                .unwrap();

                let rule = Rule {
                    rtype: rr,
                    ruleset,
                    state: rulestate,
                };

                rules.push(rule);
            }

            let policy = Policy { rules };

            self.limiter.update(endpoint, policy).await.unwrap();
        };

        Ok(response)
    }
}
