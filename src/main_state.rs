// Filename: main_state.rs
// Project: EntropicRust
// Description: Manages the main game state, particle simulation logic for different
//              chaotic attractors, rendering (particles and UI), and user input handling.
//
// Author: Emanuel Lázaro
// Contact: emanuellzr01@outlook.com
// Copyright (c) 2025 Emanuel Lázaro
//
// License: MIT License
// See LICENSE file for details.
//

use ggez::{
    event, graphics, Context, GameResult,
    input::keyboard::{KeyCode, KeyMods},
    mint::Point2,
};
use rand::Rng;

use crate::particle::{Particle, SystemType};
use crate::system_parameters::{SystemParameters, get_scale_factor};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

pub struct MainState {
    pub particles: Vec<Particle>,
    pub system_type: SystemType,
    pub parameters: SystemParameters,
    pub dt: f32,
    pub show_ui: bool,
    pub trail_enabled: bool,
    pub time_scale: f32,
    pub particle_count: usize,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let mut s = MainState {
            particles: Vec::new(),
            system_type: SystemType::Lorenz,
            parameters: SystemParameters::new(),
            dt: 0.01,
            show_ui: true,
            trail_enabled: true,
            time_scale: 1.0,
            particle_count: 50,
        };

        s.initialize_particles();

        Ok(s)
    }

    pub fn initialize_particles(&mut self) {
        self.particles.clear();
        let mut rng = rand::thread_rng();

        let (init_x_range, init_y_range, init_z_range) = match self.system_type {
            SystemType::Lorenz => (-1.0..1.0, -1.0..1.0, 15.0..25.0),
            SystemType::Rossler => (-1.0..1.0, -1.0..1.0, -1.0..1.0),
            SystemType::Aizawa => (-0.1..0.1, -0.1..0.1, -0.1..0.1),
            SystemType::ChenLee => (-1.0..1.0, -1.0..1.0, -1.0..1.0),
        };

        for _ in 0..self.particle_count {
            let x = rng.gen_range(init_x_range.clone());
            let y = rng.gen_range(init_y_range.clone());
            let z = rng.gen_range(init_z_range.clone());

            self.particles.push(Particle::new(x, y, z));
        }
    }

    pub fn update_particles(&mut self, _ctx: &mut Context) {
        let dt = self.dt * self.time_scale;

        for particle in self.particles.iter_mut() {
            let x = particle.x;
            let y = particle.y;
            let z = particle.z;

            let (dx, dy, dz) = match self.system_type {
                SystemType::Lorenz => {
                    let dx = self.parameters.sigma * (y - x);
                    let dy = x * (self.parameters.rho - z) - y;
                    let dz = x * y - self.parameters.beta * z;
                    (dx, dy, dz)
                },
                SystemType::Rossler => {
                    let dx = -y - z;
                    let dy = x + self.parameters.a * y;
                    let dz = self.parameters.b + z * (x - self.parameters.c);
                    (dx, dy, dz)
                },
                SystemType::Aizawa => {
                    let dx = (z - self.parameters.gamma) * x - self.parameters.delta * y;
                    let dy = self.parameters.delta * x + (z - self.parameters.gamma) * y;
                    let dz = self.parameters.alpha + self.parameters.beta * z - z.powi(3)/3.0 - (x*x + y*y) * (1.0 + self.parameters.epsilon * z) + self.parameters.delta * z * x*x*x;
                    (dx, dy, dz)
                },
                SystemType::ChenLee => {
                    let dx = self.parameters.p * x - y * z;
                    let dy = self.parameters.q * y + x * z;
                    let dz = self.parameters.r * z + x * y / 3.0;
                    (dx, dy, dz)
                },
            };

            let new_x = x + dx * dt;
            let new_y = y + dy * dt;
            let new_z = z + dz * dt;

            let scale_factor = get_scale_factor(self.system_type);
            let display_x = SCREEN_WIDTH / 2.0 + new_x * scale_factor;
            let display_y = SCREEN_HEIGHT / 2.0 + new_y * scale_factor;
            let screen_pos = Point2 { x: display_x, y: display_y };

            particle.update(new_x, new_y, new_z, screen_pos);
        }
    }

    pub fn draw_ui(&self, ctx: &mut Context) -> GameResult {
        let font = graphics::Font::default();
        let mut y_offset = 20.0;
        let line_height = 20.0;

        let system_name = match self.system_type {
            SystemType::Lorenz => "Lorenz",
            SystemType::Rossler => "Rossler",
            SystemType::Aizawa => "Aizawa",
            SystemType::ChenLee => "Chen-Lee",
        };

        let system_text = graphics::Text::new(graphics::TextFragment::new(
            format!("System: {} (Press 1-4 to change)", system_name)
        ).font(font).scale(graphics::PxScale::from(16.0)));

        graphics::draw(
            ctx,
            &system_text,
            graphics::DrawParam::default()
                .dest(Point2 { x: 20.0, y: y_offset })
                .color(graphics::Color::WHITE),
        )?;
        y_offset += line_height;

        let param_text_str = match self.system_type {
            SystemType::Lorenz => {
                format!(
                    "Parameters (Q/A: σ={:.2}, W/S: ρ={:.2}, E/D: β={:.2})",
                    self.parameters.sigma, self.parameters.rho, self.parameters.beta
                )
            },
            SystemType::Rossler => {
                format!(
                    "Parameters (Q/A: a={:.2}, W/S: b={:.2}, E/D: c={:.2})",
                    self.parameters.a, self.parameters.b, self.parameters.c
                )
            },
            SystemType::Aizawa => {
                format!(
                    "Parameters (Q/A: α={:.2}, W/S: γ={:.2}, E/D: δ={:.2}, R/F: ε={:.2}) (BETA: {:.2})",
                    self.parameters.alpha, self.parameters.gamma, self.parameters.delta, self.parameters.epsilon, self.parameters.beta
                )
            },
            SystemType::ChenLee => {
                format!(
                    "Parameters (Q/A: p={:.2}, W/S: q={:.2}, E/D: r={:.2})",
                    self.parameters.p, self.parameters.q, self.parameters.r
                )
            },
        };

        let param_text = graphics::Text::new(graphics::TextFragment::new(param_text_str)
            .font(font).scale(graphics::PxScale::from(16.0)));

        graphics::draw(
            ctx,
            &param_text,
            graphics::DrawParam::default()
                .dest(Point2 { x: 20.0, y: y_offset })
                .color(graphics::Color::WHITE),
        )?;
        y_offset += line_height;

        let time_text = graphics::Text::new(graphics::TextFragment::new(
            format!("Time Scale: {:.2}x (Z/X to adjust)", self.time_scale)
        ).font(font).scale(graphics::PxScale::from(16.0)));

        graphics::draw(
            ctx,
            &time_text,
            graphics::DrawParam::default()
                .dest(Point2 { x: 20.0, y: y_offset })
                .color(graphics::Color::WHITE),
        )?;
        y_offset += line_height;

        let particles_text = graphics::Text::new(graphics::TextFragment::new(
            format!("Particles: {} (C/V to adjust)", self.particle_count)
        ).font(font).scale(graphics::PxScale::from(16.0)));

        graphics::draw(
            ctx,
            &particles_text,
            graphics::DrawParam::default()
                .dest(Point2 { x: 20.0, y: y_offset })
                .color(graphics::Color::WHITE),
        )?;
        y_offset += line_height;

        let trail_text = graphics::Text::new(graphics::TextFragment::new(
            format!("Trails: {} (T to toggle)", if self.trail_enabled { "Enabled" } else { "Disabled" })
        ).font(font).scale(graphics::PxScale::from(16.0)));

        graphics::draw(
            ctx,
            &trail_text,
            graphics::DrawParam::default()
                .dest(Point2 { x: 20.0, y: y_offset })
                .color(graphics::Color::WHITE),
        )?;
        y_offset += line_height;

        let help_text = graphics::Text::new(graphics::TextFragment::new(
            "Press H to hide UI, R to reset particles, ESC to quit"
        ).font(font).scale(graphics::PxScale::from(16.0)));

        graphics::draw(
            ctx,
            &help_text,
            graphics::DrawParam::default()
                .dest(Point2 { x: 20.0, y: y_offset })
                .color(graphics::Color::WHITE),
        )?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update_particles(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.15, 1.0));
        let current_system_type = self.system_type;

        if self.trail_enabled {
            for particle in &self.particles {
                if particle.trail.len() < 2 {
                    continue;
                }
                let points: Vec<Point2<f32>> = particle.trail.iter().copied().collect();

                match graphics::Mesh::new_line(ctx, &points, 1.0, particle.color) {
                    Ok(line) => {
                        graphics::draw(ctx, &line, graphics::DrawParam::default())?;
                    }
                    Err(e) => {
                        eprintln!("Failed to create trail mesh: {:?}. Points: {:?}", e, points.len());
                    }
                }
            }
        }

        for particle in &self.particles {
            let screen_pos = particle.get_screen_pos(current_system_type);
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                screen_pos,
                2.0,
                0.1,
                particle.color,
            )?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }

        if self.show_ui {
            self.draw_ui(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Key1 => {
                if self.system_type != SystemType::Lorenz {
                    self.system_type = SystemType::Lorenz;
                    self.initialize_particles();
                }
            }
            KeyCode::Key2 => {
                if self.system_type != SystemType::Rossler {
                    self.system_type = SystemType::Rossler;
                    self.initialize_particles();
                }
            }
            KeyCode::Key3 => {
                if self.system_type != SystemType::Aizawa {
                    self.system_type = SystemType::Aizawa;
                    self.initialize_particles();
                }
            }
            KeyCode::Key4 => {
                if self.system_type != SystemType::ChenLee {
                    self.system_type = SystemType::ChenLee;
                    self.initialize_particles();
                }
            }
            KeyCode::Q => match self.system_type {
                SystemType::Lorenz => self.parameters.sigma += 0.1,
                SystemType::Rossler => self.parameters.a += 0.01,
                SystemType::Aizawa => self.parameters.alpha += 0.01,
                SystemType::ChenLee => self.parameters.p += 0.1,
            },
            KeyCode::A => match self.system_type {
                SystemType::Lorenz => self.parameters.sigma -= 0.1,
                SystemType::Rossler => self.parameters.a -= 0.01,
                SystemType::Aizawa => self.parameters.alpha -= 0.01,
                SystemType::ChenLee => self.parameters.p -= 0.1,
            },
            KeyCode::W => match self.system_type {
                SystemType::Lorenz => self.parameters.rho += 0.1,
                SystemType::Rossler => self.parameters.b += 0.01,
                SystemType::Aizawa => self.parameters.gamma += 0.01,
                SystemType::ChenLee => self.parameters.q += 0.1,
            },
            KeyCode::S => match self.system_type {
                SystemType::Lorenz => self.parameters.rho -= 0.1,
                SystemType::Rossler => self.parameters.b -= 0.01,
                SystemType::Aizawa => self.parameters.gamma -= 0.01,
                SystemType::ChenLee => self.parameters.q -= 0.1,
            },
            KeyCode::E => match self.system_type {
                SystemType::Lorenz => self.parameters.beta += 0.01,
                SystemType::Rossler => self.parameters.c += 0.01,
                SystemType::Aizawa => self.parameters.delta += 0.01,
                SystemType::ChenLee => self.parameters.r += 0.01,
            },
            KeyCode::D => match self.system_type {
                SystemType::Lorenz => self.parameters.beta -= 0.01,
                SystemType::Rossler => self.parameters.c -= 0.01,
                SystemType::Aizawa => self.parameters.delta -= 0.01,
                SystemType::ChenLee => self.parameters.r -= 0.01,
            },
            KeyCode::R => {
                if self.system_type == SystemType::Aizawa {
                    self.parameters.epsilon += 0.01;
                } else {
                    self.initialize_particles();
                }
            }
            KeyCode::F => {
                if self.system_type == SystemType::Aizawa {
                    self.parameters.epsilon -= 0.01;
                } else {
                    self.initialize_particles();
                }
            }
            KeyCode::Back => {
                self.initialize_particles();
            }
            KeyCode::Z => self.time_scale = (self.time_scale + 0.1).min(5.0),
            KeyCode::X => self.time_scale = (self.time_scale - 0.1).max(0.1),
            KeyCode::C => {
                self.particle_count = (self.particle_count + 5).min(200);
                self.initialize_particles();
            }
            KeyCode::V => {
                self.particle_count = (self.particle_count.saturating_sub(5)).max(5);
                if self.particle_count > 0 {
                    self.initialize_particles();
                }
            }
            KeyCode::T => self.trail_enabled = !self.trail_enabled,
            KeyCode::H => self.show_ui = !self.show_ui,
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }
}
