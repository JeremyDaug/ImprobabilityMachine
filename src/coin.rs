use std::time::Instant;

use macroquad::texture::{load_texture, Texture2D};


pub(crate) struct Coin {
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