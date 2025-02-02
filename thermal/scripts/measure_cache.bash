#!/bin/bash

HWMON=/sys/class/hwmon/hwmon1/temp*_input
TARGET_CORE=0
COLLECTING_CORE=7
FILENAME=cache
N=100000

make cache CFLAGS="-O0 -DMILLIS=2 -DN_MEASUREMENTS=$N"
mkdir -p data

echo "Measuring cache-aware program..."
taskset --cpu-list $TARGET_CORE ./out/good_cache &
bPID=$!
echo "Warming up..."
sleep 30
echo Measuring
taskset --cpu-list $COLLECTING_CORE ./out/temp $HWMON > data/good_$FILENAME.txt
kill $bPID
echo ...done!

echo Cooling down...
sleep 30

echo "Measuring non-cache-aware program..."
taskset --cpu-list $TARGET_CORE ./out/bad_cache &
bPID=$!
echo "Warming up..."
sleep 30
echo Measuring...
taskset --cpu-list $COLLECTING_CORE ./out/temp $HWMON > data/bad_$FILENAME.txt
kill $bPID
echo ...done!