#!/bin/bash

set -euo pipefail

# ------------------------------
# path setting
# ------------------------------

ROOTDIR=$(git rev-parse --show-toplevel)
BASEDIR=/opt/litentry
PARACHAIN_BASEDIR="$BASEDIR/parachain"
WORKER_BASEDIR="$BASEDIR/worker"
LOG_BACKUP_BASEDIR="$BASEDIR/log-backup"
WORKER_BACKUP_BASEDIR="$BASEDIR/worker-backup"
RELAYCHAIN_ALICE_BASEDIR="$PARACHAIN_BASEDIR/relay-alice"
RELAYCHAIN_BOB_BASEDIR="$PARACHAIN_BASEDIR/relay-bob"
PARACHAIN_ALICE_BASEDIR="$PARACHAIN_BASEDIR/para-alice"

# ------------------------------
# default arg setting
# ------------------------------

BUILD=false
DISCARD=false
WORKER_CONFIG=
CHAIN=rococo
ONLY_WORKER=false
PARACHAIN_HOST=localhost
PARACHAIN_PORT="9944"
DOCKER_IMAGE=litentry/litentry-parachain:tee-prod
COPY_FROM_DOCKER=false
PRODUCTION=false
ACTION=

# ------------------------------
# Some global setting
# ------------------------------

ENCLAVE_ACCOUNT=
WORKER_COUNT=
PARACHAIN_ID=
OLD_MRENCLAVE=
NEW_MRENCLAVE=
OLD_SHARD=
LATEST_FINALIZED_BLOCK=

# ------------------------------
# main()
# ------------------------------

function main {
  # 1/ create folders if missing
  for d in "$BASEDIR" "$LOG_BACKUP_BASEDIR" "$RELAYCHAIN_ALICE_BASEDIR" "$RELAYCHAIN_BOB_BASEDIR" \
    "$PARACHAIN_ALICE_BASEDIR" "$WORKER_BASEDIR"; do
    mkdir -p "$d"
  done

  # 2/ parse command lines
  echo "Parsing command line ..."
  while [ $# -gt 0 ]; do
    case "$1" in
      -h|--help)
        display_help
        exit 0
        ;;
      -b|--build)
        BUILD=true
        shift
        ;;
      -d|--discard)
        DISCARD=true
        shift
        ;;
      -c|--config)
        WORKER_CONFIG="$(realpath -s $2)"
        shift 2
        ;;
      -a|--only-worker)
        ONLY_WORKER=true
        shift
        ;;
      -x|--chain)
        CHAIN="$2"
        shift
        ;;
      -p|--parachain-port)
        PARACHAIN_PORT="$2"
        shift
        ;;
      -z|--parachain-host)
        PARACHAIN_HOST="$2"
        shift
        ;;
      -v|--copy-from-docker)
        COPY_FROM_DOCKER=true
        DOCKER_IMAGE="$2"
        shift
        ;;
      --prod)
        PRODUCTION=true
        shift
        ;;
      generate|restart|upgrade-worker)
        ACTION="$1"
        shift
        ;;
      *)
        echo "Error: unknown option or subcommand $1"
        display_help
        exit 1
        ;;
    esac
  done

  # 3/ sanity checks
  if [ ! -f "$WORKER_CONFIG" ]; then
    echo "Worker config not found: $WORKER_CONFIG"
    exit 1
  fi

  WORKER_COUNT=$(cat "$WORKER_CONFIG" | jq '.workers | length')
  echo "Worker count: $WORKER_COUNT"

  # TODO: check flags conflict, e.g.
  # - having `--discard` together with `upgrade-worker` doesn't make sense
  # - `upgrade-worker` should ignore the `--only-worker` flag

  # 4/ back up logs and workers
  backup_logs
  backup_workers

  # 5/ main business logic
  case "$ACTION" in
    generate)
      backup_services
      generate_services
      exit
      ;;
    restart)
      stop_services
      prune
      build
      setup_working_dir
      restart_services
      exit
      ;;
    upgrade-worker)
      stop_services
      get_old_mrenclave
      build
      # TODO: actually we only need the copy-up
      setup_working_dir
      upgrade_worker
      restart_services
      exit
      ;;
    *)
      echo "Unknown action: $ACTION"
      exit 1 ;;
  esac
}

# ------------------------------
# helper functions
# ------------------------------

function print_divider {
  echo "------------------------------------------------------------"
}

function display_help {
  echo "usage: ./deploy.sh <subcommands> [options]"
  echo ""
  echo "subcommands:"
  echo "  generate           Generate the parachain and worker systemd files"
  echo "  restart            Restart the services"
  echo "  upgrade-worker     Upgrade the worker"
  echo ""
  echo "options:"
  echo "  -h, --help                  Display this help message and exit"
  echo "  -b, --build                 Build the parachain and worker binaries (default: false)"
  echo "  -d, --discard               Clean the existing state for parachain and worker (default: false)"
  echo "  -c, --config <config.json>  Config file for the worker"
  echo "  -a, --only-worker           Start only the worker (default: false)"
  echo "  -x, --chain                 Chain type for launching the parachain network (default: rococo)"
  echo "  -h, --parachain-host        Parachain ws URL (default: localhost)"
  echo "  -p, --parachain-port        Parachain ws port (default: 9944)"
  echo "  -v, --copy-from-docker      Copy the parachain binary from a docker image (default: litentry/litentry-parachain:tee-prod)"
  echo "  --prod                      Use a prod configuration to build and run the worker (default: false)"
  echo ""
  echo "examples:"
  echo "  ./deploy.sh restart --build --config github-staging-one-worker.json"
  echo "  ./deploy.sh restart --build --config github-staging-one-worker.json --discard"
  echo "  ./deploy.sh upgrade-worker --build --config github-staging-one-worker.json"
  echo ""
  echo "notes:"
  echo "  - This script requires an OS that supports systemd."
  echo "  - It is mandatory to provide a JSON config file for the worker."
  echo "  - jq is required to be installed on the system "
  echo ""
  echo "For more information or assistance, please contact Litentry parachain team."
}

function backup_logs {
  echo "Backing up logs ..."
  now=$(date +"%Y%m%d-%H%M%S")
  outdir="$LOG_BACKUP_BASEDIR/log-$now"
  mkdir -p "$outdir"
  [ -d "$ROOTDIR/tee-worker/log" ] && cp -r "$ROOTDIR/tee-worker/log" "$outdir"
  cp "$PARACHAIN_BASEDIR/*.log" "$outdir" || true
  echo "Logs backed up into $outdir"
}

function backup_workers {
  echo "Backing up workers ..."
  now=$(date +"%Y%m%d-%H%M%S")
  cd "$WORKER_BASEDIR" || exit
  for i in $(ls -d * 2>/dev/null); do
    outdir="$WORKER_BACKUP_BASEDIR/$i-$now"
    cp -rf "$i" "$outdir"
    echo "Worker backed up into $outdir"
  done
}

function backup_services {
  echo "Backing up services ..."
  now=$(date +"%Y%m%d-%H%M%S")
  cd /etc/systemd/system || exit
  outdir="$WORKER_BACKUP_BASEDIR/service-$now"
  mkdir -p "$outdir"
  for f in para-alice.service relay-alice.service relay-bob.service $(ls worker*.service 2>/dev/null); do
    cp "$f" "$outdir" || true
  done
}

function prune {
  if [ "$DISCARD" = true ]; then
    echo "Pruning the existing state ..."
    rm -rf "$PARACHAIN_BASEDIR/*"
    rm -rf "$WORKER_BASEDIR/*"
  fi
}

function generate_services {
  echo "Generating systemd service files ..."
    cd "$ROOTDIR/tee-worker/scripts/litentry/release"
    cp template/* .
    sed -i "s/CHAIN/$CHAIN/g" *.service
    for ((i = 0; i < WORKER_COUNT; i++)); do
      cp worker.service worker$i.service
      sed -i "s/NUMBER/$i/g" worker$i.service
      # populate args
      flags=$(cat "$WORKER_CONFIG" | jq -r ".workers[$i].flags[]")
      subcommand_flags=$(cat "$WORKER_CONFIG" | jq -r ".workers[$i].subcommand_flags[]")
      args=
      for flag in $flags; do
        args+=" $flag"
      done
      args+=" run"
      for subcommand_flag in $subcommand_flags; do
        args+=" $subcommand_flag"
      done
      sed -i "s/ARGS/$args/" worker$i.service
    done
    rm worker.service
    cp *.service -f /etc/systemd/system/
    rm *.service
    echo "Done, please check files under /etc/systemd/system/"
    echo "Restart the services to take effect"
}

function build {
  if [ "$BUILD" = true ]; then
    echo "Building the parachain and worker binaries ..."

    # download polkadot
    echo "Downloading polkadot binary ..."
    url="https://github.com/paritytech/polkadot/releases/download/v0.9.39/polkadot"
    polkadot_bin="$PARACHAIN_BASEDIR/polkadot"
    wget -O "$polkadot_bin" -q "$url"
    chmod a+x "$polkadot_bin"
    if [ ! -s "$polkadot_bin" ]; then
      echo "$polkadot_bin is 0 bytes, download URL: $url" && exit 1
      exit 1
    fi
    if ! "$polkadot_bin" --version &> /dev/null; then
      echo "Cannot execute $polkadot_bin, wrong executable?" && exit 1
    fi

    # pull or build parachain
    if [ "$COPY_FROM_DOCKER" = true ]; then
      echo "Pulling binary from $DOCKER_IMAGE ..."
      docker pull "$DOCKER_IMAGE"
      docker cp "$(docker create --rm $DOCKER_IMAGE):/usr/local/bin/litentry-collator" "$PARACHAIN_BASEDIR"
    else
      echo "Building parachain binary ..."
      cd "$ROOTDIR" || exit
      if [ "$PRODUCTION" = true ]; then
        cargo build --locked --profile production
      else
        make build-node
      fi
      cp "$ROOTDIR/target/release/litentry-collator" "$PARACHAIN_BASEDIR"
    fi
    chmod a+x "$PARACHAIN_BASEDIR/litentry-collator"

    # build worker
    echo "Building worker ..."
    cd $ROOTDIR/tee-worker/ || exit 
    source /opt/intel/sgxsdk/environment
    if [ "$PRODUCTION" = 1 ]; then
      # we will get an error if SGX_COMMERCIAL_KEY is not set for prod
      export SGX_PRODUCTION=1
    fi
    make
  fi
}

function restart_services {
  systemctl daemon-reload
  if [ "$ONLY_WORKER" = false ]; then
    echo "Restarting parachain services ..."

    cd "$PARACHAIN_BASEDIR" || exit 
    ./polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-chain-spec.json
    ./litentry-collator export-genesis-state --chain $CHAIN-dev > genesis-state
    ./litentry-collator export-genesis-wasm --chain $CHAIN-dev > genesis-wasm

    systemctl restart relay-alice.service
    sleep 10
    systemctl restart relay-bob.service
    sleep 10
    systemctl restart para-alice.service
    sleep 10
    register_parachain
  fi

  echo "Restarting worker services ..."
  for ((i = 0; i < WORKER_COUNT; i++)); do
    systemctl restart "worker$i.service"
  done
  echo "Done"
}

function stop_services {
  echo "Stopping worker services ..."
  for ((i = 0; i < WORKER_COUNT; i++)); do
    systemctl stop "worker$i.service"
  done

  sleep 10

  # TODO: it means we can't stop parachain service alone
  #       this needs to be done directly via `systemctl`
  if [ "$ONLY_WORKER" = false ]; then
    echo "Stopping parachain services ..."
    systemctl stop para-alice.service
    systemctl stop relay-alice.service
    systemctl stop relay-bob.service
  fi
}

function register_parachain {
  echo "Register parathread now ..."
  cd "$ROOTDIR" || exit
  export PARACHAIN_ID=$(grep DEFAULT_PARA_ID node/src/chain_specs/$CHAIN.rs  | grep u32 | sed 's/.* = //;s/\;//')
  cd "$ROOTDIR/ts-tests" || exit 
  if [[ -z "$NODE_ENV" ]]; then
      echo "NODE_ENV=ci" > .env
  else
      echo "NODE_ENV=$NODE_ENV" > .env
  fi
  # The genesis state path file needs to be updated as it is hardcoded to be /tmp/parachain_dev 
  jq --arg genesis_state "$PARACHAIN_BASEDIR/genesis-state" --arg genesis_wasm "$PARACHAIN_BASEDIR/genesis-wasm" '.genesis_state_path = $genesis_state | .genesis_wasm_path = $genesis_wasm' config.ci.json > updated_config.json
  mv updated_config.json config.ci.json 
  corepack yarn
  corepack yarn register-parathread 2>&1 | tee "$PARACHAIN_BASEDIR/register-parathread.log"
  print_divider

  echo "Upgrade parathread to parachain now ..."
  # Wait for 90s to allow onboarding finish, after that we do the upgrade
  sleep 90
  corepack yarn upgrade-parathread 2>&1 | tee "$PARACHAIN_BASEDIR/upgrade-parathread.log"
  print_divider

  echo "done. please check $PARACHAIN_BASEDIR for generated files if need"
  print_divider
}

function setup_working_dir {
    cd "$ROOT_DIR/tee-worker/bin" || exit

    if [ "$PRODUCTION" = false ]; then
      for f in 'key.txt' 'spid.txt'; do
        [ -f "$f" ] || touch "$f"
      done
    fi

    for ((i = 0; i < WORKER_COUNT; i++)); do
      worker_dir="$WORKER_BASEDIR/w$i"
      mkdir -p "$worker_dir"
      for f in 'key.txt' 'spid.txt' 'enclave.signed.so' 'litentry-worker'; do
        [ -f "$f" ] && cp -f "$f" "$worker_dir"
      done

      cd "$worker_dir"
      [ -f light_client_db.bin.1 ] && cp -f light_client_db.bin.1 light_client_db.bin

      enclave_account=$(./litentry-worker signing-key | grep -oP '^Enclave account: \K.*$$')

      if [ "$PRODUCTION" = true ]; then
        echo "Transferring balance to the enclave account $enclave_account ..."
        cd $ROOTDIR/scripts/ts-utils/ || exit
        yarn install
        npx ts-node transfer.ts $enclave_account
      fi
    done
}

function get_old_mrenclave {
  # TODO: this is not entirely correct
  #       the tee-worker/ folder must be of old state
  cd $ROOTDIR/tee-worker || exit 
  OLD_MRENCLAVE=$(make mrenclave 2>&1 | grep MRENCLAVE | awk '{print $2}')

  cd bin || exit 
  OLD_SHARD=$(./litentry-worker mrenclave)
}

function upgrade_worker {
  echo "Upgrading worker ..."
  cd $ROOTDIR/tee-worker || exit 
  NEW_MRENCLAVE=$(make mrenclave 2>&1 | grep MRENCLAVE | awk '{print $2}')

  echo "Fetching Enclave Signing Key"
  log=$(cd bin && ./litentry-worker signing-key 2>&1)
  enclave_account=$(echo "$log" | awk '/Enclave account:/{print $NF}')

  if [[ -n $enclave_account ]]; then
      echo "Enclave account value: $enclave_account"
      export ENCLAVE_ACCOUNT="$enclave_account"
      echo "ENCLAVE_ACCOUNT exported successfully."
  else
      echo "Failed to extract Enclave account value."
  fi

  latest_sidechain_block
  latest_parentchain_block

  echo "Setting up the new worker on chain ..."
  cd $ROOTDIR/ts-tests/ || exit 
  corepack yarn install
  corepack yarn setup-enclave $NEW_MRENCLAVE $SCHEDULED_UPDATE_BLOCK

  # TODO: make sure the worker is stopped
  migrate_worker
}

# TODO: only works for w0
function migrate_worker {
  echo "Migrating shards for new worker ..."
  cd "$WORKER_BASEDIR/w0" || exit 

  echo "old MRENCLAVE: $OLD_MRENCLAVE"
  echo "new MRENCLAVE: $NEW_MRENCLAVE"
  ./litentry-worker migrate-shard --old-shard $OLD_MRENCLAVE --new-shard $NEW_MRENCLAVE

  cd shards || exit
  rm -rf $OLD_SHARD
  echo "Done"
}

# TODO: here we only read worker0 logs here
function latest_sidechain_block {
  block_number=$(grep -F 'Enclave produced sidechain blocks' $WORKER_BASEDIR/w0/log/worker.log | tail -n 1 | sed 's/.*\[//;s/]//')
  SCHEDULED_UPDATE_BLOCK=$((block_number + 50))
  echo "Current sidechain block: $block_number, scheduled update block: $SCHEDULED_UPDATE_BLOCK"
}

function latest_parentchain_block {
  # JSON-RPC request payload
  request='{"jsonrpc":"2.0","id":1,"method":"chain_getHeader","params":[]}'

  # Make the JSON-RPC request and retrieve the latest finalized block
  response=$(curl -s -H "Content-Type: application/json" -d "$request" http://$PARACHAIN_HOST:$PARACHAIN_PORT)
  hex_number=$(echo "$response" | grep -oP '(?<="number":")[^"]+')
  LATEST_FINALIZED_BLOCK=$(printf "%d" "$hex_number")
  echo "Current parachain block: $LATEST_FINALIZED_BLOCK"
}

main "$@"