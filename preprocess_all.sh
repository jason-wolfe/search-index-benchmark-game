#!/bin/bash

set -e

BASE=${0%/*}

INDEX_TYPES=`cat ${BASE}/INDEX_TYPES.txt`
for INDEX_TYPE in ${INDEX_TYPES} ; do
  echo "Building programs for ${INDEX_TYPE}"
  ${BASE}/${INDEX_TYPE}/build.sh && echo "success"
done