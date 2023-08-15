use tetra::graphics::text::{Text, Font};
use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

/**
 * TODO
 * 
 * Break out into multi file
 * Limit paddles from ceiling and floor
 * Add Score after ball leaves play area
 * Display score
 * Reset 
 * Make Ball direction random to start
 * Add Y reflection for ball
 * Skin with modern UI
 * Make AI
 * 
 */

const PADDING: f32 = 32.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPEED: f32 = 8.0;
const WINDOW_WIDTH: f32 = 1600.0;
const WINDOW_HEIGHT: f32 = 900.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    background: Texture,
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height()
        )
    }

    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let ball_texture = Texture::new(ctx, "./src/resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0
        );

        let player1_texture = Texture::new(ctx, "./src/resources/player1.png")?;
        let player1_position = 
            Vec2::new(PADDING, (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0);

        let player2_texture = Texture::new(ctx, "./src/resources/player2.png")?;
        let player2_position = 
            Vec2::new((WINDOW_WIDTH - player2_texture.width() as f32 - PADDING) as f32, (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0);
        

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

            self.player1.position.y -= PADDLE_SPEED;
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
