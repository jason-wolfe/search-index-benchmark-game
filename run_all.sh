#!/bin/bash

set -e

BASE=${0%/*}

PROGRAM_NAME=$0

display_usage() {
	echo -e "Build all benchmarks, create search indexes using the supplied document set, and run the supplied queries to benchmark."
	echo -e "Output goes into the outputs/ directory."
	echo -e ""
	echo -e "Usage: ${PROGRAM_NAME} document_file query_directory"
	echo -e ""
	echo -e "Parameters:"
	echo -e "    document_file     A JSON file containing one document per line, with \"url\", \"title\", and \"body\" fields"
	echo -e "    query_directory   A directory containing .txt files with one query per line"
	echo -e "    -h --help         Display this usage guide"
}

if [  $# -le 1 ]
then
    display_usage
    exit 1
fi

if [[ ( $# == "--help") ||  $# == "-h" ]]
then
    display_usage
    exit 0
fi

echo Building base benchmark code
${BASE}/benchmark/build.sh

TOP_LEVEL_OUTPUTS=${BASE}/outputs

mkdir ${TOP_LEVEL_OUTPUTS}
PIPE_FILE=${TOP_LEVEL_OUTPUTS}/.pipe

mkfifo ${PIPE_FILE}

for INDEX_TYPE in lucene tantivy; do
    echo Processing ${INDEX_TYPE}

    OUTPUT=${TOP_LEVEL_OUTPUTS}/${INDEX_TYPE}
    mkdir ${OUTPUT}
    INDEX_OUTPUT=${OUTPUT}/index
    mkdir ${INDEX_OUTPUT}

    echo Preprocessing ${INDEX_TYPE}
    ${INDEX_TYPE}/build.sh

    echo Building index for ${INDEX_TYPE} into ${INDEX_OUTPUT}
    start=`date +%s`
    ${INDEX_TYPE}/build_index.sh ${INDEX_OUTPUT} < $1
    end=`date +%s`
    runtime=$((end-start))
    echo Building index for ${INDEX_TYPE} took ${runtime} seconds
    echo ${runtime} > ${OUTPUT}/build_time.txt

    echo Querying against ${INDEX_TYPE}
    ${BASE}/benchmark/drive.sh --queries $2 <${PIPE_FILE} 2>${OUTPUT}/query_output.txt | ${INDEX_TYPE}/query.sh ${INDEX_OUTPUT} >${PIPE_FILE}
done

rm ${PIPE_FILE}