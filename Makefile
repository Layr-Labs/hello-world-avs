############################# HELP MESSAGE #############################
# Make sure the help command stays first, so that it's printed by default when `make` is called without arguments
.PHONY: help tests
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


RUST_BINDINGS_PATH:=operator/rust/crates/utils/src/bindings

-----------------------------: ##

___ANVIL_STATE___: ##
build-anvil-state-with-deployed-contracts: ## builds anvil state with deployed contracts and generates a state
	@chmod +x ./contracts/anvil/build-state.sh
	./contracts/anvil/build-state.sh

___CONTRACTS___: ##

build-contracts: ## builds all contracts
	cd contracts && forge build

deploy-eigenlayer-contracts:
	@chmod +x ./contracts/anvil/deploy-el.sh
	./contracts/anvil/deploy-el.sh

deploy-helloworld-contracts:
	@chmod +x ./contracts/anvil/deploy-helloworld.sh
	./contracts/anvil/deploy-helloworld.sh

generate-bindings:
	cd contracts && forge build --force --skip test --skip script
	rm -rf ${RUST_BINDINGS_PATH}
	forge bind --alloy --skip-build --overwrite --module \
		--root contracts/  \
		--bindings-path ${RUST_BINDINGS_PATH} \
		--select '^ECDSAStakeRegistry$$' --select '^HelloWorldServiceManager$$'

__CLI__: ##

send-fund: ## sends fund to the operator saved in tests/keys/test.ecdsa.key.json
	cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 --value 10ether \
		--private-key 

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

start-rust-challenger: ## start challenger (part of quickstart)
	cargo run --bin challenger

__REWARDS__: ##

TOKEN_ADDRESS=$(shell jq -r '.addresses.token' contracts/deployments/hello-world/31337.json)

create-avs-distributions-root:
	npm run create-distributions-root

claim-distributions:
	npm run claim-distributions

create-operator-directed-distributions-root:
	npm run create-operator-directed-distributions-root

get-deployed-token-address:
	@echo "Deployed token Address: $(TOKEN_ADDRESS)"

claimer-account-token-balance:
	cast balance --erc20 $(TOKEN_ADDRESS) 0x0000000000000000000000000000000000000001
