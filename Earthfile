FROM rust:buster
RUN --mount=type=cache,target=/var/cache/apt apt update \
  && apt install --no-install-recommends -y curl ca-certificates

lib-pg-query:
  COPY ./scripts/get_pg_query_source.sh /tmp/
  ARG LIB_PG_QUERY_TAG=13-2.0.4
  ENV LIB_PG_QUERY_TAG=${LIB_PG_QUERY_TAG}
  RUN bash /tmp/get_pg_query_source.sh
  SAVE ARTIFACT /parser AS LOCAL ./parser

build-deps:
  RUN --mount=type=cache,target=/var/cache/apt apt update \
    && apt install --no-install-recommends -y clang
  RUN cargo install cargo-chef bindgen
  RUN rustup component add rustfmt
  SAVE IMAGE libpg_query_sys__build_deps

raw-bindings:
  FROM +build-deps
  COPY +lib-pg-query/parser /parser
  RUN bindgen ./parser/include/pg_query.h > /bindings.rs
  SAVE ARTIFACT /bindings.rs AS LOCAL ./src/bindings.rs

pb-codegen:
  FROM +build-deps
  RUN apt update && apt install -y --no-install-recommends protobuf-compiler
  RUN cargo install protobuf-codegen
  COPY +lib-pg-query/parser /parser
  RUN mkdir /pb
  RUN protoc --proto_path=/parser/include/protobuf --rust_out=/pb /parser/include/protobuf/pg_query.proto 
  SAVE ARTIFACT /pb AS LOCAL ./src/pbuf
