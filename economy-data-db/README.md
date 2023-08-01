# economy-data-db

Poeledger.com uses a SurrealDB instance for storing historical item prices

## Deployment

A single-node SurrealDB instance is deployed to Fly.io along with a volume mount. This volume has automated backups that can be restored, but other than that there's very little in terms of availability/reliability features in this DB deployment. When released, we should migrate this to use the managed SurrealDB Cloud offering assuming their pricing model isn't egregious.


