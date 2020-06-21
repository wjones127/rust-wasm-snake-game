use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Array;
use rand::Rng;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static EPSILON: f64 = 0.01;

fn are_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vector {
    // Represents a two-dimensional vector
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }

    pub fn scale_by(&self, factor: f64) -> Vector {
        Vector::new(self.x * factor, self.y * factor)
    }

    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }

    pub fn normalize(&self) -> Vector {
        self.scale_by(1_f64 / self.length())
    }

    pub fn equal_to(&self, other: &Vector) -> bool {
        are_equal(self.x, other.x) && are_equal(self.y, other.y)
    }

    pub fn is_opposite(&self, other: &Vector) -> bool {
        are_equal(self.x + other.x, 0_f64) && are_equal(self.y + other.y, 0_f64)
    }
}

fn get_segments_from_vectors(vectors: &[Vector]) -> Vec<Segment> {
    let pairs = vectors[..vectors.len() - 1].iter().zip(&vectors[1..]);
    pairs.map(|(start, end)| Segment::new(start, end))
        .collect::<Vec<Segment>>()
}

pub struct Segment<'a> {
    pub start: &'a Vector,
    pub end: &'a Vector,
}

impl<'a> Segment<'a> {
    pub fn new(start: &'a Vector, end: &'a Vector) -> Segment<'a> {
        Segment { start, end }
    }

    pub fn get_vector(&self) -> Vector {
        self.end.subtract(self.start)
    }

    pub fn length(&self) -> f64 {
        self.get_vector().length()
    }

    pub fn is_point_inside(&self, point: &Vector) -> bool {
        let first = Segment::new(self.start, point);
        let second = Segment::new(point, self.end);
        are_equal(self.length(), first.length() + second.length())
    }
}

fn get_food(width: u32, height: u32, snake: &[Vector]) -> Vector {
    let segments = get_segments_from_vectors(snake);
    let mut free_positions: Vec<Vector> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let point = Vector::new(f64::from(x) + 0.5, f64::from(y) + 0.5);
            if segments.iter().all(|s| !s.is_point_inside(&point)) {
                free_positions.push(point);
            }
        }
    }

    let index = rand::thread_rng().gen_range(0, free_positions.len());

    free_positions[index]
}

#[wasm_bindgen]
pub enum Movement {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[wasm_bindgen]
pub struct Game {
    pub width: u32,
    pub height: u32, // Number of cells 
    pub speed: f64, // Cells in ms
    pub score: u32,
    pub direction: Vector,
    pub next_direction: Vector, // Stores the intended direction
    pub food: Vector,
    snake: Vec<Vector>, 
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, speed: f64, snake_length: u32, direction: Vector) -> Game {
        let head_x = (f64::from(width) / 2_f64).round() - 0.5;
        let head_y = (f64::from(width) / 2_f64).round() - 0.5;
        let head = Vector::new(head_x, head_y);
        let tailtip = head.subtract(&direction.scale_by(f64::from(snake_length)));
        let snake = vec![tailtip, head];
        let food = get_food(width, height, &snake);

        Game {
            width: width,
            height: height,
            speed: speed,
            score: 0,
            direction: direction,
            next_direction: direction,
            food: food,
            snake: snake,
        }
    }

    pub fn get_snake(&self) -> Array {
        self.snake.clone().into_iter().map(JsValue::from).collect()
    }

    fn process_movement(&mut self, timespan: f64, movement: Option<Movement>) {
        let distance = self.speed * timespan;
        let mut tail: Vec<Vector> = Vec::new();
        let mut snake_distance = distance;
        while self.snake.len() > 1 {
            let point = self.snake.remove(0);
            let next = &self.snake[0];
            let segment = Segment::new(&point, next);
            let length = segment.length();
            if length < snake_distance {
                // Just remove the segment and move onto the next segment
                snake_distance -= length;
            } else {
                // Scale this last segment and break
                let vector = segment.get_vector().normalize().scale_by(snake_distance);
                tail.push(point.add(&vector));
                break
            }
        }

        tail.append(&mut self.snake);
        self.snake = tail;

        let old_head = self.snake.pop().unwrap();
        let new_head = old_head.add(&self.direction.scale_by(distance));
        if movement.is_some() {
            let new_direction = match movement.unwrap() {
                Movement::UP => Vector { x: 0_f64, y: -1_f64 },
                Movement::DOWN => Vector { x: 0_f64, y: 1_f64 },
                Movement::RIGHT => Vector { x: 1_f64, y: 0_f64 },
                Movement::LEFT => Vector { x: -1_f64, y: 0_f64 },

            };
            self.next_direction = new_direction;
        }

        if !self.direction.is_opposite(&self.next_direction) 
           && !self.direction.equal_to(&self.next_direction) {
            let Vector { x: old_x, y: old_y } = old_head;
            let old_x_rounded = old_x.round();
            let old_y_rounded = old_y.round();
            let new_x_rounded = new_head.x.round();
            let new_y_rounded = new_head.y.round();

            let rounded_x_change = !are_equal(old_x_rounded, new_x_rounded);
            let rounded_y_change = !are_equal(old_y_rounded, new_y_rounded);

            if rounded_x_change || rounded_y_change {
                let (old, old_rounded, new_rounded) = if rounded_x_change {
                    (old_x, old_x_rounded, new_x_rounded)
                } else {
                    (old_y, old_y_rounded, new_y_rounded)
                };
                let breakpoint_component = old_rounded 
                    + (if new_rounded > old_rounded {
                        0.5_f64
                    } else {
                        -0.5_f64
                    });
                let breakpoint = if rounded_x_change {
                    Vector::new(breakpoint_component, old_y)
                } else {
                    Vector::new(old_x, breakpoint_component)
                };
                let vector = self.next_direction.scale_by(distance - (old - breakpoint_component).abs());
                let head = breakpoint.add(&vector);
                self.snake.push(breakpoint);
                self.snake.push(head);
                self.direction = self.next_direction;
            }
            
        }
        
        self.snake.push(new_head);
    }

    pub fn process_food(&mut self) {
        let snake_len = self.snake.len();
        let head_segment = Segment::new(&self.snake[snake_len - 2], &self.snake[snake_len - 1]);
        if head_segment.is_point_inside(&self.food) {
            let tail_end = &self.snake[0];
            let before_tail_end = &self.snake[1];
            let tail_segment = Segment::new(&before_tail_end, &tail_end);
            let new_tail_end = tail_end.add(&tail_segment.get_vector().normalize());
            self.snake[0] = new_tail_end;
            self.food = get_food(self.width, self.height, &self.snake);
            self.score += 1;
        };
    }

    pub fn process(&mut self, timespan: f64, movement: Option<Movement>) {
        self.process_movement(timespan, movement);
        self.process_food();
    }

}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world! This is Will 🤓!"));

    Ok(())
}
