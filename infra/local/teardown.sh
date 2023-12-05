#!/bin/bash
set -euo pipefail

# teardown poeledger cluster
kubectl config use-context kind-poeledger-local

helm uninstall nats
helm uninstall surrealdb

# use ctlptl to manage image registry and kind clusters
ctlptl delete -f k8s/clusters.yaml
ctlptl delete -f k8s/registry.yaml
