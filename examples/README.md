# Media Core Examples

This directory contains examples of how to use the `media_core` project.

## `media_core_app`

This is a standalone Rust application that demonstrates how to consume `media_core` as a library. It is configured to pull the library from GitHub.

### 1. Running Locally

Navigate to the app directory and run:

```bash
cd media_core_app
cargo run
```

**Note:** You may need to set environment variables or ensure a local RTSP stream is available.

### 2. Running with Docker

The app includes its own Docker configuration in the `docker/` subdirectory.

```bash
cd media_core_app/docker
docker-compose up --build
```

This will build a container for the example app, pulling the `media_core` library dependency during the build process.
