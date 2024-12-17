use image::{Rgb, RgbImage};
use nalgebra::Vector2;
use regex::Regex;

#[derive(Clone, Copy)]
struct Robot {
    p: Vector2<i32>,
    v: Vector2<i32>,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

// tweak these numbers until it works
// these numbers gave me only the correct answer ;3
const AVERAGE_BRIGHT: f32 = 0.045;
const OUTLIER_FACTOR: f32 = 1.065;

pub fn run(input: &String) {
    println!("--- Day 14: Restroom Redoubt ---");

    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(\-?[0-9]+),(\-?[0-9]+)").unwrap();

    let robots: Vec<Robot> = re
        .captures_iter(&input)
        .map(|cap| cap.extract::<4>().1.map(|s| s.parse().unwrap()))
        .map(|[x, y, vx, vy]| Robot {
            p: Vector2::new(x, y),
            v: Vector2::new(vx, vy),
        })
        .collect();

    let mut robots1 = robots.clone();
    step(&mut robots1, 100);

    println!("Star 1: {}", safety_factor(&robots1));

    println!("Star 2: [see images]");
    let mut robots2 = robots.clone();
    std::fs::create_dir_all("day14_images").expect("couldn't create image directory");
    for i in 0..10_000 {
        let img = to_image(&robots2);
        let brightness =
            img.pixels().map(|p| p.0[0] as i32 / 255).sum::<i32>() as f32 / (WIDTH * HEIGHT) as f32;
        if brightness > AVERAGE_BRIGHT * OUTLIER_FACTOR {
            println!("BRIGHTNESS: {brightness}");
            img.save(format!("day14_images/second_{}.png", i)).unwrap()
        }
        step(&mut robots2, 1);
    }
}

fn step(robots: &mut Vec<Robot>, count: i32) {
    for r in robots {
        r.p += r.v * count;
        r.p.x = ((r.p.x % WIDTH) + WIDTH) % WIDTH;
        r.p.y = ((r.p.y % HEIGHT) + HEIGHT) % HEIGHT;
    }
}

fn safety_factor(robots: &Vec<Robot>) -> usize {
    let mut quartiles = [0_usize; 4];
    for r in robots {
        if r.p.x == WIDTH / 2 || r.p.y == HEIGHT / 2 {
            continue;
        }

        let quartile = (r.p.x < WIDTH / 2) as usize + 2 * (r.p.y < HEIGHT / 2) as usize;

        quartiles[quartile] += 1;
    }
    quartiles.iter().product()
}

fn to_image(robots: &Vec<Robot>) -> RgbImage {
    let mut img: RgbImage = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for r in robots {
        img.put_pixel(r.p.x as u32, r.p.y as u32, Rgb([255, 255, 255]));
    }

    img
}
