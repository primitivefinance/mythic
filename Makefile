all: 
	@echo "Cleaning forge artifacts and installing dependencies."
	forge clean
	forge install
	@echo "Dependency installation complete. Building project artifacts."
	forge build
	arbiter bind
	@echo "Build complete. You're welcome ya' filthy animal!"

build:
	@echo "Building project artifacts."
	forge build
	arbiter bind
	@echo "Build complete. You're welcome ya' filthy animal!"