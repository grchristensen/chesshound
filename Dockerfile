FROM rust:1.54 AS environment

# set working directory
WORKDIR /app


FROM environment as source

COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock


FROM source as bin-compile

RUN cargo build --release


FROM environment as bin

COPY --from=bin-compile /app/target/release/chesshound ./

COPY docker/entrypoint.sh .
RUN chmod +x entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]


FROM source as fmt

RUN rustup component add rustfmt

COPY docker/fmt_entrypoint.sh .
RUN chmod +x fmt_entrypoint.sh

ENTRYPOINT ["./fmt_entrypoint.sh"]


FROM source AS test

RUN cargo build --tests

COPY docker/test_entrypoint.sh .
RUN chmod +x test_entrypoint.sh

ENTRYPOINT ["./test_entrypoint.sh"]
