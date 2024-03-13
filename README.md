# [![PoE Ledger](assets/poe_ledger_logo.png)](https://poeledger.com)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/mattsre/poeledger.com#license)

## What is poeledger.com?

PoE Ledger is a price tracking tool for Path of Exile items. It reads stash changes in real-time from the official GGG stash tab API and stores them for long-term (multi-league) historical tracking.


## Getting Started

PoE Ledger currently has two main components, the "stash ingestion" and the API + website. Getting running is a bit more challenging on the ingestion side, so I'll cover them separately. 

### Running Stash Ingestion

Stash ingestion is setup as a simple distributed system capable of scaling to meet the massive influx of changes from league starts. The [river-crawler](river-crawler/) service is responsible for crawling the stash river, where it puts stashes into a [NATS stream](https://docs.nats.io/nats-concepts/jetstream/streams). Stashes are then processed by the aptly named [stash-processor](stash-processor/) service and written to a [Clickhouse](https://clickhouse.com/docs/en/intro) database.

To get started, you'll need to have registered an application with GGG to get a client ID and client secret. You'll also need to make a user-agent header for your application. Once you have these, you can get running locally. This getting started assumes you have the NATS and Clickhouse CLIs installed and configure them.

```sh
# Run NATS and Clickhouse
# Configure NATS and Clickhouse CLIs after this
docker compose up -d

# Create necessary NATS resources
nats stream add --config infra/local/nats/streams/PublicStashStream.json
nats stream add --config infra/local/nats/streams/PublicStashChangeIds.json
nats kv add ratelimiter
nats consumer add --config infra/local/nats/consumers/RiverCrawler.json PublicStashChangeIds
nats consumer add --config infra/local/nats/consumers/StashProcessor.json PublicStashStream

# Create Clickhouse resources
# See full queries in stash-processor/sql
clickhouse-client --query="..."

# Run the river-crawler
cd river-crawler && export CLIENT_ID=... && export CLIENT_SECRET=... && export USER_AGENT=...
cargo run

# Run the stash-processor
cd stash-processor
cargo run

# Get the system running by pushing a stash change id to NATS
# You can check https://poe.ninja/stats to get an up-to-date one
nats pub river.changeids ...
```

From this point, you should be ingesting listings into Clickhouse. You can verify with:

```sh
clickhouse-client --query="SELECT count() FROM ledger.listings"
```


### Running the API and website

The API and website are much simpler to get started with. You'll just need the Clickhouse DB running. You'll need to create a `.env` file in the web-next dir so the website knows how to call the API. Check the [.env.example](web-next/.env.example) file for an example. Once created, getting running is easy.

```sh
cd price-history-api
cargo run

cd web-next
npm run dev
```

## License

The poeledger.com codebase is open-source and permissively licensed under the MIT License: [LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
