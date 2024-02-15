all: 
	@echo "Cleaning forge artifacts and installing dependencies."
	forge clean
	forge install
	@echo "Dependency installation complete. Building project artifacts."
	forge build --via-ir
	forge bind --via-ir --revert-strings debug -b crates/bindings --crate-name bindings --overwrite --force
	@echo "Build complete. You're welcome ya' filthy animal!"

build:
	@echo "Building project artifacts."
	forge bind --via-ir --revert-strings debug -b crates/bindings --crate-name bindings --overwrite --force
	@echo "Build complete. You're welcome ya' filthy animal!"

clean:
	@echo "Cleaning repository."
	forge clean
	@echo "Clean complete. You made a mess!"
