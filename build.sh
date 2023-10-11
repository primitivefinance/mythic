#!/bin/bash

cd box-contracts

forge clean

forge install

forge bind --crate-name bindings
