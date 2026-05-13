FROM rust:1.89

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY apps ./apps
COPY crates ./crates
COPY workers ./workers
COPY scripts ./scripts
COPY schemas ./schemas
COPY docs ./docs
COPY README.md ./

RUN cargo build --workspace

CMD ["cargo", "run", "-p", "dn-cli", "--", "health"]
