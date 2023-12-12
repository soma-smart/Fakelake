#!/bin/bash

pip install -r bench/requirements.txt

hyperfine.exe \
    --export-markdown bench\\BENCHMARK.md \
    --warmup 1 \
    'target\release\fakelake.exe generate bench\fakelake_input.yaml' \
    'python bench\mimesis_bench.py' \
    'python bench\faker_bench.py'

if [ $? == 0 ]
then
    echo "BENCHMARK.md created in bench/"
else
    echo "Issue while creating BENCHMARK.md"
fi