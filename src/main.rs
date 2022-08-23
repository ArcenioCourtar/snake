/* My first Rust project, for getting used to the language and syntax
I used https://robertheaton.com/2018/12/02/programming-project-5-snake/
by Robert Heaton for inspiration/guidance. These exercises are fun! */

use rand::Rng;
use std::collections::VecDeque; // Vec Deque lets you pop elements off the vector in reverse order. Important!
use std::env;
use std::io;

// consts used for the snake direction
const UP: i32 = 0;
const RIGHT: i32 = 1;
const DOWN: i32 = 2;
const LEFT: i32 = 4;
// consts used to define board state
const EMPTY: i32 = 0;
const HEAD: i32 = 1;
const TAIL: i32 = 2;
const APPLE: i32 = 3;
const PORTAL: bool = false;
const WALL: bool = true;

struct Snake {
	head: (i32, i32, i32), // (x coord, y coord, lifetime of tail segments)
	tail: VecDeque<(i32, i32, i32)>, // (x coord, y coord, lifetime) of segment
	direction: i32,        // what da snake doin?
}

impl Snake {
	// place the snake in the middle of the board
	pub fn initialize(x: i32, y: i32) -> Self {
		Self {
			head: (x / 2, y / 2, 1),
			tail: VecDeque::new(),
			direction: RIGHT,
		}
	}

	// snake moves based on the value of direction
	pub fn take_step(&mut self, game: &mut Game) {
		let mut flag = false;
		self.tail.push_back(self.head);
		match self.direction {
			UP => self.head.1 -= 1,
			RIGHT => self.head.0 += 1,
			DOWN => self.head.1 += 1,
			LEFT => self.head.0 -= 1,
			_ => panic!("invalid direction. How'd you do that?"),
		}

		// lower the "lifetime" of each tail segment by 1
		for mut segment in &mut self.tail {
			segment.2 -= 1;
			if segment.2 == 0 {
				flag = true;
			}
		}

		// if lifetime of tail segment reaches zero, remove oldest segment
		if flag == true {
			let temp = self.tail.pop_front();
			match temp {
				Some(i) => game.board[i.0 as usize][i.1 as usize] = 0,
				None => panic!("This should never happen"),
			}
		}
	}

	// take user input, change snake direction accordingly
	// uses WASD + enter. only checks the first character put in
	pub fn set_direction(&mut self, game: &Game) {
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let first_char = input.chars().next().unwrap();
		match first_char {
			'w' if self.direction != DOWN => self.direction = UP,
			'd' if self.direction != LEFT => self.direction = RIGHT,
			's' if self.direction != UP => self.direction = DOWN,
			'a' if self.direction != RIGHT => self.direction = LEFT,
			't' => game.end_game(),
			_ => (),
		}
	}
}

// It's an apple
struct Apple {
	location: (i32, i32),
	_value: i32, // score value, CURRENTLY UNUSED. Maybe grant bonus points for speed
}

impl Apple {
	// first apple always appears diagionally lower right from the snake head
	pub fn initialize(x: i32, y: i32) -> Self {
		Self {
			location: (x/2 + 1, y/2 + 1),
			_value: 1,
		}
	}

	// places new apple in random location on the board after eating.
	pub fn new_loc(&mut self, game: &Game) {
		loop {
			self.location.0 = rand::thread_rng().gen_range(0..game.width);
			self.location.1 = rand::thread_rng().gen_range(0..game.height);
			// If the generated location is already occupied by a snake it tries a new location
			// VERY INEFFICIENT and never ends looping if the player somehow fills the entier board with it's own body
			if game.board[self.location.0 as usize][self.location.1 as usize] == 0 {
				break;
			}
		}
	}
}

struct Game {
	width: i32,
	height: i32,
	board: [[i32; 25]; 25], // board of 256 by 256 array.
	                          // TODO: make this a 1d vec with div/mod nonsense
	edge_state: bool,
	score: i32,
}

impl Game {
	// initialize the board, completely empty
	pub fn initialize(width: i32, height: i32, edge_state: bool) -> Self {
		Self {
			width,
			height,
			board: [[0; 25]; 25],
			edge_state,
			score: 0,
		}
	}

	// Check if snek goes OOB, eats, or treads on itself
	pub fn check_state(&mut self, snake: &mut Snake, apple: &mut Apple) {

		// Snake wraps around the edges when the edges are portals (default)
		if self.edge_state == PORTAL {
			if snake.head.0 < 0 {
				snake.head.0 = self.width - 1;
			} else if snake.head.0 >= self.width {
				snake.head.0 = 0;
			} else if snake.head.1 < 0 {
				snake.head.1 = self.height - 1;
			} else if snake.head.1 >= self.height {
				snake.head.1 = 0;
			} // How do I turn this into a match expression??
		}

		// Snake dies when running into an edge when the edges are walls
		if self.edge_state == WALL {
			if snake.head.0 < 0 {
				self.end_game();
			} else if snake.head.0 >= self.width {
				self.end_game();
			} else if snake.head.1 < 0 {
				self.end_game();
			} else if snake.head.1 >= self.height {
				self.end_game();
			}
		}

		// did the snake succeed in eating a healthy vegetarian diet?
		if snake.head.0 == apple.location.0 && snake.head.1 == apple.location.1 {
			snake.head.2 += 1; // EXTEND!
			self.score += 1;
			apple.new_loc(&self);
			// increase the lifetime of all existing tail segments by 1 so the snake extends on the next tick
			for segment in &mut snake.tail {
				segment.2 += 1;
			}
		}

		// Don't tread on yourself. Doing that ends the game.
		for segment in &snake.tail {
			if snake.head.0 == segment.0 && snake.head.1 == segment.1 {
				self.end_game();
			}
		}
	}

	// Update the board based on snek position
	pub fn update(&mut self, snake: &Snake, apple: &Apple) {
		self.board[snake.head.0 as usize][snake.head.1 as usize] = HEAD;
		for segment in &snake.tail {
			self.board[segment.0 as usize][segment.1 as usize] = TAIL;
		}
		self.board[apple.location.0 as usize][apple.location.1 as usize] = APPLE;
	}

	// Render the board
	pub fn render(&self) {
		let mut x = -1;
		let mut y = -1;
		while y < (self.height + 1) {
			while x < (self.width + 1) {
				if x == -1 || x == self.width || y == -1 || y == self.height {
					if self.edge_state == WALL {
						print!("█");
					}
					if self.edge_state == PORTAL {
						print!("░");
					}
				} else {
					match self.board[x as usize][y as usize] {
						EMPTY => print!(" "),
						HEAD => print!("X"),
						TAIL => print!("O"),
						APPLE => print!("@"),
						_ => panic!("This should never happen."),
					}
				}
				x += 1;
			}
			print!("\n");
			x = -1;
			y += 1;
		}
		println!("Score {}", self.score);
	}

	// Function that ends the game and displays the player's score
	fn end_game(&self) {
		println!("Game over!");
		println!("Your score: {}", self.score);
		std::process::exit(0);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let width: i32;
	let height: i32;
	let edge_state: bool;

	// Parsing the command line args, if any.
	if args.len() == 3 || args.len() == 4 {
		width = match args[1].parse::<i32>() {
			Ok(number) if number >= 5 && number <= 25 => number,
			_ => 10,
		};
		height = match args[2].parse::<i32>() {
			Ok(number) if number >= 5 && number <= 25 => number,
			_ => 10,
		};
		if args.len() == 4 {
			if args[3] == "Wall" {
				edge_state = WALL;
			} else {
				edge_state = PORTAL;
			}
		} else {
			edge_state = PORTAL;
		}
	} else {
		width = 10;
		height = 10;
		edge_state = PORTAL;
	}

	// intialize state of structs we'll be using based on user input
	let mut game = Game::initialize(width, height, edge_state);
	let mut snake = Snake::initialize(width, height);
	let mut apple = Apple::initialize(width, height);

	// endlessly loop until Game::end_game() gets called by another function
	loop {
		game.check_state(&mut snake, &mut apple);
		game.update(&snake, &apple);
		game.render();
		snake.set_direction(&game);
		snake.take_step(&mut game);
	}
}
