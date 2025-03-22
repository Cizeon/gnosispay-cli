#!/bin/sh

export KUBECONFIG=~/.k3s/k3s.yaml
kubectl delete pod gnosispay-monitor-pod
kubectl apply -f pod.yaml
