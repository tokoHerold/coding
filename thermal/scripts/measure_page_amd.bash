#!/bin/bash

# ACPITZ (1) | TCTL (1) | DELL_SMM (3)
HWMON="/sys/class/hwmon/hwmon1/temp1_input /sys/class/hwmon/hwmon5/temp1_input /sys/class/hwmon/hwmon6/temp*_input"
NCPUS=4
MILLIS=50
N=500
# NOTE: using this configuration, a peak at measurement 140

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

taskset --cpu-list $((NCPUS - 1)) ./out/page_prefetch 0x7ffffffff000 &
bPID=$!
sleep $(bc <<< "scale=1; $MILLIS/100")
./out/temp $HWMON > data/non_present_page.txt
kill $bPID
echo ...done!
