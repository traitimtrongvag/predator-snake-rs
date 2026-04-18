use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<Pos>,
    dir: Dir,
    pending_growth: usize,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            body: vec![Pos { x, y }],
            dir: Dir::Right,
            pending_growth: 0,
        }
    }

    pub fn head(&self) -> Pos {
        self.body[0]
    }

    pub fn body(&self) -> &Vec<Pos> {
        &self.body
    }

    pub fn set_direction(&mut self, dir: Dir) {
        self.dir = dir;
    }

    pub fn advance(&mut self) {
        let head = self.head();
        let new_head = match self.dir {
            Dir::Up => Pos { x: head.x, y: head.y.saturating_sub(1) },
            Dir::Down => Pos { x: head.x, y: head.y + 1 },
            Dir::Left => Pos { x: head.x.saturating_sub(1), y: head.y },
            Dir::Right => Pos { x: head.x + 1, y: head.y },
        };

        self.body.insert(0, new_head);

        if self.pending_growth > 0 {
            self.pending_growth -= 1;
        } else {
            self.body.pop();
        }
    }

    pub fn grow(&mut self) {
        self.pending_growth += 1;
    }

    pub fn is_dead(&self, width: u16, height: u16) -> bool {
        let head = self.head();
        // out-of-bounds covers wall hits (Down/Right overflow, Up/Left clamp-then-trap)
        head.x >= width
            || head.y >= height
            || self.body.iter().skip(1).any(|&pos| pos == head)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, blink: bool) {
        let color = if blink { Color::White } else { Color::Green };
        
        for pos in &self.body {
            if pos.x < area.width && pos.y < area.height {
                let x = area.x + pos.x;
                let y = area.y + pos.y;
                
                let span = Span::styled("●", Style::default().fg(color));
                frame.render_widget(span, Rect { x, y, width: 1, height: 1 });
            }
        }
    }
}
