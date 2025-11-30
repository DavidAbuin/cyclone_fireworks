/* According to Millington:
   Vector3 is a vector that holds 3 dimensions. Four data members are allocated to ensure
   alignment in an array.
 */


 /* This is a minimal translation of core.h that focuses on what's need for Particle and the
    fireworks demo.
    :: means Path Separator - directs you to the location

  */

use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign, Mul, MulAssign};

use crate::precision::{Real, real_sqrt};

//Rust version of Cyclone Vector 3:
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vector3 {
    //Construct a new vector
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }

    //Set all components to zero (Cyclone's 'clear()').
    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    // Invert this vector (negate all components).
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    // Magnitude (length) of the vector.
    pub fn magnitude(&self) -> Real {
        real_sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    // Squared magnitude (avoids sqrt, useful for comparisons)
    pub fn square_magnitude(&self) -> Real {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Normalize this vector (make it unit length)
    pub fn normalize(&mut self) {
        let l = self.magnitude();
        if l > 0.0 {
            let inv = 1.0 / l;
            self.x *= inv;
            self.y *= inv;
            self.z *= inv;
        }
    }
    // Cyclone's 'addScaledVector': 'this += vector * scale'
    pub fn add_scaled_vector(&mut self, other: &Vector3, scale: Real) {
        self.x += other.x * scale;
        self.y += other.y * scale;
        self.z += other.z * scale;
    }

    // Component-wise product (Cyclone's 'componentProduct').
    pub fn component_product(&self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    //Update this vector to its component-wise product with 'other'
    // Cyclone's 'componentProductUpdate'
    pub fn component_product_update(&mut self, other: &Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }

    // Dot product (Cyclone's 'scalarProduct')
    pub fn scalar_product(&self, other: &Vector3) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product (Cyclone's 'vectorProduct')
    pub fn vector_product(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.x * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
// Standard directional vectors (equivalent to Millington's statics)
pub const GRAVITY: Vector3           = Vector3 { x: 0.0, y: -9.81, z: 0.0 };
pub const HIGH_GRAVITY: Vector3      = Vector3 { x: 0.0, y: -19.62, z: 0.0 };
pub const UP: Vector3                = Vector3 { x: 0.0, y: 1.0,  z: 0.0 };
pub const RIGHT: Vector3             = Vector3 { x: 1.0, y: 0.0,  z: 0.0 };
pub const OUT_OF_SCREEN: Vector3     = Vector3 { x: 0.0, y: 0.0,  z: 1.0 };
pub const X_AXIS: Vector3            = Vector3 { x: 1.0, y: 0.0,  z: 0.0 };
pub const Y_AXIS: Vector3            = Vector3 { x: 0.0, y: 1.0,  z: 0.0 };
pub const Z_AXIS: Vector3            = Vector3 { x: 0.0, y: 0.0,  z: 1.0 };


// Operator overloads
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
// Allow v[0], v[1], v[2] indexing
impl Index <usize> for Vector3 {
    type Output = Real;

    fn index (&self, i: usize) -> &Real {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic! ("Vector3 index {} is out of range!", i),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut Real {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic! ("Vector3 index {} is out of range!", i),
        }
}
}


impl Sub for Vector3 {
    type Output = Vector3;

    fn sub (self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl Mul<Real> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Real) -> Vector3 {
        Vector3::new(self.x * rhs, self.y *rhs, self.z * rhs)
    }
}

impl MulAssign<Real> for Vector3 {
    fn mul_assign(&mut self, rhs: Real) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}



