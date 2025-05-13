// Filename: system_parameters.rs
// Project: EntropicRust
// Description: Defines the SystemParameters struct holding the constants for various
//              chaotic systems (Lorenz, Rossler, etc.) and related helper functions like scaling.
//
// Author: Emanuel Lázaro
// Contact: emanuellzr01@outlook.com
// Copyright (c) 2025 Emanuel Lázaro
//
// License: MIT License
// See LICENSE file for details.
//

use crate::particle::SystemType;

#[derive(Clone, Copy)]
pub struct SystemParameters {
    // Lorenz
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
    // Rossler
    pub a: f32,
    pub b: f32,
    pub c: f32,
    // Aizawa
    pub alpha: f32,
    pub gamma: f32,
    pub delta: f32,
    pub epsilon: f32,
    // Chen-Lee
    pub p: f32,
    pub q: f32,
    pub r: f32,
}

impl SystemParameters {
    pub fn new() -> Self {
        SystemParameters {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            a: 0.2,
            b: 0.2,
            c: 5.7,
            alpha: 0.95,
            gamma: 0.6,
            delta: 3.5,
            epsilon: 0.25,
            p: 5.0,
            q: -10.0,
            r: -0.38,
        }
    }
}

pub fn get_scale_factor(system_type: SystemType) -> f32 {
    match system_type {
        SystemType::Lorenz => 10.0,
        SystemType::Rossler => 30.0,
        SystemType::Aizawa => 100.0,
        SystemType::ChenLee => 30.0,
    }
}
