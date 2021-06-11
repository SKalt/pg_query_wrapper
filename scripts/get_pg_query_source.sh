#!/bin/bash
LIB_PG_QUERY_TAG="${LIB_PG_QUERY_TAG:-13-2.0.4}"
TMP_ZIP=/tmp/libpg_query.tar.gz
INTERMEDIATE_DIR="/libpg_query"
TARGET_DIR=/parser

download_src() {
  local tag="${1:-LIB_PG_QUERY_TAG}"
  local tmp_zip="${2:-$TMP_ZIP}"
  local url="https://codeload.github.com/pganalyze/libpg_query/tar.gz/${tag}"
  curl --silent --disable -o "$tmp_zip" "$url"
}

untar_to() {
  local tmp_zip="${1:-$TMP_ZIP}"
  local intermediate_dir="${2:-$INTERMEDIATE_DIR}"
  mkdir -p "$intermediate_dir" &&
    cd "$intermediate_dir" &&
    tar -xzf "$tmp_zip" -C "$intermediate_dir" --strip-components=1
}

consolidate_to_single_dir() {
  local intermediate_dir="${1:-$INTERMEDIATE_DIR}"
  local target_dir="${2:-$TARGET_DIR}"
  mkdir -p "$target_dir"                                                             &&
    cp -a "$intermediate_dir"/src/* "$target_dir"/                                   &&
    rm "$target_dir"/pg_query_outfuncs_protobuf_cpp.cc                               &&
    mv "$target_dir"/postgres/* "$target_dir"/                                       &&
    rmdir "$target_dir"/postgres                                                     &&
    cp -a "$intermediate_dir"/pg_query.h "$target_dir"/include                       &&
    mv "$target_dir"/*{_conds,_defs,_helper}.c "$target_dir"/include                 &&
    mkdir -p "$target_dir"/include/protobuf                                          &&
    cp -a "$intermediate_dir"/protobuf/pg_query.proto "$target_dir"/include/protobuf &&
    cp -a "$intermediate_dir"/protobuf/*.h "$target_dir"/include/protobuf            &&
    cp -a "$intermediate_dir"/protobuf/*.c "$target_dir"/                            &&
    mkdir -p "$target_dir"/include/protobuf-c                                        && # Protobuf library code
    cp -a "$intermediate_dir"/vendor/protobuf-c/*.h "$target_dir"/include            &&
    cp -a "$intermediate_dir"/vendor/protobuf-c/*.h "$target_dir"/include/protobuf-c &&
    cp -a "$intermediate_dir"/vendor/protobuf-c/*.c "$target_dir"/                   &&
    mkdir -p "$target_dir"/include/xxhash                                            && # xxhash library code
    cp -a "$intermediate_dir"/vendor/xxhash/*.h "$target_dir"/include                &&
    cp -a "$intermediate_dir"/vendor/xxhash/*.h "$target_dir"/include/xxhash         &&
    cp -a "$intermediate_dir"/vendor/xxhash/*.c "$target_dir"/
  # Other support files
}


cleanup() {
  rm -fr "$TMP_ZIP" "$INTERMEDIATE_DIR"
}

main() {
  set -x -euo pipefail
  download_src "$LIB_PG_QUERY_TAG" $TMP_ZIP
  untar_to $TMP_ZIP "$INTERMEDIATE_DIR"
  consolidate_to_single_dir "$INTERMEDIATE_DIR" "$TARGET_DIR"
  cleanup
}

if [ "$0" = "${BASH_SOURCE[0]}" ]; then main "$@"; fi
