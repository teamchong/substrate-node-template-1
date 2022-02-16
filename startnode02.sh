./target/release/node-template purge-chain --base-path /tmp/node02 --chain local -y
./target/release/node-template \
--base-path /tmp/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX

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

./target/release/node-template \
--base-path /tmp/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name MyNode02 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLmrYDLoNTyTYtRdDyZLWDe1paxzxTw5RgjmHLfzW96SX