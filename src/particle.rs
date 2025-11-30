/* From Millington's Cyclone:
    A particle is the simplest object that can be simulated in the physics system.

    It has position data (no orientation data), along with velocity.
    It can be integrated forward through time, and have linear forces, and impulses to it.
    The particle manages its state through a set of methods.

 */
use crate::core::Vector3;
use crate::precision::{Real, REAL_MAX, real_pow};

/* The Characteristic Data and State:
    Characteristics are properties of the particle independent of its current kinematic situation.
    This includes mass, moment of inertia, and damping properties. Two identical particles will
    have the same values for their characteristics.

    State includes all the characteristics and also includes the kinematic situation of the particle
    in the current simulation. By setting the whole state data, a particle's  exact game state can
    be replicated. Note that the state does not include any forces applied to the body. Two
    identical rigid bodies in the same simulation will not share the same state values.

    The state values make up the smallest set of independent data for the particle. Other state
    data is calculated from their current values. When state data is changed the dependent values
    need to be updated: this can be achieved either by integrating the simulation, or by calling
    the calculateInternals function. This two stage process is used because recalculating
    internals could be a costly process: all state changes should be carried out at the same time,
    allowing for a single call.

 */

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    //Characteristic data--------------------------------------------------------------------------

    /* inverse_mass: holds the inverse of the mass of the particle. It is more useful to hold the
        inverse mass because integration is simpler, and because in real time simulation, it is more
        useful to have objects with infinite mass (immovable) than zero mass (completely unstable
        in numerical simulation.
     */
    inverse_mass: Real,

    /* Holds the amount of damping applied to linear motion. Damping is required to remove energy
        added through numerical instability in the integrator. */
    damping: Real,

    //State----------------------------------------------------------------------------------------

    /* Holds the linear position of the particle in world space.*/
    position: Vector3,

    /* Holds the linear velocity of the particle in world space. */
    velocity: Vector3,

    //Force accumulator + constant acceleration

    /* Holds the accumulated force to be applied at the next simulation iteration only.
        This value is zeroed at each integration step. */
    force_accum: Vector3,

    /* Holds the acceleration of the particle. This value can be used to set acceleration due to
        gravity (its primary use), or any other constant acceleration. */
    acceleration: Vector3,
}

impl Particle {
    /* Create a default particle:
            mass = 1
            damping = 0.99
            zeroed vectors
     */
    pub fn new() -> Self {
        Self {
            inverse_mass: 1.0,
            damping: 0.99,
            position: Vector3::default(),
            velocity: Vector3::default(),
            force_accum: Vector3::default(),
            acceleration: Vector3::default(),
        }
    }
    // -----------------------------------------------------------------
    // Integration (Newton–Euler)
    // -----------------------------------------------------------------
    pub fn integrate (&mut self, duration: Real) {
        // Infinite-mass objects do not integrate
        if self.inverse_mass <= 0.0 {
            return;
        }

        assert!(duration > 0.0);

        // Update Linear position: position += velocity * duration
        self.position.add_scaled_vector (&self.velocity, duration);

        // Work out the acceleration from teh force accumulators.
        let mut resulting_acc = self.acceleration;
        resulting_acc.add_scaled_vector(&self.force_accum, self.inverse_mass);

        // Update linear velocity
        self.velocity.add_scaled_vector (&resulting_acc, duration);

        // Apply damping: velocity *= damping^duration
        self.velocity *= real_pow(self.damping, duration);

        // Clear accumulated forces
        self.clear_accumulator();

    }
    // -----------------------------------------------------------------
    // Mass / Inverse Mass
    // -----------------------------------------------------------------
    pub fn set_mass(&mut self, mass: Real) {
        assert!(mass != 0.0);
        self.inverse_mass = 1.0/mass;
    }

    pub fn get_mass(&self) -> Real {
        if self.inverse_mass == 0.0 {
            REAL_MAX
        } else {
            1.0 / self.inverse_mass
        }
    }
    pub fn set_inverse_mass(&mut self, inverse_mass: Real) {
        self.inverse_mass = inverse_mass;
    }

    pub fn get_inverse_mass(&self) -> Real {
        self.inverse_mass
    }

    pub fn has_finite_mass(&self) -> bool {
        // Millington: return inverseMasss >= 0.0f;
        self.inverse_mass >= 0.0
    }
    // -----------------------------------------------------------------
    // Damping
    // -----------------------------------------------------------------
    pub fn set_damping(&mut self, damping: Real) {
        self.damping = damping;
    }
    pub fn get_damping(&self) -> Real {
        self.damping
    }
    // -----------------------------------------------------------------
    // Position Accessors
    // -----------------------------------------------------------------
    pub fn set_position_vec(&mut self, position: Vector3) {
        self.position = position;
    }
    pub fn set_position(&mut self, x: Real, y: Real, z: Real) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }
    pub fn get_position_to(&self, out: &mut Vector3) {
        *out = self.position
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }
    // -----------------------------------------------------------------
    // Velocity Accessors
    // -----------------------------------------------------------------
    pub fn set_velocity_vec(&mut self, velocity: Vector3) {
        self.velocity = velocity;
    }

    pub fn set_velocity(&mut self, x: Real, y: Real, z: Real) {
        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
    }

    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }

    // -----------------------------------------------------------------
    // Acceleration Accessors
    // -----------------------------------------------------------------

    pub fn set_acceleration_vec(&mut self, acceleration: Vector3) {
        self.acceleration = acceleration;
    }

    pub fn set_acceleration(&mut self, x: Real, y: Real, z: Real) {
        self.acceleration.x = x;
        self.acceleration.y = y;
        self.acceleration.z = z;
    }

    pub fn get_acceleration_to(&self, out: &mut Vector3) {
        *out = self.acceleration;
    }

    pub fn get_acceleration(&self) -> Vector3 {
        self.acceleration
    }

    // -----------------------------------------------------------------
    // Forces
    // -----------------------------------------------------------------
    pub fn clear_accumulator(&mut self) {
        self.force_accum.clear();
    }

    pub fn add_force(&mut self, force: Vector3) {
        self.force_accum += force;
    }


}

