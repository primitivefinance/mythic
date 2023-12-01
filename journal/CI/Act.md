# Testing CI/CD Locally

To test CI/CD locally, you can use Docker and Act. Here are the steps:

1. Install Docker: You can download it from the official Docker website and follow the installation instructions.

2. Install Act: Act is a tool that allows you to run your GitHub Actions locally. You can install it by running `curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash`. You can also install with `brew install act` if you are on a Mac.

3. Run Act: Navigate to your project directory and run `act`. This will start the process of running your GitHub Actions locally.

Please note that Act uses the `.github/workflows/` directory to find the workflow files, so ensure your workflow files are located there.

You can run a specific workflow by using the `-j` flag. For example, if you want to run the `build` job, you can run `act -j build`.
