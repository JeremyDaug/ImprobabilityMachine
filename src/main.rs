pub(crate) mod money;
pub(crate) mod coin_toss;
pub mod game;
pub mod common_state;
mod coin;
pub mod coin_toss_cmd;

use std::{env, fs::File, io::{stdout, Write}, ops::Index, thread::sleep, time::{Duration, Instant}};

use crossterm::{cursor, execute, style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType}, ExecutableCommand, QueueableCommand};
use macroquad::{prelude::*, ui::widgets::Button};

use crate::{coin::Coin, coin_toss::CoinToss, coin_toss_cmd::{holding_screen, select_screen}, common_state::{ButtonAction, CommonState}};

#[macroquad::main("Improbability Machine")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let start_time = Instant::now();
    let mut common_state = CommonState { 
        money: 20.0*12.0, 
        entropy: 100.0, 
        active_game: 0, 
        current_bet: 10.0, 
        button_clicked: ButtonAction::None
    };
    let mut coin_toss = CoinToss::new();

    if mode == "cmd" {
        println!("\n\n\n\n\n\n\n\n");
        println!("-------------- Command Line Interface Selected. Starting up -----------");
        println!("\n\n\n\n\n");

        loop {
            if let None = select_screen(&mut common_state, &mut coin_toss, start_time) {
                println!("\n\nThank you for playing!");
                break;
            }
        }
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

struct Point {
    pub x: f32,
    pub y: f32
} 