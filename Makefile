.PHONY: help
help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

.PHONY: prepare-circuit
prepare-circuit: clear-circuit-artifacts # Build the circuit artifacts
	@cd circuit && \
		npm install && \
		circom grayscale_step.circom --r1cs --wasm

.PHONY: clear-circuit-artifacts
clear-circuit-artifacts: # Clear the circuit artifacts
	@rm -rf circuit/grayscale_step_js circuit/grayscale_step.r1cs

.PHONY: run
run: # Run the experiment
	@cargo run --release