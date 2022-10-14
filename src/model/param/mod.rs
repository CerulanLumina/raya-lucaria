use std::ops::{Deref, DerefMut};

pub struct Param<T> {
    rows: Vec<Row<T>>,
    definition: ParamDef,
}




pub struct ParamDef {

}

pub struct Row<T> {
    name: Option<String>,
    id: u64,
    data: T,
}

impl<T> Deref for Row<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Row<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn test() {
    let mut x: Row<ShopLineup> = unimplemented!();
}

struct SpEffect {
    pub iconId: i32,
    pub conditionHp: f32,
}

struct ShopLineup {
    pub nyaa: u32,
    pub next_level: u64,
}