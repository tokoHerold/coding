#!/bin/bash

echo "Warmup (pun not intended)"
taskset --cpu-list 0 ./out/page_prefetch &
bPID=$!
sleep 60
kill $bPID

mkdir -p data
make page
echo Measuring prefetchnta for present page
taskset --cpu-list 0 ./out/page_prefetch &
bPID=$!
sleep 10
./out/temp > data/present_prefetchnta
kill $bPID
sleep 10

echo Measuring prefetchnta for non-present page
# taskset --cpu-list 0 ./out/page_prefetch 0x7ffffffff000 &
taskset --cpu-list 0 ./out/page_prefetch 0x0 &
bPID=$!
sleep 10
./out/temp > data/nonpresent_prefetchnta
kill $bPID
sleep 10

make clean
make page CFLAGS="-O0 -DINSTR=\\\"prefetcht2\\\""

echo Measuring prefetcht2 for present page
taskset --cpu-list 0 ./out/page_prefetch &
bPID=$!
sleep 10
./out/temp > data/present_prefetcht2
kill $bPID
sleep 10

echo Measuring prefetcht2 for non-present page
# taskset --cpu-list 0 ./out/page_prefetch 0x7ffffffff00 &
taskset --cpu-list 0 ./out/page_prefetch 0x0 &
bPID=$!
sleep 10
./out/temp > data/nonpresent_prefetcht2
kill $bPID
