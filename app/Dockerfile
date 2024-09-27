FROM rust:1.81.0 AS build

WORKDIR /opt/app

# build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
RUN cargo build --release && rm src/main.rs ./target/release/deps/app*

# copy the source code and build for release
COPY . ./
RUN cargo build --release

# the final base image
FROM gcr.io/distroless/cc

WORKDIR /opt/app

# copy the build artifacts from the build stage
COPY --from=build /opt/app/target/release/app ./
COPY --from=build /opt/app/templates ./templates/
COPY --from=build /opt/app/static ./static/

# run the binary
CMD ["./app"]
