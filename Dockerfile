# Use a base image with the latest version of Rust installed
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy the real application code into the container
COPY . .

# Build the application
RUN cargo build --release

# (Optional) Remove debug symbols
RUN strip target/release/prmoji

# Use a slim image for running the application
FROM debian:12-slim as runtime

# Install the system dependencies required by the application
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy only the compiled binary from the builder stage to this image
COPY --from=builder /app/target/release/prmoji /bin/prmoji

# Copy the configuration file from the host to the container
COPY config config

# Specify the command to run when the container starts
CMD ["/bin/prmoji"]
