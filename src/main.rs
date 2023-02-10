use bevy::prelude::*;
use bevy::time::FixedTimestep;
// use rand::prelude::random;

const WINDOW_SIZE: f32 = 600.0;
const GRID_SIZE: f32 = 10.;
const TILE_SIZE: f32 = WINDOW_SIZE / GRID_SIZE;

// Colors
// const SNAKE_HEAD_COLOR: Color = Color::rgb_u8(200, 200, 200);
// const SNAKE_TAIL_COLOR: Color = Color::rgb_u8(150, 150, 150);
// const FOOD_COLOR: Color = Color::rgb_u8(255, 0, 0);

#[derive(Debug, Clone, Copy, Component)]
struct SnakeHead {
	x: i32,
	y: i32,
}

// #[derive(Component, Clone, Copy, PartialEq, Eq)]
// struct Position {
// 	x: i32,
// 	y: i32,
// }

#[derive(Component)]
struct Food;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Snake".to_string(),
				width: WINDOW_SIZE,
				height: WINDOW_SIZE,
				resizable: false,
				..default()
			},
			..default()
		}))
		.add_startup_system(setup_camera)
		.add_system(snake_movement)
		.add_system_set(
			SystemSet::new()
				.with_run_criteria(FixedTimestep::step(0.150))
				.with_system(snake_vel),
		)
		.add_system(eat)
		.insert_resource(ClearColor(Color::rgb_u8(100, 100, 100)))
		.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());

	commands
		.spawn((SpriteBundle {
			transform: Transform {
				translation: Vec3::new(
					-(WINDOW_SIZE / 2.) + ((5 as f32 - 0.5) as f32 * TILE_SIZE),
					-(WINDOW_SIZE / 2.) + ((5 as f32 - 0.5) as f32 * TILE_SIZE),
					0.0,
				),
				scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::rgb_u8(200, 200, 200),
				..default()
			},
			..default()
		},))
		.insert(SnakeHead { x: 0, y: 0 });
	// .insert(Position { x: 5, y: 5 });
	commands
		.spawn((SpriteBundle {
			transform: Transform {
				translation: Vec3::new(
					-(WINDOW_SIZE / 2.) + ((2 as f32 - 0.5) as f32 * TILE_SIZE),
					-(WINDOW_SIZE / 2.) + ((2 as f32 - 0.5) as f32 * TILE_SIZE),
					0.0,
				),
				scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::rgb_u8(255, 0, 0),
				..default()
			},
			..default()
		},))
		.insert(Food);
}

fn snake_vel(mut head: Query<(Entity, &mut SnakeHead, &mut Transform)>) {
	for (_, head, mut transform) in head.iter_mut() {
		transform.translation.x += head.x as f32 * TILE_SIZE;
		transform.translation.y += head.y as f32 * TILE_SIZE;
	}
}

fn eat(
	head: Query<(Entity, &mut SnakeHead, &mut Transform)>,
	food: Query<(Entity, &mut Food, &mut Transform)>,
) {
	let head_pos = head.iter().next().unwrap().2.translation;
	let food_pos = food.iter().next().unwrap().2.translation;
	if head_pos.x == food_pos.x && head_pos.y == food_pos.y {
		println!("HI");
	}
}

fn snake_movement(
	keyboard_input: Res<Input<KeyCode>>,
	mut head_positions: Query<(Entity, &mut SnakeHead)>,
) {
	for (_, mut pos) in head_positions.iter_mut() {
		if keyboard_input.pressed(KeyCode::Left) {
			pos.x = -1;
			pos.y = 0;
		}
		if keyboard_input.pressed(KeyCode::Right) {
			pos.x = 1;
			pos.y = 0;
		}
		if keyboard_input.pressed(KeyCode::Down) {
			pos.x = 0;
			pos.y = -1;
		}
		if keyboard_input.pressed(KeyCode::Up) {
			pos.x = 0;
			pos.y = 1;
		}
	}
}
