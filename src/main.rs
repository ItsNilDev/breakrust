use macroquad::prelude::{*, scene::clear};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

struct Player {
	rec_width: f32,
	rec_height: f32,
	x: f32,
}

struct Ball {
	x: f32,
	y: f32,
	dx: f32,
	dy: f32,
	launched: bool,
}

fn conf() -> Conf {
	Conf {
		window_title: String::from("Macroquad"),
		window_width: 800,
		window_height: 800,
		fullscreen: false,
		..Default::default()
	}
}


#[macroquad::main("BasicShapes")]
async fn main() {
	let mut player = Player {
		rec_width: 130.,
		rec_height: 20.,
		x: screen_width() / 2.0 - 60.,
	};
	let mut game_over = false;
	let mut ball = Ball {
		x: player.x + 65.0,
		y: screen_height() - 65.0,
		dx: 300.,
		dy: -300.,
		launched: false,
	};
	TermLogger::new(
		LevelFilter::Info,
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto);

	// fn launch_ball(ball: &mut Ball, movement_speed: f32) {
	// 	let delta = get_frame_time();
	// 	if (*ball).width > screen_width() - 20. {
	// 		(*ball).width
	// 	}
	// 	(*ball).width  += delta * movement_speed;
	// 	(*ball).height += delta * movement_speed;
	// 	debug!("reached {}", (*ball).width);
	// }


	loop {
		if !game_over {
			if is_key_down(KeyCode::I) {
				// 30 is for the max width that player can go
				if player.x <= screen_width() - player.rec_width - 30. {
					player.x += 10.;
				}
				if !ball.launched {
					ball.x = player.x + 65.0;
				}
			}

			if is_key_down(KeyCode::N) {
				// 30 is min here
				if player.x > 30. {
					player.x -= 10.;
				}
				if !ball.launched {
					ball.x = player.x + 65.0;
				}

			}

			if is_key_pressed(KeyCode::E) {
				ball.launched = true;
			}

			if ball.launched {
				let delta = get_frame_time();

				// right collision
				if ball.x >= screen_width() {
					ball.dx = -ball.dx;
					ball.x  = screen_width();
				}

				// left collision
				if ball.x <= 0. {
					ball.dx = -ball.dx;
					ball.x  = 0.;
				}

				// up collision
				if ball.y < 0.{
					ball.dy = -ball.dy;
					ball.y = 0.;
				}

				// down collision
				if ball.y > screen_height() {
					// can we set these things to default by any chance?
					game_over = true;
					player.x = screen_width() / 2.0 - 60.;
					ball.x = player.x + 65.0;
					ball.y = screen_height() - 65.0;
					ball.dx = 300.;
					ball.dy = -300.;
					ball.launched = false;
				}

				// player-ball collision
				// height detection:
				if ball.y >= screen_height() - player.rec_height - 40.
				// width detection:
					&& ball.x <= player.x + player.rec_width
					&& ball.x >= player.x {
						// FIXME: some bugs
						ball.dy = -ball.dy;
					}
				ball.x  += delta * ball.dx;
				ball.y += delta * ball.dy;
			}
		}

		if !game_over {
			clear_background(BLACK);
			draw_rectangle(player.x, screen_height() - 50., player.rec_width, player.rec_height, RED);

			draw_circle(ball.x, ball.y, 15.0, WHITE);
		} else {
			clear_background(RED);
			if is_key_pressed(KeyCode::Enter) {
				// let mut ball = Ball {
				// ..ball
				// };
				game_over = false;
			}
		}
		next_frame().await
	}
}
