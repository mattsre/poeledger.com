# economy-data-ingest

Currently implemented as a manually-ran script that reads from local CSV files and writes records to the specified SurrealDB instance. Requires the following env vars to function:
- `SURREAL_HOST`: Hostname to connect to, this should contain the connection protocol (ws | wss)
- `SURREAL_USER`: Username this service uses to connect to the DB
- `SURREAL_PASS`: Password this service uses to connect to the DB

## Current Issues

- Poor performance writing records to DB

There's numerous potential performance improvements here, some notable ones are batching inserts and improving the CSV string -> Rust struct conversion performance. Benchmarking could be useful to identify further target areas.

- Poor UX for managing datasets

Since this is implemented as a one-off manual script, it requires code changes for just specifying which league dataset we want to read from. Additionally, there's no protections in place to prevent accidentally loading the same dataset multiple times, or writing duplicate records. The UX around managing datasets should likely be overhauled entirely. A backend "admin" UI with an "ingest-api" that powers it could be implemented as an improvement. The "ingest-api" could use the poe.ninja API to get the available dumps using the endpoint `https://poe.ninja/api/data/getdumps` and manage a SurrealDB table that tracks which data dumps have been loaded. It could also include logic for downloading, unzipping, and then writing datasets to the Surreal DB instance. Managing datasets could take place via the admin UI at admin.poeledger.com or similar.
