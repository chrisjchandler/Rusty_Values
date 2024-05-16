
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


How to enable TLS

To secure the API with TLS, you will need to obtain TLS certificates. You can use self-signed certificates for testing or certificates issued by a Certificate Authority (CA) for production environments. Below are instructions on how to generate self-signed certificates and configure the server to use TLS.

Generating Self-Signed Certificates
You can use OpenSSL to generate a self-signed certificate and private key. Run the following commands in your terminal:

# Generate a Private Key
openssl genrsa -out key.pem 2048

# Generate a Self-Signed Certificate
openssl req -new -x509 -key key.pem -out cert.pem -days 365

During the certificate creation process, you will be prompted to enter details such as the country, state, and common name (domain name). For local testing, you can use localhost as the common name.

Configuring the Server for TLS
Set the Environment Variable: Before starting the server, set the USE_TLS environment variable to true to enable TLS:

export USE_TLS=true

On Windows, you can set the environment variable like this:

set USE_TLS=true

Move Certificates to Accessible Location: Ensure your certificates (cert.pem and key.pem) are placed in a directory accessible by the server application, or update the paths in the server code to match where you have stored these files.
Start the Server: Run your server application. If the USE_TLS environment variable is set, it will start with TLS enabled and use the provided certificates:




Enabling TLS
To secure the API with TLS, you will need to obtain TLS certificates. You can use self-signed certificates for testing or certificates issued by a Certificate Authority (CA) for production environments. Below are instructions on how to generate self-signed certificates and configure the server to use TLS.

Generating Self-Signed Certificates
You can use OpenSSL to generate a self-signed certificate and private key. Run the following commands in your terminal:

bash
Copy code
# Generate a Private Key
openssl genrsa -out key.pem 2048

# Generate a Self-Signed Certificate
openssl req -new -x509 -key key.pem -out cert.pem -days 365
During the certificate creation process, you will be prompted to enter details such as the country, state, and common name (domain name). For local testing, you can use localhost as the common name.

Configuring the Server for TLS
Set the Environment Variable: Before starting the server, set the USE_TLS environment variable to true to enable TLS:
bash
Copy code
export USE_TLS=true
On Windows, you can set the environment variable like this:
cmd
Copy code
set USE_TLS=true
Move Certificates to Accessible Location: Ensure your certificates (cert.pem and key.pem) are placed in a directory accessible by the server application, or update the paths in the server code to match where you have stored these files.
Start the Server: Run your server application. If the USE_TLS environment variable is set, it will start with TLS enabled and use the provided certificates:
bash
Copy code
cargo run


Testing TLS Configuration
To test if TLS is working, you can use curl to make a request over HTTPS:

curl -k https://localhost:3030/api/get/yourkey

The -k flag tells curl to not validate the certificate, which is useful if you are using self-signed certificates.

Notes
Certificate Validation: For production environments, you should use certificates issued by a trusted CA and ensure your client validates these certificates appropriately.
Firewall and Network Configuration: Make sure that the appropriate ports are open and accessible, and that TLS ports (typically 443 for HTTPS) are configured if different from your application's default.
Troubleshooting
TLS Errors: If you encounter errors related to TLS, verify the certificate and private key paths, ensure the environment variable is correctly set, and check that your certificates are not expired.
Permission Issues: Ensure that the server has the necessary permissions to read the certificate and key files.
Connection Refused: Check firewall settings and whether the correct port is being used and is open on your network.


