use crate::vector::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Controller{
    main_axis:Vector2,

    buttons:Vec<String>
}

impl Controller{
    pub fn new(main_axis:Vector2,buttons:&Vec<String>)->Controller{
        Controller{
            main_axis:main_axis,
            buttons:buttons.clone()
        }
    }
    pub fn get_main_axis(&self)->&Vector2{
        &self.main_axis
    }
    pub fn get_buttons(&self)->&Vec<String>{
        &self.buttons
    }
}