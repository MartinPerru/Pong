use macroquad::prelude::*;

const INITIAL_WIDTH: f32 = 800.0;
const INITIAL_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 150.0;
const PADDLE_SPEED: f32 = 7.0;
const BALL_SIZE: f32 = 15.0;
const BALL_SPEED: f32 = 9.0;

#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    OnePlayer,
    TwoPlayers,
}

#[derive(Clone, Copy, PartialEq)]
enum AppState {
    Menu,
    Playing,
}

struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Paddle { x, y }
    }

    fn move_up(&mut self) {
        self.y -= PADDLE_SPEED;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    fn move_down(&mut self) {
        self.y += PADDLE_SPEED;
        if self.y + PADDLE_HEIGHT > screen_height() {
            self.y = screen_height() - PADDLE_HEIGHT;
        }
    }

    fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT, color);
    }

    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT)
    }
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    fn new() -> Self {
        Ball {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            vx: BALL_SPEED,
            vy: BALL_SPEED,
        }
    }

    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        // Bounce off top and bottom
        if self.y <= 0.0 || self.y + BALL_SIZE >= screen_height() {
            self.vy = -self.vy;
        }
    }

    fn reset(&mut self, direction: f32) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.vx = BALL_SPEED * direction;
        self.vy = macroquad::rand::gen_range(-BALL_SPEED * 0.5, BALL_SPEED * 0.5);
    }

    fn draw(&self) {
        draw_circle(self.x + BALL_SIZE / 2.0, self.y + BALL_SIZE / 2.0, BALL_SIZE, YELLOW);
    }

    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, BALL_SIZE, BALL_SIZE)
    }
}

struct GameState {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    score1: i32,
    score2: i32,
    game_over: bool,
    winner: u8,
    mode: GameMode,
}

impl GameState {
    fn new(mode: GameMode) -> Self {
        GameState {
            player1: Paddle::new(30.0, screen_height() / 2.0 - PADDLE_HEIGHT / 2.0),
            player2: Paddle::new(screen_width() - 50.0, screen_height() / 2.0 - PADDLE_HEIGHT / 2.0),
            ball: Ball::new(),
            score1: 0,
            score2: 0,
            game_over: false,
            winner: 0,
            mode,
        }
    }

    fn on_resize(&mut self) {
        self.player1.x = 30.0;
        self.player2.x = screen_width() - 50.0;
        self.player1.y = self.player1.y.clamp(0.0, screen_height() - PADDLE_HEIGHT);
        self.player2.y = self.player2.y.clamp(0.0, screen_height() - PADDLE_HEIGHT);
    }

    fn check_collision(&mut self) {
        let ball_rect = self.ball.rect();
        let p1_rect = self.player1.rect();
        let p2_rect = self.player2.rect();

        if ball_rect.overlaps(&p1_rect) {
            let hit_pos = ((self.ball.y + BALL_SIZE / 2.0) - (self.player1.y + PADDLE_HEIGHT / 2.0))
                / (PADDLE_HEIGHT / 2.0);
            let hit_pos = hit_pos.clamp(-1.0, 1.0);
            self.ball.vy = hit_pos * BALL_SPEED * 1.5;
            self.ball.vx = (self.ball.vx.abs() * 1.05).min(BALL_SPEED * 2.0);
        }

        if ball_rect.overlaps(&p2_rect) {
            let hit_pos = ((self.ball.y + BALL_SIZE / 2.0) - (self.player2.y + PADDLE_HEIGHT / 2.0))
                / (PADDLE_HEIGHT / 2.0);
            let hit_pos = hit_pos.clamp(-1.0, 1.0);
            self.ball.vy = hit_pos * BALL_SPEED * 1.5;
            self.ball.vx = -(self.ball.vx.abs() * 1.05).min(BALL_SPEED * 2.0);
        }
    }

    fn check_scoring(&mut self) {
        if self.ball.x <= 0.0 {
            self.score2 += 1;
            self.ball.reset(1.0);
            if self.score2 >= 7 {
                self.game_over = true;
                self.winner = 2;
            }
        }

        if self.ball.x >= screen_width() {
            self.score1 += 1;
            self.ball.reset(-1.0);
            if self.score1 >= 7 {
                self.game_over = true;
                self.winner = 1;
            }
        }
    }

    fn update(&mut self) {
        if !self.game_over {
            // Player 1 controls (W/S)
            if is_key_down(KeyCode::W) {
                self.player1.move_up();
            }
            if is_key_down(KeyCode::S) {
                self.player1.move_down();
            }

            // Player 2: human or AI
            match self.mode {
                GameMode::TwoPlayers => {
                    if is_key_down(KeyCode::Up) {
                        self.player2.move_up();
                    }
                    if is_key_down(KeyCode::Down) {
                        self.player2.move_down();
                    }
                }
                GameMode::OnePlayer => {
                    let paddle_center = self.player2.y + PADDLE_HEIGHT / 2.0;
                    let ball_center = self.ball.y + BALL_SIZE / 2.0;
                    if self.ball.vx > 0.0 {
                        if ball_center < paddle_center - 5.0 {
                            self.player2.move_up();
                        } else if ball_center > paddle_center + 5.0 {
                            self.player2.move_down();
                        }
                    }
                }
            }

            self.ball.update();
            self.check_collision();
            self.check_scoring();
        }
    }

    fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(BLACK);

        draw_rectangle(sw / 2.0 - 2.0, 0.0, 4.0, sh, GRAY);

        self.player1.draw(GREEN);
        self.player2.draw(RED);

        self.ball.draw();

        // Controls hint
        let hint = match self.mode {
            GameMode::OnePlayer => "P1: W/S",
            GameMode::TwoPlayers => "P1: W/S    P2: UP/DOWN",
        };
        draw_text(hint, 10.0, sh - 10.0, 20.0, DARKGRAY);

        let score_text = format!("{}    {}", self.score1, self.score2);
        let font_size = 60.0;
        let text_size = measure_text(&score_text, None, font_size as u16, 1.0);
        draw_text(
            &score_text,
            sw / 2.0 - text_size.width / 2.0,
            50.0,
            font_size,
            WHITE,
        );

        if self.game_over {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.7));

            let winner_text = format!("Player {} Wins!", self.winner);
            let winner_font_size = 80.0;
            let winner_text_size = measure_text(&winner_text, None, winner_font_size as u16, 1.0);
            draw_text(
                &winner_text,
                sw / 2.0 - winner_text_size.width / 2.0,
                sh / 2.0 - 20.0,
                winner_font_size,
                YELLOW,
            );

            let restart_text = "Press R to restart  |  M for menu";
            let restart_font_size = 30.0;
            let restart_text_size = measure_text(restart_text, None, restart_font_size as u16, 1.0);
            draw_text(
                restart_text,
                sw / 2.0 - restart_text_size.width / 2.0,
                sh / 2.0 + 50.0,
                restart_font_size,
                WHITE,
            );
        }
    }
}

fn draw_menu(selected: usize) {
    let sw = screen_width();
    let sh = screen_height();

    clear_background(BLACK);

    let title = "PONG";
    let title_size = 100.0;
    let ts = measure_text(title, None, title_size as u16, 1.0);
    draw_text(title, sw / 2.0 - ts.width / 2.0, sh / 3.0, title_size, WHITE);

    let options = ["1 Player  (vs AI)", "2 Players"];
    let option_size = 40.0;

    for (i, label) in options.iter().enumerate() {
        let color = if i == selected { YELLOW } else { GRAY };
        let prefix = if i == selected { "> " } else { "  " };
        let text = format!("{}{}", prefix, label);
        let ts = measure_text(&text, None, option_size as u16, 1.0);
        draw_text(
            &text,
            sw / 2.0 - ts.width / 2.0,
            sh / 2.0 + i as f32 * 60.0,
            option_size,
            color,
        );
    }

    let hint = "UP/DOWN to select  |  ENTER to confirm";
    let hint_size = 22.0;
    let hs = measure_text(hint, None, hint_size as u16, 1.0);
    draw_text(hint, sw / 2.0 - hs.width / 2.0, sh - 40.0, hint_size, DARKGRAY);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Ping Pong".to_owned(),
        window_width: INITIAL_WIDTH as i32,
        window_height: INITIAL_HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = AppState::Menu;
    let mut menu_selection: usize = 0;
    let mut game = GameState::new(GameMode::OnePlayer);
    let mut last_size = (screen_width(), screen_height());

    loop {
        let current_size = (screen_width(), screen_height());
        if current_size != last_size {
            if app_state == AppState::Playing {
                game.on_resize();
            }
            last_size = current_size;
        }

        match app_state {
            AppState::Menu => {
                if is_key_pressed(KeyCode::Up) && menu_selection > 0 {
                    menu_selection -= 1;
                }
                if is_key_pressed(KeyCode::Down) && menu_selection < 1 {
                    menu_selection += 1;
                }
                if is_key_pressed(KeyCode::Enter) {
                    let mode = if menu_selection == 0 {
                        GameMode::OnePlayer
                    } else {
                        GameMode::TwoPlayers
                    };
                    game = GameState::new(mode);
                    app_state = AppState::Playing;
                }
                draw_menu(menu_selection);
            }
            AppState::Playing => {
                if is_key_pressed(KeyCode::M) {
                    app_state = AppState::Menu;
                } else if is_key_pressed(KeyCode::R) {
                    game = GameState::new(game.mode);
                } else {
                    game.update();
                }
                game.draw();
            }
        }

        next_frame().await
    }
}
