use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::f64::consts::PI;
use std::fmt::{Display, Result, Formatter};
use lerp::lerp;

#[derive(Copy, Clone)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32
}
impl Add for Vec2
{
    type Output = Vec2;
    fn add(self, a: Vec2) -> Vec2 {Vec2{x: self.x + a.x, y: self.y + a.y}}
}
impl Sub for Vec2
{
    type Output = Vec2;
    fn sub(self, a: Vec2) -> Vec2 {Vec2{x: self.x - a.x, y: self.y - a.y}}
}
impl Mul<f32> for Vec2
{
    type Output = Vec2;
    fn mul(self, a: f32) -> Vec2 {Vec2{x: self.x * a, y: self.y * a}}
}
impl Div<f32> for Vec2
{
    type Output = Vec2;
    fn div(self, a: f32) -> Vec2 {Vec2{x: self.x / a, y: self.y / a}}
}
impl AddAssign for Vec2
{
    fn add_assign(&mut self, a: Vec2)
    {
        *self = *self + a;
    }
}
impl SubAssign for Vec2
{
    fn sub_assign(&mut self, a: Vec2)
    {
        *self = *self - a; 
    }
}
impl DivAssign<f32> for Vec2
{
    fn div_assign(&mut self, a: f32)
    {
        *self = *self / a;
    }
}
impl MulAssign<f32> for Vec2
{
    fn mul_assign(&mut self, a: f32)
    {
        *self = *self * a;
    }
}
impl Vec2
{
    pub fn new() -> Vec2 {Vec2{x: 0.0, y: 0.0}}
    pub fn from(x: f32, y: f32) -> Vec2 {Vec2{x: x, y: y}}
}
impl Display for Vec2
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Vec2
{
    pub fn lerp(a: Vec2, b: Vec2, v: f32) -> Vec2 {Vec2{x: lerp(a.x, b.x, v), y: lerp(a.y, b.y, v)}}
    pub fn dot(a: Vec2, b: Vec2) -> f32 {a.x * b.x + a.y * b.y}
    pub fn rotate(a: Vec2, angle: f32) -> Vec2
    {
        if angle == 0.0 {return Vec2::from(a.x, a.y);}
        let rad = angle as f64 * PI / 180.0;
        let c = rad.cos() as f32;
        let s = rad.sin() as f32;
        Vec2{x: a.x * c - a.y * s, y: a.x * s + a.y * c}
    }
    pub fn normal(a: Vec2) -> Vec2
    {
        if a.x == 0.0 && a.y == 0.0 {return Vec2{x: 0.0, y: 0.0}}
        let mag = (a.x * a.x + a.y * a.y).sqrt();
        a / mag
    }
    pub fn sqrmag(a: Vec2) -> f32 {a.x * a.x + a.y * a.y}
    pub fn mag(a: Vec2) -> f32 {(a.x * a.x + a.y * a.y).sqrt()}
}