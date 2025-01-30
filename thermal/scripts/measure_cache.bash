#!/bin/bash

HWMON=/sys/class/hwmon/hwmon1/temp*_input
NCPUS=8

make cache CFLAGS="-O0 -DMILLIS=2"
mkdir -p data

for i in $(seq 1 4);
do
    echo "Measuring cache-aware program..."
    taskset --cpu-list $((NCPUS - 1)) ./out/good_cache &
    bPID=$!
    ./out/temp $HWMON > data/good_cache_$i.txt
    kill $bPID
    echo ...done!
    echo Cooling down...

    sleep 3

    echo "Measuring non-cache-aware program..."
    taskset --cpu-list $((NCPUS - 1)) ./out/bad_cache &
    bPID=$!
    ./out/temp $HWMON > data/bad_cache_$i.txt
    kill $bPID
    echo ...done!
    sleep 3
done