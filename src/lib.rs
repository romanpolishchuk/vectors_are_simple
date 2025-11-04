use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

/// Add
/// Vector + Vector

impl<T> ops::Add for Vector2<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Add for Vector3<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> ops::Add for Vector4<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

/// Add
/// Vector + number

impl<T> ops::Add<T> for Vector2<T>
where
    T: Copy + ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> ops::Add<T> for Vector3<T>
where
    T: Copy + ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> ops::Add<T> for Vector4<T>
where
    T: Copy + ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

/// Sub
/// Vector - Vector

impl<T> ops::Sub for Vector2<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::Sub for Vector3<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> ops::Sub for Vector4<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/// Sub
/// Vector - number

impl<T> ops::Sub<T> for Vector2<T>
where
    T: Copy + ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T> ops::Sub<T> for Vector3<T>
where
    T: Copy + ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> ops::Sub<T> for Vector4<T>
where
    T: Copy + ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

/// Mul
/// Vector * number

impl<T> ops::Mul<T> for Vector2<T>
where
    T: Copy + ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> ops::Mul<T> for Vector3<T>
where
    T: Copy + ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> ops::Mul<T> for Vector4<T>
where
    T: Copy + ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

/// Dot

impl<T> Vector2<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T>,
{
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> Vector3<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T>,
{
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Vector4<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T>,
{
    fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

/// Cross

impl<T> Vector3<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    fn cross(self, rhs: Self) -> Self {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec2 = Vector2 { x: 5, y: 10 };
        vec2 = vec2 + 3;
        println!("{:?}", vec2)
    }
}
