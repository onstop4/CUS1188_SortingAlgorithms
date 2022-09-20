use std::{
    io::{self, Write},
    time::Instant,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use sorting::{insertion_sort, selection_sort};

const ITERATIONS: usize = 1000;

fn main() {
    println!("Insertion sort:");
    timeit(insertion_sort);
    println!("\nSelection sort:");
    timeit(selection_sort);
}

fn timeit(f: fn(&mut [i32])) {
    let rng = &mut thread_rng();
    let mut times = Vec::with_capacity(ITERATIONS);

    for n in [100, 1000, 10000, 100000] {
        let mut vec: Vec<i32> = (0..n).collect();
        for i in 1..=ITERATIONS {
            vec.shuffle(rng);
            let start = Instant::now();
            f(&mut vec);
            times.push(start.elapsed().as_millis());

            print!("Completed {} out of {}\r", i, ITERATIONS);
            io::stdout().flush().unwrap();
        }

        let times_mean = mean(&times);
        let times_stdev = stdev(&times, times_mean);
        println!(
            "Input size {} has a mean of {} milliseconds and a standard deviation of {} milliseconds",
            n, times_mean, times_stdev
        );
        times.clear();
    }
}

fn mean(times: &Vec<u128>) -> f64 {
    let mut sum = 0;

    for time in times.iter() {
        sum += time;
    }

    sum as f64 / times.len() as f64
}

fn stdev(times: &Vec<u128>, mean: f64) -> f64 {
    let mut sum = 0.0;

    for time in times.iter() {
        sum += (*time as f64 - mean).powf(2.0);
    }

    (sum / (times.len() - 1) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::{mean, stdev};

    #[test]
    fn test_mean() {
        let vec = vec![2, 4, 4, 4, 5, 5, 7, 9];
        assert_eq!(mean(&vec), 5.0);
    }

    #[test]
    fn test_stdev() {
        let vec = vec![3, 4, 5];
        assert_eq!(stdev(&vec, mean(&vec)), 1.0);
    }
}
