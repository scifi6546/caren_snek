extern crate wasm_bindgen;
#[allow(unused_imports)]
use serde_wasm_bindgen::*;
#[macro_use]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, std::cmp::PartialEq, Debug)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}
impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    pub fn within_one_of(&self,other:&Vector2)->bool{
        if i32::abs(self.x-other.x)<=1 && i32::abs(self.y-other.y)<=1{
            true
        }else{
            false
        }
    }
}
impl std::ops::Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn add_vec() {
        let v1 = Vector2::new(0, 0) + Vector2::new(0, 0);
        assert_eq!(v1.x, 0);
        assert_eq!(v1.y, 0);
        let v2 = Vector2::new(1, 1) + Vector2::new(1, 1);
        assert_eq!(v2.x, 2);
        assert_eq!(v2.y, 2);
    }
    #[test]
    fn add_vec_assign() {
        let mut v1 = Vector2::new(0, 0);
        v1 += Vector2::new(1, 1);
        assert_eq!(v1.x, 1);
        assert_eq!(v1.y, 1);
    }
    #[test]
    fn within_one_of_test(){
        let v1 = Vector2::new(0, 0);
        assert_eq!(v1.within_one_of(&Vector2::new(0, 0)),true);
        assert_eq!(v1.within_one_of(&Vector2::new(0, 1)),true);
        assert_eq!(v1.within_one_of(&Vector2::new(0, 5)),false);
    }
}