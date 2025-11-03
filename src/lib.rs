use std::ops;

#[derive(Debug)]
struct Vector2<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug)]
struct Vector4<T> {
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
