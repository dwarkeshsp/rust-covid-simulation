use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::Rng;
use std::f32;

#[derive(PartialEq)]
enum Status {
    Sick,
    Healthy,
}

pub struct Person {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    status: Status,
}

pub fn create_people(amount: i32, width: f32, heigth: f32) -> Vec<Person> {
    let mut rng = rand::thread_rng();

    let mut start_pos = |pos: f32| pos / 2.0 + 100.0 * rng.gen::<f32>();

    let new_person = |_| Person {
        x: start_pos(width),
        y: start_pos(heigth),
        dx: 0.0,
        dy: 0.0,
        status: Status::Healthy,
    };

    (0..amount).map(new_person).collect()
}

pub fn update_person(person: &mut Person, width: f32, heigth: f32) {
    let mut rng = rand::thread_rng();

    let mut new_d = |old_d: f32, pos: f32, center: f32| {
        let gravity = (center - pos).powi(2) / 10000.0;
        let mut result = if pos > center {
            old_d - gravity
        } else {
            old_d + gravity
        };
        result += rng.gen::<f32>() - 0.5;
        result
    };
    person.dx = new_d(person.dx, person.x, width / 2.0);
    person.dy = new_d(person.dy, person.y, heigth / 2.0);
    person.x += person.dx;
    person.y += person.dy;
}

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
        5.0,
        2.0,
        color,
    )?;
    graphics::draw(ctx, &circle, (na::Point2::new(person.x, person.y),))?;
    Ok(())
}
