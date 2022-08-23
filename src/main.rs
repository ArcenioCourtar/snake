/* My first Rust project, for getting used to the language and syntax
 I used https://robertheaton.com/2018/12/02/programming-project-5-snake/ 
 by Robert Heaton for inspiration/guidance. These exercises are fun! */

use std::collections::VecDeque; // Vec Deque lets you pop elements off the vector in reverse order. Important!
use std::io;
use rand::Rng;

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

struct Snake {
	head: (i32, i32, i32), 				// (x coord, y coord, lifetime of tail segments)
	tail: VecDeque<(i32, i32, i32)>,	// (x coord, y coord, lifetime) of segment
	direction: i32,						// what da snake doin?
}

impl Snake {
	pub fn initialize(x: i32, y: i32) -> Self {
		Self {
			head: (x, y, 1),
			tail: VecDeque::new(),
			direction: RIGHT,
		}
	}

	// snake moves based on the value of direction
	pub fn take_step(&mut self) {
		let mut flag = false;
		self.tail.push_back(self.head);
		match self.direction {
			UP => self.head.1 -= 1,
			RIGHT => self.head.0 += 1,
			DOWN => self.head.1 += 1,
			LEFT => self.head.0 -= 1,
			_ => panic!("invalid direction"),
		}

		// lower the "lifetime" of each tail segment by 1
		// TODO: don't lower the lifetime on the tick an apple gets eaten
		// Currently segments get added too late due to how segment lifetime works atm
		for mut segment in &mut self.tail {
			segment.2 -= 1;
			if segment.2 == 0 {
				flag = true;
			}
		}

		// if lifetime of tail segment reaches zero, remove oldest segment
		if flag == true {
			self.tail.pop_front();
		}
	}

	// take user input, change snake direction accordingly
	// uses WASD + enter. only checks the first character put in
	pub fn set_direction(&mut self) {
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let first_char = input.chars().next().unwrap();
		if first_char == 'w' && self.direction != DOWN {
			self.direction = UP;
		} else if first_char == 'd' && self.direction != LEFT {
			self.direction = RIGHT;
		} else if first_char == 's' && self.direction != UP {
			self.direction = DOWN;
		} else if first_char == 'a' && self.direction != RIGHT {
			self.direction = LEFT;
		}
	}
}

// It's an apple
struct Apple {
	location: (i32, i32), 
	_value: i32, // score value, CURRENTLY UNUSED. Maybe grant bonus points for speed
}

impl Apple {
	// TODO: don't hardcode intial apple location. Probably don't need this function
	pub fn initialize() -> Self {
		Self {
			location: (2, 6),
			_value: 1,
		}
	}

	// places new apple in random location on the board after eating.
	// TODO: check if the selected location is already occupied by the snake
	pub fn new_loc(&mut self, game: &Game){
		self.location.0 = rand::thread_rng().gen_range(0..game.width);
		self.location.1 = rand::thread_rng().gen_range(0..game.height);
	}
}

struct Game {
	width: i32,
	height: i32,
	board: [[i32; 256]; 256], // board of 256 by 256 array.
	// TODO: make this a 1d vec with div/mod nonsense
}

impl Game {
	// initialize the board, completely empty
	pub fn initialize(width: i32, height: i32) -> Self {
		Self {
			width,
			height,
			board: [[0; 256]; 256],
		}
	}

	// Clears the board before checking so the update() function can update the board properly
	// TODO: as opposed to resetting the board entirely, remove this function
	// And set the tile behind the snake's fluffy tail to 0 every tick
	pub fn clear(&mut self) {
		for (_x, row) in self.board.iter_mut().enumerate() {
			for (_y, col) in row.iter_mut().enumerate() {
				*col = 0;
			}
		}
	}

	// Check if snek goes OOB, eats, or treads on itself
	pub fn check_state(&mut self, snake: &mut Snake, apple: &mut Apple) {
		// The snake wraps to the other side when going OOB.
		// Conveniently lets me skip bounds checking for now. :)
		// TODO: add option to make walls lethal
		if snake.head.0 < 0 {
			snake.head.0 = self.width - 1;
		} else if snake.head.0 >= self.width {
			snake.head.0 = 0;
		} else if snake.head.1 < 0 {
			snake.head.1 = self.height - 1;
		} else if snake.head.1 >= self.height {
			snake.head.1 = 0;
		}

		// did the snake succeed in eating a healthy vegetarian diet?
		if snake.head.0 == apple.location.0 && snake.head.1 == apple.location.1 {
			snake.head.2 += 1; // EXTEND!
			apple.new_loc(&self);
			for segment in &mut snake.tail {
				segment.2 += 1;
			}
		}

		// Don't tread on yourself. Doing that ends the game.
		for segment in &snake.tail {
			if snake.head.0 == segment.0 && snake.head.1 == segment.1 {
				println!("Game over!");
				std::process::exit(0);
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
					print!("■");
				} else {
					if self.board[x as usize][y as usize] == EMPTY {
						print!(" ");
					} else if self.board[x as usize][y as usize] == HEAD {
						print!("X");
					} else if self.board[x as usize][y as usize] == TAIL {
						print!("O");
					} else if self.board[x as usize][y as usize] == APPLE {
						print!("@");
					} 
				}
				x += 1;
			}
			print!("\n");
			x = -1;
			y += 1;
		}
	}
}

fn main() {
	let mut snake = Snake::initialize(5, 5);
	let mut game = Game::initialize(10, 10);
	let mut apple = Apple::initialize();

	// Bunch of TODOs: let game progress with a single keypress instead of having to press enter every time
	// Maybe make it progress with time?? :)
	// Add score display.
	// Add option to determine board size through command line args
	loop {
		game.clear();
		game.check_state(&mut snake, &mut apple);
		game.update(&snake, &apple);
		game.render();
		snake.set_direction();
		snake.take_step();
	}
}
