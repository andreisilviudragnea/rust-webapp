#!/usr/bin/env sh

set -e

VERSION=$1

docker build -t "rust-webapp:$VERSION" .

unset KUBECONFIG
minikube update-context
minikube image load "rust-webapp:$VERSION"
kubectl set image deployment/rust-webapp rust-webapp="rust-webapp:$VERSION"
kubectl get pods

#kubectl create deployment rust-webapp --image=rust-webapp:1.0
#kubectl expose deployment rust-webapp --type=LoadBalancer --port=8080
#kubectl create configmap config-map --from-file config-map.json -o yaml --dry-run=client | kubectl replace -f -
#kubectl label configmaps config-map "app.kubernetes.io/name"="config-map"
#kubectl rollout restart deployment/rust-webapp
