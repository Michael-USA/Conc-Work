// Hello world OpenMP program by Michael Zuppardo
// 
// Prints a hello world message from "x" threads in order.
// Accepts an argument to specify the number of threads to use.
//
// (I know I didn't have to order them, but I did it anyways).

#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

int main(int argc, char *argv[]) {
    int num_threads, current_thread = 0;

    // Set the number of threads from command line arguments if provided
    if (argc > 1) {
        num_threads = atoi(argv[1]);
        omp_set_num_threads(num_threads);
    } else {
        num_threads = omp_get_max_threads();
    }

    // Print the hello world message from each thread, ordered lowest to highest.
    #pragma omp parallel shared(current_thread)
    {
        int id = omp_get_thread_num();
        while (1) {
            #pragma omp flush(current_thread)
            if (id == current_thread) {
                printf("Hello from thread %d\n", id);
                #pragma omp atomic
                current_thread++;
                break;
            }
        }
    }
    return 0;
}
