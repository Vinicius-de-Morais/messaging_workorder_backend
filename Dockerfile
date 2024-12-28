# # 1. This tells docker to use the Rust official image
# FROM rust:1.83

# # 2. Copy the files in your machine to the Docker image
# COPY ./ ./

# # Build your program for release
# RUN cargo build --release

# # Run the binary
# CMD ["./target/release/messaging_workorder_backend"]

# Rust as the base image
FROM rust:1.83 AS build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin messaging_workorder_backend
WORKDIR /messaging_workorder_backend

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/messaging_workorder_backend*
RUN cargo build --release

# our final base
#FROM rust:1.83.0-slim-bullseye
FROM rust:1.83

# copy the build artifact from the build stage
COPY --from=build /messaging_workorder_backend/target/release/messaging_workorder_backend .

# set the startup command to run your binary
CMD ["./messaging_workorder_backend"]