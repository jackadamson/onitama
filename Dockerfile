FROM rust:1.71 as client-builder
ARG BUILD_VERSION

# Until feature(array_map) makes it to stable, requires nightly toolchain
RUN rustup default nightly

WORKDIR /src

# Install Node and Yarn from fnm
RUN cargo install fnm
COPY .nvmrc ./
RUN fnm install
RUN fnm exec npm install --global yarn

# Install node package dependencies
COPY yarn.lock package.json ./
RUN fnm exec yarn install --frozen-lockfile

COPY ./ /src/

# build react app including wasm library
ENV GENERATE_SOURCEMAP=false
ENV REACT_APP_LOCAL_AI=true
ENV REACT_APP_BUILD_VERSION ${BUILD_VERSION}
RUN fnm exec yarn run build --production

FROM rust:1.71 as server-builder

# Until feature(array_map) makes it to stable, requires nightly toolchain
RUN rustup default nightly

RUN apt-get update && apt-get install -y musl-dev musl-tools
# Install musl target for statically linked binaries
RUN rustup target add x86_64-unknown-linux-musl

# Install node package dependencies
WORKDIR /src
COPY Cargo.toml Cargo.lock /src/
COPY onitamalib /src/onitamalib
COPY onitamaserver /src/onitamaserver

# build onitamaserver binary
RUN cargo build --target x86_64-unknown-linux-musl --release --bin onitamaserver

FROM scratch

COPY --from=server-builder /src/target/x86_64-unknown-linux-musl/release/onitamaserver /
COPY --from=client-builder /src/build /build
ENV RUST_LOG=info
ENV PORT=80
EXPOSE 80
CMD ["/onitamaserver"]
