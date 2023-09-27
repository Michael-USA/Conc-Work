extern crate mpi;  // MPI bindings
extern crate rand; // random number generation

use mpi::traits::*;
use rand::Rng;
use std::env;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size() as usize;
    let rank = world.rank();

    // Parse N from command line argument
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();

    // Initialize the array with random doubles
    let mut rng = rand::thread_rng();
    let mut array: Vec<f64> = (0..n).map(|_| rng.gen_range(0.0..100.0)).collect();

    // Master process prints out the initial arrays
    if rank == 0 {
        for i in 0..size {
            let mut received_array = vec![0.0; n];
            if i == 0 {
                received_array.copy_from_slice(&array);
            } else {
                world.process_at_rank(i as i32).receive_into(&mut received_array[..]);
            }
            println!("Process {} array: {:?}", i, received_array);
        }
    } else {
        world.process_at_rank(0).send(&array);
    }

    // Calculate the global average and broadcast
    let total: f64 = array.iter().sum();
    let mut avg: f64 = 0.0;

    // Assuming that the library provides a default sum operation
    world.all_reduce_into(&total, &mut avg, mpi::op::Sum);

    avg /= (size * n) as f64;
    
    // Assuming that the root can simply be specified by rank
    world.broadcast_into(&mut avg, 0);

    // ... Remaining logic
}
