#!/bin/env bash

DIR=$(dirname "$(readlink -f "$0")")

mkdir -p $DIR/openapi
docker run --rm -v $DIR:/local \
  --user $(id -u):$(id -g) \
  -w /local \
  openapitools/openapi-generator-cli generate \
  --enable-post-process-file \
	-i chess-openapi-spec.yaml \
	-g rust \
	-o openapi \
  -c config.yaml

## I really don't want my data structure to have integer timestamps
pushd $DIR/openapi
for field in date move_by last_activity start_time joined last_online; do
    # Struct timestamps first
    sed -i "s/pub $field: i32/#[serde(with = \"chrono::serde::ts_seconds\")]\n\tpub $field: chrono::DateTime<chrono::Utc>/g" src/**/*.rs
    sed -i "s/pub $field: Option<i32>/#[serde(with = \"chrono::serde::ts_seconds_option\")]\n\tpub $field: Option<chrono::DateTime<chrono::Utc>>/g" src/**/*.rs

    # method timestamps (non-optional only)
    sed -i "s/$field: i32/$field: chrono::DateTime<chrono::Utc>/g" src/**/*.rs
done

cargo check
popd
