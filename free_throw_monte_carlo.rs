use rand::prelude::*;
use std::fs::File;
use std::io::{self, Write};

const SHOTS_TO_MAKE_CONSEQUTIVELY: u8 = 40;
const FREE_THROW_PROP: f64 = 0.9;
const NUMBER_OF_SIMULATIONS: usize = 1000;

fn simulate_free_throw_sequence() -> u32 {
    let mut rng = rand::thread_rng();
    let mut free_throw_attempts = 0;
    let mut shots_consequtively = 0;
    while shots_consequtively < SHOTS_TO_MAKE_CONSEQUTIVELY {
        free_throw_attempts += 1;
        let free_throw: f64 = rng.gen();
        let sucessful_free_throw: bool = free_throw < FREE_THROW_PROP;
        if sucessful_free_throw {
            shots_consequtively += 1;
        } else {
            shots_consequtively = 0;
        }
    }
    return free_throw_attempts;
}

// run the monte carlo simulation
fn monte_carlo() -> [u32; NUMBER_OF_SIMULATIONS] {
    let mut monte_carlo: [u32; NUMBER_OF_SIMULATIONS] = [0; NUMBER_OF_SIMULATIONS];
    for i in 0..NUMBER_OF_SIMULATIONS {
        monte_carlo[i] = simulate_free_throw_sequence();
        // print("simulation {} done", i)
    }
    return monte_carlo;
}

fn mean(array: [u32; NUMBER_OF_SIMULATIONS]) -> f64 {
    let total: u32 = array.into_iter().sum();
    return total as f64 / (NUMBER_OF_SIMULATIONS as f64);
}

fn write_array_to_file<T>(array: &[T], file_path: &str) -> io::Result<()>
where
    T: std::fmt::Display,
{
    let mut file = File::create(file_path)?;
    write!(file, "[")?;
    for i in 0..array.len() - 1 {
        write!(file, "{}, ", array[i])?;
    }
    write!(file, "{}]", array[array.len() - 1])?;
    Ok(())
}

fn main() {
    let data: Vec<f64> = (0..10000)
        .into_iter()
        .map(|_| mean(monte_carlo()))
        .collect();

    let file_path = "output.txt";
    if let Err(err) = write_array_to_file(&data, &file_path) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("Array successfully written to file.");
    }
}

// println!(
//     "the mean sum of free throws required to hit {} free throws with {} accuracy is {}",
//     SHOTS_TO_MAKE_CONSEQUTIVELY,
//     FREE_THROW_PROP,
//     mean(monte_carlo())
// );
