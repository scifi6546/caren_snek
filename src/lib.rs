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
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static TILE_SIZE: u32 = 20;
mod grid;
use grid::*;
#[wasm_bindgen]
pub struct State {
    entities: Vec<Entity>,
    grid: Grid,
    input: InputComponent,
    grid_component: GridComponent,
    damage_component: EnemyDamageComponent,
}
#[wasm_bindgen]
impl State {
    pub fn process(&mut self, input: Vector2) {
        let mut entities_v = vec![];
        for ent in self.entities.iter() {
            let check = ent.component_checklist.clone();
            if check.input_component == true {
                let test = self.input.apply(
                    ent.clone(),
                    self.grid.clone(),
                    input.clone(),
                    &self.entities,
                );
                entities_v.push(test.0);
                self.grid = test.1;
            } else {
                entities_v.push(ent.clone());
            }
        }
        self.entities=entities_v;
        entities_v = vec![];
        for ent in self.entities.iter() {
            let check = ent.component_checklist.clone();
            if check.input_component == true {
                let test = self.input.apply(
                    ent.clone(),
                    self.grid.clone(),
                    input.clone(),
                    &self.entities,
                );
                entities_v.push(test.0);
                self.grid = test.1;
            } else {
                entities_v.push(ent.clone());
            }
        }
        self.entities = entities_v;
        entities_v = vec![];
        for ent in self.entities.iter() {
            let check = ent.component_checklist.clone();
            if check.grid_component {
                let test = self.grid_component.apply(
                    ent.clone(),
                    self.grid.clone(),
                    input.clone(),
                    &self.entities,
                );
                entities_v.push(test.0);
                self.grid = test.1;
            } else {
                entities_v.push(ent.clone())
            }
        }
        self.entities = entities_v;
    }
    #[allow(dead_code)]
    fn get_tile(&self, position: Vector2) -> Option<Tile> {
        return self.grid.get_tile(position);
    }
    fn get_entity(&self, position: Vector2) -> Vec<&Entity> {
        let mut v = vec![];
        for ent in self.entities.iter() {
            if ent.position == position {
                v.push(ent);
            }
        }
        return v;
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut draws = self.grid.draw();
        for ent in self.entities.iter() {
            draws.append(&mut ent.draw());
        }
        return draws;
    }
    pub fn game_loop_js(&mut self,input:JsValue)->JsValue{
        self.process(serde_wasm_bindgen::from_value(input).ok().unwrap());
        serde_wasm_bindgen::to_value(&self.draw()).ok().unwrap()
        
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

fn new_player(position: Vector2) -> Entity {
    let mut e = Entity::new(position, 10, 10, 0x00ff00, EntityTeam::Player);
    e.component_checklist.input_component = true;
    e.component_checklist.grid_component = true;
    e.component_checklist.damage_component = true;
    return e;
}
fn new_enemy(position: Vector2) -> Entity {
    let mut e = Entity::new(position, 10, 10, 0xff0000, EntityTeam::Enemy);
    e.component_checklist.input_component = false;
    e.component_checklist.grid_component = true;
    e.component_checklist.damage_component = true;
    return e;
}
fn new_prize(position: Vector2) -> Entity {
    let mut e = Entity::new(position, 10, 10, 0xffec00, EntityTeam::Player);
    e.component_checklist.input_component = false;
    e.component_checklist.grid_component = true;
    e.component_checklist.damage_component = false;
    return e;
}
trait Component {
    fn apply(
        &self,
        entity: Entity,
        world: Grid,
        input: Vector2,
        entities: &Vec<Entity>,
    ) -> (Entity, Grid);
}

#[derive(Serialize, Deserialize)]
struct InputComponent {}
impl Component for InputComponent {
    fn apply(
        &self,
        entity: Entity,
        world: Grid,
        input: Vector2,
        entities: &Vec<Entity>,
    ) -> (Entity, Grid) {
        let mut entity_n = entity;
        entity_n.delta_position = input;
        return (entity_n, world);
    }
}
#[derive(Serialize, Deserialize)]
struct GridComponent {}
impl Component for GridComponent {
    fn apply(
        &self,
        entity: Entity,
        world: Grid,
        _input: Vector2,
        entities: &Vec<Entity>,
    ) -> (Entity, Grid) {
        if let Some(tile) = world.get_tile(entity.position.clone() + entity.delta_position.clone())
        {
            let mut entity_m = entity;
            if tile != Tile::Wall {
                entity_m.position += entity_m.delta_position;
            }
            entity_m.delta_position = Vector2::new(0, 0);
            return (entity_m, world);
        }
        let mut entity_m = entity;
        entity_m.delta_position = Vector2::new(0, 0);
        return (entity_m, world);
    }
}
#[derive(Serialize, Deserialize)]
struct EnemyDamageComponent {}
impl Component for EnemyDamageComponent {
    fn apply(
        &self,
        entity: Entity,
        world: Grid,
        _input: Vector2,
        entities: &Vec<Entity>,
    ) -> (Entity, Grid) {
        let mut ent_m = entity;
        if ent_m.health==0{
            ent_m.delta_position=Vector2::new(0, 0);
            ent_m.component_checklist.damage_component=false;
            ent_m.component_checklist.input_component=false;
            return (ent_m,world);
            
        }
        let pos = ent_m.position.clone()+ent_m.delta_position.clone();
        for ent in entities.iter() {
            if ent.position == pos && ent.team != ent_m.team && ent_m.health>0{
                ent_m.health -= 1;
                ent_m.delta_position=Vector2::new(0, 0);
            }
        }
        (ent_m, world)
    }
}

pub fn game_loop(input: Vector2, state: State) -> MainOutput {
    let mut state_m = state;
    state_m.process(input);

    MainOutput {
        draw_calls: state_m.draw(),
        state: state_m,
    }
}

pub fn init_state() -> State {
    State {
        entities: vec![
            new_player(Vector2::new(1, 1)),
            new_enemy(Vector2::new(2, 3)),
            new_prize(Vector2::new(7,7)),
        ],
        grid: Grid::new(
            10,
            10,
            vec![
                Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Floor,Tile::Wall ,
                Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,Tile::Wall ,

            ],
        ),
        input: InputComponent {},
        grid_component: GridComponent {},
        damage_component: EnemyDamageComponent {},
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
        let g = Grid::new(1, 1, vec![Tile::Wall]);
        assert_eq!(
            g.draw(),
            vec![Tile::Wall.get_color(), 0, 0, TILE_SIZE, TILE_SIZE]
        )
    }
    #[test]
    fn run_frame() {
        let s = init_state();
        game_loop(Vector2::new(0, 0), s);
    }

    #[test]
    fn run_frame_input() {
        let mut s = init_state();
        s = game_loop(Vector2::new(1, 0), s).state;
    }
    #[test]
    fn player_draw() {
        let mut p = new_player(Vector2::new(0, 0));
        assert_eq!(p.draw(), vec![0x00ff00, 0, 0 as u32, TILE_SIZE, TILE_SIZE]);
        p.health = 0;
        assert_eq!(p.draw(), vec![0xffffff, 0, 0 as u32, TILE_SIZE, TILE_SIZE]);
    }
}
