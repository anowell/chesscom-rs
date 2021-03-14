# Chesscom

[![Crate Badge](https://buildstats.info/crate/chesscom)](https://crates.io/crates/chesscom)

Easy-to-use rust client for the [chess.com](chess.com) [data API](https://www.chess.com/news/view/published-data-api).

**Status**: Work-in-progress prototype...

The repo includes a WIP [openapi spec for chess.com](chess-openapi-spec.yaml) which is used to generate an internal `chesscom-openapi` crate.

## Usage

For basic usage, see [player-dump.rs](examples/player-dump.rs) which prints select details and stats about a user:

```
$ player-dump rgnever
Username: rgnever
Status: premium
Name: Anthony Nowell
Joined: 2020-11-12
Last Online: 0 mins ago
Rapid: 1119 rating (rd=208) with 2-0-0 record
Blitz: 530 rating (rd=68) with 29-21-3 record
```

## Build

The `chesscom-openapi` crate is not committed, so you must run `openapi-gen.sh` (requires docker) to generate it. Then the ergonomic wrapper can be built.

```
./openapi-gen.sh
cargo build
```

(Last confirmed working with `openapitools/openapi-generator-cli:b531042729e8`)
