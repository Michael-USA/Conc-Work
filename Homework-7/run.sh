#!/bin/sh
#

default_threads=15
num_threads=${1:-$default_threads}
sed -i "s/^export OMP_NUM_THREADS=.*/export OMP_NUM_THREADS=$num_threads/" scriptOpenMP

run_script scriptOpenMP
