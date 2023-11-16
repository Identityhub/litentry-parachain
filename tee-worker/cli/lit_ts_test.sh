#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo ""
    echo "This is a script for tee-worker ts-test. Preparing to test: $1"
    echo ""

}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

echo "//npm.pkg.github.com/:_authToken=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" > /root/.npmrc
npm config set @litentry:registry https://npm.pkg.github.com
cd /ts-tests
npm config list
pnpm install
pnpm --filter integration-tests run $TEST:staging
