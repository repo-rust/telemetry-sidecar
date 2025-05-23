# Telemetry Sidecar Agent written in Rust

## Build and Run Tests

* For local build run:

```bash
cargo build
```

* For unit-tests run:

```bash
cargo nextest run
```

## Static Analysis

```bash
cargo clippy
```

## Integration Run

* Run server using:

```bash
cargo run --bin telemetry-sidecar
```

* Run client using:

```bash
cargo run --bin client
```

## Running Server and Client using Docker compose

* start compose

```bash
./run-docker.sh
```

* stop compose service

```bash
./stop-docker.sh
```

