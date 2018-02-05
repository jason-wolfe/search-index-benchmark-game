#!/bin/bash

SCRIPT_PATH=${0%/*}

cd ${SCRIPT_PATH}

cargo build --release
