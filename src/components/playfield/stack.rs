#![allow(dead_code)]
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use data::block_data::COLS;
use std::ops::Index;

pub struct Stack {
    block_entities: Vec<Entity>,
    pub cursor_entity: Entity,
}

impl Stack {
    pub fn new(block_entities: Vec<Entity>, cursor_entity: Entity) -> Stack {
        Stack {
            block_entities,
            cursor_entity,
        }
    }

    // convert an x and y coordinate to i
    // use this if you want to back convert from an x and y
    // this is most often used when only one parameter changes and the other one stays
    // example: for x in 0..10 {
    // 		xy2i(x, 0) // searches through 0 until 10 from y at 0
    // }
    pub fn xy2i(x: usize, y: usize) -> usize {
        y * COLS + x
    }

    // use this instead of calling from_xy multiple times
    // converts an iterator i back to x and y
    pub fn i2xy(i: usize) -> (usize, usize) {
        (i % COLS, i / COLS)
    }
}

impl Index<usize> for Stack {
    type Output = Entity;

    fn index(&self, i: usize) -> &Entity {
        &self.block_entities[i]
    }
}

impl Index<(usize, usize)> for Stack {
    type Output = Entity;

    fn index(&self, (x, y): (usize, usize)) -> &Entity {
        &self.block_entities[Stack::xy2i(x, y)]
    }
}

impl Component for Stack {
    type Storage = DenseVecStorage<Self>;
}
