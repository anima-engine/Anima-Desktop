// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use math::Quaternion;

/// A simple vector `struct` tailored specifically for graphics.
///
/// # Examples
///
/// ```
/// # use anima_engine::math::Vector;
/// let v1 = Vector::zero();
/// let v2 = Vector::one();
///
/// assert_eq!(v1 + v2, Vector::one());
/// assert_eq!(v1 * v2, Vector::zero());
/// assert_eq!(v1.dot(v2), 0.0);
///
/// let v3 = v1;
///
/// assert_eq!(v1.dot(v2), 0.0);
/// assert_eq!((v3 + Vector::one() * 2.0).dot(v2), 6.0);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    /// `f32` *x* coordinate value
    pub x: f32,
    /// `f32` *y* coordinate value
    pub y: f32,
    /// `f32` *z* coordinate value
    pub z: f32
}

impl Vector {
    /// Creates a vector using 3 values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new(0.0, 1.0, 2.0);
    ///
    /// assert_eq!(v, Vector { x: 0.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    /// Creates a vector using an array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new_arr([0.0, 1.0, 2.0]);
    ///
    /// assert_eq!(v, Vector { x: 0.0, y: 1.0, z: 2.0 });
    /// ```
    pub fn new_arr(array: [f32; 3]) -> Vector {
        Vector { x: array[0], y: array[1], z: array[2] }
    }

    /// Creates a uniform vector using 1 value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new_unf(1.0);
    ///
    /// assert_eq!(v, Vector { x: 1.0, y: 1.0, z: 1.0 });
    /// ```
    pub fn new_unf(v: f32) -> Vector {
        Vector { x: v, y: v, z: v }
    }

    /// Creates a zero (0.0, 0.0, 0.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::zero(), Vector { x: 0.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Creates a one (1.0, 1.0, 1.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::one(), Vector { x: 1.0, y: 1.0, z: 1.0 });
    /// ```
    pub fn one() -> Vector {
        Vector { x: 1.0, y: 1.0, z: 1.0 }
    }

    /// Creates a back (0.0, 0.0, -1.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::back(), Vector { x: 0.0, y: 0.0, z: -1.0 });
    /// ```
    pub fn back() -> Vector {
        Vector { x: 0.0, y: 0.0, z: -1.0 }
    }

    /// Creates a down (0.0, -1.0, 0.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::down(), Vector { x: 0.0, y: -1.0, z: 0.0 });
    /// ```
    pub fn down() -> Vector {
        Vector { x: 0.0, y: -1.0, z: 0.0 }
    }

    /// Creates a forward (0.0, 0.0, 1.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::forward(), Vector { x: 0.0, y: 0.0, z: 1.0 });
    /// ```
    pub fn forward() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Creates a left (-1.0, 0.0, 0.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::left(), Vector { x: 1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn left() -> Vector {
        Vector { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Creates a right (1.0, 0.0, 0.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::right(), Vector { x: -1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn right() -> Vector {
        Vector { x: -1.0, y: 0.0, z: 0.0 }
    }

    /// Creates an up (0.0, 1.0, 0.0) Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// assert_eq!(Vector::up(), Vector { x: 0.0, y: 1.0, z: 0.0 });
    /// ```
    pub fn up() -> Vector {
        Vector { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Computes the length of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    ///
    /// assert_eq!(v.len(), 3.0);
    /// ```
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Computes the normalized version of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// let v = Vector::new(1.0, 2.0, 2.0);
    /// let n = v.norm();
    ///
    /// assert_eq!(n.len(), 1.0); // Keep precision in mind when comparing floats.
    /// ```
    pub fn norm(&self) -> Vector {
        let length = self.len();

        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length
        }
    }

    /// Computes the dot product between two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
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
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
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

    /// Rotates a vector according to the rotation represented by a quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// # use anima_engine::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let v = Vector::new(1.0, 0.0, 0.0);
    ///
    /// assert_eq!(v.rot(q), Vector { x: -1.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn rot(&self, quaternion: Quaternion) -> Vector {
        let result = quaternion *
                     Quaternion::new(self.x, self.y, self.z, 0.0) *
                     quaternion.conj();

        Vector { x: result.x, y: result.y, z: result.z }
    }

    /// Rotates a vector according to the rotation represented by the quaternion around a point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// # use anima_engine::math::Quaternion;
    /// let q = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let v = Vector::new(1.0, 0.0, 0.0);
    /// let p = Vector::new(2.0, 0.0, 0.0);
    ///
    /// assert_eq!(v.rot_around(q, p), Vector { x: 3.0, y: 0.0, z: 0.0 });
    /// ```
    pub fn rot_around(self, quaternion: Quaternion, point: Vector) -> Vector {
        (self - point).rot(quaternion) + point
    }

    /// Computes the angle in radians between two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// # use std::f32::consts;
    /// let v1 = Vector::new(1.0, 0.0, 0.0);
    /// let v2 = Vector::new(0.0, 2.0, 0.0);
    ///
    /// assert_eq!(v1.angle(v2), consts::PI / 2.0);
    /// ```
    pub fn angle(&self, other: Vector) -> f32 {
        self.norm().dot(other.norm()).acos()
    }

    /// Computes the distance between two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anima_engine::math::Vector;
    /// let v1 = Vector::new(0.0, 0.0, 0.0);
    /// let v2 = Vector::new(0.0, 1.0, 0.0);
    ///
    /// assert_eq!(v1.dist(v2), 1.0);
    /// ```
    pub fn dist(self, other: Vector) -> f32 {
        (self - other).len()
    }
}

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Neg;
use std::cmp::Ordering;
use math::Interpolate;

use mrusty::*;

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

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
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

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.len().partial_cmp(&other.len())
    }
}

impl Interpolate for Vector {
    fn interpolate(&self, other: Vector, ratio: f32) -> Vector {
        Vector {
            x: self.x * (1.0 - ratio) + other.x * ratio,
            y: self.y * (1.0 - ratio) + other.y * ratio,
            z: self.z * (1.0 - ratio) + other.z * ratio
        }
    }
}

mrusty_class!(Vector, {
    def!("initialize", |x: f64, y: f64, z: f64| {
        Vector::new(x as f32, y as f32, z as f32)
    });

    def_self!("from_a", |mruby, _slf: Value, array: Vec| {
        let x = array[0].to_f64().unwrap();
        let y = array[1].to_f64().unwrap();
        let z = array[2].to_f64().unwrap();

        let vector = Vector::new(x as f32, y as f32, z as f32);

        mruby.obj(vector)
    });

    def_self!("uniform", |mruby, _slf: Value, value: f64| {
        let value = value as f32;
        let vector = Vector::new_unf(value);

        mruby.obj(vector)
    });

    def_self!("zero", |mruby, _slf: Value| {
        mruby.obj(Vector::zero())
    });

    def_self!("one", |mruby, _slf: Value| {
        mruby.obj(Vector::one())
    });

    def_self!("back", |mruby, _slf: Value| {
        mruby.obj(Vector::back())
    });

    def_self!("down", |mruby, _slf: Value| {
        mruby.obj(Vector::down())
    });

    def_self!("forward", |mruby, _slf: Value| {
        mruby.obj(Vector::forward())
    });

    def_self!("left", |mruby, _slf: Value| {
        mruby.obj(Vector::left())
    });

    def_self!("right", |mruby, _slf: Value| {
        mruby.obj(Vector::right())
    });

    def_self!("up", |mruby, _slf: Value| {
        mruby.obj(Vector::up())
    });

    def!("==", |mruby, slf: Vector, other: Vector| {
        let result = slf.x == other.x &&
                     slf.y == other.y &&
                     slf.z == other.z;

        mruby.bool(result)
    });

    def!("to_s", |mruby, slf: Vector| {
        let string = format!("<Vector: @x={} @y={} @z={}>", slf.x, slf.y, slf.z);

        mruby.string(&string)
    });

    def!("+", |mruby, slf: Vector, other: Vector| {
        mruby.obj((*slf).clone() + (*other).clone())
    });

    def!("-", |mruby, slf: Vector, other: Vector| {
        mruby.obj((*slf).clone() - (*other).clone())
    });

    def!("*", |mruby, slf: Vector, other: Value| {
        match other.class().to_str() {
            "Float" => {
                let scalar = other.to_f64().unwrap();

                mruby.obj((*slf).clone() * (scalar as f32))
            }
            "Vector" => {
                let vector = other.to_obj::<Vector>().unwrap();

                mruby.obj((*slf).clone() * (*vector).clone())
            }
            _ => mruby.raise("TypeError", "expecting Float or Vector")
        }
    });

    def!("-@", |mruby, slf: Vector| {
        mruby.obj(-(*slf).clone())
    });

    def!("x", |mruby, slf: Vector| {
        mruby.float(slf.x as f64)
    });

    def!("y", |mruby, slf: Vector| {
        mruby.float(slf.y as f64)
    });

    def!("z", |mruby, slf: Vector| {
        mruby.float(slf.z as f64)
    });

    def!("len", |mruby, slf: Vector| {
        mruby.float(slf.len() as f64)
    });

    def!("norm", |mruby, slf: Vector| {
        mruby.obj(slf.norm())
    });

    def!("dot", |mruby, slf: Vector, other: Vector| {
        mruby.float(slf.dot((*other).clone()) as f64)
    });

    def!("cross", |mruby, slf: Vector, other: Vector| {
        mruby.obj(slf.cross((*other).clone()))
    });

    def!("rot", |mruby, slf: Vector, quternion: Quaternion| {
        mruby.obj(slf.rot((*quternion).clone()))
    });

    def!("rot_around", |mruby, slf: Vector, quternion: Quaternion, point: Vector| {
        mruby.obj(slf.rot_around((*quternion).clone(), (*point).clone()))
    });

    def!("angle", |mruby, slf: Vector, other: Vector| {
        mruby.float(slf.angle((*other).clone()) as f64)
    });

    def!("dist", |mruby, slf: Vector, other: Vector| {
        mruby.float(slf.dist((*other).clone()) as f64)
    });

    def!("<=>", |mruby, slf: Vector, other: Vector| {
        mruby.float((slf.len() - other.len()) as f64)
    });

    def!("interpolate", |mruby, slf: Vector, other: Vector, ratio: f64| {
        mruby.obj(slf.interpolate((*other).clone(), ratio as f32))
    });
});

#[cfg(test)]
mod tests {
    use mrusty::*;

    use super::Vector;
    use super::super::Quaternion;

    describe!(Vector, (Quaternion), "
      context 'when default' do
        it 'creates zero vector' do
          expect(Vector.zero).to eql Vector.uniform 0.0
        end

        it 'creates one vector' do
          expect(Vector.one).to eql Vector.uniform 1.0
        end

        it 'creates back vector' do
          expect(Vector.back).to eql Vector.new 0.0, 0.0, -1.0
        end

        it 'creates down vector' do
          expect(Vector.down).to eql Vector.new 0.0, -1.0, 0.0
        end

        it 'creates forward vector' do
          expect(Vector.forward).to eql Vector.new 0.0, 0.0, 1.0
        end

        it 'creates left vector' do
          expect(Vector.left).to eql Vector.new 1.0, 0.0, 0.0
        end

        it 'creates right vector' do
          expect(Vector.right).to eql Vector.new -1.0, 0.0, 0.0
        end

        it 'creates up vector' do
          expect(Vector.up).to eql Vector.new 0.0, 1.0, 0.0
        end
      end

      context 'when unit' do
        subject { Vector.uniform 1.0 }

        it 'returns x on #x' do
          expect(subject.x).to eql 1.0
        end

        it 'returns y on #y' do
          expect(subject.y).to eql 1.0
        end

        it 'returns z on #z' do
          expect(subject.z).to eql 1.0
        end

        it 'converts to String on #to_s' do
          expect(subject.to_s).to eql '<Vector: @x=1 @y=1 @z=1>'
        end

        it 'returns vector length on #len' do
          expect(subject.len).to be_within(0.000001).of 1.73205
        end

        it 'returns normalized vector on #norm' do
          norm = subject.norm

          expect(norm.x).to be_within(0.000001).of 0.57735
          expect(norm.y).to be_within(0.000001).of 0.57735
          expect(norm.z).to be_within(0.000001).of 0.57735
        end

        it 'computes dot product on #dot' do
          expect(subject.dot(Vector.new 1.0, 2.0, 3.0)).to eql 6.0
        end

        it 'computes cross product on #cross' do
          expect(subject.cross(Vector.new 1.0, 2.0, 3.0)).to eql Vector.new 1.0, -2.0, 1.0
        end

        it 'rotates on #rot' do
          up = subject.cross(Vector.new(-1.0, 0.0, 1.0))
          rotated = subject.rot Quaternion.rotation(up, Math::PI)

          expect(rotated.x).to be_within(0.000001).of -1.0
          expect(rotated.y).to be_within(0.000001).of -1.0
          expect(rotated.z).to be_within(0.000001).of -1.0
        end

        it 'rotates around a point on #rot_around' do
          rotated = subject.rot_around(Quaternion.rotation(Vector.up, Math::PI), subject)

          expect(rotated.x).to be_within(0.000001).of 1.0
          expect(rotated.y).to be_within(0.000001).of 1.0
          expect(rotated.z).to be_within(0.000001).of 1.0
        end

        it 'computes angle on #angle' do
          expect(subject.angle(Vector.new -1.0, -1.0, -1.0)).to be_within(0.01).of 3.14
        end

        it 'computes distance on #angle' do
          expect(subject.dist(Vector.new 1.0, -1.0, 1.0)).to eql 2.0
        end

        it 'adds vectors on #+' do
          expect(subject + Vector.new(1.0, 2.0, 3.0)).to eql Vector.new 2.0, 3.0, 4.0
        end

        it 'subtracts vectors on #-' do
          expect(subject - Vector.new(1.0, 2.0, 3.0)).to eql Vector.new 0.0, -1.0, -2.0
        end

        it 'multiplies vectors on #*' do
          expect(subject * Vector.new(1.0, 2.0, 3.0)).to eql Vector.new 1.0, 2.0, 3.0
        end

        it 'multiplies vector with a scalar on #*' do
          expect(subject * 2.0).to eql Vector.uniform 2.0
        end

        it 'returns the negative on #-@' do
          expect(-subject).to eql Vector.uniform -1.0
        end

        it 'interpolates on #interpolate' do
          expect(subject.interpolate(Vector.uniform(3.0), 0.5)).to eql Vector.uniform 2.0
        end
      end

      context 'when initialized from array' do
        subject { Vector.from_a [1.0, 2.0, 3.0] }

        it { is_expected.to eql Vector.new 1.0, 2.0, 3.0 }
      end

      context 'when in an array' do
        it 'sorts vectors by their length' do
          array = [Vector.uniform(2.0), Vector.uniform(3.0), Vector.uniform(1.0)]
          sorted = [Vector.uniform(1.0), Vector.uniform(2.0), Vector.uniform(3.0)]

          expect(array.sort).to eql sorted
        end
      end
    ");
}
