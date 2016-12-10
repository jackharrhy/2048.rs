extern crate rand;

use std::io;
use rand::Rng;

type Array = [[i32; 4]; 4];

fn display_game(grid: &Array) {
	for x in 0..4 {
		for y in 0..4 {
			print!("{} ", grid[y][x]);
		}
		println!("");
	}
}

fn prompt(mut input: &mut String) {
	io::stdin().read_line(&mut input)
		.expect("Failed to read line");
}

fn random_insertion(mut grid: &mut Array) {
	loop {
		let random_x = rand::thread_rng().gen_range(0,4);
		let random_y = rand::thread_rng().gen_range(0,4);

		//println!("{} , {}", random_x, random_y);
		if grid[random_x][random_y] == 0 {
			grid[random_x][random_y] = 2;
			break;
		}
	}
}

fn get_direction_vector(mut direction_vector: &mut [i32; 2]) {
	let mut direction = String::new();
	prompt(&mut direction);
	println!("direction: {}", direction);
}

fn game_loop(mut grid: &mut Array) {
	let mut direction_vector = [0;2];
	get_direction_vector(&mut direction_vector);

	println!("{}", direction_vector[0]);
}

fn main() {
	let mut grid: Array = [[0;4];4];

	random_insertion(&mut grid);
	display_game(&grid);
	game_loop(&mut grid);
}
