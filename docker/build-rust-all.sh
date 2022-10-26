#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}
FEATURES=${FEATURES:-""}

echo "Building all rust-based docker images"
echo "PROFILE: $PROFILE"
echo "FEATURES: $FEATURES"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p pont \
    -p pont-faucet \
    -p pont-node-checker \
    -p pont-openapi-spec-generator \
    -p pont-telemetry-service \
    -p pont-fn-check-client \
    -p backup-cli \
    -p db-bootstrapper \
    -p forge-cli \
    -p transaction-emitter \
    "$@"

# Build pont-node separately
cargo build --locked --profile=$PROFILE \
    -p pont-node \
    "$@"

# Build and overwrite the pont-node binary with features if specified
if [ -n "$FEATURES" ]; then
    echo "Building pont-node with features ${FEATURES}"
    (cd pont-node && cargo build --profile=$PROFILE --features=$FEATURES "$@")
fi

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    pont
    pont-faucet
    pont-node
    pont-node-checker
    pont-openapi-spec-generator
    pont-telemetry-service
    pont-fn-check-client
    db-backup
    db-backup-verify
    db-bootstrapper
    db-restore
    forge
    transaction-emitter
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp target/$PROFILE/$BIN dist/$BIN
done

# Build the Pont Move framework and place it in dist. It can be found afterwards in the current directory.
(cd dist && cargo run --package framework -- release)
