# economy-data-api

## Deployment

The data API connects to SurrealDB and acts as an intermediary between the web client and the database. It's currently deployed on Fly.io with no internal state, and can be horizontally scaled as needed. The Fly configuration allows scaling down to 0 instances to save on costs, if needed this can be changed to 1 instance to prevent cold-start loading times. Below are the necessary env variables for the API to function:
- `SURREAL_HOST`: Hostname to connect to, this should contain the connection protocol (ws | wss)
- `SURREAL_USER`: Username this service uses to connect to the DB
- `SURREAL_PASS`: Password this service uses to connect to the DB

Locally these are set in the `docker-compose.yml` file, in Prod these are set using Fly Secrets.
