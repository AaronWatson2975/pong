use std::cmp::{ min, max };
use tetra::Context;
use tetra::input::{ self, Key };
use tetra::graphics::{ self, Texture, Color };
use tetra::graphics::text::{ Font, Text };
use tetra::State;
use tetra::math::Vec2;
use crate::entity::Entity;
use crate::constants::{
    BALL_ACC,
    BALL_SPEED,
    PADDING,
    PADDLE_SPEED,
    PADDLE_SPIN,
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    FONT_SIZE_LG,
    FONT_SIZE_MD,
    FONT_SIZE_SM,
};

pub struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    background: Texture,
    font_sm: Font,
    font_md: Font,
    font_lg: Font,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let font_lg = Font::vector(ctx, "./src/resources/tron.ttf", FONT_SIZE_LG)?;
        let font_md = Font::vector(ctx, "./src/resources/tron.ttf", FONT_SIZE_MD)?;
        let font_sm = Font::vector(ctx, "./src/resources/tron.ttf", FONT_SIZE_SM)?;

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
            font_lg,
            font_md,
            font_sm,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y = max(
                (self.player1.position.y - PADDLE_SPEED) as i32,
                PADDING as i32
            ) as f32;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y = min(
                (self.player1.position.y + PADDLE_SPEED) as i32,
                ((WINDOW_HEIGHT - PADDING) as i32) - self.player1.texture.height()
            ) as f32;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y = max(
                (self.player2.position.y - PADDLE_SPEED) as i32,
                PADDING as i32
            ) as f32;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y = min(
                (self.player2.position.y + PADDLE_SPEED) as i32,
                ((WINDOW_HEIGHT - PADDING) as i32) - self.player2.texture.height()
            ) as f32;
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

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x = -(
                self.ball.velocity.x +
                BALL_ACC * self.ball.velocity.x.signum()
            );

            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if
            self.ball.bounds().top() < PADDING ||
            self.ball.bounds().bottom() > WINDOW_HEIGHT - PADDING
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.bounds().left() < -WINDOW_WIDTH / 2.0 {
            self.player2.score += 1;
            self.ball.position = Vec2::new(
                WINDOW_WIDTH / 2.0 - (self.ball.texture.width() as f32) / 2.0,
                WINDOW_HEIGHT / 2.0 - (self.ball.texture.height() as f32) / 2.0
            );
            self.ball.velocity = Vec2::new(BALL_SPEED, 0.0);
        } else if self.ball.bounds().right() > WINDOW_WIDTH * 1.5 {
            self.player1.score += 1;
            self.ball.position = Vec2::new(
                WINDOW_WIDTH / 2.0 - (self.ball.texture.width() as f32) / 2.0,
                WINDOW_HEIGHT / 2.0 - (self.ball.texture.height() as f32) / 2.0
            );
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.background.draw(ctx, Vec2::zero());

        let mut title_text = Text::new("Pong", self.font_lg.clone());
        let title_text_width = title_text.get_bounds(ctx).unwrap().width;

        title_text.draw(
            ctx,
            Vec2::new(WINDOW_WIDTH / 2.0 - (title_text_width as f32) / 2.0, PADDING)
        );

        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);

        let mut player_one_text = Text::new("Player 1", self.font_md.clone());
        player_one_text.draw(ctx, Vec2::new(PADDING, PADDING));

        let mut player_one_score = Text::new(
            "Score: ".to_owned() + &self.player1.score.to_string(),
            self.font_sm.clone()
        );
        player_one_score.draw(ctx, Vec2::new(PADDING, PADDING + FONT_SIZE_MD * 2.0));

        let mut player_two_text = Text::new("Player 2", self.font_md.clone());
        let player_two_text_width = player_two_text.get_bounds(ctx).unwrap().width;

        player_two_text.draw(
            ctx,
            Vec2::new(WINDOW_WIDTH - PADDING - player_two_text_width, PADDING)
        );

        let mut player_two_score = Text::new(
            "Score: ".to_owned() + &self.player2.score.to_string(),
            self.font_sm.clone()
        );
        let player_two_score_width = player_two_score.get_bounds(ctx).unwrap().width;

        player_two_score.draw(
            ctx,
            Vec2::new(WINDOW_WIDTH - PADDING - player_two_score_width, PADDING + FONT_SIZE_MD * 2.0)
        );

        title_text.draw(
            ctx,
            Vec2::new(WINDOW_WIDTH / 2.0 - (title_text_width as f32) / 2.0, PADDING)
        );

        Ok(())
    }
}
