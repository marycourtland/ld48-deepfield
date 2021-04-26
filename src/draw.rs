#![allow(dead_code)]

use anyhow::*;
use std::{
    f64::consts,
    cmp
};
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

use super::points::*;
use super::utils::get_canvas_by_id;

pub mod common_colors {
    pub const BLACK: &str = "black";
    pub const WHITE : &str = "white";
}

pub struct CanvasDrawParams {
    // canvas properties
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub line_cap: Option<String>,
    pub line_width: Option<i32>,
    pub global_alpha: Option<f64>,

    // flags
    pub do_not_close: bool,
    pub do_not_fill: bool,
    pub do_not_stroke: bool,
}

impl CanvasDrawParams {
    pub fn new() -> Self {
        Self {
            fill: None,
            stroke: None,
            line_cap: None,
            line_width: None,
            global_alpha: None,
            do_not_close: false,
            do_not_fill: false,
            do_not_stroke: false,
        }
    }

    pub fn defaults() -> Self {
        let mut default_params = Self::new();
        default_params.fill = Some("transparent".to_string());
        default_params.stroke = Some("black".to_string());
        default_params.line_cap = Some("round".to_string());
        default_params.line_width = Some(1);
        default_params.global_alpha = Some(1.0);
        default_params.do_not_close = true;
        default_params.do_not_fill = false;
        default_params.do_not_stroke = false;
        default_params
    }

    pub fn fill(mut self, value: String) -> Self {
        self.fill = Some(value);
        self
    }
    pub fn stroke(mut self, value: String) -> Self {
        self.stroke = Some(value);
        self
    }
    pub fn line_cap(mut self, value: String) -> Self {
        self.line_cap = Some(value);
        self
    }
    pub fn line_width(mut self, value: i32) -> Self {
        self.line_width = Some(value);
        self
    }
    pub fn global_alpha(mut self, value: f64) -> Self {
        self.global_alpha = Some(value);
        self
    }
    pub fn do_not_close(mut self, value: bool) -> Self {
        self.do_not_close = value;
        self
    }
    pub fn do_not_fill(mut self, value: bool) -> Self {
        self.do_not_fill = value;
        self
    }
    pub fn do_not_stroke(mut self, value: bool) -> Self {
        self.do_not_stroke = value;
        self
    }

    pub fn config_context(&self, ctx: &CanvasRenderingContext2d) {
        if let Some(fill) = &self.fill {
            ctx.set_fill_style(&fill.into());
        }
        if let Some(stroke) = &self.stroke {
            ctx.set_stroke_style(&stroke.into());
        }
        if let Some(line_cap) = &self.line_cap {
            ctx.set_line_cap(&line_cap);
        }
        if let Some(line_width) = self.line_width {
            ctx.set_line_width(line_width as f64);
        }
        if let Some(global_alpha) = self.global_alpha {
            ctx.set_global_alpha(global_alpha as f64);
        }
    }
}

pub struct Draw {
    ctx: CanvasRenderingContext2d
}

impl Draw {
    pub fn from_canvas_context(context: CanvasRenderingContext2d) -> Self {
        Self {
            ctx: context
        }
    }

    pub fn from_canvas_id(canvas_id: String) -> Result<Self> {
        let canvas = get_canvas_by_id(canvas_id)?;
            
        let context = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| anyhow!("Draw::from_canvas_id: couldn't initialize the HTML canvas context"))?;

        Ok(Self {
            ctx: context
        })
    }

    pub fn draw<F>(
        &self,
        params: &CanvasDrawParams,
        draw_function: F
    ) where F: Fn(&CanvasRenderingContext2d, &CanvasDrawParams) -> () {
        self.ctx.save();
        params.config_context(&self.ctx);
        self.ctx.begin_path();
        draw_function(&self.ctx, &params);
        if !params.do_not_close { self.ctx.close_path(); }
        if !params.do_not_fill { self.ctx.fill(); }
        if !params.do_not_stroke { self.ctx.stroke(); }
        self.ctx.restore();
    }

    pub fn clear(
        &self,
        p0: Point, // good default: {x: -ctx.canvas().unwrap().width()/2, y:-ctx.canvas().unwrap().height()/2}
        size: Point // good default: {x: ctx.canvas().unwrap().width(), y: ctx.canvas().unwrap().height()}
    ) {
        self.ctx.close_path();
        self.ctx.clear_rect(p0.x, p0.y, size.x, size.y);
    }

    pub fn fill_all(
        &self,
        color: String
    ) {
        self.draw(&CanvasDrawParams::new().fill(color), |ctx, _params| {
            ctx.fill_rect(0.0, 0.0, ctx.canvas().unwrap().width() as f64, ctx.canvas().unwrap().height() as f64);
        })
    }

    pub fn line(
        &self,
        p0: Point,
        p1: Point,
        params: &CanvasDrawParams
    ) {
        self.draw(params, |ctx, _params| {
            ctx.move_to(p0.x, p0.y);
            ctx.line_to(p1.x, p1.y);
            ctx.move_to(p1.x, p1.y);
        })
    }

    pub fn rect(
        &self,
        p0: Point,
        p1: Point,
        params: &CanvasDrawParams
    ) {
        self.draw(params, |ctx, _params| {
            ctx.rect(
                p0.x + 0.5,
                p0.y + 0.5,
                p1.x - p0.x,
                p1.y - p0.y
            );
        })
    }

    pub fn rect_rounded(
        &self,
        p0: Point,
        p1: Point,
        corner_radius: f64,
        params: &CanvasDrawParams
    ) {
        let x0: f64 = p0.x;
        let y0: f64 = p0.y;
        let x1: f64 = p1.x;
        let y1: f64 = p1.y;
        let r: f64 = corner_radius; // for concision
        self.draw(
            params, |ctx, _params| {
            ctx.move_to(x0 + r, y0);
            ctx.line_to(x1 - r, y0);
            ctx.arc_to(x1, y0, x1, y0 + r, r).ok();
            ctx.line_to(x1, y1 - r);
            ctx.arc_to(x1, y1, x1 - r, y1, r).ok();
            ctx.line_to(x0 + r, y1);
            ctx.arc_to(x0, y1, x0, y1 - r, r).ok();
            ctx.line_to(x0, y0 + r);
            ctx.arc_to(x0, y0, x0 + r, y0, r).ok();
        })
    }

    pub fn circle(
        &self,
        center: Point,
        radius: f64,
        params: &CanvasDrawParams
    ) {
        self.draw(params, |ctx, _params| {
            ctx.arc(center.x, center.y, radius, 0.0, 2.0 * consts::PI).ok();
        })
    }

    pub fn arc(
        &self,
        center: Point,
        radius: f64,
        angle1: f64,
        angle2: f64,
        params: &CanvasDrawParams // Good default: do_not_close = true
    ) {
        self.draw(params, |ctx, _params| {
            ctx.arc(center.x, center.y, radius, angle1, angle2).ok();
        })
    }


    pub fn bezier(
        &self,
        p0: Point,
        p1: Point,
        c0: Point,
        c1: Point,
        params: &CanvasDrawParams // Good defaults: do_not_fill = true, do_not_close = true
    ) {
        self.draw(params, |ctx, _params| {
            ctx.move_to(p0.x, p0.y);
            ctx.bezier_curve_to(c0.x, c0.y, c1.x, c1.y, p1.x, p1.y);
        });
        // if (params.show_controls) {
        //     marker(ctx, xy(c0.x, c0.y));
        //     marker(ctx, xy(c1.x, c1.y));
        // }
    }

    pub fn polygon(
        &self,
        pts: Vec<Point>,
        params: &CanvasDrawParams // #fff is a good default line_style 
    ) {
        self.draw(params, |ctx, _params| {
            ctx.move_to(pts[0].x, pts[0].y);
            pts.iter().for_each(|p| {
                ctx.line_to(p.x, p.y);
            })
        })
    }

    pub fn text(
        &self,
        text_lines: Vec<String>,
        pos: RelativePoint,
        pos_loc: Anchor,
        fontsize: f64,
        params: &CanvasDrawParams
    ) {
        self.ctx.save();
        params.config_context(&self.ctx);
        // config_context(self.ctx, params);
        // The width of the whole thing will be the width of the largest line of text
        let text_width = text_lines.iter().fold(0, |max_width, next_line| {
            cmp::max(max_width, self.ctx.measure_text(next_line).unwrap().width().ceil() as i64)
        }) as f64;
    
        let text_height: f64 = fontsize; // todo: get this to handle multiple lines

        let mut position = Point::xy(0.0, 0.0);
        let w = self.ctx.canvas().unwrap().width() as f64;
        let h = self.ctx.canvas().unwrap().height() as f64;

        if let RelativePoint::CENTER = pos {
            position = Point::xy(
                w / 2.0 - text_width / 2.0,
                h / 2.0 + text_height / 2.0

            )
        }
        else {
            // let mut pos = pos.clone();
            let offset = pos_loc.into_point(text_width as f64, text_height);
            position += offset;
        }

        text_lines.iter().for_each(|text_line| {
            self.ctx.fill_text(text_line, position.x as f64, position.y as f64).ok();
            position += Point::xy(0.0, fontsize);
        });

        self.ctx.restore();
    }

    pub fn marker(
        &self,
        pos: Point,
        params: &CanvasDrawParams
    ) {
        self.circle(pos, 2.0, params);
    }

    /// :)
    pub fn get_pixel(
        &self,
        pos: Point
    ) -> [u8; 4] {
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        let c = self.ctx.canvas().unwrap();
        let w = c.width() as f64;
        let h = c.height() as f64;
        let imd = self.ctx.get_image_data(0.0, 0.0, w, h).unwrap().data();
        let pixel_index = 4 * (w as i32 * y + x) as usize;
        let mut rgba: [u8; 4] = [0, 0, 0, 0];
        for (index, value) in imd[pixel_index .. pixel_index + 4].iter().enumerate() {
            rgba[index] = *value;
        }
        rgba
    }

    pub fn line_gradient(
        &self,
        p0: Point,
        p1: Point,
        color0: String,
        color1: String,
        params: &CanvasDrawParams
    ) {
        self.draw(params, |ctx, params| {
            let dp = p1 - p0;
            let angle = dp.th();
            let length = dp.r();
            let w = params.line_width.unwrap_or(1) as f64;
            
            // translate context to p0 and rotate it so that the line is horizontal
            ctx.save();
            ctx.translate(p0.x, p0.y).ok();
            ctx.rotate(angle).ok();
            
            // Draw it
            let line = ctx.create_linear_gradient(0.0, 0.0, length, 0.0);
            line.add_color_stop(0.0, &color0).ok();
            line.add_color_stop(1.0, &color1).ok();
            ctx.set_fill_style(&line);
            ctx.fill_rect(0.0, -w / 2.0, length, w);
            
            // Put the context back how it was before
            ctx.restore();
        });
    }

    pub fn in_each_quadrant<F>(
        &self,
        draw_function: F
    ) where F: Fn() -> () {
        // Do the drawing four times, NE, NW, SE, and SW from the origin
        draw_function(); // se quadrant
        &self.ctx.scale(-1.0, 1.0).ok();
        draw_function(); // sw quadrant
        &self.ctx.scale(1.0, -1.0).ok();
        draw_function(); // nw quadrant
        &self.ctx.scale(-1.0, 1.0).ok();
        draw_function(); // ne quadrant
        &self.ctx.scale(1.0, -1.0).ok();
    }

    /// Draw a smiley for testing :)
    /// Example
    /// ```
    /// let draw = Draw::from_canvas_id("game-canvas".to_string()).unwrap();
    /// draw.smiley(
    ///     Point::xy(200.0, 75.0),
    ///     50.0,
    ///     &CanvasDrawParams::defaults().stroke("red".to_string())
    /// );
    /// ```
    pub fn smiley(
        &self,
        head_position: Point,
        head_size: f64,
        params: &CanvasDrawParams
    ) {
        self.circle(head_position, head_size, &params);
    
        // Smile
        self.arc(head_position, 0.7 * head_size, ANGLE_EAST, ANGLE_WEST, &params);
    
        // Eyes
        let eye_position_right = head_position + Point::xy(0.3,-0.2).scale(head_size);
        let eye_position_left = head_position + Point::xy(-0.3,-0.2).scale(head_size);
        let eye_position_center = head_position + Point::xy(0.0,-0.3).scale(head_size);
        
        self.circle(eye_position_right, 0.1 * head_size, &params);
        self.circle(eye_position_left, 0.1 * head_size, &params);
        self.circle(eye_position_center, 0.1 * head_size, &params);
    }
}