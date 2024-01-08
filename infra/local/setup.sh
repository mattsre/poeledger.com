#!/bin/bash
set -euo pipefail

# use ctlptl to create image registry and kind clusters
ctlptl apply -f k8s/registry.yaml
ctlptl apply -f k8s/clusters.yaml

# setup poeledger cluster
kubectl config use-context kind-poeledger-local

helm repo add nats https://nats-io.github.io/k8s/helm/charts/
helm repo update
helm upgrade nats nats/nats --install --values helm/nats.yaml

helm repo add surrealdb https://helm.surrealdb.com
helm repo update
helm upgrade surrealdb surrealdb/surrealdb --install --values helm/surrealdb.yaml

# setup port-forwarding for next steps
kubectl port-forward services/nats 4222:4222 > /dev/null 2>&1 &

pid=$!
echo nats pf pid: $pid

# kill the port-forward regardless of how this script exits
trap '{
    # echo killing $pid
    kill $pid
}' EXIT

# create streams and KV
nats stream add --config nats/streams/PublicStashStream.json
nats stream add --config nats/streams/PublicStashChangeIds.json
nats kv add ratelimiter

# create consumers
nats consumer add --config nats/consumers/RiverCrawler.json PublicStashChangeIds
nats consumer add --config nats/consumers/StashProcessor.json PublicStashStream
