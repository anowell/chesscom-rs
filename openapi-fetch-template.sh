#!/bin/env bash

DIR=$(dirname "$(readlink -f "$0")")

docker run --rm -v $DIR:/local openapitools/openapi-generator-cli author \
  template \
	-g rust \
	-o /local/templates

sudo chown -R $(id -u):$(id -g) $DIR/templates
