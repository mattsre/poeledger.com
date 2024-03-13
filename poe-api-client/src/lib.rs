pub mod api;
pub mod fetch;
pub mod ratelimit;

use api::stashes::PublicStashesResponse;
use ratelimit::limiter::{RateLimiter, RateLimiterError};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    StatusCode,
};
use thiserror::Error;

pub type HttpStatusCode = StatusCode;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("encountered error with HTTP status code: {0}")]
    HttpError(StatusCode),
    #[error("reqwest failed to send request: {0}")]
    SendFailed(reqwest::Error),
    #[error("reqwest couldn't deserialize body: {0}")]
    DeserializeError(reqwest::Error),
    #[error("encountered rate limit")]
    RateLimited,
    #[error("failed processing rate limiter rules: {0}")]
    RateLimiterRuleError(RateLimiterError),
    #[error("failed to authenticate or authentication was rejected")]
    AuthError,
    #[error("invalid request")]
    BadRequest,
    #[error("unexpected internal error")]
    UnknownError,
}

pub struct Client<L: RateLimiter> {
    access_token: Option<String>,
    http_client: reqwest::Client,
    limiter: L,
}

impl<L: RateLimiter> Client<L> {
    pub fn new(user_agent: &str, rate_limiter: L) -> Result<Self, ClientError> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(default_headers)
            .build()
            .expect("API client should build successfully, did you provide a valid user agent?");

        Ok(Self {
            access_token: None,
            http_client,
            limiter: rate_limiter,
        })
    }

    pub async fn authorize(
        &mut self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<(), ClientError> {
        let endpoint = "oauth/token";
        let request = self
            .http_client
            .post(format!("https://www.pathofexile.com/{endpoint}"))
            .form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("grant_type", "client_credentials"),
            ]);

        let response = self.fetch_api_response(endpoint, request).await?;
        match response.status() {
            StatusCode::OK => {
                let body = response
                    .json::<serde_json::Value>()
                    .await
                    .map_err(ClientError::DeserializeError)?;

                self.access_token = Some(body["access_token"].as_str().unwrap().to_owned());

                Ok(())
            }
            _ => Err(ClientError::HttpError(response.status())),
        }
    }

    pub async fn get_public_stashes(
        &mut self,
        next_change_id: Option<&str>,
    ) -> Result<(PublicStashesResponse, StatusCode), ClientError> {
        let endpoint = "public-stash-tabs";

        let token = match &self.access_token {
            Some(t) => t,
            None => return Err(ClientError::AuthError),
        };

        let stash_url = match next_change_id {
            Some(id) => format!("https://api.pathofexile.com/{endpoint}?id={id}"),
            None => format!("https://api.pathofexile.com/{endpoint}"),
        };

        let request = self.http_client.get(stash_url).bearer_auth(token);

        let response = self.fetch_api_response(endpoint, request).await?;
        let status = response.status();
        match status {
            StatusCode::OK => {
                let body = response
                    .json::<PublicStashesResponse>()
                    .await
                    .map_err(ClientError::DeserializeError)?;

                Ok((body, status))
            }
            StatusCode::UNAUTHORIZED => {
                tracing::warn!("unauthorized. debug headers: {:#?}", response.headers());

                Err(ClientError::AuthError)
            }
            _ => Err(ClientError::HttpError(response.status())),
        }
    }
}
