####################################################################################################
# Build time container, use a Rust base image with Cargo installed
####################################################################################################
FROM rust:1.85.0 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ARG USER=dockeruser
ARG UID=10001
ARG TARGET_PLATFORM=x86_64-unknown-linux-musl
ARG APP_NAME=telemetry-sidecar

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build --release --target ${TARGET_PLATFORM}

# Copy source code
COPY src ./src

# Build main application code
RUN cargo build --release --target ${TARGET_PLATFORM} --bin ${APP_NAME}

####################################################################################################
# Runtime container, use smallest possible base image
####################################################################################################
FROM scratch

ARG TARGET_PLATFORM=x86_64-unknown-linux-musl
ARG APP_NAME=telemetry-sidecar

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Set the working directory
WORKDIR /app

# Copy the built binary from builder container
COPY --from=builder /app/target/${TARGET_PLATFORM}/release/${APP_NAME} ./main-app

USER ${USER}:${USER}

# Main entrypoint
ENTRYPOINT ["/app/main-app"]

