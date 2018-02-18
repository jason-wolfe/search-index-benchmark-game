# Search Index Benchmark Game

A set of standardized benchmarks for comparing the speed of various aspects of search engine technologies.

This is useful both for comparing different libraries and as tooling for more easily and comprehensively
 comparing versions of the same technology. 

## Getting Started

These instructions will get you a copy of the project up and running on your local machine.

### Prerequisites

The lucene benchmarks requires Gradle. This can be installed from [the Gradle website](https://gradle.org/).

The tantivy benchmarks and benchmark driver code requires Cargo. This can be installed using [rustup](https://www.rustup.rs/). 


### Installing

Clone this repo.

```
git clone git@github.com:jason-wolfe/search-index-benchmark-game.git
```

And that's it!

## Running

You can now pass any file containing articles in JSON format, and a directory containing queries. 
A minimal example of articles is included [in the project](./common/datasets/minimal.json).
A small set of queries is included [in the project](./common/queries). 

Running with the examples can be done like so

```
./run_all.sh ./common/datasets/minimal.json ./common/queries
```

This will:
1. build the benchmark driving code
2. For each engine being tested:
    1. Build the code necessary to use it
    2. Build an index using the supplied documents, and output timing in seconds to `output/$engine/build_time.txt`.
    3. Run all of the supplied queries a number of times, recording the time taken to run in `output/$engine/query_output.txt`.

The supplied queries can be a directory, which will be searched recursively for `.txt` files to run, 
or it can be a `.txt` file itself, which will be used directly.

The output goes into the `output` subdirectory. 
It contains one folder per engine tested.

## Running more

Maybe you want to query again after you know the page cache is warmed up, to better represent your production workflow.
Or maybe you're debugging something or trying to improve query performance, and would like to run some queries without building the indexes again.
For these use-cases, the `query_all.sh` script allows you to run the given set of queries against the already built indexes.

The argument format is the same as [`drive_queries.rs`](./benchmark/src/bin/drive_queries.rs), 
which differs from `run_all.sh`, but allows more flexibility than `run_all.sh` currently offers.

```bash
./query_all.sh --queries ./common/queries/my_expensive_queries.txt -n 1
```

Important note: 
This assumes that each of your projects is already compiled as you wish them to be.
If this is not the case, run `preprocess_all.sh` and they will build per the standard process.

## TODO

Supply a better representative training set for easy use.

Support more engines.

Improve `benchmark/run_all.sh` to allow passing more parameters to the `drive.sh` program.

Output a more consumable summary format of any measurements made, to make comparison easier.