############################# HELP MESSAGE #############################
# Make sure the help command stays first, so that it's printed by default when `make` is called without arguments
.PHONY: help tests
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

AGGREGATOR_ECDSA_PRIV_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
CHALLENGER_ECDSA_PRIV_KEY=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a

CHAINID=31337
# Make sure to update this if the strategy address changes
# check in contracts/script/output/${CHAINID}/hello_world_avs_deployment_output.json
STRATEGY_ADDRESS=0x7a2088a1bFc9d81c55368AE168C2C02570cB814F
DEPLOYMENT_FILES_DIR=contracts/script/output/${CHAINID}


-----------------------------: ##

___CONTRACTS___: ##

build-contracts: ## builds all contracts
	cd contracts && forge build

deploy-eigenlayer-contracts:
						@chmod +x ./contracts/anvil/deploy-el.sh
						./contracts/anvil/deploy-el.sh

deploy-helloworld-contracts:
						@chmod +x ./contracts/anvil/deploy-helloworld.sh
						./contracts/anvil/deploy-helloworld.sh

__CLI__: ##

send-fund: ## sends fund to the operator saved in tests/keys/test.ecdsa.key.json
	cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 --value 10ether --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

-----------------------------: ##
# We pipe all zapper logs through https://github.com/maoueh/zap-pretty so make sure to install it
# TODO: piping to zap-pretty only works when zapper environment is set to production, unsure why
____OFFCHAIN_SOFTWARE___:
start-operator: ## start operator (part of quickstart)
	tsc && node dist/index.js

spam-tasks: ## start tasks spamming (part of quickstart)
	tsc && node dist/createNewTasks.js

-----------------------------: ##
_____HELPER_____: ##
tests-contract: ## runs all forge tests
	cd contracts && forge test

___RUST_OFFCHAIN_SOFTWARE___:
start-rust-operator: ## start operator (part of quickstart) 
	cargo run --bin start_operator

spam-rust-tasks:  ## start tasks spamming (part of quickstart)
	cargo run --bin spam_tasks
