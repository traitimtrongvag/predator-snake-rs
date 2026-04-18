use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::snake::Snake;
use crate::food::Food;
use crate::pathfinding;

pub struct Game {
    snake: Snake,
    food: Food,
    width: u16,
    height: u16,
    max_score: usize,
    current_score: usize,
    death_blink_counter: Option<u8>,
}

impl Game {
    pub fn new(max_score: usize) -> Self {
        Self {
            snake: Snake::new(5, 5),
            food: Food::new(10, 10),
            width: 0,
            height: 0,
            max_score,
            current_score: 0,
            death_blink_counter: None,
        }
    }

    pub fn tick(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        if self.food.pos().x >= self.width || self.food.pos().y >= self.height {
            self.food.respawn(self.width, self.height, &self.snake.body());
        }

        if let Some(counter) = self.death_blink_counter {
            if counter == 0 {
                self.reset();
            } else {
                self.death_blink_counter = Some(counter - 1);
            }
            return;
        }

        let target = self.food.pos();
        let head = self.snake.head();
        
        let next_dir = pathfinding::find_direction(
            head,
            target,
            &self.snake.body(),
            self.width,
            self.height,
        );

        // None means no safe move exists; advance anyway so is_dead() fires below
        if let Some(dir) = next_dir {
            self.snake.set_direction(dir);
        }
        self.snake.advance();

        if self.snake.is_dead(self.width, self.height) {
            self.death_blink_counter = Some(4);
            return;
        }

        if self.snake.head() == self.food.pos() {
            self.snake.grow();
            self.current_score += 1;
            
            if self.current_score >= self.max_score {
                self.reset();
            } else {
                self.food.respawn(self.width, self.height, &self.snake.body());
            }
        }

        if self.snake.body().len() as u16 >= (self.width * self.height) / 2 {
            self.reset();
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.size();
        
        self.width = area.width.saturating_sub(2);
        self.height = area.height.saturating_sub(2);

        // Score goes in the border title to avoid overwriting the bottom border line
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" SCORE: {} ", self.current_score))
            .style(Style::default().fg(Color::White));
        
        frame.render_widget(block, area);

        let inner = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: self.width,
            height: self.height,
        };

        self.snake.render(frame, inner, self.is_blinking());
        self.food.render(frame, inner);
    }

    fn reset(&mut self) {
        self.snake = Snake::new(5, 5);
        self.food = Food::new(10, 10);
        self.current_score = 0;
        self.death_blink_counter = None;
    }

    fn is_blinking(&self) -> bool {
        if let Some(counter) = self.death_blink_counter {
            counter % 2 == 0
        } else {
            false
        }
    }
}
