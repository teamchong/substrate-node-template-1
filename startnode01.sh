./target/release/node-template purge-chain --base-path /tmp/node01 --chain local -y -y
./target/release/node-template \
--base-path /tmp/node01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode01
./target/release/node-template key insert --base-path /tmp/node02 \
--chain customSpecRaw.json \
--scheme Sr25519 \
--suri <second-participant-secret-seed> \
--password-interactive \
--key-type aura
./target/release/node-template key insert --base-path /tmp/node02 \
--chain customSpecRaw.json \
--scheme Ed25519 \
--suri <second-participant-secret-seed> \
--password-interactive \
--key-type gran

# ./target/release/node-template key insert --base-path /tmp/node01 \
# --chain customSpecRaw.json \
# --scheme Sr25519 \
# --suri <your-secret-seed> \
# --password-interactive \
# --key-type aura
