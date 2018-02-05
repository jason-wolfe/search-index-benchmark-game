#!/bin/bash

SCRIPT_PATH=${0%/*}

cd ${SCRIPT_PATH}

gradle clean shadowJar
