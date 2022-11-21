#!/usr/bin/env bash

set -eo pipefail

# This script benchmarks the runtime or pallet weight locally.
#
# When benchmarking pallet weight, only our own pallets are supported.
# Therefore substrate (or other github) pallets are not supported:
# they are benchmarked by the source anyway (e.g. SubstrateWeigt)
# The `litentry-collator` binary must be compiled with `runtime-benchmarks` feature.
#
# When benchmarking runtime weight, a third parameter is needed to
# define the runtime: litentry or litmus.

function usage() {
    echo "Usage: $0 litentry|litmus pallet-name runtime|pallet"
}

[ $# -ne 3 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

PALLET=${2//-/_}

echo "running $1-$3 benchmark for $PALLET locally ..."

case "$3" in
    runtime)
        OUTPUT="--output=./runtime/$1/src/weights/$PALLET.rs"
        TEMPLATE=
        CHAIN="--chain=$1-dev"
        ;;
    pallet)
        OUTPUT="$(cargo pkgid -q $2 | sed 's/.*litentry-parachain/\./;s/#.*/\/src\/weights.rs/')"
        TEMPLATE="--template=./templates/benchmark/pallet-weight-template.hbs"
        CHAIN="--chain=$1-dev"
        if [[ $OUTPUT == *"github.com"* ]]; then
            echo "only local pallets can be benchmarked"
            exit 2
        else
            OUTPUT="--output=$OUTPUT"
        fi
        ;;
    *)
        usage
        exit 3
        ;;
esac

if [[ $PALLET == *"pallet-parachain-staking"* ]]; then

    echo "will run  $PALLET pallet benchmark code"
    STEPS=50
    REPEAT=20
    echo "${STEPS} <====> ${REPEAT} "
else
    echo "will run other $PALLET pallet benchmark code"
    STEPS=20
    REPEAT=50
    echo "${STEPS} <====> ${REPEAT} "
fi

./target/release/litentry-collator benchmark pallet \
      $CHAIN \
      --execution=wasm  \
      --db-cache=20 \
      --wasm-execution=compiled \
      --pallet="$PALLET" \
      --extrinsic=* \
      --heap-pages=4096 \
      --steps="${STEPS}" \
      --repeat="${REPEAT}" \
      --header=./LICENSE_HEADER \
      $TEMPLATE \
      $OUTPUT
