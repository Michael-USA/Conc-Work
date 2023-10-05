#!/bin/bash

source /opt/asn/etc/asn-bash-profiles-special/modules.sh
export LD_LIBRARY_PATH=/usr/mpi/gcc/openmpi-4.1.2a1/lib64:$LD_LIBRARY_PATH
module load openmpi
mpirun programFile 5

