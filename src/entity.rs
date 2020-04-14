use crate::vector::*;
static TILE_SIZE: u32 = 20;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EntityTeam {
    Player,
    Enemy,
    Food,
    Snake,
}
#[derive(Debug, Clone)]
pub struct Entity {
    state: EntityState,
    components: Vec<Box<dyn Component>>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EntityState {
    pub position: Vector2,
    pub delta_position: Vector2,
    pub health: u32,
    pub max_health: u32,
    pub base_color: u32,
    pub team: EntityTeam,
    dead: bool,
}
impl Entity {
    pub fn new(
        pos: Vector2,
        health: u32,
        max_health: u32,
        base_color: u32,
        team: EntityTeam,
        components: Vec<Box<dyn Component>>,
    ) -> Entity {
        Entity {
            state: EntityState {
                position: pos,
                delta_position: Vector2::new(0, 0),
                health: health,
                max_health: max_health,
                base_color: base_color,
                team: team,
                dead: false,
            },
            components: components,
        }
    }
    pub fn draw(&self) -> Vec<u32> {
        let health = (self.state.max_health as f64 - self.state.health as f64)
            / (self.state.max_health as f64);
        let current_red = (self.state.base_color >> 16) & 0x0000ff;
        let red = (((0xff - current_red) as f64) * health) as u32 & 0x0000ff;
        let current_green = (self.state.base_color & 0x00ff00) >> 8;
        let green = (((0xff - current_green) as f64) * health) as u32;
        let current_blue = self.state.base_color & 0x0000ff;
        let blue = (((0xff - current_blue) as f64) * health) as u32;
        vec![
            (red << 16) + (green << 8) + blue + self.state.base_color,
            (self.state.position.x as u32 * TILE_SIZE) as u32,
            (self.state.position.y as u32 * TILE_SIZE) as u32,
            TILE_SIZE,
            TILE_SIZE,
        ]
    }
    pub fn process(
        &mut self,
        input: &Vector2,
        grid: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        let mut entity_vec = vec![];
        for component in self.components.iter_mut() {
            entity_vec.append(&mut component.process(input, &mut self.state, grid, entities));
        }
        return entity_vec;
    }
    pub fn get_position(&self) -> Vector2 {
        self.state.position.clone()
    }
}
pub trait Component: std::fmt::Debug {
    fn process(
        &mut self,
        user_input: &Vector2,
        state: &mut EntityState,
        world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity>;
    fn box_clone(&self) -> Box<dyn Component>;
}
impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
#[derive(Debug, Clone)]
pub struct InputComponent {}
impl InputComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(InputComponent {})
    }
}
impl Component for InputComponent {
    fn process(
        &mut self,
        user_input: &Vector2,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        state.delta_position = user_input.clone();
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}

#[derive(Debug, Clone)]
pub struct GridComponent {}
impl Component for GridComponent {
    fn process(
        &mut self,
        _user_input: &Vector2,
        state: &mut EntityState,
        world: &crate::grid::Grid,
        _entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if let Some(tile) = world.get_tile(state.position.clone() + state.delta_position.clone()) {
            if tile != crate::grid::Tile::Glass {
                state.position += state.delta_position.clone();
            }
            state.delta_position = Vector2::new(0, 0);
            vec![]
        } else {
            state.delta_position = Vector2::new(0, 0);
            vec![]
        }
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl GridComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(GridComponent {})
    }
}
#[derive(Debug, Clone)]
pub struct EnemyDamageComponent {}
impl Component for EnemyDamageComponent {
    fn process(
        &mut self,
        _user_input: &Vector2,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if state.health == 0 {
            state.delta_position = Vector2::new(0, 0);
            state.dead = true;
        }
        let pos = state.position.clone() + state.delta_position.clone();
        for ent in entities.iter() {
            if ent.state.position == pos && ent.state.team != state.team && state.health > 0 {
                state.health -= 1;
                state.delta_position = Vector2::new(0, 0);
            }
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}

impl EnemyDamageComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(EnemyDamageComponent {})
    }
}
#[derive(Debug, Clone)]
pub struct GravityComponent {
    ticker: u32,
    fall_time: u32, //number of frames before Gravity component falls one unit
}
impl Component for GravityComponent {
    fn process(
        &mut self,
        _user_input: &Vector2,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        self.ticker += 1;
        if self.ticker > self.fall_time {
            state.delta_position.y += 1;
            self.ticker = 0;
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}

impl GravityComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(GravityComponent {
            ticker: 0,
            fall_time: 30,
        })
    }
}
#[derive(Debug, Clone)]
pub struct SnakeBodyComponent {
    cool_down: u32,
}
impl Component for SnakeBodyComponent {
    fn process(
        &mut self,
        _user_input: &Vector2,
        state: &mut EntityState,
        _world: &crate::grid::Grid,
        entities: &Vec<Entity>,
    ) -> Vec<Entity> {
        if self.cool_down < 10000 {
            self.cool_down += 1;
        }
        for ent in entities.iter() {
            if ent.state.team == EntityTeam::Food
                && state.position.within_one_of(&ent.state.position)
            {
                if self.cool_down > 100 {
                    self.cool_down = 0;
                    return vec![new_snake_entity(ent.state.position.clone())];
                }
            }
        }
        vec![]
    }
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}
impl SnakeBodyComponent {
    pub fn new() -> Box<dyn Component> {
        Box::new(SnakeBodyComponent { cool_down: 0 })
    }
}
pub fn new_snake_entity(position: Vector2) -> Entity {
    Entity::new(
        position,
        1,
        1,
        0x007b12,
        EntityTeam::Snake,
        vec![SnakeBodyComponent::new(), GridComponent::new()],
    )
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn player_draw() {
        let mut p = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![
                InputComponent::new(),
                GridComponent::new(),
                EnemyDamageComponent::new(),
            ],
        );
        assert_eq!(p.draw(), vec![0x00ff00, 0, 0 as u32, TILE_SIZE, TILE_SIZE]);
        p.state.health = 0;
        assert_eq!(p.draw(), vec![0xffffff, 0, 0 as u32, TILE_SIZE, TILE_SIZE]);
    }
    #[test]
    fn player_process_draw() {
        let p = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![
                InputComponent::new(),
                GridComponent::new(),
                EnemyDamageComponent::new(),
            ],
        );
        p.draw();
    }
    #[test]
    fn player_empty_process() {
        let mut e = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![
                InputComponent::new(),
                GridComponent::new(),
                EnemyDamageComponent::new(),
            ],
        );
        e.process(
            &Vector2::new(0, 0),
            &crate::grid::Grid::new(0, 0, vec![]),
            &vec![],
        );
    }
    #[test]
    fn component_clone() {
        let c: Box<dyn Component> = InputComponent::new();
        let _c2 = c.clone();
    }
    #[test]
    fn entity_clone() {
        let e = Entity::new(
            Vector2::new(0, 0),
            10,
            10,
            0x00ff00,
            EntityTeam::Player,
            vec![InputComponent::new()],
        );
        let _e2 = e.clone();
    }
}
