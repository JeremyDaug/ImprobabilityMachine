pub(crate) mod money;
pub(crate) mod coin_toss;
pub mod game;
pub mod common_state;

use std::{env, fs::File, ops::Index, time::Instant};

use macroquad::{prelude::*, ui::widgets::Button};

#[macroquad::main("Improbability Machine")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let start_time = Instant::now();

    if mode == "cmd" {
        println!("\n\n\n\n\n\n\n\n");
        println!("-------------- Command Line Interface Selected. Starting up -----------");
        println!("\n\n\n\n\n");
    } else if mode == "ui" {
        let mut change = 0.0;

        // load coin textures
        let coin = Coin::load_coin().await;
        build_textures_atlas();

        let i = 0;
        loop {
            clear_background(DARKGRAY);
            //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
            //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

            let hi_point = Point {x: 20.0, y: 40.0};
            let intro_dim: TextDimensions = draw_text("Hello, Macroquad!", 
                hi_point.x, hi_point.y, 30.0, LIGHTGRAY);

            draw_rectangle_lines(hi_point.x, hi_point.y-20.0, intro_dim.width, 
                intro_dim.height+change, 4.0, BLACK);

            // if change > 20.0 {
            //     change = 0.0;
            // } else {
            //     change += 1.0;
            // }
            
            let side = (Instant::now() - start_time).as_secs() % 2 == 1;

            draw_texture(coin.flip_coin(start_time, Some(side)), screen_height() * 0.3, 
                screen_width() * 0.3, WHITE);

            let mouse = mouse_position();
            draw_circle(mouse.0, mouse.1, 5.0, YELLOW);

            if is_mouse_button_down(MouseButton::Left) {
                draw_circle(mouse.0, mouse.1, 2.0, GREEN);
            }
            if is_mouse_button_down(MouseButton::Right) {
                draw_circle(mouse.0, mouse.1, 2.0, BLUE);
            }

            next_frame().await
        }
    } else if is_help_cmd(mode) {
        println!("The Improbability machine has a 2 modes it can run in.\n");
        println!("cmd: Command Line mode. Used for more direct debugging. Very basic.");
        println!("ui: The Game UI that will be used. Currently only barely functional, don't expect much.");
    } else {
        println!("Mode command not given. Try -- help for modes")
    }
}

fn is_help_cmd(arg: &String) -> bool {
    arg == "help" ||
    arg == "Help" ||
    arg == "h" ||
    arg == "H"
}

struct Coin {
    pub heads: Texture2D,
    pub flipped_heads: Texture2D,
    pub flipped_tails: Texture2D,
    pub tails: Texture2D,
}

impl Coin {
    /// # Flip Coin
    /// 
    /// Selects the texture to show for the current flip.
    /// 
    /// Dictates which texture to show based on the current time loop.
    /// 
    /// Includes a start time, which keys off the start of the program and
    /// a select Optional<bool> which allows us to select heads or tails.
    pub fn flip_coin(&self, start_time: Instant, select: Option<bool>) -> &Texture2D {
        // check for the select.
        if let Some(side) = select {
            if side {
                return &self.heads;
            } else {
                return &self.tails;
            }
        }
        // go in quarter second steps.
        let step = ((Instant::now() - start_time).as_secs_f32() * 4.0).floor() as i32 % 6;
        // circle around the number, 7 steps h->fh->ft->t->ft->fh->back to start
        if step == 0 {
            &self.heads
        } else if step == 1 || step == 5 {
            &self.flipped_heads
        } else if step == 2 || step == 4 {
            &self.flipped_tails
        } else { // if step == 3
            &self.tails
        }
    }

    pub async fn load_coin() -> Self {
        Self {
            heads: load_texture("src/resources/coin_flip1.png").await.unwrap(),
            flipped_heads: load_texture("src/resources/coin_flip2.png").await.unwrap(),
            flipped_tails: load_texture("src/resources/coin_flip3.png").await.unwrap(),
            tails: load_texture("src/resources/coin_flip4.png").await.unwrap(),
        }
    }
}

struct Point {
    pub x: f32,
    pub y: f32
} 