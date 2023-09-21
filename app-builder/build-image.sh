#!/bin/sh
DOCKER_BUILDKIT=0 docker buildx build --ulimit nofile=1024000:1024000 -t babylon-ledger-app-builder:latest .
