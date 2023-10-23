#!/bin/bash

cd analysis

# Install pyenv if not installed
if ! command -v pyenv &> /dev/null
then
    echo "pyenv not found! Installing..."
    # Add install commands for pyenv here
fi

# Install Python version specified in .python-version
pyenv install --skip-existing

# Create a new virtual environment using pyenv
pyenv virtualenv $(cat .python-version) py_env
echo "created venv"

# Activate the virtual environment
pyenv local py_env
"echo activated venv"

# Install dependencies
pip install --upgrade pip
pip install -r py_requirements.txt

cd ..

echo "Setup complete!"
