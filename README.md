# Search Index Benchmark Game

A set of standardized benchmarks for comparing the speed of various aspects of search engine technologies. 

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

The output goes into the `output` subdirectory. 
It contains one folder per engine tested.
