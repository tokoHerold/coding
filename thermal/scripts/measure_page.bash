#!/bin/bash

HWMON=/sys/class/hwmon/hwmon1/temp*_input
TARGET_CPU=0
COLLECTING_CPU=7
MILLIS=2
N=100000

make page CFLAGS="-O0 -DMILLIS=$MILLIS -DN_MEASUREMENTS=$N"
mkdir -p data

echo "Measuring Baseline"
./out/temp $HWMON > data/baseline_page.txt

echo "Measuring page accesses program..."
taskset --cpu-list $TARGET_CPU ./out/page_prefetch &
bPID=$!
sleep $(bc <<< "scale=1; $MILLIS/100")
tasksetk --cpu-list  ./out/temp $HWMON > data/present_page.txt
kill $bPID
echo ...done!
echo Cooling down...

sleep 120

echo "Measuring unpaged accesses program..."
taskset --cpu-list $TARGET_CPU ./out/page_prefetch 0x7ffffffff000 &
bPID=$!
sleep $(bc <<< "scale=1; $MILLIS/100")
tasksetk --cpu-list $COLLECTING_CPU ./out/temp $HWMON > data/non_present_page.txt
kill $bPID
echo ...done!
