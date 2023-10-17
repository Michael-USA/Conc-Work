 
============================================================
=====     Queued job information at submit time        =====
============================================================
  The submitted file is: test.sh
  The time limit is 00:05:00 HH:MM:SS.
  The target directory is: /home/usaclsc0052/mzWork/Conc-Work/Homework-4/src
  The memory limit is: 16gb
  The job will start running after: 2023-10-05T12:37:20
  Job Name: testshSCRIPT
  Virtual queue: small
  QOS: 
ERROR:  Requested memory exceeds limit for small
  Constraints: --constraint=dmc
  Command typed:
/apps/scripts/run_script_mpi test.sh     
  Queue submit command:
sbatch 
ERROR:  Requested memory exceeds limit for small -J testshSCRIPT --begin=2023-10-05T12:37:20 --requeue --mail-user=mvz2021@jagmail.southalabama.edu -o testshSCRIPT.o$SLURM_JOB_ID -t 00:05:00 -n 16 --mem-per-cpu=1000mb --constraint=dmc 
  Job number: 
 
