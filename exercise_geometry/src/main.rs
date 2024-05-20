// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(vector: &[f64]) -> f64 {
    let mut mag_suqared = 0.0;
    for n in vector {
        mag_suqared += n * n;
    }
    mag_suqared.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(vector: &mut [f64]) {
    let mag = magnitude(vector);
    for n in vector {
        *n /= mag;
    }
}

// Use the following `main` to test your work.

fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0, 2.3, 1.1];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
