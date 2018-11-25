
use ::errors::Result;

use super::{Horizontal, Vertical};

#[derive(Debug)]
pub struct Vec2d<T>
{
    width: Horizontal,
    height: Vertical,
    xy_vec: Vec<Vec<T>>,
}

impl<T> Vec2d<T>
{
    pub fn new(width: Horizontal, height: Vertical) -> Vec2d<T>
    {
        Vec2d {
            width,
            height,
            xy_vec: Vec::with_capacity(height.0),
        }
    }

    pub fn get(&self, x: &Horizontal, y: &Vertical) -> Option<&T>
    {
        match self.xy_vec.get(y.0)
        {
            Some(row) => row.get(x.0),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, x: &Horizontal, y: &Vertical) -> Option<&mut T>
    {
        match self.xy_vec.get_mut(y.0)
        {
            Some(row) => row.get_mut(x.0),
            _ => None,
        }
    }

    pub fn fill<F>(&mut self, item_func: F) -> Result<()>
        where F: Fn() -> T
    {
        if !self.xy_vec.is_empty()
        {
            self.xy_vec.clear();
        }

        for _ in 0..self.height.0
        {
            let mut row = Vec::with_capacity(self.width.0);
            for _ in 0..self.width.0
            {
                row.push(item_func());
            }
            self.xy_vec.push(row);
        }
        Ok(())
    }

    pub fn as_vec_vec(&self) -> &Vec<Vec<T>>
    {
        &self.xy_vec
    }
}