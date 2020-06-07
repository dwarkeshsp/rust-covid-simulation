use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use rand::Rng;

mod person;

struct MainState {
    people: Vec<person::Person>,
    // screen dimensions
    width: f32,
    height: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (width, height) = graphics::drawable_size(ctx);
        let people = person::create_people(width, height);
        let state = MainState {
            people: people,
            width: width,
            height: height,
        };

        Ok(state)
    }

    fn handle_interactions(&mut self) {
        let mut rng = rand::thread_rng();

        // if a healthy person collides with a sick person, he might get sick
        for a in 0..self.people.len() {
            for b in 0..self.people.len() {
                let p_a = &self.people[a];
                let p_b = &self.people[b];

                if a != b
                    && !person::is_sick(p_a)
                    && person::is_sick(p_b)
                    && person::are_colliding(p_a, p_b)
                {
                    const CONTRACTION_PROB: f32 = 0.1;
                    if CONTRACTION_PROB > rng.gen::<f32>() {
                        person::make_sick(&mut self.people[a])
                    }
                };
            }
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for p in &mut self.people {
            person::update_person(p, self.width, self.height)
        }

        self.handle_interactions();
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
    let cb = ggez::ContextBuilder::new("contagion", "ggez")
        .window_setup(conf::WindowSetup::default().title("Contagion Simulation"));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
