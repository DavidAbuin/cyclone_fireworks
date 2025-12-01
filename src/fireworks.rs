use crate::core::{Vector3, GRAVITY};
use crate::particle::Particle;
use crate::precision::Real;
use crate::random::Random;

#[derive(Debug, Clone, Copy)]
pub struct Firework {
    pub particle: Particle,
    pub type_id: u32,
    pub age: Real,
}

impl Default for Firework {
    fn default() -> Self {
        Self {
            particle: Particle::new(),
            type_id: 0,
            age: 0.0,
        }
    }
}

impl Firework {
    pub fn update(&mut self, duration: Real) -> bool {
        self.particle.integrate(duration);
        self.age -= duration;

        let pos = self.particle.get_position();
        self.age < 0.0 || pos.y < 0.0
    }

}

// Firework rules control fuse length, velocities, damping, and payloads.
#[derive(Debug, Clone)]
pub struct FireworkRule {
    pub type_id: u32,
    pub min_age: Real,
    pub max_age: Real,
    pub min_velocity: Vector3,
    pub max_velocity: Vector3,
    pub damping: Real,
    pub payloads: Vec<Payload>,
}

// The payload is the new firework type(s) to create when this one detonates.
#[derive(Debug, Clone, Copy)]
pub struct Payload {
    pub type_id: u32,
    pub count: u32,
}

impl Payload {
    pub fn new(type_id: u32, count: u32) -> Self {
        Self { type_id, count}
    }
}
//---------------------------------------------------------------------------------------
impl FireworkRule {
    pub fn new(
        type_id: u32,
        min_age: Real,
        max_age: Real,
        min_velocity: Vector3,
        max_velocity: Vector3,
        damping: Real,
        payloads: Vec<Payload>
    ) -> Self {
        Self {
            type_id,
            min_age,
            max_age,
            min_velocity,
            max_velocity,
            damping,
            payloads,
        }
    }


    // Creates a new firework of this type, initializing it based on an optional parent firework
    //  and a mutable random stream.
    pub fn create (
        &self,
        firework: &mut Firework,
        parent: Option<&Firework>,
        rng: &mut Random,
    ) {
        firework.type_id = self.type_id;
        firework.age = rng.random_real_range(self.min_age, self.max_age);

        let mut vel = Vector3::default();

        if let Some(parent_fw) = parent {
            // Position & velocity based on the parent.
            firework
                .particle
                .set_position_vec(parent_fw.particle.get_position());
            vel += parent_fw.particle.get_velocity();
        } else {
            // Launch from ground with x = -5, 0, or +5.
            let mut start = Vector3::default();
            let x_index = rng.random_int(3) as i32 - 1; //-1, 0, or 1
            start.x = 5.0 * x_index as Real;
            firework.particle.set_position_vec(start);
        }
        vel += rng.random_vector_range(&self.min_velocity, &self.max_velocity);

        // Add random velocity within [min_velocity, max_velocity].
        firework.particle.set_velocity_vec(vel);

        // Mass is always 1 in the demo
        firework.particle.set_mass(1.0);

        firework.particle.set_damping(self.damping);

        // Gravity
        firework.particle.set_acceleration_vec(GRAVITY);

        //Clear forces.
        firework.particle.clear_accumulator();
    }
}

// Rules for Fireworks --------------------------------------------------------------

//How many different firework types that are supported.
pub const RULE_COUNT: usize = 9;

// Builds the full set of firework rules, mirroring Millington's initFireworksRules().
pub fn build_default_rules() -> Vec<FireworkRule> {
    let mut rules = Vec::with_capacity(RULE_COUNT);

    /* Rule 1 - Launch the main rocket.
            Behavior - upward velocity of 25 to 28,
            Damping - the rocket rises, slows, hovers slightly, then detonates
            Payload - secondary bursts of type 3 and 5
    */
    rules.push(FireworkRule::new(
            1,
            0.5, 1.4,
            Vector3::new(-5.0, 25.0, -5.0),
            Vector3::new(5.0, 28.0, 5.0),
            0.1,
            vec! [
                Payload::new(3,5),
                Payload::new(5,5),
            ],
    ));

    /* Rule 2 - Small Secondary burst
            - upward velocity of y = 10 - 20
            - high damping
            - short fuse explodes quickly
            - Two fireworks of type 2
     */

    // rules[1]
    rules.push(FireworkRule::new(
        2,
        0.5, 1.0,
        Vector3::new(-5.0, 10.0, -5.0),
        Vector3::new(5.0, 20.0, 5.0),
        0.8,
        vec![
            Payload::new(4, 2),
        ],
    ));

    /* Rule 3 - Soft SPark Puff
        Very low velocities (all directions)
        Can even drift downward initially (min y = -5)
        Damping = 0.1, dissipates quickly
        No payload
     */

    // rules[2]
    rules.push(FireworkRule::new(
        3,
        0.5, 1.5,
        Vector3::new(-5.0, -5.0, -5.0),
        Vector3::new(5.0, 5.0, 5.0),
        0.1,
        vec![],
    ));

    /* Rule 4 - Sideways Burst - horizontal jet of sparks like a fan shape
        large sideways velocity (+- 20
     */

    // rules[3]
    rules.push(FireworkRule::new(
        4,
        0.25, 0.5,
        Vector3::new(-20.0, 5.0, -5.0),
        Vector3::new(20.0, 5.0, 5.0),
        0.2,
        vec![],
    ));

    /* Rule 5 - Fast Upward Spark with Secondary Puff
        - upward between 2 and 18
        - very low damping
        - explodes into 5 soft sparks

     */
    // rules[4]
    rules.push(FireworkRule::new(
        5,
        0.5, 1.0,
        Vector3::new(-20.0, 2.0, -5.0),
        Vector3::new(20.0, 18.0, 5.0),
        0.01,
        vec![
            Payload::new(3, 5),
        ],
    ));
    /* Rule 6 - Slow Rising Big Shell with no payload
        - a slow drifting orb of sparks
        - long-lived (3-5 seconds)
        - high damping
        - looks like embers slowing moving up and fading

     */
    // rules[5]
    rules.push(FireworkRule::new(
        6,
        3.0, 5.0,
        Vector3::new(-5.0, 5.0, -5.0),
        Vector3::new(5.0, 10.0, 5.0),
        0.95,
        vec![],
    ));

    /* Rule 7 - Big Shell that releases a type 8 burst shell
        - large upward velocity (50-60)
        - very low damping so huge rise
        - lasts about 4-5 seconds

     */
    // rules[6]
    rules.push(FireworkRule::new(
        7,
        4.0, 5.0,
        Vector3::new(-5.0, 50.0, -5.0),
        Vector3::new(5.0, 60.0, 5.0),
        0.01,
        vec![
            Payload::new(8, 10),
        ],
    ));

    /* Rule 8 - White Spark Flash
        - quick flash of tiny sparks, glitter cloud
        - short-lived, random velocities, low damping

     */
    // rules[7]
    rules.push(FireworkRule::new(
        8,
        0.25, 0.5,
        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new(1.0, 1.0, 1.0),
        0.01,
        vec![],
    ));

    /* Rule 9 - Wide Upward Spray
        - long-lasting angled spark plume
        - upward velocities between 10-15, sideways +- 15
        - high damping so persistent drift
     */
    // rules[8]
    rules.push(FireworkRule::new(
        9,
        3.0, 5.0,
        Vector3::new(-15.0, 10.0, -5.0),
        Vector3::new(15.0, 15.0, 5.0),
        0.95,
        vec![],
    ));

    rules
}

// This is the FireworksWorld -----------------------------------------------------
// Match Millington's maxFireworks = 1024
pub const MAX_FIREWORKS: usize = 1024;

// A world that manages a pool of fireworks and their rules.
pub struct FireworkWorld {
    // Fixed-size pool of fireworks (active and inactive).
    pub fireworks: [Firework; MAX_FIREWORKS],
    // Index of the next firework slot to use (ring buffer).
    pub next_firework: usize,
    // All firework rules (type 1..=rules.len()).
    pub rules: Vec<FireworkRule>,
}

impl FireworkWorld {
    // Create a new world with the given rules and an empty firework pool.
    pub fn new(rules: Vec<FireworkRule>) -> Self {
        FireworkWorld {
            fireworks: [Firework::default(); MAX_FIREWORKS],
            next_firework: 0,
            rules,
        }
    }
    // Helper: map a firework type_id (1..N) to a rule.
    /*fn find_rule(&self, type_id: u32) -> Option<&FireworkRule> {
        if type_id == 0 {
            return None;
        }
        let idx = (type_id - 1) as usize;
        self.rules.get(idx)
    } */

    // Spawn a single firework of a given type, optionally using a parent for position & velocity.
    pub fn spawn_single(
        &mut self,
        type_id: u32,
        parent: Option<&Firework>,
        rng: &mut Random,
    ) {
        if type_id == 0 {
            return;
        }
        let rule_index = (type_id - 1) as usize;
        if rule_index >= self.rules.len() {
            return;
        }
        // 1. Decide which firework slot to use and update the ring buffer index.
        let idx = self.next_firework;
        self.next_firework = (self.next_firework + 1) % MAX_FIREWORKS;

        // 2. Only borrow the rule immutably (after we've mutated self).
        let rule = &self.rules[rule_index];

        // 3. Create the firework in that slot.
        rule.create(&mut self.fireworks[idx], parent, rng);
    }

    // Spawn `count` fireworks of a given type using the same parent.
    pub fn spawn_many(
        &mut self,
        type_id: u32,
        count: u32,
        parent: &Firework,
        rng: &mut Random,
    ) {
        for _ in 0..count {
            self.spawn_single(type_id, Some(parent), rng);
        }
    }
    // Advance the simulation by `dt` seconds, updating all fireworks and
    // spawning payloads when fireworks die.
    pub fn update(&mut self, dt: Real, rng: &mut Random) {
        for i in 0..MAX_FIREWORKS {
            // Only process active fireworks (type_id > 0)
            if self.fireworks[i].type_id == 0 {
                continue;
            }

            // Step 1: update the physics & age
            let dead = {
                let fw = &mut self.fireworks[i];
                fw.update(dt)
            };
            if !dead {
                continue;
            }

            // Firework is dead: figure out its type
            let type_id = self.fireworks[i].type_id;
            if type_id == 0 {
                continue;
            }

            // Compute the rule index from the type_id
            let rule_index = (type_id - 1) as usize;

            //Make a copy of the firework to use as the parent for payloads.
            //  Firework is Copy, so it does not borrow self.
            let parent_snapshot = self.fireworks[i];

            //Mark this slot as unused before borrowing rules.
            self.fireworks[i].type_id = 0;

            // If there is no matching rule, stop here.
            if rule_index >= self.rules.len() {
                continue;
            }

            // Take a temporary copy of the payload data so we don't keep an immutable borrow
            //      of 'self' while calling 'spawn_many'.
            let payloads: Vec<Payload> = self.rules[rule_index].payloads.clone();

            //Now the immutable borrow ends here, we only use the local 'payloads' Vec
            for payload in payloads {
                self.spawn_many(payload.type_id, payload.count, &parent_snapshot, rng);
            }


        }
    } // fn update close
}// impl FireworkWorld close

