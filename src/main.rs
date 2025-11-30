use cyclone_fireworks::random::Random;
use cyclone_fireworks::core::Vector3;
use cyclone_fireworks::precision::Real;
fn main() {

    // Create a reproducible random stream
    let mut rng = Random::with_seed(12345);

    println!("=== Random binomial numbers ===");
    for i in 0..5 {
        let val: Real = rng.random_binomial(1.0);
        println!("binomial[{i}] = {val}");
    }

    println!("\n=== Random binomial vectors (scale = 1.0) ===");
    for i in 0..5 {
        let v: Vector3 = rng.random_vector_scale(1.0);
        println!("vector[{i}] = ({:.3}, {:.3}, {:.3})", v.x, v.y, v.z);
    }

    println!("\n=== Random XZ vectors (scale = 2.0) ===");
    for i in 0..5 {
        let v: Vector3 = rng.random_xz_vector(2.0);
        println!("xz_vector[{i}] = ({:.3}, {:.3}, {:.3})", v.x, v.y, v.z);
    }
}
