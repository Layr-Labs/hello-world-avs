# Rust
FROM rust:1.81 AS rs-builder
WORKDIR /usr/src/hello-world
COPY ./Cargo.toml ./Cargo.lock rust-toolchain.toml ./
COPY ./operator/rust ./operator/rust
RUN cargo build --release --bin start_operator --bin spam_tasks

FROM ubuntu:24.04 AS operator-rs
WORKDIR /app
COPY --from=rs-builder /usr/src/hello-world/target/release/start_operator /usr/local/bin/start_operator
CMD ["start_operator"]

FROM ubuntu:24.04 AS traffic-generator-rs
WORKDIR /app
COPY --from=rs-builder /usr/src/hello-world/target/release/spam_tasks /usr/local/bin/spam_tasks
CMD ["spam_tasks"]


# TypeScript
FROM node:22 AS operator-ts

WORKDIR /app

COPY package.json ./
COPY tsconfig.json ./
RUN npm install

COPY . .

CMD ["npm", "start"]
