#!/usr/bin/env bash

docker volume rm telemetry-sidecar-volume
docker volume create telemetry-sidecar-volume

docker run --rm --name telemetry-sidecar --user root -v telemetry-sidecar-volume:/tmp/ telemetry-sidecar:latest &
docker run --rm --name telemetry-sidecar-client -v telemetry-sidecar-volume:/tmp/ telemetry-sidecar-client:latest &