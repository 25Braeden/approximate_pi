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

 use std::f64::consts::PI;

 fn count_lattice(n: i64) -> i64 {
     let mut points = 0;
     for x in 0..=n {
         let y_max = ((n * n - x * x) as f64).sqrt() as i64;
         points += y_max + 1;
     }
     4 * points - 3
 }
 
 fn approximate_pi(n: i64) -> f64 {
     let points = count_lattice(n);
     points as f64 / (n * n) as f64
 }
 
 fn main() {
     // larger this num is better approximation
     let radius = 1_000_000_000;
     let pi_approx = approximate_pi(radius);
     println!("Approximated radius: {}", pi_approx);
     println!("Actual value of pi: {}", PI);
 }
