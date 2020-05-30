use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

mod person;

struct MainState {
    people: Vec<person::Person>,
    width: f32,
    height: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (width, height) = graphics::drawable_size(ctx);

        let people = person::create_people(100, width, height);

        let s = MainState {
            people: people,
            width: width,
            height: height,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for p in &mut self.people {
            person::update_person(p, self.width, self.height)
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for p in &self.people {
            person::draw_person(ctx, p)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
