#!/bin/bash

N_MEASUREMENTS=500
ITERATIONS=4
MILLIS=100
FILENAME=data/alternating
N_CPUS=1
# Number of measurements * time between measurements in milliseconds


echo "Measuring $ITERATIONS times for $TIME_FOR_ITERATIONS each"
# Compile targets to run in an alternating order
make page CFLAGS="-DN_MEASUREMENTS=$N_MEASUREMENTS -DMILLIS=$MILLIS -DINSTR=\\\"prefetcht2\\\""

# Wamup
# Start measurement
#

function execute {
    for ((i=0; i<$N_CPUS; i++))
    do
        taskset --cpu-list $i $* &
        PIDS[$i]=$! # Store process ID
        echo "Started instance $i with PID ${PIDS[$i]}"
    done
    }

function terminate {
    for pid in ${PIDS[@]}
    do
        kill $pid
        echo "Killed process with PID $pid"
    done
}

function run {
    # $1: iterations
    output_file_1=$2
    output_file_2=$3
    # $3: output file name 2
    for i in $(seq 1 $1);
    do
    echo "Iteration number $i"

    if [ $2 != "/dev/null" ]; then
        output_file_1=$2"_"$i".txt"
    fi

    if [ $3 != "/dev/null" ]; then
        output_file_2=$3"_"$i".txt"
    fi
    echo $output_file_1 $output_file_2

    # Execute target application for present page
    # taskset --cpu-list $((i % 3)) ./out/page_prefetch &
    # targetPID=$!
    execute ./out/page_prefetch
    ./out/temp > $output_file_1
    # kill $targetPID
    terminate
    sleep 120

    # taskset --cpu-list $((i % 3 + (N_CPUS/2))) ./out/page_prefetch 0x0 &
    # targetPID=$!
    execute ./out/page_prefetch 0x0
    ./out/temp > $output_file_2
    # kill $targe3tPID
    terminate
    sleep 120
    done
}

echo Warmup...
run 8 /dev/null /dev/null
run 4 data/mixedt2_present data/mixedt2_nonpresent
