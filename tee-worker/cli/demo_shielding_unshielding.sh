#!/bin/bash

# Demonstrates how to shield tokens from the parentchain into the sidechain.
#
# setup:
# run all on localhost:
#   integritee-node purge-chain --dev
#   integritee-node --dev -lruntime=debug
#   rm light_client_db.bin
#   export RUST_LOG=integritee_service=info,ita_stf=debug
#   integritee-service init_shard
#   integritee-service shielding-key
#   integritee-service signing-key
#   integritee-service run
#
# then run this script

# usage:
#  demo_shielding_unshielding.sh -p <NODEPORT> -P <WORKERPORT> -t <TEST_BALANCE_RUN> -m file
#
# TEST_BALANCE_RUN is either "first" or "second"
# if -m file is set, the mrenclave will be read from file

while getopts ":m:p:P:t:u:V:C:" opt; do
    case $opt in
        t)
            TEST=$OPTARG
            ;;
        m)
            READMRENCLAVE=$OPTARG
            ;;
        p)
            NPORT=$OPTARG
            ;;
        P)
            WORKER1PORT=$OPTARG
            ;;
        u)
            NODEURL=$OPTARG
            ;;
        V)
            WORKER1URL=$OPTARG
            ;;
        C)
            CLIENT_BIN=$OPTARG
            ;;
    esac
done

# Using default port if none given as arguments.
NPORT=${NPORT:-9944}
NODEURL=${NODEURL:-"ws://127.0.0.1"}

WORKER1PORT=${WORKER1PORT:-2000}
WORKER1URL=${WORKER1URL:-"wss://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/integritee-cli"}

echo "Using client binary ${CLIENT_BIN}"
echo "Using node uri ${NODEURL}:${NPORT}"
echo "Using trusted-worker uri ${WORKER1URL}:${WORKER1PORT}"
echo ""

# the parachain LIT is 12 decimal
UNIT=$(( 10 ** 12 ))

# we have to make these amounts greater than ED, see
# https://github.com/litentry/litentry-parachain/issues/1162
AMOUNTSHIELD=$(( 6 * UNIT ))
AMOUNTTRANSFER=$(( 2 * UNIT ))
AMOUNTUNSHIELD=$(( 1 * UNIT ))

CLIENT="${CLIENT_BIN} -p ${NPORT} -P ${WORKER1PORT} -u ${NODEURL} -U ${WORKER1URL}"

echo "* Query on-chain enclave registry:"
WORKERS=$($CLIENT list-workers)
echo "WORKERS: "
echo "${WORKERS}"
echo ""

if [ "$READMRENCLAVE" = "file" ]
then
    read MRENCLAVE <<< $(cat ~/mrenclave.b58)
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # this will always take the first MRENCLAVE found in the registry !!
    read MRENCLAVE <<< $(echo "$WORKERS" | awk '/  MRENCLAVE: / { print $2; exit }')
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }

echo "* Get balance of Alice's on-chain account"
${CLIENT} balance "//Alice"
echo ""

echo "* Get balance of Bob's on-chain account"
${CLIENT} balance "//Bob"
echo ""

echo "* Create a new incognito account for Alice"
ICGACCOUNTALICE=//AliceIncognito
echo "  Alice's incognito account = ${ICGACCOUNTALICE}"
echo ""

echo "* Create a new incognito account for Bob"
ICGACCOUNTBOB=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} new-account)
echo "  Bob's incognito account = ${ICGACCOUNTBOB}"
echo ""

# Sometimes we get a nonce clash here, so let's wait a little bit to prevent that.
sleep 10

echo "* Shield ${AMOUNTSHIELD} tokens to Alice's incognito account"
${CLIENT} shield-funds //Alice ${ICGACCOUNTALICE} ${AMOUNTSHIELD} ${MRENCLAVE}
echo ""

echo "* Waiting 30 seconds"
sleep 30
echo ""

echo "* Get balance of Alice's on-chain account"
${CLIENT} balance "//Alice"
echo ""

echo "Get balance of Alice's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTALICE}
echo ""

echo "Get balance of Bob's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTBOB}
echo ""

echo "* Send ${AMOUNTTRANSFER} funds from Alice's incognito account to Bob's incognito account"
$CLIENT trusted --mrenclave ${MRENCLAVE} transfer ${ICGACCOUNTALICE} ${ICGACCOUNTBOB} ${AMOUNTTRANSFER}
echo ""

echo "* Get balance of Alice's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTALICE}
echo ""

echo "* Bob's incognito account balance"
${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTBOB}
echo ""

echo "* Un-shield ${AMOUNTUNSHIELD} tokens from Alice's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} --xt-signer //Alice unshield-funds ${ICGACCOUNTALICE} //Alice ${AMOUNTUNSHIELD}
echo ""

echo "* Waiting 30 seconds"
sleep 30
echo ""

echo "Get balance of Alice's incognito account"
BALANCE_INCOGNITO_ALICE=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTALICE}  | xargs)
echo $BALANCE_INCOGNITO_ALICE

echo "Get balance of Bob's incognito account"
BALANCE_INCOGNITO_BOB=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTBOB}  | xargs)
echo $BALANCE_INCOGNITO_BOB

echo "* Get balance of Alice's on-chain account"
${CLIENT} balance "//Alice"
echo ""


# The following tests are for automated CI.
# They only work if you're running from fresh genesis.
case $TEST in
    first)
        # Incognito Alice: 6 - 2 - 1 = 3
        # Incognito Bob:   2
        if [ "$BALANCE_INCOGNITO_ALICE" -eq $(( 3 * UNIT)) ] && [ "$BALANCE_INCOGNITO_BOB" -eq $(( 2 * UNIT)) ]; then
            echo "test passed (1st time)"
            exit 0
        else
            echo "test ran through but balance is wrong. have you run the script from fresh genesis?"
            exit 1
        fi
        ;;
    second)
        # Incognito Alice: 3 + 6 - 2 - 1 = 6 (doubled)
        # Incognito Bob:   2 (as Bob has a new account each time)
        if [ "$BALANCE_INCOGNITO_ALICE" -eq $(( 6 * UNIT)) ] && [ "$BALANCE_INCOGNITO_BOB" -eq $(( 2 * UNIT)) ]; then
            echo "test passed (2nd time)"
            exit 0
        else
            echo "test ran through but balance is wrong. is this really the second time you run this since genesis?"
            exit 1
        fi
        ;;
    *)
        echo "unsupported test mode"
        exit 1
        ;;
esac

exit 0
