
This application is a simple key-value store with a RESTful API, built in Rust.

## Building and Running Locally

1. Build the project with Cargo:

cargo run --release




#Dockerfile Instructions

create a Dockerfile in the root directory of your Rust project. This file will instruct Docker on how to build an image of your Rust application:


# Use the official Rust image as the base image
FROM rust:1.56 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin key_value_store
WORKDIR /key_value_store

# Copy your project's Cargo.toml and Cargo.lock files and build your project's dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code of your application
COPY ./src ./src

# Build your application
RUN rm ./target/release/deps/key_value_store*
RUN cargo build --release

# Use the Debian Buster image for the final base image
FROM debian:buster-slim

# Copy the binary from the builder stage to the final stage
COPY --from=builder /key_value_store/target/release/key_value_store /usr/local/bin/key_value_store

# Set the CMD to your binary
CMD ["key_value_store"]

# Kubernetes Deployment

create a Kubernetes deployment file, key_value_store_deployment.yaml, to deploy your application:
apiVersion: apps/v1
kind: Deployment
metadata:
  name: key-value-store-deployment
spec:
  replicas: 1
  selector:
    matchLabels:
      app: key-value-store
  template:
    metadata:
      labels:
        app: key-value-store
    spec:
      containers:
      - name: key-value-store
        image: key-value-store-image:latest
        ports:
        - containerPort: 3030

POST to Insert a Key-Value Pair
To insert a new key-value pair into the store, you can use a POST request with a JSON body specifying the key and value. Replace localhost:3030 with the appropriate host and port where your service is running if different.

curl -X POST http://localhost:3030/insert \
     -H "Content-Type: application/json" \
     -d '{"key":"placementKey", "value":"placementValue"}'

This command sends a POST request to the /insert endpoint with a JSON payload containing the key "placementKey" and the value "placementValue". The -H "Content-Type: application/json" header tells the server to expect JSON data.


GET to Retrieve a Value by Key
To retrieve the value associated with a key, you can use a GET request. Replace "placementKey" with the key you wish to retrieve, and adjust localhost:3030 as necessary.

curl http://localhost:3030/get/placementKey

This command sends a GET request to the /get/exampleKey endpoint, which should return the value associated with "exampleKey" if it exists in the store.

Additional Tips
Ensure your server is running and accessible at the specified address and port before using these curl commands.
If your application or Kubernetes service is not hosted locally, replace localhost:3030 with the appropriate hostname or IP address and port number.
For debugging or additional information, you can add the -v (verbose) flag to your curl commands to get more details about the request and response.
