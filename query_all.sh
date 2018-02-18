#!/bin/bash

set -e

BASE=${0%/*}

PREFIX=$1

if [[ ! ${PREFIX} =~ ^[a-zA-Z0-9]+$ ]] ; then
    echo Output prefix \(${PREFIX}\) must consist of only alpha-numeric characters
    exit 1
fi

shift

TOP_LEVEL_OUTPUTS=${BASE}/outputs
mkdir -p ${TOP_LEVEL_OUTPUTS}
PIPE_FILE=${TOP_LEVEL_OUTPUTS}/.pipe
mkfifo ${PIPE_FILE} || echo Pipe file already exists

INDEX_TYPES=`cat ${BASE}/INDEX_TYPES.txt`
for INDEX_TYPE in ${INDEX_TYPES} ; do
    OUTPUT=${TOP_LEVEL_OUTPUTS}/${INDEX_TYPE}
    INDEX_OUTPUT=${OUTPUT}/index
    echo Querying against ${INDEX_TYPE}
    ${BASE}/benchmark/drive.sh $@ <${PIPE_FILE} 2>${OUTPUT}/${PREFIX}_query_output.txt | ${INDEX_TYPE}/query.sh ${INDEX_OUTPUT} >${PIPE_FILE}
done