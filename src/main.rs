use crate::astar::{shortest_path, Edge};
use crate::map::{Map, Renderable};
use macroquad::prelude::*;
use tile::Tile;
use unit::Unit;

mod astar;
mod map;
mod tile;
mod unit;

#[macroquad::main(":::")]
async fn main() {
    let _texture = Texture2D::from_file_with_format(include_bytes!("rust.png"), None);

    let mut map = Map::new();
    for y in 0..25 {
        for x in 0..25 {
            let tile = Tile::new(x, y);
            map.tiles.push(tile);
        }
    }

    let u = Unit::new(1.0, 1.0);

    let mut graph: Vec<Vec<Edge>> = Vec::new();
    for t in map.tiles.iter() {
        let i = map.tiles.iter().position(|tile| tile == t).unwrap();
        if i != 0 && i != 1 && i != 2 && i != 3 && i != 4 && i != 5 && i != 6 && i != 7 && i != 8 && i != 9 && i != 10 && i != 11 && i != 12 &&  i != 13 &&  i != 14 &&  i != 15 &&  i != 16 &&  i != 17 &&  i != 18 &&  i != 19 &&  i != 20 &&  i != 21 &&  i != 22 &&  i != 23 &&  i != 24  {
            graph.push(vec![Edge { node: i-1 as usize, cost: 1},Edge { node: i+1 as usize, cost: 1},Edge { node: i-25 as usize, cost: 1},Edge { node: i+25 as usize, cost: 1}]);
            // graph.push(vec![Edge { node: i+1 as usize, cost: 1}]);
            // graph.push(vec![Edge { node: i-25 as usize, cost: 1}]);
            // graph.push(vec![Edge { node: i+25 as usize, cost: 1}]);
        }
        // graph.push(vec![]);
    }

    loop {
        clear_background(BLACK);
        map.render();
        u.render();
        let (mx, my) = mouse_position();
        let found = map.get_tile(mx as i32, my as i32);
        let index = map.tiles.iter().position(|t| t == found).unwrap();
        draw_rectangle_lines(
            (found.x * 32) as f32,
            (found.y * 32) as f32,
            32.0,
            32.0,
            2.0,
            WHITE,
        );
        
        draw_text(
            format!("X: {} Y: {} ID: {}", found.x, found.y , index).as_str(),
            (found.x * 32 - 32) as f32,
            (found.y * 32 - 2) as f32,
            20.0,
            SKYBLUE,
        );
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            20.0,
            20.0,
            30.0,
            SKYBLUE,
        );
        print!("{:?}", shortest_path(&graph, 0,26));
        next_frame().await
    }
}
