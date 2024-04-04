
// https://www.youtube.com/watch?v=U-X51GsTAzA
// Video explicativo sobre este software

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;


use vector::Vector;

fn main() {
    let window: Window = Window::new_centered(title: "Pendulum", size: (800, 480)).unwrap();
    let win: MyWindowHandler = MyWindowHandler{
        p: Pendulum::new(x: 400.0, y: 0.0, r: 200.0),
    };
    window.run_loop(handler: win);
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D){
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        self.p.draw(graphics);
    }
}

struct Pendulum{
    // Este vetor é a posição do pendulo
    origin: vector::Vector,
    // Este vetor é a posição da bola
    position: vector::Vector,

    // Angulo do pendulo
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, // Comprimento do pendulo
    m: f32, // Massa da bola
    g: f32, // Gravidade
}

impl Pendulum{
    fn new(x: f32, y: f32, r: f32) -> Pendulum{
        Pendulum{ origin: vector::Vector::new(x, y), position: vector::Vector::new(x: 0.0, y 0.0), angle: () angular_velocity: (), angular_acceleration: (), r: (), m: (), g: (),}
    }

    fn update(&mut self){
        // Usando a equação do pendulo para calcular a aceleração angular
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        // Velocidade angular é a velocidade angular mais aceleração angular
        self.angular_velocity += self.angular_acceleration;

        // O angulo é o angulo mais a velocidade angular
        self.angle += self.angular_velocity;

        // A posição é a coordenada polar traduzida em coordenadas cartesianas
        self.position Vector
        .set(x: self.r * self.angle.sin(), y: self.r * self.angle.cos())

        // Aposição final da bola é a original mais a posição do vetor
        self.position.add(&self.origin);

    }

    fn draw(&self, graphics: &mut Graphics2D){
        graphics.draw_line(
            start_position: (self.origin.x, self.origin.y),
            end_position: (self.position.x, self.position.y),
            thickness: 3.0,
            color: Color::RED,
        );
        graphics.draw_circle(center_position: (self.position.x, self.position.y), radius: 30.0,)
    }
}

mod vector {
    pub struct Vector{
        pub x: f32,
        pub y: f32,
    }

    impl Vector{
        pub fn new(x: f32, y: f32) -> Vector{
            Vector{
                x,
                y,
            }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }
        
        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}