use termion::clear::All;
use std::io::{Write, stdout};
use termsize::get;
use vec::Vec2;
use std::time::Instant;
use voxel::{displayvoxel, setpos, setcollider};
use collider::Boxcol;
use rigidbody::Rigid;
use rand::prelude::*;

const delay: u128 = 20;
const fps: u128 = 1000/30;
const radius: f32 = 1.5;

static mut width: usize = 0;
static mut height: usize = 0;

static mut particle: Vec<Rigid> = Vec::new();
static mut cols: Vec<Boxcol> = Vec::new();
static mut voxel: Vec<i32> = Vec::new();

fn termsize()
{
    unsafe
    {
        match get()
        {
            Some(n) => {width = n.cols as usize; height = (n.rows as usize - 1_usize) * 2_usize},
            None => {}
        }
    }
}
fn cls()
{
    println!("{}", All);
}
fn update()
{
    unsafe
    {
        voxel = vec![0; width * height];
        Rigid::update(&mut particle, delay as f32 * 4.0 / 1000.0, cols.clone());
        for i in particle.iter_mut()
        {
            setpos(&mut voxel, 1, width, height, i.pos, radius);
        }
        for i in cols.iter()
        {
            let mut col = *i;
            col.size.x -= radius;
            col.size.y -= radius;
            setcollider(&mut voxel, 1, width, height, col);
        }
    }
}
fn render()
{
    cls();
    unsafe
    {
        displayvoxel(voxel.clone(), width, height);
    }
}
fn main()
{
    let mut start = Instant::now();
    let mut start1 = Instant::now();
    unsafe
    {
        termsize();
        voxel = vec![0; width * height];

        let center = Vec2::from(width as f32 / 2.0, height as f32 / 2.0);
        
        cols = vec![
            Boxcol::from(Vec2::from(center.x, center.y + 20.0), Vec2::from(80.0, 5.0), 0.0, 0.0)
        ];

        let w = 6;
        let h = 6;
        for x in -w..w
        {
            for y in -h..h
            {
                let rng = rand::thread_rng();

                let mut pos = Vec2::from(x as f32 + rand::random::<f32>() / 2.0 - 0.25, y as f32 + rand::random::<f32>() / 2.0 - 0.25) * radius * 2.0 + center;

                particle.push(Rigid::from(pos, Vec2::new(), radius));
            }
        }
    }
    loop
    {
        if start.elapsed().as_millis() > delay
        {
            start = Instant::now();
            update();
        }
        if start1.elapsed().as_millis() > fps
        {
            start1 = Instant::now();
            render();
        }
    }
}