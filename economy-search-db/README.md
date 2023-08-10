# economy-search-db

Poeledger.com uses a Meilisearch instance for fast, relevant, and type-tolerant searching

## Deployment

A single-node Meilisearch instance is deployed to Fly.io along with a volume mount. This volume has automated backups that can be restored, but other than that there's very little in terms of availability/reliability features in this DB deployment. If usage warrants, this could be moved to Meilisearch Cloud

