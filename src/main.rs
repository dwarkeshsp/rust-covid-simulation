use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

mod person;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut person::Person::new(person::Status::Sick)?;
    event::run(ctx, event_loop, state)
}
