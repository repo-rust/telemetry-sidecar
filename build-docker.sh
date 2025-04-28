#!/usr/bin/env bash

echo "Building 'telemetry-sidecar' image"
docker build --build-arg APP_NAME=telemetry-sidecar -f Dockerfile -t telemetry-sidecar:latest .

echo "Building 'telemetry-sidecar CLIENT' image"
docker build --build-arg APP_NAME=client -f Dockerfile -t telemetry-sidecar-client:latest .
