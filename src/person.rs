use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::Rng;

#[derive(PartialEq)]
enum Status {
    Sick,
    Healthy,
}

pub struct Person {
    pos_x: f32,
    pos_y: f32,
    dx: f32,
    dy: f32,
    status: Status,
}

pub fn create_people(amount: i32, width: f32, heigth: f32) -> Vec<Person> {
    let mut rng = rand::thread_rng();

    let new_person = |_| {
        let mut person = Person {
            pos_x: width / 2.0,
            pos_y: heigth / 2.0,
            dx: 0.0,
            dy: 0.0,
            status: Status::Healthy,
        };
        person
    };
    (0..amount).map(new_person).collect()
}

pub fn update_person(person: &mut Person) {}

pub fn draw_person(ctx: &mut Context, person: &Person) -> GameResult {
    let color = if Status::Sick == person.status {
        graphics::BLACK
    } else {
        graphics::WHITE
    };
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(0.0, 0.0),
        20.0,
        2.0,
        color,
    )?;
    graphics::draw(ctx, &circle, (na::Point2::new(person.pos_x, person.pos_y),))?;
    Ok(())
}
/*

impl Person {
    pub fn new(status: Status) -> GameResult<Person> {
        let s = Person {
            pos_x: 0.0,
            pos_y: 0.0,
            status: Status::Sick,
        };
        Ok(s)
    }
}
impl event::EventHandler for Person {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        self.pos_y = self.pos_y % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let color = if Status::Sick == self.status {
            graphics::BLACK
        } else {
            graphics::WHITE
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            20.0,
            2.0,
            color,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(self.pos_x, self.pos_y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
*/
