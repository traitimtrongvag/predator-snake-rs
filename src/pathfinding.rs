use crate::snake::{Dir, Pos};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn find_direction(
    head: Pos,
    target: Pos,
    body: &[Pos],
    width: u16,
    height: u16,
) -> Option<Dir> {
    let possible_dirs = get_safe_directions(head, body, width, height);

    if possible_dirs.is_empty() {
        // fully trapped — let the caller decide (will trigger death on next tick)
        return None;
    }

    if let Some(path) = bfs(head, target, body, width, height) {
        if path.len() > 1 {
            let next_dir = direction_to(head, path[1]);
            if possible_dirs.contains(&next_dir) {
                return Some(next_dir);
            }
        }
    }

    possible_dirs.iter()
        .max_by_key(|&&dir| {
            let next_pos = apply_direction(head, dir, width, height);
            let space = count_reachable_space(next_pos, body, width, height);
            let dist = distance_to_target(next_pos, target);
            space * 1000 - dist as usize
        })
        .copied()
        .map(Some)
        .unwrap_or(None)
}

fn get_safe_directions(head: Pos, body: &[Pos], width: u16, height: u16) -> Vec<Dir> {
    let all_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    let is_outside = head.x >= width || head.y >= height;
    
    all_dirs.iter()
        .filter(|&&dir| {
            let next = apply_direction_unchecked(head, dir);
            
            if is_outside {
                !body.contains(&next)
            } else {
                next.x < width && next.y < height && !body.contains(&next)
            }
        })
        .copied()
        .collect()
}

fn apply_direction(pos: Pos, dir: Dir, width: u16, height: u16) -> Pos {
    let next = apply_direction_unchecked(pos, dir);
    let is_outside = pos.x >= width || pos.y >= height;
    
    if is_outside {
        return next;
    }
    
    if next.x < width && next.y < height {
        next
    } else {
        pos
    }
}

fn apply_direction_unchecked(pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::Up => Pos { x: pos.x, y: pos.y.saturating_sub(1) },
        Dir::Down => Pos { x: pos.x, y: pos.y + 1 },
        Dir::Left => Pos { x: pos.x.saturating_sub(1), y: pos.y },
        Dir::Right => Pos { x: pos.x + 1, y: pos.y },
    }
}

fn count_reachable_space(start: Pos, body: &[Pos], width: u16, height: u16) -> usize {
    // HashSet avoids the stride-mismatch bug from the old bool-vec approach.
    // No arbitrary cap — we let BFS run the full reachable area so the
    // fallback heuristic actually reflects available space.
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in neighbors(current, width, height) {
            if !visited.contains(&neighbor) && !body.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    visited.len()
}

fn distance_to_target(from: Pos, to: Pos) -> u16 {
    let dx = (from.x as i32 - to.x as i32).abs();
    let dy = (from.y as i32 - to.y as i32).abs();
    (dx + dy) as u16
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


