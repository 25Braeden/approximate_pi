use std::thread;
use std::time::Instant;

#[inline(always)]
fn integer_sqrt_approx(num: u128) -> u64 {
    let approx = ((num as f64).sqrt()) as u64;
    if (approx as u128 + 1) * (approx as u128 + 1) <= num {
        approx + 1
    } else {
        approx
    }
}

fn count_lattice_points(n: u64, num_threads: usize) -> u64 {
    let n_squared = (n as u128) * (n as u128);
    let chunk_size = n / (num_threads as u64);
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = thread_id as u64 * chunk_size;
        let end = if thread_id == num_threads - 1 {
            n
        } else {
            (thread_id as u64 + 1) * chunk_size
        };

        handles.push(thread::spawn(move || {
            let mut local_points = 0;
            for x in start..=end {
                let x_squared = (x as u128) * (x as u128);
                if x_squared > n_squared {
                    continue;
                }
                let y_max = integer_sqrt_approx(n_squared - x_squared);
                local_points += y_max + 1;
            }
            local_points
        }));
    }

    handles.into_iter().map(|handle| handle.join().unwrap()).sum()
}

fn main() {
    let radius = 10_000_000_000;
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let start = Instant::now();

    let points = count_lattice_points(radius, num_threads);
    let pi_approx = 4.0 * (points as f64) / ((radius as f64) * (radius as f64));
    let duration = start.elapsed();

    let absolute_error = (pi_approx - std::f64::consts::PI).abs();
    let relative_error = (absolute_error / std::f64::consts::PI) * 100.0;

    println!("Approximated pi: {:.12}", pi_approx);
    println!("Actual value of pi: {:.12}", std::f64::consts::PI);
    println!("Absolute Error: {:.12}", absolute_error);
    println!("Relative Error: {:.12}%", relative_error);
    println!("Calculation took: {:?}", duration);
}
