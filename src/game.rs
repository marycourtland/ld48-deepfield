use wasm_bindgen::prelude::*;

use web_sys::console;

use anyhow::*;
#[macro_use] use super::utils;
use super::draw::*;
use super::points::Point;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::distributions::WeightedIndex;

// Game settings
const GAME_CANVAS_WIDTH: u32 = 800;
const GAME_CANVAS_HEIGHT: u32 = 600;

const GAME_CANVAS_ID: &str = "game-canvas";

const COLOR_SKY: &str = "#08011a";
const COLOR_GROUND_AT_DUSK: &str = "#24201a";
const COLOR_BLACK: &str = "black";
const COLOR_WHITE : &str = "white";

const RANDOM_SEED: u64 = 29292929;

pub fn start() {
    utils::set_panic_hook();
    configure_canvas(GAME_CANVAS_ID.to_string(), GAME_CANVAS_WIDTH, GAME_CANVAS_HEIGHT);
    let draw = Draw::from_canvas_id(GAME_CANVAS_ID.to_string()).unwrap();
    let rng = SmallRng::seed_from_u64(RANDOM_SEED);
    draw_background(&draw);
    draw_stars(&draw, rng, 800);
    draw_ground(&draw)
}

fn configure_canvas(canvas_id: String, width: u32, height: u32) {
    let canvas = utils::get_canvas_by_id(canvas_id).unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
}

fn draw_background(draw: &Draw) {
    draw.fill_all(COLOR_SKY.to_string());
}

fn draw_stars(draw: &Draw, mut rng: SmallRng, n: usize) {
    // star magnitude distributions
    let star_mag_buckets = [
        (1.0, 0.262),
        (1.2, 0.194),
        (1.4, 0.144),
        (1.6, 0.106),
        (1.8, 0.079),
        (2.0, 0.058),
        (2.2, 0.043),
        (2.4, 0.032),
        (2.6, 0.024),
        (2.8, 0.018),
        (3.0, 0.013),
        // (3.2, 0.010),
        // (3.4, 0.007),
        // (3.6, 0.005),
        // (3.8, 0.004),
        // (4.0, 0.003),
    ];
    let star_color_buckets = [
        ("#d6f0ff", 1),
        ("#e0e8ff", 2),
        ("#f8f7ff", 4),
        ("#fff4ea", 2),
        ("#ffe9d2", 1),
    ];
    let star_mag_dist = WeightedIndex::new(star_mag_buckets.iter().map(|star| star.1)).unwrap();
    let star_color_dist = WeightedIndex::new(star_color_buckets.iter().map(|star| star.1)).unwrap();
    let mut params = CanvasDrawParams::new().fill(COLOR_WHITE.to_string()).do_not_stroke(false);
    let star_mags: Vec<f64> = star_mag_dist.sample_iter(&mut rng).take(n).map(|i| star_mag_buckets[i].0).collect();
    let star_colors: Vec<&str> = star_color_dist.sample_iter(&mut rng).take(n).map(|i| star_color_buckets[i].0).collect();
    for i in 0..n {
        // Draw a star!
        let x: i64 = rng.gen_range(0..GAME_CANVAS_WIDTH).into();
        let y: i64 = rng.gen_range(0..GAME_CANVAS_HEIGHT).into();
        let r: f64 = star_mags[i];
        let a: f64 = rng.gen_range(0.3..1.0);
        let should_stroke = r >= 2.0; // adding an outline looks nice only on the larger stars
        params = params.fill(star_colors[i].to_string()).do_not_stroke(should_stroke).global_alpha(a);
        log!("star {} {} {}", i, r, star_colors[i]);
        draw.circle(Point::xy(x as f64, y as f64), r, &params);
    }
}

fn draw_ground(draw: &Draw) {
    let params = CanvasDrawParams::new().fill(COLOR_GROUND_AT_DUSK.to_string());
    draw.rect(
        Point::xy(0.0, GAME_CANVAS_HEIGHT as f64),
        Point::xy(GAME_CANVAS_WIDTH as f64, (GAME_CANVAS_HEIGHT - 50) as f64),
        &params
    );
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn refresh(v: Vec<i32>);
}

#[wasm_bindgen]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    x: i32,
    y: i32,
}

#[wasm_bindgen]
pub struct Canvas {
    pixels: Vec<Pixel>
}

#[wasm_bindgen]
impl Canvas {
    pub fn wref(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}


#[wasm_bindgen]
pub fn init() {
    log!("Hello, deep field!");

    let c = Canvas {
        pixels: vec![
            Pixel{ r: 251, g:1, b:11, a: 201, x: -1111, y: 101 },
            Pixel{ r: 252, g:2, b:22, a: 202, x: 222, y: 202 },
            Pixel{ r: 253, g:3, b:33, a: 203, x: 333, y: 303 },
        ]
    };

    log!("Canvas location in memory: {:?}", &c.wref());

    unsafe {
        refresh(vec![5,6,7,8]);
    }
}

