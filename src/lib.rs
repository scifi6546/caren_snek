extern crate wasm_bindgen;
#[allow(unused_imports)]
use serde_wasm_bindgen::*;
#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
extern crate wee_alloc;
mod vector;
use vector::*;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static TILE_SIZE: u32 = 20;
#[derive(Serialize, Deserialize, Clone, std::cmp::PartialEq, Debug)]
enum Tile {
    Wall,
    Floor,
}

impl Tile {
    fn get_color(&self) -> u32 {
        match self {
            Self::Floor => 0x191919,
            Self::Wall => 0x033499,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
struct Grid {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32, tiles: Vec<Tile>) -> Grid {
        Grid {
            tiles: tiles,
            width: width,
            height: height,
        }
    }
    pub fn get_tile(&self, position: Vector2) -> Option<Tile> {
        let index = (position.x as u32 * self.width + position.y as u32) as usize;
        if index < self.tiles.len() {
            return Some(self.tiles[index].clone());
        }
        None
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                out.append(&mut vec![
                    self.tiles[(x * self.width + y) as usize].get_color(),
                    x * TILE_SIZE,
                    y * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                ])
            }
        }
        return out;
    }
}
#[derive(Serialize, Deserialize)]
pub struct State {
    entities: Vec<Entity>,
    grid: Grid,
    input: InputComponent,
    grid_component: GridComponent,
    damage_component: EnemyDamageComponent,
}
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
            if check.damage_component == true {
                let test = self.damage_component.apply(
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
    #[allow(dead_code)]
    fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
#[derive(Serialize)]
pub struct MainOutput {
    pub state: State,
    pub draw_calls: Vec<u32>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum EntityTeam {
    Player,
    Enemy,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Entity {
    position: Vector2,
    delta_position: Vector2,
    component_checklist: EntityComponentChecklist,
    health: u32,
    max_health: u32,
    base_color: u32,
    team: EntityTeam,
}
impl Entity {
    pub fn new(
        pos: Vector2,
        health: u32,
        max_health: u32,
        base_color: u32,
        team: EntityTeam,
    ) -> Entity {
        Entity {
            position: pos,
            delta_position: Vector2::new(0, 0),
            component_checklist: EntityComponentChecklist::new(),
            health: health,
            max_health: max_health,
            base_color: base_color,
            team: team,
        }
    }
    pub fn draw(&self) -> Vec<u32> {
        let health = (self.max_health as f64 - self.health as f64) / (self.max_health as f64);
        let current_red = (self.base_color >> 16) & 0x0000ff;
        let red = (((0xff - current_red) as f64) * health) as u32 & 0x0000ff;
        let current_green = (self.base_color & 0x00ff00) >> 8;
        let green = (((0xff - current_green) as f64) * health) as u32;
        let current_blue = (self.base_color & 0x0000ff);
        let blue = (((0xff - current_blue) as f64) * health) as u32;
        vec![
            (red << 16) + (green << 8) + blue + self.base_color,
            (self.position.x as u32 * TILE_SIZE) as u32,
            (self.position.y as u32 * TILE_SIZE) as u32,
            TILE_SIZE,
            TILE_SIZE,
        ]
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct EntityComponentChecklist {
    input_component: bool,
    damage_component: bool,
    grid_component: bool,
}
impl EntityComponentChecklist {
    pub fn new() -> EntityComponentChecklist {
        EntityComponentChecklist {
            input_component: false,
            grid_component: false,
            damage_component: false,
        }
    }
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
#[wasm_bindgen]
pub fn game_loop_js(input: JsValue, state_in: JsValue) -> JsValue {
    let mut input:Vector2=serde_wasm_bindgen::from_value(input).ok().unwrap();
    input.y=input.y*-1;
    return serde_wasm_bindgen::to_value(&game_loop(
        input,
        serde_wasm_bindgen::from_value(state_in).ok().unwrap(),
    ))
    .ok()
    .unwrap();
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
pub fn init_state_js() -> JsValue {
    serde_wasm_bindgen::to_value(&init_state()).ok().unwrap()
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
