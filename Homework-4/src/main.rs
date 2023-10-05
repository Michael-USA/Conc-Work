extern crate mpi;  // MPI bindings
extern crate rand; // random number generation

use mpi::traits::*;
use rand::Rng;
use std::env;

fn main() {
    // Initialize the MPI environment and get the global communicator.
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    // Get the total number of processes and the rank of the current process.
    let size = world.size() as usize;
    let rank = world.rank();
    // Parse the size of the array (N) from command-line arguments.
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    // Generate an array of size N with random floating-point values between 0.0 and 100.0.
    let mut rng = rand::thread_rng();
    let mut array: Vec<f64> = (0..n).map(|_| rng.gen_range(0.0..100.0)).collect();

    // If the current process is the master (rank 0), it prints all processes' arrays.
    if rank == 0 {
        for i in 0..size {
            if i == 0 {
                println!("Process 0 array: {:?}", array);
                continue;
            } 
            // Receive the size of the array from the i-th process.
            let (received_size, _): (i32, _) = world.process_at_rank(i as i32).receive();
            // Allocate a buffer for the array and receive it.
            let mut received_buffer = vec![0.0; received_size as usize];
            world.process_at_rank(i as i32).receive_into(&mut received_buffer[..]);
            println!("Process {} array: {:?}", i, received_buffer);
        }
    } else {
        // All other processes send the size and content of their arrays to the master.
        let array_size = array.len() as i32;
        world.process_at_rank(0).send(&array_size);
        world.process_at_rank(0).send(&array);
    }

    // Calculate the total sum of the elements in the array.
    let total: f64 = array.iter().sum();
    let mut avg: f64 = 0.0;
    // Compute the global average of all elements across all processes.
    world.all_reduce_into(&total, &mut avg, mpi::collective::SystemOperation::sum());
    avg /= (size * n) as f64;
    // Master broadcasts the global average to all processes.
    world.process_at_rank(0).broadcast_into(&mut avg);

    // Each process adjusts its array by subtracting the average.
    for value in &mut array {
        *value -= avg;
    }
    
    // Master randomly selects a process (besides itself) and notifies it.
    let selected_process;
    if rank == 0 {
        selected_process = rand::thread_rng().gen_range(1..size as i32);
        println!("Selected process: {}", selected_process);
        for i in 1..size as i32 {
            world.process_at_rank(i).send(&selected_process);
        }
    }
    else {
        // Non-master receives the ID of the selected process.
        let (received_value, _) = world.process_at_rank(0).receive::<i32>();
        selected_process = received_value;
    }

    // The selected process adds 2 * average to its array, others subtract.
    if rank == selected_process {
        for value in &mut array {
            *value += 2.0 * avg;
        }
    }

    // Master receives arrays after the adjustment and prints them.
    if rank == 0 {
        for i in 0..size {
            if i == 0 {
                println!("Process 0 array (after avg adjustment): {:?}", array);
                continue;
            } 
            let (received_size, _): (i32, _) = world.process_at_rank(i as i32).receive();
            let mut received_buffer = vec![0.0; received_size as usize];
            world.process_at_rank(i as i32).receive_into(&mut received_buffer[..]);
            println!("Process {} array (after avg adjustment): {:?}", i, received_buffer);
        }
    } else {
        // Non-master processes send their arrays to the master after the adjustment.
        let array_size = array.len() as i32;
        world.process_at_rank(0).send(&array_size);
        world.process_at_rank(0).send(&array);
    }

    // Find the maximum value in the current process's array.
    let max_value = *array.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    // Gather all the max values from all processes.
    let mut all_max_values: Vec<f64> = vec![0.0; size as usize];
    world.all_gather_into(&max_value, &mut all_max_values);

    // Adjust the array values based on the neighbor's max value.
    let neighbor_max = all_max_values[((rank + 1) % (size as i32)) as usize];
    for value in &mut array {
        if rank == selected_process {
            *value -= neighbor_max;
        }
        else {
            *value += neighbor_max;
        }
    }

    // Master receives adjusted arrays from all processes and prints them.
    if rank == 0 {
        println!("Process 0 array (after max adjustment): {:?}", array);
        for i in 1..size as i32 {
            let (received_size, _): (i32, _) = world.process_at_rank(i).receive();
            let mut received_array = vec![0.0; received_size as usize];
            world.process_at_rank(i).receive_into(&mut received_array[..]);
            println!("Process {} array (after max adjustment): {:?}", i, received_array);
        }
    }
    else {
        // Non-master processes send their adjusted arrays to the master.
        let array_size = array.len() as i32;
        world.process_at_rank(0).send(&array_size);
        world.process_at_rank(0).send(&array);
    }

    // Every process adds its ID (as a double) to its array values.
    for value in &mut array {
        *value += rank as f64;
    }

    // Find the minimum value in the current process's array and compute the global minimum.
    let min_value = *array.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let mut global_min_buf: f64 = 0.0;
    world.all_reduce_into(&min_value, &mut global_min_buf, mpi::collective::SystemOperation::min());
    let global_min = global_min_buf;

    // Master prints the global minimum value.
    if rank == 0 {
        println!("Minimum value of all elements: {}", global_min);
    }
}
