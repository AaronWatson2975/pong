use tetra::ContextBuilder;
mod game_state;
mod entity;
mod constants;

/**
 * TODO
 *
 * Display score
 * Make AI
 *
 */
fn main() -> tetra::Result {
    ContextBuilder::new("Pong", constants::WINDOW_WIDTH as i32, constants::WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(game_state::GameState::new)
}
