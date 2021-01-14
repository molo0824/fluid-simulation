use vec::Vec2;
use collider::Boxcol;

const FULL: &str = "\u{2588}";
const UP: &str = "\u{2580}";
const DOWN: &str = "\u{2584}";

pub fn displayvoxel(voxel: Vec<i32>, width: usize, mut height: usize)
{
    if height % 2 != 0 {height -= 1;}
    for y in (0..height).step_by(2)
    {
        for x in 0..width
        {
            let a = voxel[x + y * width] > 0;
            let b = voxel[x + (y + 1) * width] > 0;

            let mut txt = "";

            if a && b {txt = FULL}
            if !a && !b {txt = " "}
            if !a && b {txt = DOWN}
            if a && !b {txt = UP}

            print!("{}", txt);
        }
        println!();
    }
}
pub fn setpos(voxel: &mut Vec<i32>, lvl: i32, width: usize, height: usize, pos: Vec2, thickness: f32)
{
    for y in 0..height
    {
        for x in 0..width
        {
            let crnt = Vec2::from(x as f32, y as f32);
            if Vec2::mag(crnt - pos) <= thickness {*voxel.get_mut(x + y * width).unwrap() = lvl;}
        }
    }
}
pub fn setcollider(voxel: &mut Vec<i32>, lvl: i32, width: usize, height: usize, mut col: Boxcol)
{
    for y in 0..height
    {
        for x in 0..width
        {
            let crnt = Vec2::rotate(Vec2::from(x as f32, y as f32) - col.pos, -col.angle) + col.pos;
            let diff = crnt - col.pos;
            if diff.x.abs() <= col.size.x / 2.0 && diff.y.abs() <= col.size.y / 2.0
            {
                *voxel.get_mut(x + y * width).unwrap() = lvl;
            }
        }
    }
}