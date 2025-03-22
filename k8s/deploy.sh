#!/bin/sh
if [ $# -ne 1 ] ; then
    echo "Usage: $0 K8S_HOST"
    exit 1
else
    K8S_HOST=$1
fi

docker build .. -t gnosispay-monitor --platform linux/amd64
docker save gnosispay-monitor | ssh $K8S_HOST sudo k3s ctr images import -