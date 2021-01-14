use vec::Vec2;
use lerp::inverselerp;

#[derive(Copy, Clone)]
pub struct Boxcol
{
    pub pos: Vec2,
    pub size: Vec2,
    pub angle: f32,
    pub bouncy: f32
}
impl Boxcol
{
    pub fn new() -> Boxcol
    {
        Boxcol{pos: Vec2::new(), size: Vec2::new(), angle: 0.0, bouncy: 0.0}
    }
    pub fn from(pos: Vec2, size: Vec2, angle: f32, bouncy: f32) -> Boxcol
    {
        Boxcol{pos: pos, size: size, angle: angle, bouncy: bouncy}
    }
    pub fn cancollide(a: Boxcol, mut point: Vec2) -> Vec2
    {
        point = Vec2::rotate(point - a.pos, -a.angle) + a.pos;

        let size = a.size / 2.0;
        let start = a.pos - size;
        let end = a.pos + size;
        
        let margins = [point.x - start.x, point.y - start.y, end.x - point.x, end.y - point.y];
        let mut lowest = 0_usize;
        for i in 0_usize..4_usize
        {
            if margins[i] < 0.0 {return Vec2::new()}
            if margins[lowest] > margins[i] {lowest = i}
        }

        match lowest
        {
            1 => Vec2::rotate(Vec2::from(0.0, -1.0), a.angle),
            2 => Vec2::rotate(Vec2::from(1.0, 0.0), a.angle),
            3 => Vec2::rotate(Vec2::from(0.0, 1.0), a.angle),
            _ => Vec2::rotate(Vec2::from(-1.0, 0.0), a.angle)
        }
    }
}