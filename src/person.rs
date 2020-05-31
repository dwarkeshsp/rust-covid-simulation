use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::Rng;

#[derive(PartialEq)]
enum Status {
    Sick,
    Healthy,
    Recovered,
}

pub struct Person {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    status: Status,
}

pub fn is_sick(person: &Person) -> bool {
    person.status == Status::Sick
}

pub fn make_sick(person: &mut Person) {
    person.status = Status::Sick;
}

pub fn create_people(width: f32, heigth: f32) -> Vec<Person> {
    let mut rng = rand::thread_rng();
    // slightly randomize start location
    let mut start_pos = |pos: f32| pos / 2.0 + 250.0 * (rng.gen::<f32>() - 0.5);

    let mut new_person = |status: Status| Person {
        x: start_pos(width),
        y: start_pos(heigth),
        dx: 0.0,
        dy: 0.0,
        status: status,
    };

    const HEALTHY: u32 = 250;
    const SICK: u32 = 5;

    let mut people: Vec<Person> = (0..HEALTHY).map(|_| new_person(Status::Healthy)).collect();
    let mut sick_people: Vec<Person> = (0..SICK).map(|_| new_person(Status::Sick)).collect();
    people.append(&mut sick_people);
    people
}

pub fn update_person(person: &mut Person, width: f32, heigth: f32) {
    let mut rng = rand::thread_rng();

    let mut new_d = |old_d: f32, pos: f32, center: f32| {
        // prevents the people from zooming off the screen
        const SPREAD: f32 = 100000.0;
        let gravity = (center - pos).powi(2) / SPREAD;

        let mut result = if pos > center {
            old_d - gravity
        } else {
            old_d + gravity
        };
        result += rng.gen::<f32>() - 0.5;

        // prevents insane speeds
        const DECELERATION: f32 = 0.99;
        result * DECELERATION
    };

    person.dx = new_d(person.dx, person.x, width / 2.0);
    person.dy = new_d(person.dy, person.y, heigth / 2.0);
    person.x += person.dx;
    person.y += person.dy;

    if person.status == Status::Sick {
        const RECOVERY_PROB: f32 = 0.005;
        if RECOVERY_PROB > rng.gen::<f32>() {
            person.status = Status::Recovered
        }
    }
}

pub fn draw_person(ctx: &mut Context, person: &Person) -> GameResult {
    let color = if Status::Sick == person.status {
        graphics::BLACK
    } else if Status::Healthy == person.status {
        graphics::WHITE
    } else {
        // recovered
        graphics::Color::new(0.0078, 0.7647, 0.6039, 1.0)
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
