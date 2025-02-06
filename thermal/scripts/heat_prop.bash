#!/bin/bash

HWMON="/sys/class/hwmon/hwmon1/temp2_input"
MILLIS=2
N=10000
NCPUS=8
COLLECTING_CPU=7

make page CFLAGS="-O0 -DMILLIS=$MILLIS -DN_MEASUREMENTS=$N"
mkdir -p data

for i in $(seq 1 $((NCPUS-1))); do

echo "Measuring heat inference from core $i on core 0"
taskset -c $i ./out/page_prefetch &
tPID=$!

sleep 5
taskset -c $COLLECTING_CPU ./out/temp $HWMON > data/prop$i.txt
kill $tPID
sleep 5

done