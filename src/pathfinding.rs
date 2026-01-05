use crate::snake::{Dir, Pos};
use std::collections::{HashMap, VecDeque};

pub fn find_direction(
    head: Pos,
    target: Pos,
    body: &[Pos],
    width: u16,
    height: u16,
) -> Dir {
    if let Some(path) = bfs(head, target, body, width, height) {
        if path.len() > 1 {
            return direction_to(head, path[1]);
        }
    }
   
    // Fallback when no path is found
    let dx = target.x as i32 - head.x as i32;
    let dy = target.y as i32 - head.y as i32;
    
    if dx.abs() > dy.abs() {
        if dx > 0 { Dir::Right } else { Dir::Left }
    } else {
        if dy > 0 { Dir::Down } else { Dir::Up }
    }
}

fn bfs(
    start: Pos,
    target: Pos,
    body: &[Pos],
    width: u16,
    height: u16,
) -> Option<Vec<Pos>> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    
    queue.push_back(start);
    visited.insert(start, None);
    
    while let Some(current) = queue.pop_front() {
        if current == target {
            return Some(reconstruct_path(start, target, &visited));
        }
        
        for neighbor in neighbors(current, width, height) {
            if !visited.contains_key(&neighbor) && !body.contains(&neighbor) {
                visited.insert(neighbor, Some(current));
                queue.push_back(neighbor);
            }
        }
    }
    
    None
}

fn reconstruct_path(
    start: Pos,
    end: Pos,
    visited: &HashMap<Pos, Option<Pos>>,
) -> Vec<Pos> {
    let mut path = Vec::new();
    let mut current = end;
    
    while current != start {
        path.push(current);
        if let Some(Some(prev)) = visited.get(&current) {
            current = *prev;
        } else {
            break;
        }
    }
    
    path.push(start);
    path.reverse();
    path
}

fn neighbors(pos: Pos, width: u16, height: u16) -> Vec<Pos> {
    let mut result = Vec::new();
    
    if pos.y > 0 {
        result.push(Pos { x: pos.x, y: pos.y - 1 });
    }
    if pos.y < height - 1 {
        result.push(Pos { x: pos.x, y: pos.y + 1 });
    }
    if pos.x > 0 {
        result.push(Pos { x: pos.x - 1, y: pos.y });
    }
    if pos.x < width - 1 {
        result.push(Pos { x: pos.x + 1, y: pos.y });
    }
    
    result
}

fn direction_to(from: Pos, to: Pos) -> Dir {
    if to.x > from.x {
        Dir::Right
    } else if to.x < from.x {
        Dir::Left
    } else if to.y > from.y {
        Dir::Down
    } else {
        Dir::Up
    }
}

impl std::hash::Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
