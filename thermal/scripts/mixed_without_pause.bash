#!/bin/bash

HWMON="/sys/class/hwmon/hwmon5/temp1_input /sys/class/hwmon/hwmon6/temp2_input"
N_MEASUREMENTS=500
ITERATIONS=4
MILLIS=50
FILENAME=data/alternating

# Number of measurements * time between measurements in milliseconds


echo "Measuring $ITERATIONS times for $TIME_FOR_ITERATIONS each"
# Compile targets to run in an alternating order
make page CFLAGS="-DN_MEASUREMENTS=$N_MEASUREMENTS -DMILLIS=$MILLIS"

# Wamup
echo Starting Warmup
taskset --cpu-list 0 ./out/page_prefetch &
targetPID=$!
sleep 25
kill $targetPID

bPID=$!
# Start measurement
for i in $(seq 1 $ITERATIONS);
do
   echo "Iteration number $i"

   # Execute target application for present page
   taskset --cpu-list 0 ./out/page_prefetch &
   targetPID=$!
   ./out/temp $HWMON > $FILENAME"_present_$i.txt"
   kill $targetPID

   taskset --cpu-list 0 ./out/page_prefetch 0x0 &
   targetPID=$!
   ./out/temp $HWMON > $FILENAME"_nonpresent_$i.txt"
   kill $targetPID
done
