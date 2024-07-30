use rand::{self, prelude::*};
use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

const PAD_HEIGHT: i32 = 2;

struct PlayerState {
    pub position: Vec2,
    pub v_direction: i32,
    pub h_direction: i32,
    pub score: usize,
}

struct GameState {
    pub dimension: Vec2,
    pub left_player: PlayerState,
    pub right_player: PlayerState,
    pub ball_position: Vec2,
    pub ball_speed: Vec2,
}

impl GameState {
    pub fn new(dim: Vec2) -> Self {
        Self {
            dimension: dim,
            left_player: PlayerState {
                position: Vec2::xy(1, dim.y / 2),
                v_direction: 0,
                h_direction: 0,
                score: 0,
            },
            right_player: PlayerState {
                position: Vec2::xy(dim.x - 3, dim.y / 2),
                v_direction: 0,
                h_direction: 0,
                score: 0,
            },
            ball_position: dim / 2,
            ball_speed: Self::random_ball_direction(),
        }
    }

    pub fn random_ball_direction() -> Vec2 {
        let mut rng = rand::thread_rng();
        let neg_x: bool = rng.gen();
        let neg_y: bool = rng.gen();
        Vec2::xy(if neg_x { -1 } else { 1 }, if neg_y { -1 } else { 1 })
    }

    pub fn update(&mut self) {
        self.ball_position += self.ball_speed;

        if self.left_player.position.y + PAD_HEIGHT < self.dimension.y
            && self.left_player.v_direction > 0
            || self.left_player.position.y - PAD_HEIGHT > 0 && self.left_player.v_direction < 0
        {
            self.left_player.position.y += self.left_player.v_direction;
        }

        if self.left_player.position.x < (self.dimension.x / 2) - 2
            && self.left_player.h_direction > 0
            || self.left_player.position.x > 1 && self.left_player.h_direction < 0
        {
            self.left_player.position.x += self.left_player.h_direction;
        }

        if self.right_player.position.y + PAD_HEIGHT < self.dimension.y
            && self.right_player.v_direction > 0
            || self.right_player.position.y - PAD_HEIGHT > 0 && self.right_player.v_direction < 0
        {
            self.right_player.position.y += self.right_player.v_direction;
        }

        if self.right_player.position.x > (self.dimension.x / 2) + 1
            && self.right_player.h_direction < 0
            || self.right_player.position.x < self.dimension.x - 3
                && self.right_player.h_direction > 0
        {
            self.right_player.position.x += self.right_player.h_direction;
        }

        if self.ball_position.y >= self.dimension.y - 1 && self.ball_speed.y > 0 {
            self.ball_position.y = self.dimension.y - 1;
            self.ball_speed.y = -self.ball_speed.y;
        }

        if self.ball_position.y <= 0 && self.ball_speed.y < 0 {
            self.ball_position.y = 0;
            self.ball_speed.y = -self.ball_speed.y;
        }

        if self.ball_position.x <= self.left_player.position.x + 1
            && self.ball_position.y <= self.left_player.position.y + PAD_HEIGHT
            && self.ball_position.y >= self.left_player.position.y - PAD_HEIGHT
        {
            self.ball_position.x = self.left_player.position.x + 1;
            self.ball_speed.x = -self.ball_speed.x;
        }

        if self.ball_position.x >= self.right_player.position.x
            && self.ball_position.y <= self.right_player.position.y + PAD_HEIGHT
            && self.ball_position.y >= self.right_player.position.y - PAD_HEIGHT
        {
            self.ball_position.x = self.right_player.position.x;
            self.ball_speed.x = -self.ball_speed.x;
        }

        if self.ball_position.x <= 0 {
            self.right_player.score += 1;
            self.ball_position = self.dimension / 2;
            self.ball_speed = Self::random_ball_direction();
        }

        if self.ball_position.x >= self.dimension.x - 1 {
            self.left_player.score += 1;
            self.ball_position = self.dimension / 2;
            self.ball_speed = Self::random_ball_direction();
        }

        self.left_player.v_direction = 0;
        self.left_player.h_direction = 0;
        self.right_player.v_direction = 0;
        self.right_player.h_direction = 0;
    }

    fn render(&self, window: &mut Window, score_msg: &str, win_size: Vec2) {
        Pencil::new(window.canvas_mut())
            .set_origin(Vec2::xy(
                (win_size.x - score_msg.len() as i32) / 2,
                (win_size.y - self.dimension.y) / 2 - 1,
            ))
            .draw_text(score_msg, Vec2::xy(0, 0))
            .set_origin((win_size - self.dimension) / 2)
            .draw_rect(
                &RectCharset::simple_round_lines(),
                Vec2::zero(),
                self.dimension,
            )
            .draw_vline('#', Vec2::xy(self.dimension.x / 2, 1), self.dimension.y - 2)
            .set_foreground(Color::Blue)
            .draw_rect(
                &RectCharset::simple_round_lines(),
                self.left_player.position - Vec2::y(PAD_HEIGHT),
                Vec2::xy(2, PAD_HEIGHT * 2),
            )
            .set_foreground(Color::Green)
            .draw_rect(
                &RectCharset::simple_round_lines(),
                self.right_player.position - Vec2::y(PAD_HEIGHT),
                Vec2::xy(2, PAD_HEIGHT * 2),
            )
            .set_foreground(Color::Yellow)
            .set_style(Style::Bold)
            .draw_char('⬤', self.ball_position);
    }
}

fn main() {
    let mut app = App::default();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 4) / 5);

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::W => state.left_player.v_direction = -1,
                Key::S => state.left_player.v_direction = 1,
                Key::A => state.left_player.h_direction = -1,
                Key::D => state.left_player.h_direction = 1,
                Key::I => state.right_player.v_direction = -1,
                Key::K => state.right_player.v_direction = 1,
                Key::J => state.right_player.h_direction = -1,
                Key::L => state.right_player.h_direction = 1,
                _ => (),
            }
        }

        if app_state.step() % 2 == 0 {
            state.update();
        }

        let score_msg = &format!(
            "Left score: {}  -  Right score: {}",
            state.left_player.score, state.right_player.score
        );

        state.render(window, score_msg, win_size);
    });
}
