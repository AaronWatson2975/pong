use std::cmp::{ min, max };
use tetra::Context;
use tetra::input::{ self, Key };
use tetra::graphics::{ self, Texture, Color };
use tetra::State;
use tetra::math::Vec2;
use crate::entity::Entity;
use crate::constants::{ PADDLE_SPEED, WINDOW_WIDTH, WINDOW_HEIGHT, PADDING, BALL_SPEED };

pub struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    background: Texture,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let ball_texture = Texture::new(ctx, "./src/resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - (ball_texture.width() as f32) / 2.0,
            WINDOW_HEIGHT / 2.0 - (ball_texture.height() as f32) / 2.0
        );

        let player1_texture = Texture::new(ctx, "./src/resources/player1.png")?;
        let player1_position = Vec2::new(
            PADDING,
            (WINDOW_HEIGHT - (player1_texture.height() as f32)) / 2.0
        );

        let player2_texture = Texture::new(ctx, "./src/resources/player2.png")?;
        let player2_position = Vec2::new(
            (WINDOW_WIDTH - (player2_texture.width() as f32) - PADDING) as f32,
            (WINDOW_HEIGHT - (player2_texture.height() as f32)) / 2.0
        );

        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        let background = Texture::new(ctx, "./src/resources/bg.png")?;

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            background,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y = max(
                (self.player1.position.y as i32) - (PADDLE_SPEED as i32),
                PADDING as i32
            ) as f32;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        self.ball.position += self.ball.velocity;

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if paddle_hit.is_some() {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // let font = Font::new(ctx, "./src/resources/tron.ttf")?;
        // let text = Text::new("Hello, world!", font);
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.background.draw(ctx, Vec2::zero());
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);

        Ok(())
    }
}
