use tetra::ContextBuilder;
mod game_state;
mod entity;
mod constants;

/**
 * TODO
 *
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
fn main() -> tetra::Result {
    ContextBuilder::new("Pong", constants::WINDOW_WIDTH as i32, constants::WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(game_state::GameState::new)
}
