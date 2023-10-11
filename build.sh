#!/bin/bash

cd contracts

forge clean

forge install

forge bind --crate-name bindings
