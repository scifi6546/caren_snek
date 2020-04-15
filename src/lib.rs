extern crate wasm_bindgen;
#[allow(unused_imports)]
use serde_wasm_bindgen::*;
#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
extern crate wee_alloc;
mod vector;
use vector::*;
mod entity;
use entity::*;
mod controller;
use controller::*;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[allow(dead_code)]
static TILE_SIZE: u32 = 20;
mod grid;
use grid::*;
#[wasm_bindgen]
pub struct State {
    entities: Vec<Entity>,
    grid: Grid,
}
#[wasm_bindgen]
impl State {
    pub fn process(&mut self, input: Controller) {
        let old_entities = &self.entities.clone();
        let mut new_entities = vec![];
        for entity in self.entities.iter_mut() {
            new_entities.append(&mut entity.process(&input, &self.grid, old_entities));
        }
        self.entities.append(&mut new_entities);
        self.kill_dead();
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut draws = self.grid.draw();
        for ent in self.entities.iter() {
            draws.append(&mut ent.draw());
        }
        return draws;
    }
    pub fn kill_dead(&mut self){
        let mut new_entities = vec![];
        for entity in self.entities.iter(){
            if entity.get_dead()==false{
                new_entities.push(entity.clone());
            }
        }
        self.entities = new_entities;
    }
    pub fn game_loop_js(&mut self, input: JsValue) -> JsValue {
        serde_wasm_bindgen::to_value(
            &self.game_loop(serde_wasm_bindgen::from_value(input).ok().unwrap()),
        )
        .ok()
        .unwrap()
    }
    pub fn game_loop(&mut self, input: Controller) -> Vec<u32> {
        self.process(input);
        self.draw()
    }
    #[allow(dead_code)]
    fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

pub struct MainOutput {
    pub state: State,
    pub draw_calls: Vec<u32>,
}

fn new_cursor(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0x00ff00,
        EntityTeam::Player,
        vec![
            entity::InputComponent::new(),
            entity::GridComponent::new(),
            entity::SpawnFoodComponent::new(),
        ],
    )
}
#[allow(dead_code)]
fn new_enemy(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0xff0000,
        EntityTeam::Enemy,
        vec![
            entity::InputComponent::new(),
            entity::GridComponent::new(),
            entity::EnemyDamageComponent::new(),
        ],
    )
}
#[allow(dead_code)]
fn new_prize(position: Vector2) -> Entity {
    Entity::new(
        position,
        10,
        10,
        0xffec00,
        EntityTeam::Player,
        vec![entity::GridComponent::new()],
    )
}

pub fn init_state() -> State {
    let mut map = vec![];
    for y in 0..32 {
        for x in 0..32 {
            if x < 2 || x > 29 || y > 29 {
                map.push(Tile::Glass);
            } else {
                map.push(Tile::Background);
            }
        }
    }
    State {
        entities: vec![
            new_cursor(Vector2::new(2, 2)),
            new_snake_entity(Vector2::new(16, 29)),
        ],
        grid: Grid::new(32, 32, map),
    }
}
#[wasm_bindgen]
pub fn init_state_js() -> State {
    init_state()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_grid() {
        let v: Vec<Tile> = vec![];
        let g = Grid::new(0, 0, v);
        assert!(g.draw().len() == 0)
    }
    #[test]
    fn one_by_one_grid() {
        let g = Grid::new(1, 1, vec![Tile::Glass]);
        assert_eq!(
            g.draw(),
            vec![Tile::Glass.get_color(), 0, 0, TILE_SIZE, TILE_SIZE]
        )
    }
    #[test]
    fn test_init_state() {
        init_state();
    }
    #[test]
    fn draw_state() {
        let s = init_state();
        s.draw();
    }
    #[test]
    fn run_frame() {
        let mut s = init_state();
        s.game_loop(Controller::new(Vector2::new(0, 0),&vec![]));
    }
}
