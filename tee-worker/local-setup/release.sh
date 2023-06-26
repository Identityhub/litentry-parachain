#!/bin/bash

export LOG="/var/log"
export TARGET_DIRECTORY="/usr/local/bin"

# TODO: Set Pipe fail if any of the commands fail
generate_service_file() {
  if [ "$#" -ne 4 ]; then
    echo "Usage: generate_service_file <service_name> <description> <command> <working_directory>"
    return 1
  fi

  local service_name="$1"
  local description="$2"
  local command="$3"
  local working_directory="$4"

  local service_template="[Unit]
Description=${description}

[Service]
ExecStart=${command}
WorkingDirectory=${working_directory}
Restart=always

[Install]
WantedBy=multi-user.target
"

  local service_filename="${service_name}.service"
  echo "$service_template" > "$service_filename"
  echo "Service file \"${service_filename}\" generated successfully."
}

generate_worker_service_file() {
  local command='./integritee-service --clean-reset -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev'
  local service_name='worker'
  local description='Worker Service for Litentry Side chain'
  local working_directory='/usr/local/bin'

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"
}

generate_upgrade_worker_service_file() {
  local command='./integritee-service -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev'
  local service_name='worker'
  local description='Worker Service for Litentry Side chain'
  local working_directory='/opt/worker/'

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"
}

generate_parachain_service_file() {
  local command='/bin/bash -c "cd /home/faisal/litentry-parachain && scripts/launch-local-binary.sh rococo"'
  local service_name='litentry-parachain'
  local description='Parachain Setup for Litentry'
  local working_directory=$PARACHAIN_SOURCE

  generate_service_file "${service_name}" "${description}" "${command}" "${working_directory}"

}

current_mrenclave(){
  # TODO: Correct Working Directory
  output=$(make mrenclave 2>&1)
  if [[ $? -eq 0 ]]; then
      mrenclave_value=$(echo "$output" | awk '{print $2}')
      echo "MRENCLAVE value: $mrenclave_value"
      export OLD_MRENCLAVE="$mrenclave_value"
  else
      echo "Failed to extract MRENCLAVE value."
  fi
}

enclave_account(){
  log=$(cd bin && ./integritee-service signing-key 2>&1)
  enclave_account=$(echo "$log" | awk '/Enclave account:/{print $NF}')
  if [[ -n $enclave_account ]]; then
      echo "Enclave account value: $enclave_account"
      export ENCLAVE_ACCOUNT="$enclave_account"
      echo "ENCLAVE_ACCOUNT exported successfully."
  else
      echo "Failed to extract Enclave account value."
  fi
}

new_mrenclave(){
  output=$(make mrenclave 2>&1)
  if [[ $? -eq 0 ]]; then
      mrenclave_value=$(echo "$output" | awk '{print $2}')
      echo "MRENCLAVE value: $mrenclave_value"
      export NEW_MRENCLAVE="$mrenclave_value"
  else
      echo "Failed to extract MRENCLAVE value."
  fi
}

latest_sync_block(){
  # Fetch Latest Block Produced
  if [ "$PRODUCTION" = "1" ]; then
    line=$(grep '\[.*\]$' /data/log/worker0.log | tail -n 1 2>&1)
    number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
    current_sidechain_end_block=$((number + 50))
    echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"
    export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"
  else
    line=$(grep '\[.*\]$' log/worker0.log | tail -n 1 2>&1)
    number=$(echo "$line" | sed -E 's/.*\[([0-9]+)\]$/\1/')
    current_sidechain_end_block=$((number + 50))
    echo "The next enclave is scheduled to start producing blocks after: $current_sidechain_end_block blocks"
    export SCHEDULE_UPDATE_BLOCK="$current_sidechain_end_block"
  fi
}

start_service_files(){
  echo "Moving worker binaries to /usr/local/bin"
  setup_working_dir "bin" "/usr/local/bin"
  echo "Moving service files to /etc/systemd/system"
  cp -r *.service /etc/systemd/system/

  echo "Performing Daemon Reload"
  systemctl daemon-reload

  echo "Starting Parachain Service"
  systemctl start litentry-parachain.service

  echo "Sleep for 60s, Parachain can be started"
  sleep 60
  # TODO: Check for block finalization instead of 60s
  echo "Starting Working Service"
  systemctl start worker.service
  # TODO: Check for block production via Logs

  echo "Parachain and Worker Service have started succesfully, You can check logs at /data/logs"

}

perform_upgrade(){
  echo "Setting up the new Worker on Chain"
  ../scripts/ts-utils/setup_enclave.sh

  echo "Waiting for the old worker to stop producing blocks"
  scripts/litentry/stop_old_worker.sh

  echo "Performing migration for the worker"
  scripts/litentry/migrate_worker.sh
}

upgrade_worker(){
  current_mrenclave
  enclave_account

  export SGX_COMMERCIAL_KEY="/home/faisal/litentry-parachain/tee-worker/enclave-runtime/Enclave_private.pem"
  export SGX_PRODUCTION="1"
  make

  new_mrenclave
  latest_sync_block
  perform_upgrade

  if [ "$PRODUCTION" = "1" ]; then
      generate_upgrade_worker_service_file
      cp worker.service /etc/systemd/system/
      systemctl daemon-reload
      systemctl start worker.service
  else
      export RUST_LOG='info,integritee_service=debug,ws=warn,sp_io=error,substrate_api_client=warn,
      itc_parentchain_light_client=info,
      jsonrpsee_ws_client=warn,jsonrpsee_ws_server=warn,enclave_runtime=debug,ita_stf=debug,
      its_rpc_handler=warn,itc_rpc_client=warn,its_consensus_common=debug,its_state=warn,
      its_consensus_aura=warn,aura*=warn,its_consensus_slots=warn,
      itp_attestation_handler=debug,http_req=debug,lc_mock_server=warn,itc_rest_client=debug,
      lc_credentials=debug,lc_identity_verification=debug,lc_stf_task_receiver=debug,lc_stf_task_sender=debug,
      lc_data_providers=debug,itp_top_pool=debug,itc_parentchain_indirect_calls_executor=debug'

      echo "Starting new worker"
      cd tmp/w0

      # Redirect stdout to a log file
      log_file="../../log/worker0.log"

      exec ./integritee-service -P 2000 -w 2001 -r 3443 -h 4545 --running-mode mock --enable-mock-server --parentchain-start-block 0 run --skip-ra --dev >"$log_file" 2>&1
  fi
}

setup_working_dir() {
    source_dir=$1
    target_dir=$2

    optional=("key.txt" "spid.txt")

    for file in "${optional[@]}"; do
        source="${source_dir}/${file}"
        target="${target_dir}/${file}"

        if [ -f "$source" ]; then
            cp "$source" "$target"
        else
            echo "$source does not exist, this is fine, but you can't perform remote attestation with this."
        fi
    done

    mandatory=("enclave.signed.so" "integritee-service")

    for file in "${mandatory[@]}"; do
        source="${source_dir}/${file}"
        target="${target_dir}/${file}"

        if [ -f "$source" ]; then
            cp "$source" "$target"
        else
            echo "$source does not exist. Did you run make?"
        fi
    done
}

# Example usage
# generate_service_file "my_service" "My Service Description" "/path/to/my_script.sh" "/path/to/working_directory"
if [ "$PRODUCTION" = "1" ]; then
    echo "Running in production mode."
    generate_worker_service_file
    generate_parachain_service_file
else
    echo "Not running in production mode."
fi

if [ "$AUTO_START" = "true" ]; then
  # Start the services
  if [ "$PRODUCTION" = "1" ]; then
    echo "Starting Services"
    start_service_files
  else
    ./local-setup/launch.sh
  fi
fi

if [ "$UPGRADE_WORKER" = "true" ]; then
  # Assuming the worker has already stopped
  echo "Preparing to Upgrade Worker"
  upgrade_worker
fi

# TODO: Need to also perform clean reset of the service
if [ "$CLEAN_RESET" = "true" ]; then
  # Assuming the worker has already stopped
  echo "Preparing to Upgrade Worker"
fi