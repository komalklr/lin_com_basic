#!/bin/bash

taskname=REPLACE_ME
inputfile=REPLACE_ME

# You can add more input and output files by providing more `--file filename` args
node "../emscripten-module-wrapper/prepare.js" "target/wasm32-unknown-emscripten/release/$taskname.js" --file "$inputfile.data" --run --debug --out=truebit_run
