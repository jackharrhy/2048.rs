extern crate rand;

use std::io;
use std::process::Command;
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

fn get_direction_vector() -> [i32;2] {
	let mut direction = String::new();
	prompt(&mut direction);
	println!("direction: {}", direction);
	match direction.as_str() {
		"h\n" => [-1,0],
		"j\n" => [0,1],
		"k\n" => [0,-1],
		"l\n" => [1,0],
		_ => [0,0]
	}
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

fn clear_screen() {
	let output = Command::new("clear").output().unwrap_or_else(|e| {
    	panic!("failed to execute process: {}", e)
  	});
  	println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
	let mut grid: Array = [[0;4];4];
	let mut prev_grid: Array;

	loop {
		random_insertion(&mut grid);

		clear_screen();
		display_game(&grid);

		let direction_vector: [i32;2] = get_direction_vector();
		println!("{},{}", direction_vector[0], direction_vector[1]);

		prev_grid = [[0;4];4];
		while grid != prev_grid {
			prev_grid = grid;
			for x in 0..4 {
				for y in 0..4 {
					let current_value: i32 = grid[x][y];

					if current_value != 0 {
						let new_x: i32 = direction_vector[0] + (x as i32);
						let new_y: i32 = direction_vector[1] + (y as i32);
						println!("{}, {}", new_x, new_y);
						if new_x >= 0 && new_x <= 3 && new_y >= 0 && new_y <= 3 {
							grid[x][y] = 0;
							grid[new_x as usize][new_y as usize] = current_value;
						}
					}
				}
			}
		}
	}
}
