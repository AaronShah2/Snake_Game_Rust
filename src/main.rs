extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

//used to store additional snake instances
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
// Directions Snake can go
enum Direction {
    Right, Left, Up, Down
}
struct Game {
    // graphics object that renders game
    gl: GlGraphics,
    
    // snake object in game
    snake: Snake,

    // food object in game
    food: Food,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        
        let PURPLE: [f32; 4] = [0.5, 0.2, 0.7, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(PURPLE, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn{
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

// snake class
struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    // renders snake on screen
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] =   [1.0, 0.0, 0.0, 1.0];

        // shape that will be rendered
        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x,y)| {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64,
                    20_f64)
            })
            .collect();
         
        // lets snake be rendered
        gl.draw(args.viewport(), |c,gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }
    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            // updates x coordinates of head
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            // updates y coordinates of head
            Direction::Up => new_head.1 -=1,
            Direction::Down => new_head.1 +=1,
        }

        // Code needed to update snake Positions
        // updates front position with new head
        self.body.push_front(new_head);

        // updates old position with old head
        self.body.pop_back().unwrap();
    }
}

// food class
struct Food {
    pos_x: i32,
    pos_y: i32,
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] =   [1.0, 0.0, 0.0, 1.0];

        // initializes food position & size
        let square = graphics::rectangle::square(
            (self.pos_x*20 + 5) as f64,
            (self.pos_y*20 + 5) as f64,
            10_f64);
        
        // lets food be rendered
        gl.draw(args.viewport(), |c,gl| {
            let transform = c.transform;
            graphics::rectangle(RED, square, transform, gl);
        });

    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    // initializes window
    let mut window: GlutinWindow = WindowSettings::new("Snake Game",[200, 200]) 
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    // initializes game
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()),
            dir: Direction::Right
            },
        food: Food {
            pos_x: 2,
            pos_y: 2,
        },
    };

    // Game driver
    let mut events = Events::new(EventSettings::new()).ups(6);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        // captures button presses
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
