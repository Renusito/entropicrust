// Filename: main.rs
// Project: EntropicRust
// Description: Main entry point for the EntropicRust application. Initializes the
//              ggez game engine context and runs the main event loop with MainState.
//
// Author: Emanuel Lázaro
// Contact: emanuellzr01@outlook.com
// Copyright (c) 2025 Emanuel Lázaro
//
// License: MIT License
// See LICENSE file for details.
//

mod particle;
mod system_parameters;
mod main_state;

use ggez::{conf, event, GameResult};
use main_state::MainState;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("entropicrust", "emanuellcs")
        .window_setup(conf::WindowSetup::default().title("EntropicRust"))
        .window_mode(conf::WindowMode::default().dimensions(main_state::SCREEN_WIDTH, main_state::SCREEN_HEIGHT));

    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
