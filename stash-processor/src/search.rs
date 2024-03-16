use std::env;

use serde::{Deserialize, Serialize};

use crate::listing::Listing;

pub struct MeilisearchHandler {
    client: meilisearch_sdk::Client,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueListingDocument {
    pub id: String,
    pub name: String,
}

impl UniqueListingDocument {
    pub fn new(name: &str) -> Self {
        Self {
            id: name_to_id(name),
            name: name.to_owned(),
        }
    }
}

impl MeilisearchHandler {
    pub async fn new() -> Self {
        let meili_url = env::var("MEILISEARCH_URL").unwrap_or("http://localhost:7700".to_owned());
        let meili_key = env::var("MEILISEARCH_API_KEY").unwrap_or("local-key".to_owned());

        let client = meilisearch_sdk::Client::new(meili_url, Some(meili_key));

        if let Err(e) = client.health().await {
            tracing::error!("failed connecting to meilisearch instance: {e}");
            tracing::error!("exiting!");

            std::process::exit(1);
        }

        tracing::info!("connected to meilisearch!");

        Self { client }
    }

    pub async fn add_document_batch(
        &self,
        index: &str,
        listings: &Vec<Listing>,
    ) -> anyhow::Result<()> {
        let iox = self.client.index(index);

        let mut docs = Vec::new();
        for l in listings {
            let doc = UniqueListingDocument::new(&l.name);

            docs.push(doc);
        }

        iox.add_or_update(&docs, Some("id")).await?;

        Ok(())
    }
}

fn name_to_id(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', ' '][..], "")
}

#[cfg(test)]
mod tests {
    use crate::search::name_to_id;

    #[test]
    fn simple_name() {
        let name1 = "Mageblood";
        assert_eq!(name_to_id(name1), "mageblood");

        let name2 = "Impresence";
        assert_eq!(name_to_id(name2), "impresence");

        let name3 = "The Eternal Struggle";
        assert_eq!(name_to_id(name3), "theeternalstruggle");
    }

    #[test]
    fn names_with_quotes() {
        let name1 = "Replica Hyrri's Ire";
        assert_eq!(name_to_id(name1), "replicahyrrisire");

        let name2 = "Lioneye's Fall";
        assert_eq!(name_to_id(name2), "lioneyesfall");
    }

    #[test]
    fn complex_names() {
        let name1 = "Caer Blaidd, Wolfpack's Den";
        assert_eq!(name_to_id(name1), "caerblaiddwolfpacksden");
    }
}
