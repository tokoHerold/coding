#!/bin/bash

HWMON=/sys/class/hwmon/hwmon1/temp*_input
NCPUS=8
MILLIS=2
N=100000

make page CFLAGS="-O0 -DMILLIS=$MILLIS -DN_MEASUREMENTS=$N"
mkdir -p data

echo "Measuring Baseline"
./out/temp $HWMON > data/baseline_page.txt

echo "Measuring page accesses program..."
taskset --cpu-list $((NCPUS - 1)) ./out/page_prefetch &
bPID=$!
sleep $(bc <<< "scale=1; $MILLIS/100")
./out/temp $HWMON > data/present_page.txt
kill $bPID
echo ...done!
echo Cooling down...

sleep 120

echo "Measuring unpaged accesses program..."
taskset --cpu-list $((NCPUS - 1)) ./out/page_prefetch 0x0 &
bPID=$!
sleep $(bc <<< "scale=1; $MILLIS/100")
./out/temp $HWMON > data/non_present_page.txt
kill $bPID
echo ...done!
