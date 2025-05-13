// Filename: particle.rs
// Project: EntropicRust
// Description: Defines the Particle struct representing a single point in the simulation,
//              including its position, color, and trail. Also defines the SystemType enum.
//
// Author: Emanuel Lázaro
// Contact: emanuellzr01@outlook.com
// Copyright (c) 2025 Emanuel Lázaro
//
// License: MIT License
// See LICENSE file for details.
//

use ggez::graphics;
use ggez::mint::Point2;
use rand::Rng;
use std::collections::VecDeque;

pub const MAX_TRAIL_LENGTH: usize = 100;

#[derive(Clone, Copy, PartialEq)]
pub enum SystemType {
    Lorenz,
    Rossler,
    Aizawa,
    ChenLee,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub trail: VecDeque<Point2<f32>>,
    pub color: graphics::Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let mut rng = rand::thread_rng();
        Particle {
            x,
            y,
            z,
            trail: VecDeque::with_capacity(MAX_TRAIL_LENGTH),
            color: graphics::Color::new(
                rng.gen_range(0.5..1.0),
                rng.gen_range(0.5..1.0),
                rng.gen_range(0.5..1.0),
                1.0,
            ),
        }
    }

    pub fn update(&mut self, new_x: f32, new_y: f32, new_z: f32, screen_pos: Point2<f32>) {
        if self.trail.len() >= MAX_TRAIL_LENGTH && MAX_TRAIL_LENGTH > 0 {
            self.trail.pop_front();
        }
        if MAX_TRAIL_LENGTH > 0 {
            if self.trail.is_empty() {
                self.trail.push_back(screen_pos);
            }
            self.trail.push_back(screen_pos);
        }
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    pub fn get_screen_pos(&self, system_type: SystemType) -> Point2<f32> {
        let scale_factor = crate::system_parameters::get_scale_factor(system_type);
        Point2 {
            x: crate::main_state::SCREEN_WIDTH / 2.0 + self.x * scale_factor,
            y: crate::main_state::SCREEN_HEIGHT / 2.0 + self.y * scale_factor,
        }
    }
}
