use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast};
use gloo_timers::future::TimeoutFuture;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::distributions::WeightedIndex;
use rand::seq::IteratorRandom;
use super::draw::*;
use super::points::Point;
use web_sys::{EventTarget, MouseEvent};
use gloo_events::*;

#[macro_use]
mod types;
mod data;
use super::utils;

use types::*;

// Game settings
const GAME_CANVAS_WIDTH: u32 = 800;
const GAME_CANVAS_HEIGHT: u32 = 600;

const GAME_CANVAS_ID: &str = "game-canvas";

const COLOR_SKY: &str = "#08011a";
const COLOR_GROUND_AT_DUSK: &str = "#24201a";

const GAME_LOOP_MS: u32 = 1000;

// const RANDOM_SEED: u64 = 29292929;

pub fn start() {
    utils::set_panic_hook();
    configure_canvas(GAME_CANVAS_ID.to_string(), GAME_CANVAS_WIDTH, GAME_CANVAS_HEIGHT);
    let draw = Draw::from_canvas_id(GAME_CANVAS_ID.to_string()).unwrap();
    // let mut rng = SmallRng::seed_from_u64(RANDOM_SEED);
    let mut rng = SmallRng::from_entropy();
    draw_background(&draw);
    draw_stars(&draw, &mut rng, 800);
    draw_ground(&draw);

    let canvas = EventTarget::from(utils::query_html(&format!("#{}", GAME_CANVAS_ID)).unwrap());

    let on_click = EventListener::new(&canvas, "click", move |event| {
        let event = event.dyn_ref::<MouseEvent>().unwrap_throw();
        let mouse_point = Point::xy(event.layer_x().into(), event.layer_y().into());
        log!("Observing a spot in the sky: {} {}", mouse_point.x, mouse_point.y);
    });

    let cursor = utils::query_html("#eye-cursor").unwrap();

    let move_cursor_icon = EventListener::new(&canvas, "mousemove", move |event| {
        let event = event.dyn_ref::<MouseEvent>().unwrap_throw();
        let mouse_point = Point::xy(event.layer_x().into(), event.layer_y().into());
        let css = cursor.style();
        css.set_property("top", &format!("{}px", mouse_point.y - 20.0)).ok();
        css.set_property("left", &format!("{}px", mouse_point.x - 20.0)).ok();
    });

    // let these event listeners outlive this function
    on_click.forget();
    move_cursor_icon.forget();

    let mut g = Game::new();
    g.init();
    g.main_loop();


}

struct Game {
    pub state: GameState,
    pub astro_objects: Vec<AstroObject>,
    pub telescopes: TelescopeIndex,
    pub rng: SmallRng,
    pub generation: i32
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            state: GameState::init(),
            astro_objects: vec![],
            telescopes: data::game_telescopes(),
            rng: SmallRng::from_entropy(),
            generation: 0
        };
        let mut objects = data::game_objects();
        game.state.add_data(&mut objects);
        game
    }

    pub fn init(&mut self) {
        // Start with just your eye
        self.state.add_telescope(self.telescopes.get_by_key("eye").unwrap());
    }

    #[cfg(test)]
    pub fn test(&mut self) {
        self.state.add_telescope(self.telescopes.get_by_key("eye").unwrap());
        self.state.log();
        self.random_observation();
        self.random_observation();
        self.random_observation();
        self.random_observation();
        self.state.log();
    }

    pub fn main_loop(mut self) {
        log!("Game generation: {}", self.generation);

        self.acquire_telescopes();
        self.random_observation();
        self.state.log();

        self.proceed();
    }

    pub fn proceed(mut self) {
        self.generation += 1;
        spawn_local(async {
            TimeoutFuture::new(GAME_LOOP_MS).await;
            self.main_loop();
        });
    }

    pub fn acquire_telescopes(&mut self) {
        // todo
    }

    // Make a random observation.
    pub fn random_observation(&mut self) {
        // choose a random observable
        if self.state.observables.len() == 0 {
            log!("> There's nothing I can observe right now.");
            return;
        }
        let obj = self.state.observables.iter().choose(&mut self.rng).unwrap().clone();
        self.state.observables.remove(&obj);
        let detail_level = obj.detail.iter().fold(0, |level, next_detail| {
            if next_detail.power_needed <= self.state.max_power {
                next_detail.level
            }
            else {
                level
            }
        });
        log!("> You are observing {}. {}", obj.name, obj.detail[detail_level].discovery_text);
        self.state.observed.insert(obj, detail_level);
    }
}

fn configure_canvas(canvas_id: String, width: u32, height: u32) {
    let canvas = utils::get_canvas_by_id(canvas_id).unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
}

fn draw_background(draw: &Draw) {
    draw.fill_all(COLOR_SKY.to_string());
}

#[allow(clippy::many_single_char_names)]
fn draw_stars(draw: &Draw, mut rng: &mut SmallRng, n: usize) {
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
    let mut params = CanvasDrawParams::new().fill(common_colors::WHITE.to_string()).do_not_stroke(false);
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
        // log!("star {} {} {}", i, r, star_colors[i]);
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
