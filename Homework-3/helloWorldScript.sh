#!/bin/sh
source /opt/asn/etc/asn-bash-profiles-special/modules.sh
module load openmpi
srun --mpi=pmi2 helloWorldMPI

