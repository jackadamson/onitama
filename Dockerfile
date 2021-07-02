FROM rust:1.53 as client-builder

# Install Node 14 and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor > /usr/share/keyrings/yarnkey.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" > /etc/apt/sources.list.d/yarn.list \
    && apt-get update \
    && apt-get install -y yarn nodejs

# Until feature(array_map) makes it to stable, requires nightly toolchain
RUN rustup default nightly

# Install node package dependencies
WORKDIR /src
COPY yarn.lock package.json ./
RUN yarn install

COPY ./ /src/

# Build react app including wasm library
ENV GENERATE_SOURCEMAP=false
RUN yarn run build --production

FROM rust:1.53 as server-builder

RUN apt-get update && apt-get install -y musl-dev musl-tools
# Until feature(array_map) makes it to stable, requires nightly toolchain
RUN rustup default nightly
# Install musl target for statically linked binaries
RUN rustup target add x86_64-unknown-linux-musl

# Install node package dependencies
WORKDIR /src
COPY Cargo.toml Cargo.lock /src/
COPY onitamalib /src/onitamalib
COPY onitamaserver /src/onitamaserver

# Build onitamaserver binary
RUN cargo build --target x86_64-unknown-linux-musl --release --bin onitamaserver

FROM scratch

COPY --from=server-builder /src/target/x86_64-unknown-linux-musl/release/onitamaserver /
COPY --from=client-builder /src/build /build
USER 1000
ENV RUST_LOG=info
EXPOSE 8080
CMD ["/onitamaserver"]
