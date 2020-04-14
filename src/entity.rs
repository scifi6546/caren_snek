use crate::vector::*;
static TILE_SIZE: u32 = 20;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EntityTeam {
    Player,
    Enemy,
}
#[derive(Debug)]
pub struct Entity {
    state:EntityState,
    components:Vec<Box<dyn Component>>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct EntityState{
    pub position: Vector2,
    pub delta_position: Vector2,
    pub component_checklist: EntityComponentChecklist,
    pub health: u32,
    pub max_health: u32,
    pub base_color: u32,
    pub team: EntityTeam,
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
            state:EntityState{
            position: pos,
            delta_position: Vector2::new(0, 0),
            component_checklist: EntityComponentChecklist::new(),
            health: health,
            max_health: max_health,
            base_color: base_color,
            team: team,
            },
            components:vec![]

        }
    }
    pub fn draw(&self) -> Vec<u32> {
        let health = (self.state.max_health as f64 - self.state.health as f64) / (self.state.max_health as f64);
        let current_red = (self.state.base_color >> 16) & 0x0000ff;
        let red = (((0xff - current_red) as f64) * health) as u32 & 0x0000ff;
        let current_green = (self.state.base_color & 0x00ff00) >> 8;
        let green = (((0xff - current_green) as f64) * health) as u32;
        let current_blue = (self.state.base_color & 0x0000ff);
        let blue = (((0xff - current_blue) as f64) * health) as u32;
        vec![
            (red << 16) + (green << 8) + blue + self.state.base_color,
            (self.state.position.x as u32 * TILE_SIZE) as u32,
            (self.state.position.y as u32 * TILE_SIZE) as u32,
            TILE_SIZE,
            TILE_SIZE,
        ]
    }
    pub fn process(&mut self,input:Vector2,grid:crate::grid::Grid,entitys:&Vec<Entity>){
        for component in self.components.iter_mut(){
            component.process(input,&mut self.state,grid,entitys);
        }
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EntityComponentChecklist {
    pub input_component: bool,
    pub damage_component: bool,
    pub grid_component: bool,
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

trait Component:std::fmt::Debug{
    fn process(&mut self,user_input:Vector2,state:&mut EntityState,world:crate::grid::Grid,entities:&Vec<Entity>);
}
#[derive(Debug)]
struct InputComponent{

}
impl Component for InputComponent{
    fn process(&mut self,user_input:Vector2,state:&mut EntityState,world:crate::grid::Grid,entities:&Vec<Entity>){
        state.delta_position=user_input;
    }
}

#[derive(Debug)]
struct GridComponent {}
impl Component for GridComponent {
    fn process(&mut self,user_input:Vector2,state:&mut EntityState,world:crate::grid::Grid,entities:&Vec<Entity>){
        if let Some(tile) = world.get_tile(state.position.clone() + state.delta_position.clone())
        {
            if tile != crate::grid::Tile::Wall {
                state.position += state.delta_position;
            }
            state.delta_position = Vector2::new(0, 0);
        }
        state.delta_position = Vector2::new(0, 0);
    }
}
#[derive(Debug)]
struct EnemyDamageComponent {}
impl Component for EnemyDamageComponent {
    fn process(&mut self,user_input:Vector2,state:&mut EntityState,world:crate::grid::Grid,entities:&Vec<Entity>){
        if state.health==0{
            state.delta_position=Vector2::new(0, 0);
            state.component_checklist.damage_component=false;
            state.component_checklist.input_component=false;
            
        }
        let pos = state.position.clone()+state.delta_position.clone();
        for ent in entities.iter() {
            if ent.state.position == pos && ent.state.team != state.team && state.health>0{
                state.health -= 1;
                state.delta_position=Vector2::new(0, 0);
            }
        }
    }
}