use cyclone_fireworks::random::Random;
use cyclone_fireworks::core::Vector3;
use cyclone_fireworks::precision::Real;
use cyclone_fireworks::fireworks::{Firework, FireworkRule, Payload, build_default_rules, FireworkWorld};

fn main() {
    let mut rng = Random::with_seed(12345);
    let rules = build_default_rules();
    let mut world = FireworkWorld::new(rules);

    // Spawn a single type-1 rocket (like pressing '1' in Millington's demo)
    world.spawn_single(1, None, &mut rng);

    let dt: Real = 0.1;

    println!("Simulating fireworks world:");
    for step in 0..50 {
        println!("--- step {} ---", step);

        let mut active = 0;
        for (i, fw) in world.fireworks.iter().enumerate() {
            if fw.type_id > 0 {
                active += 1;
                let pos = fw.particle.get_position();
                println!(
                    "slot {:3} | type {:2} | age={:5.2} | pos=({:6.2}, {:6.2}, {:6.2})",
                    i,
                    fw.type_id,
                    fw.age,
                    pos.x, pos.y, pos.z
                );
            }
        }

        println!("active fireworks: {}", active);

        world.update(dt, &mut rng);
    }


    /* Testing for a Single Firework
    let mut rng = Random::with_seed(12345);

    //A simple rule, loosely based on rules[0] in the Millington code.as
    let rule = FireworkRule::new(
        1,                              //type_id
        0.5, 1.4,                       //min_age, max_age
        Vector3::new(-5.0, 25.0, -5.0), //min_velocity
        Vector3::new(5.0, 28.0, 5.0),   //max_velocity
        0.1,                            //damping
        vec![Payload::new(3, 5)],        //one payload, type 3, count 5 (not used yet)
    );

    // Tracking a small number of fireworks
    const MAX_FIREWORKS: usize = 5;
    let mut fireworks: [Firework; MAX_FIREWORKS] = [Firework::default(); MAX_FIREWORKS];

    // Create one root firework in slot 0.
    rule.create(&mut fireworks[0], None, &mut rng);

    let dt: Real = 0.1;

    println!("Simulating a single firework:");
    for step in 0..20 {
        let fw = &mut fireworks[0];
        let dead = fw.update(dt);
        let pos = fw.particle.get_position();
        println!(
            "step {:2}: age={:5.2}, pos=({:6.2}, {:6.2}, {:6.2}), dead = {}",
            step, fw.age, pos.x, pos.y, pos.z, dead
        );
        if dead {
            println!("Firework died at step {step}");
            break;
        }
    }
*/

    /* Testing Random

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
        } */

}