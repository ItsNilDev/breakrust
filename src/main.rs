use macroquad::prelude::{*, scene::clear};
use simplelog::*;

struct Player {
	rec_width: f32,
	rec_height: f32,
	width: f32,
}

struct Ball {
	width: f32,
	height: f32,
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
		rec_width: 130.0,
		rec_height: 20.0,
		width: screen_width() / 2.0 - 60.0,
	};
	let mut game_over = false;
	let mut ball = Ball {
		width: player.width + 65.0,
		height: screen_height() - 65.0,
		dx: 300.0,
		dy: -300.0,
		launched: false,
	};
	TermLogger::new(
		LevelFilter::Info,
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto);

	// fn launch_ball(ball: &mut Ball, movement_speed: f32) {
	// 	let delta = get_frame_time();
	// 	if (*ball).width > screen_width() - 20.0 {
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
				if player.width <= screen_width() - player.rec_width - 30.0 {
					player.width += 10.0;
				}
				if !ball.launched {
					ball.width = player.width + 65.0;
				}
			}

			if is_key_down(KeyCode::N) {
				// 30 is min here
				if player.width > 30.0 {
					player.width -= 10.0;
				}
				if !ball.launched {
					ball.width = player.width + 65.0;
				}

			}

			if is_key_pressed(KeyCode::E) {
				ball.launched = true;
			}

			if ball.launched {
				// launch_ball(&mut ball, 100.0);
				let delta = get_frame_time();
				if ball.width > screen_width() || ball.width < 0.0 {
					ball.dx = -ball.dx;
				}
				if ball.height < 0.0{
					ball.dy = -ball.dy;
				}
				if ball.height > screen_height() {
					game_over = true;
					player.width = screen_width() / 2.0 - 60.0;
					ball.width = player.width + 65.0;
					ball.height = screen_height() - 65.0;
					ball.dx = 300.0;
					ball.dy = -300.0;
					ball.launched = false;
				}
				ball.width  += delta * ball.dx;
				ball.height += delta * ball.dy;
			}
		}

		if !game_over {
			clear_background(DARKGRAY);
			draw_rectangle(player.width, screen_height() - 50.0, player.rec_width, player.rec_height, GREEN);

			draw_circle(ball.width, ball.height, 15.0, GREEN);
		} else {
			clear_background(RED);
			if is_key_pressed(KeyCode::Enter) {
				// let mut ball = Ball {
				// ..ball
				// };
				game_over = false;
			}
		}
		debug!("r");
		next_frame().await
	}
}
