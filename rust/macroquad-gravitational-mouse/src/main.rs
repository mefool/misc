use std::f32::consts::PI;

use macroquad::prelude::*;
use rand::Rng;
extern crate rand;

const WINDOW_SIZE: Vec2 = Vec2::from_array([800., 400.]);

fn window_config() -> Conf {
    Conf {
        window_title: "Collision Simulator".to_owned(),
        window_width: WINDOW_SIZE.x.round() as i32,
        window_height: WINDOW_SIZE.y.round() as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, Copy, Debug)]
struct Ball {
    pos: Vec2,
    v: Vec2,
    r: f32,
    mass: f32,
    color: Color,
}

impl Ball {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.color);
    }

    fn special_draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r*1.5, Color {
            r: 1.0, //self.color.r,
            g: 0.0, //self.color.g,
            b: 0.0, //self.color.b,
            a: 0.5,});
    }

    fn update(&mut self, dt: f32, acc: Vec2) {
        self.v += acc * dt;

        if self.pos.x < self.r && self.v.x < 0.
            || WINDOW_SIZE.x - self.pos.x < self.r && self.v.x > 0.
        {
            self.v.x *= -1.;
        }
        if self.pos.y < self.r && self.v.y < 0.
            || WINDOW_SIZE.y - self.pos.y < self.r && self.v.y > 0.
        {
            self.v.y *= -1.;
        }
        self.pos += self.v;
    }

    fn check_collision(&self, other: &Ball) -> bool {
        other.pos.distance(self.pos) <= other.r + self.r
    }

    // Does collision effect for both self and the other object
    // Based on https://www.vobarian.com/collisions/2dcollisions2.pdf
    // The individual steps from the document are commented
    fn collide(&mut self, other: &mut Ball) {
        let pos_diff = self.pos - other.pos;

        // 1
        let unit_normal = pos_diff.normalize();
        let unit_tangent = Vec2::from((-unit_normal.y, unit_normal.x));

        // 3
        let v1n = self.v.dot(unit_normal);
        let v1t = self.v.dot(unit_tangent);
        let v2n = other.v.dot(unit_normal);
        let v2t = other.v.dot(unit_tangent);

        // 5
        let new_v1n =
            (v1n * (self.mass - other.mass) + 2. * other.mass * v2n) / (self.mass + other.mass);
        let new_v2n =
            (v2n * (other.mass - self.mass) + 2. * self.mass * v1n) / (self.mass + other.mass);

        // 6
        let final_v1n = new_v1n * unit_normal;
        let final_v1t = v1t * unit_tangent;
        let final_v2n = new_v2n * unit_normal;
        let final_v2t = v2t * unit_tangent;

        // 7
        let final_v1 = final_v1n + final_v1t;
        let final_v2 = final_v2n + final_v2t;

        // The if statement makes them not get stuck in each other
        if (self.v - other.v).dot(self.pos - other.pos) < 0. {
            self.v = final_v1;
            other.v = final_v2;
        }
    }
}

// calculate atractor a



// Shameless copy-paste from collision simulator
#[macroquad::main(window_config)]
async fn main() {
    let mut rng = rand::thread_rng();
    let mut paused = true;
    let mut drawing_enabled = true;

    let n_balls = 40;
    let mut balls = Vec::with_capacity(n_balls);

    for i in 0..n_balls {
        let max_r = 8.;
        let r = rng.gen::<f32>() * max_r + max_r / 2.;
        balls.push(Ball {
            pos: Vec2::from((r * 2. + r * 2. * i as f32, r * 2. + r * i as f32)),
            v: Vec2::from((rng.gen::<f32>() * 4. - 2., rng.gen::<f32>() * 4. - 2.)),
            r: r,
            mass: PI * r.powf(2.),
            color: Color {
                r: rng.gen::<f32>() + 0.25,
                g: rng.gen::<f32>() + 0.25,
                b: rng.gen::<f32>() + 0.25,
                a: 1.,
            },
        })
    }

    // acceleration
    let mut a;
    let strength = 5.;

    // println!("{}", std::mem::size_of::<Ball>());

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::V) {
            drawing_enabled = !drawing_enabled;
        }
        if is_key_pressed(KeyCode::Z) {
            // println!(" id: speed");
            println!(" id: accel");
            let mut i = 0;
            for ball in &balls {
                let (x,y) = mouse_position();
                let mouse_vec2 = Vec2::new(x, y);

                let pos_diff = ball.pos - mouse_vec2;
                let unit_normal = pos_diff.normalize();
                println!("{:>3}: {:?}", i, unit_normal * 9.8 * ball.mass /
                         (ball.pos - mouse_vec2).length_squared());
                i += 1;
            }
        }
        if is_key_pressed(KeyCode::X) {
            println!("mouse position: {:?}", mouse_position());
            println!("mouse_button::left down: {:?}", is_mouse_button_down(MouseButton::Left));
            println!("mouse_button::right down: {:?}", is_mouse_button_down(MouseButton::Right));
        }

        a = Vec2::ZERO;
        if is_key_down(KeyCode::Left) {
            a.x = -strength;
        }
        if is_key_down(KeyCode::Up) {
            a.y = -strength;
        }
        if is_key_down(KeyCode::Right) {
            a.x = strength;
        }
        if is_key_down(KeyCode::Down) {
            a.y = strength;
        }

        let mut mouse_atractor = false;
        if is_mouse_button_down(MouseButton::Left) {
            mouse_atractor = true;
        }
        let mut mouse_detractor = false;
        if is_mouse_button_down(MouseButton::Right) {
            mouse_detractor = true;
        }

        if is_key_down(KeyCode::S) {
            for ball in &mut balls {
                ball.v *= 0.9;
            }
        }

        if is_key_down(KeyCode::A) {
            for ball in &mut balls {
                ball.v *= 0.99;
            }
        }

        if !paused {
            let dt = get_frame_time();

            for ball in balls.iter_mut() {
                if mouse_atractor || mouse_detractor {
                    let (x,y) = mouse_position();
                    let mouse_vec2 = Vec2::new(x, y);

                    let pos_diff = ball.pos - mouse_vec2;
                    let unit_normal = pos_diff.normalize();
                    let r_sq = (ball.pos - mouse_vec2).length_squared();
                    let clamp = 20.0;
                    let r_sq_clamp = if r_sq < clamp {clamp} else {r_sq};
                    let g = 20.0;
                    if mouse_atractor {
                        ball.update(dt, a - unit_normal * g * ball.mass / r_sq_clamp);
                    } else { // mouse_detractor
                        ball.update(dt, a + unit_normal * g * ball.mass / r_sq_clamp);
                    }
                } else {
                    ball.update(dt, a);
                }
            }

            balls.sort_by(|a, b| a.pos.x.partial_cmp(&b.pos.x).unwrap());
            let mut left_ball = 0;
            let mut right_bound = balls[left_ball].pos.x + balls[left_ball].r;

            for i in 1..balls.len() {
                if balls[i].pos.x - balls[i].r <= right_bound {
                    if balls[i].pos.x + balls[i].r > right_bound {
                        right_bound = balls[i].pos.x + balls[i].r;
                    }

                    let (left, right) = balls.split_at_mut(i);

                    for other_ball in &mut left[left_ball..i] {
                        if right[0].check_collision(other_ball) {
                            right[0].collide(other_ball);
                        }
                    }
                } else {
                    left_ball = i;
                    right_bound = balls[i].pos.x + balls[i].r;
                }
            }
        }

        clear_background(BLACK);
        if drawing_enabled {
            for ball in &balls {
                if ball.v.length_squared() > 1.0 {
                    ball.special_draw(); // added
                }
                if ball.pos.x < 1400. && ball.pos.y < 1000. {
                    ball.draw();
                }
            }
        }

        next_frame().await
    }
}
