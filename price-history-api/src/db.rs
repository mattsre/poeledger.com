use std::env;

use anyhow::Context;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn create_client() -> anyhow::Result<Surreal<Client>> {
    let client: Surreal<Client> = Surreal::init();

    let url = env::var("SURREAL_URL").unwrap_or("localhost:8000".to_string());
    client
        .connect::<Ws>(&url)
        .await
        .context("failed connecting to surreal instance at {url}")?;

    let username = env::var("SURREAL_USER").unwrap_or("admin".to_string());
    let password = env::var("SURREAL_PASS").unwrap_or("password".to_string());
    client
        .signin(Root {
            username: &username,
            password: &password,
        })
        .await
        .context("failed authenticating for user: {username}")?;

    let ns_name = "poeledger";
    let db_name = "river";
    client
        .use_ns(ns_name)
        .use_db(db_name)
        .await
        .context("failed to use NS {ns_name} and DB {db_name}")?;

    Ok(client)
}
