use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    Frame,
};
use rand::Rng;

use crate::snake::Pos;

pub struct Food {
    pos: Pos,
}

impl Food {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            pos: Pos { x, y },
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn respawn(&mut self, width: u16, height: u16, snake_body: &[Pos]) {
        let mut rng = rand::thread_rng();
        
        loop {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            let new_pos = Pos { x, y };
            
            if !snake_body.contains(&new_pos) {
                self.pos = new_pos;
                break;
            }
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.pos.x < area.width && self.pos.y < area.height {
            let x = area.x + self.pos.x;
            let y = area.y + self.pos.y;
            
            let span = Span::styled("●", Style::default().fg(Color::Red));
            frame.render_widget(span, Rect { x, y, width: 1, height: 1 });
        }
    }
}
