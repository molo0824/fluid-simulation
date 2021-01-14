use vec::Vec2;
use collider::Boxcol;
use lerp::*;

#[derive(Copy, Clone)]
pub struct Rigid
{
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32
}
impl Rigid
{
    pub fn new() -> Rigid {Rigid{pos: Vec2::new(), vel: Vec2::new(), radius: 0.0}}
    pub fn from(pos: Vec2, vel: Vec2, radius: f32) -> Rigid {Rigid{pos: pos, vel: vel, radius: radius}}
    pub fn update(a: &mut Vec<Rigid>, delta: f32, cols: Vec<Boxcol>)
    {
        let mut clone = a.clone();
        for i in a.iter_mut()
        {
            for v in clone.iter_mut()
            {
                let dist = Vec2::mag(i.pos - v.pos);
                if dist < i.radius + v.radius
                {
                    let col = Vec2::normal(i.pos - v.pos);

                    let mut rel = Vec2::from(col.x * i.vel.x, col.y * i.vel.y);
                    if rel.x > 0.0 {rel.x = 0.0}
                    if rel.y > 0.0 {rel.y = 0.0}

                    let mut rel1 = Vec2::from(-col.x * v.vel.x, -col.y * v.vel.y);
                    if rel1.x > 0.0 {rel1.x = 0.0}
                    if rel1.y > 0.0 {rel1.y = 0.0}

                    i.vel += col * Vec2::mag(rel);
                    v.vel += col * Vec2::mag(rel1);
                }
            }
            i.vel.y += 9.18 * delta;
            for v in cols.iter()
            {
                let col = Boxcol::cancollide(*v, i.pos);
                let mut rel = Vec2::from(col.x * i.vel.x, col.y * i.vel.y);
                if rel.x > 0.0 {rel.x = 0.0}
                if rel.y > 0.0 {rel.y = 0.0}

                i.vel += col * Vec2::mag(rel);
            }
            i.pos += i.vel * delta;
        }
    }
}