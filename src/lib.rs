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
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> ops::Add for Vector3<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> ops::Add for Vector4<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
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
    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl<T> ops::Add<T> for Vector3<T>
where
    T: Copy + ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T> ops::Add<T> for Vector4<T>
where
    T: Copy + ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

/// AddAssign
/// Vector + Vector

impl<T> ops::AddAssign for Vector2<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> ops::AddAssign for Vector3<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> ops::AddAssign for Vector4<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

/// AddAssign
/// Vector + number

impl<T> ops::AddAssign<T> for Vector2<T>
where
    T: Copy + ops::AddAssign,
{
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
    }
}

impl<T> ops::AddAssign<T> for Vector3<T>
where
    T: Copy + ops::AddAssign,
{
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl<T> ops::AddAssign<T> for Vector4<T>
where
    T: Copy + ops::AddAssign,
{
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
        self.w += other;
    }
}

/// Sub
/// Vector - Vector

impl<T> ops::Sub for Vector2<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> ops::Sub for Vector3<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> ops::Sub for Vector4<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
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
    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl<T> ops::Sub<T> for Vector3<T>
where
    T: Copy + ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T> ops::Sub<T> for Vector4<T>
where
    T: Copy + ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}

/// SubAssign
/// Vector + Vector

impl<T> ops::SubAssign for Vector2<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> ops::SubAssign for Vector3<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> ops::SubAssign for Vector4<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

/// SubAssign
/// Vector + number

impl<T> ops::SubAssign<T> for Vector2<T>
where
    T: Copy + ops::SubAssign,
{
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
    }
}

impl<T> ops::SubAssign<T> for Vector3<T>
where
    T: Copy + ops::SubAssign,
{
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl<T> ops::SubAssign<T> for Vector4<T>
where
    T: Copy + ops::SubAssign,
{
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
        self.w -= other;
    }
}

/// Mul (Component Wise Multiplication)
/// Vector * Vector

impl<T> ops::Mul for Vector2<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> ops::Mul for Vector3<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> ops::Mul for Vector4<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
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
    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> ops::Mul<T> for Vector3<T>
where
    T: Copy + ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> ops::Mul<T> for Vector4<T>
where
    T: Copy + ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

/// MulAssign (Component Wise Multiplication)
/// Vector * Vector

impl<T> ops::MulAssign for Vector2<T>
where
    T: ops::MulAssign,
{
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T> ops::MulAssign for Vector3<T>
where
    T: ops::MulAssign,
{
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T> ops::MulAssign for Vector4<T>
where
    T: ops::MulAssign,
{
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

/// MulAssign
/// Vector * number

impl<T> ops::MulAssign<T> for Vector2<T>
where
    T: Copy + ops::MulAssign,
{
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T> ops::MulAssign<T> for Vector3<T>
where
    T: Copy + ops::MulAssign,
{
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T> ops::MulAssign<T> for Vector4<T>
where
    T: Copy + ops::MulAssign,
{
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

/// Div (Component Wise Division)
/// Vector / Vector

impl<T> ops::Div for Vector2<T>
where
    T: ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> ops::Div for Vector3<T>
where
    T: ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> ops::Div for Vector4<T>
where
    T: ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

/// Div
/// Vector / number

impl<T> ops::Div<T> for Vector2<T>
where
    T: Copy + ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> ops::Div<T> for Vector3<T>
where
    T: Copy + ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> ops::Div<T> for Vector4<T>
where
    T: Copy + ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

/// DivAssign (Component Wise Division)
/// Vector * Vector

impl<T> ops::DivAssign for Vector2<T>
where
    T: ops::DivAssign,
{
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl<T> ops::DivAssign for Vector3<T>
where
    T: ops::DivAssign,
{
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T> ops::DivAssign for Vector4<T>
where
    T: ops::DivAssign,
{
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

/// DivAssign
/// Vector * number

impl<T> ops::DivAssign<T> for Vector2<T>
where
    T: Copy + ops::DivAssign,
{
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}

impl<T> ops::DivAssign<T> for Vector3<T>
where
    T: Copy + ops::DivAssign,
{
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T> ops::DivAssign<T> for Vector4<T>
where
    T: Copy + ops::DivAssign,
{
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

/// Neg

impl<T> ops::Neg for Vector2<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> ops::Neg for Vector3<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> ops::Neg for Vector4<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

/// Dot

impl<T> Vector2<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T> Vector3<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Vector4<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T>,
{
    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

/// Cross

impl<T> Vector3<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    fn cross(&self, other: &Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

/// Magnitude

impl<T> Vector2<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + Into<f64>,
{
    fn magnitude(&self) -> f64 {
        f64::sqrt((self.x * self.x + self.y * self.y).into())
    }
}

impl<T> Vector3<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + Into<f64>,
{
    fn magnitude(&self) -> f64 {
        f64::sqrt((self.x * self.x + self.y * self.y + self.z * self.z).into())
    }
}

impl<T> Vector4<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + Into<f64>,
{
    fn magnitude(&self) -> f64 {
        f64::sqrt((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).into())
    }
}

/// Distance

impl<T> Vector2<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Into<f64>,
{
    fn distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) * (self.x - other.x);
        let dy = (self.y - other.y) * (self.y - other.y);
        f64::sqrt((dx + dy).into())
    }
}

impl<T> Vector3<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Into<f64>,
{
    fn distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) * (self.x - other.x);
        let dy = (self.y - other.y) * (self.y - other.y);
        let dz = (self.z - other.z) * (self.z - other.z);
        f64::sqrt((dx + dy + dz).into())
    }
}

impl<T> Vector4<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Into<f64>,
{
    fn distance(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) * (self.x - other.x);
        let dy = (self.y - other.y) * (self.y - other.y);
        let dz = (self.z - other.z) * (self.z - other.z);
        let dw = (self.w - other.w) * (self.w - other.w);
        f64::sqrt((dx + dy + dz + dw).into())
    }
}

/// Distance Squared

impl<T> Vector2<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>,
{
    fn distance_squared(&self, other: &Self) -> T {
        let dx = (self.x - other.x) * (self.x - other.x);
        let dy = (self.y - other.y) * (self.y - other.y);
        dx + dy
    }
}

impl<T> Vector3<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>,
{
    fn distance_squared(&self, other: &Self) -> T {
        let dx = (self.x - other.x) * (self.x - other.x);
        let dy = (self.y - other.y) * (self.y - other.y);
        let dz = (self.z - other.z) * (self.z - other.z);
        dx + dy + dz
    }
}

impl<T> Vector4<T>
where
    T: Copy + ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T>,
{
    fn distance_squared(&self, other: &Self) -> T {
        let dx = (self.x - other.x) * (self.x - other.x);
        let dy = (self.y - other.y) * (self.y - other.y);
        let dz = (self.z - other.z) * (self.z - other.z);
        let dw = (self.w - other.w) * (self.w - other.w);
        dx + dy + dz + dw
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
