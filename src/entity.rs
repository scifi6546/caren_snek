use crate::vector::*;
static TILE_SIZE: u32 = 20;
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EntityTeam {
    Player,
    Enemy,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Entity {
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
trait Component{

}