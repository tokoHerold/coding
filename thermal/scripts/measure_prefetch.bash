
T_N=1000
FILE=prefetches
instrs=("prefetcht0" "prefetcht1" "prefetcht2" "prefetchnta")
CORE_0=0
CORE_N_MINUS_1=7

for instr in "${instrs[@]}"; do
    make page CFLAGS="-O0 -DINSTR=\\\"$isntr\\\" -DN_MEASUREMENTS=$T_N"
    # Present page
    taskset -c $CORE_0 ./out/page_prefetch &
    tPID=$!
    sleep 0.02
    taskset -c $CORE_N_MINUS_1 ./out/temp $HWMON >"data/$FILE-p-$instr.txt"
    kill $tPID
    sleep 5

    # Non-present page
    taskset -c $CORE_0 ./out/page_prefetch 0xcafebabe &
    tPID=$!
    sleep 0.02
    taskset -c $CORE_N_MINUS_1 ./out/temp $HWMON >"data/$FILE-n-$instr.txt"
    kill $tPID
    sleep 5
done