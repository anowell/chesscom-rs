build-all: openapi
    cargo build

openapi:
    ./openapi-gen.sh

doc:
    @cd openapi && cargo doc --no-deps -q
    @echo file://{{justfile_directory()}}/openapi/target/doc/chesscom_openapi/index.html
    @cargo doc --no-deps -q
    @echo file://{{justfile_directory()}}/target/doc/chesscom/index.html

refresh-templates:
    docker run --rm -v {{justfile_directory()}}:/local \
      --user `id -u`:`id -g` \
      openapitools/openapi-generator-cli author \
      template \
      -g rust \
      -o /local/templates

