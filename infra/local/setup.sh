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
