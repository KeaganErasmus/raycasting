use macroquad::prelude::*;

#[derive(Clone)]
struct Boundry {
    a: Vec2,
    b: Vec2,
}

impl Boundry {
    pub fn draw(&mut self) {
        draw_line(self.a.x, self.a.y, self.b.x, self.b.y, 2., WHITE);
    }
}

struct Ray {
    position: Vec2,
    dir: Vec2,
}

impl Ray {
    pub fn look_at(&mut self, x: f32, y: f32) {
        self.dir.x  = x - self.position.x;
        self.dir.y  = y - self.position.y;
        self.dir    = self.dir.normalize();
    }

    pub fn draw(&mut self) {
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.dir.x,
            self.position.y + self.dir.y,
            2.0,
            WHITE,
        );
    }

    pub fn cast(&mut self, wall: Boundry) -> Option<Vec2> {
        let x1 = wall.a.x;
        let y1 = wall.a.y;
        let x2 = wall.b.x;
        let y2 = wall.b.y;

        let x3 = self.position.x;
        let y3 = self.position.y;
        let x4 = self.position.x + self.dir.x;
        let y4 = self.position.y + self.dir.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if den == 0. {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if t > 0. && t < 1. && u > 0. {
            let mut pt: Vec2 = Vec2::new(0., 0.);
            pt.x = x1 - t * (x2 - x1);
            pt.y = y1 + t * (y2 - y1);

            return Some(pt);
        } else {
            return None;
        }
    }
}

struct Particle {
    pos: Vec2,
    rays: Vec<Ray>
}

impl Particle {
    
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut walls: Vec<Boundry> = Vec::new();
    for _ in 0..5{
        walls.push(Boundry {
            a: Vec2::new(300., 100.),
            b: Vec2::new(300., 300.),
        })
    }

    let mut ray: Ray = Ray {
        position: Vec2::new(100., 200.),
        dir: Vec2::new(1., 0.),
    };

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        clear_background(BLACK);

        for wall in walls.iter_mut() {
            wall.draw();
            let pt: Option<Vec2> = ray.cast(wall.clone());
            match pt {
                Some(pos) => {
                    draw_circle(pos.x, pos.y, 5.0, RED);
                }
                _ => {}
            }
        }

        ray.look_at(mouse_x, mouse_y);
        ray.draw();
        next_frame().await
    }
}
