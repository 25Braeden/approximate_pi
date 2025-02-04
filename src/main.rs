/*
 * Intuition:
 * One way to approximate pi as I saw in a 3b1b video is to count
 * the amount of lattice points in a circle with radius size n
 * and as n -> inf, the approximation becomes better and better
 *
 * So to approximate pi I can find all points in first quadrant
 * where x^2 + y^2 <= n^2 then multiply by 4 to account for all
 * quadrants
 *
 */

use std::thread;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Instant;

use std::f64::consts::PI;

fn count_lattice(n: i64) -> i64 {
    let n_squared = (n as i128) * (n as i128); // Prevent overflow
    let points = AtomicI64::new(0);

    let num_threads = thread::available_parallelism()
        .map(|n| n.get() as i64)
        .unwrap_or(1);

    let chunk_size = n / num_threads;

    thread::scope(|s| {
        for thread_id in 0..num_threads {
            let points = &points;
            s.spawn(move || {
                let start = thread_id * chunk_size;
                let end = if thread_id == num_threads - 1 {
                    n
                } else {
                    (thread_id + 1) * chunk_size
                };

                let mut local_points = 0;

                for x in start..=end {
                    let x_squared = (x as i128) * (x as i128);
                    if x_squared > n_squared {
                        continue;
                    }
                    let y_max = integer_sqrt(n_squared - x_squared);
                    local_points += y_max + 1;
                }

                points.fetch_add(local_points, Ordering::Relaxed);
            });
        }
    });

    4 * points.into_inner()
}


// Newton-Raphson integer square root
fn integer_sqrt(num: i128) -> i64 {
    if num < 2 {
        return num as i64;
    }
    let mut guess = num;
    while guess * guess > num {
        guess = (guess + num / guess) / 2;
    }
    guess as i64
}

fn approximate_pi(n: i64) -> f64 {
    let points = count_lattice(n);
    points as f64 / (n * n) as f64
}

fn main() {
    let time = Instant::now();
    let radius = 100_000_000;
    let pi_approx = approximate_pi(radius);

    println!("Approximated pi: {:.12}", pi_approx);
    println!("Actual value of pi: {:.12}", PI);
    println!("Completed in: {:.5?}s", time.elapsed().as_secs_f64());
}
