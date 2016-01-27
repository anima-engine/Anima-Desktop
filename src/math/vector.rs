// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

/// A simple vector `struct` tailored specifically for graphics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    /// `f32` x coordinate value
    pub x: f32,
    /// `f32` y coordinate value
    pub y: f32,
    /// `f32` y coordinate value
    pub z: f32
}

impl Vector {
    /// Creates a vector using 3 values.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(0.0, 1.0, 2.0);
    ///
    /// assert_eq!(v, Vector { x: 0.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    /// Creates a vector using an array.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new_arr([0.0, 1.0, 2.0]);
    ///
    /// assert_eq!(v, Vector { x: 0.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn new_arr(array: [f32; 3]) -> Vector {
        Vector { x: array[0], y: array[1], z: array[2] }
    }

    /// Creates a uniform vector using 1 value.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new_unf(1.0);
    ///
    /// assert_eq!(v, Vector { x: 1.0, y: 1.0, z: 1.0 });
    /// ```
    pub fn new_unf(v: f32) -> Vector {
        Vector { x: v, y: v, z: v }
    }

    /// Creates a zero (0.0, 0.0, 0.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::zero(), Vector { x: 0.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Creates a one (1.0, 1.0, 1.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::one(), Vector { x: 1.0, y: 1.0, z: 1.0 });
    /// ```
    pub fn one() -> Vector {
        Vector { x: 1.0, y: 1.0, z: 1.0 }
    }

    /// Creates a back (0.0, 0.0, -1.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::back(), Vector { x: 0.0, y: 0.0, z: -1.0 });
    /// ```
    pub fn back() -> Vector {
        Vector { x: 0.0, y: 0.0, z: -1.0 }
    }

    /// Creates a down (0.0, -1.0, 0.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::down(), Vector { x: 0.0, y: -1.0, z: 0.0 });
    /// ```
    pub fn down() -> Vector {
        Vector { x: 0.0, y: -1.0, z: 0.0 }
    }

    /// Creates a forward (0.0, 0.0, 1.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::forward(), Vector { x: 0.0, y: 0.0, z: 1.0 });
    /// ```
    pub fn forward() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Creates a left (-1.0, 0.0, 0.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::left(), Vector { x: -1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn left() -> Vector {
        Vector { x: -1.0, y: 0.0, z: 0.0 }
    }

    /// Creates a right (1.0, 0.0, 0.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::right(), Vector { x: 1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn right() -> Vector {
        Vector { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Creates an up (0.0, 1.0, 0.0) Vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// assert_eq!(Vector::up(), Vector { x: 0.0, y: 1.0, z: 0.0 });
    /// ```
    pub fn up() -> Vector {
        Vector { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Computes the length of a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    ///
    /// assert_eq!(v.length(), 3.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Computes the normalized version of a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    /// let n = v.normalize();
    ///
    /// assert_eq!(n.length(), 1.0); // Keep precision in mind when comparing floats.
    /// ```
    pub fn normalize(&self) -> Vector {
        let length = self.length();

        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length
        }
    }

    /// Computes the dot product between two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 2.0);
    /// let v2 = Vector::new(3.0, 3.0, 1.0);
    ///
    /// assert_eq!(v1.dot(v2), 11.0);
    /// ```
    pub fn dot(&self, other: Vector) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    /// Computes the cross product between two vectors.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 2.0);
    /// let v2 = Vector::new(3.0, 3.0, 1.0);
    ///
    /// assert_eq!(v1.cross(v2), Vector { x: -4.0, y: 5.0, z: -3.0 });
    /// ```
    pub fn cross(&self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    /// Converts the vector to an array of homogeneous coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// # use anima::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(v.to_array(), [1.0, 2.0, 3.0, 1.0]);
    /// ```
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, 1.0]
    }
}

use std::ops::Add;
use std::ops::Mul;
use std::cmp::Ordering;

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}
