
use ::errors::Result;

use ::common::{Horizontal, Vertical};
use ::common::vec2d::Vec2d;

use rand;
use rand::Rng;
use rand::distributions::Uniform;

use std::collections::VecDeque;


impl Vertical
{
    fn neighbors(&self) -> Vec<Vertical>
    {
        let mut neighbors = Vec::with_capacity(3);

        if let Some(below) = self.0.checked_sub(1)
        {
            neighbors.push(Vertical(below))
        };

        neighbors.push(Vertical(self.0));

        if let Some(above) = self.0.checked_add(1)
        {
            neighbors.push(Vertical(above))
        };

        neighbors
    }
}

impl Horizontal
{
    fn neighbors(&self) -> Vec<Horizontal>
    {
        let mut neighbors = Vec::with_capacity(3);

        if let Some(left) = self.0.checked_sub(1)
        {
            neighbors.push(Horizontal(left))
        };

        neighbors.push(Horizontal(self.0));

        if let Some(right) = self.0.checked_add(1)
        {
            neighbors.push(Horizontal(right))
        };

        neighbors
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tile
{
    pub num_bombs_around: usize,
    pub has_flag: bool,
    pub is_bomb: bool,
    pub is_shown: bool,
    pub was_clicked: bool,
}

impl Tile
{
    fn default() -> Tile 
    {
        Tile {
            num_bombs_around: 0,
            has_flag: false,
            is_bomb: false,
            is_shown: false,
            was_clicked: false,
        }
    }

    fn copy_from(&mut self, source: &Tile)
    {

        self.num_bombs_around = source.num_bombs_around;
        self.has_flag = source.has_flag;
        self.is_bomb = source.is_bomb;
        self.is_shown = source.is_shown;
        self.was_clicked = source.was_clicked;
    }
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "_type")]
pub enum Action
{
    Flag { x_position: usize, y_position: usize },
    Unflag { x_position: usize, y_position: usize },
    Expose { x_position: usize, y_position: usize },
    Start { width: usize, height: usize, num_bombs: usize },
    Quit,
}

#[derive(PartialEq, Eq, Debug)]
pub enum State
{
    Won,
    Loss,
    New,
    InProgress,
}

#[derive(Debug)]
pub struct Minesweeper 
{
    /// Starts initially empty so that _after_ the first move it can be populated
    internal_field: Vec2d<Tile>,
    /// External representation of the field where `is_bomb` is always false until a bomb is clicked
    external_field: Vec2d<Tile>,

    width: Horizontal,
    height: Vertical,

    num_bombs: usize,
    num_flags: usize,
    num_correct_flags: usize,
    state: State,
}

impl Minesweeper
{
    pub fn new(width: Horizontal, height: Vertical, num_bombs: usize) -> Result<Minesweeper>
    {
        let temp_horizontal = Horizontal(0);
        let temp_vertical = Vertical(0);

        let mut minesweeper = Minesweeper {
            internal_field: Vec2d::new(temp_horizontal, temp_vertical),
            external_field: Vec2d::new(temp_horizontal, temp_vertical), 

            width: temp_horizontal,
            height: temp_vertical,

            num_bombs: 0,
            num_flags: 0,
            num_correct_flags: 0,
            state: State::New,
        };

        minesweeper.resize(width, height, num_bombs)?;

        Ok(minesweeper)
    }

    pub fn resize(&mut self, width: Horizontal, height: Vertical, num_bombs: usize) -> Result<()>
    {
        self.internal_field = Vec2d::new(width, height);
        self.external_field = Vec2d::new(width, height); 

        self.width = width;
        self.height = height;

        self.num_bombs = match width.0.checked_mul(height.0)
        {
            None => bail!("width:{} * height:{} overflowed", width.0, height.0),
            Some(width_height) => match num_bombs
            {
                0 => bail!("Need at least one bomb"),
                _ if width_height - 1 < num_bombs => bail!("{} tiles doesn't leave room for initial move + {} bombs", width_height, num_bombs),
                _ => num_bombs,
            }
        };

        self.num_flags = 0;
        self.num_correct_flags = 0;
        self.state = State::New;

        self.external_field.fill(|| Tile::default())?;

        Ok(())
    }

    fn initialize_internal_field(&mut self, x: &Horizontal, y: &Vertical) -> Result<()>
    {
        self.internal_field.fill(|| Tile::default())?;

        /* Place bombs not in the clicked x,y position */
        let mut bombs_placed = 0;
        let mut horizontal_rng = rand::thread_rng();
        let horizontal_range = Uniform::from(0..self.width.0);
        let mut horizontal_iter = horizontal_rng.sample_iter(&horizontal_range);

        let mut vertical_rng = rand::thread_rng();
        let vertical_range = Uniform::from(0..self.height.0);
        let mut vertical_iter = vertical_rng.sample_iter(&vertical_range);

        while bombs_placed < self.num_bombs 
        {
            match (horizontal_iter.next(), vertical_iter.next())
            {
                (Some(bomb_x), Some(bomb_y)) =>
                {
                    let bomb_x = Horizontal(bomb_x);
                    let bomb_y = Vertical(bomb_y);

                    if x != &bomb_x || y != &bomb_y
                    {
                        let was_bomb_placed = match self.internal_field.get_mut(&bomb_x, &bomb_y)
                        {
                            None => false,
                            Some(tile) =>
                            {
                                if !tile.is_bomb
                                {
                                    bombs_placed+= 1;
                                    tile.is_bomb = true;

                                    true
                                }else {
                                    false
                                }
                            }
                        };

                        if was_bomb_placed
                        {
                            self.mutate_neighbors(&bomb_x, &bomb_y, |internal, _, _| { internal.num_bombs_around+=1 });
                        }
                    }
                }
                _ => bail!("Unable to get next random x,y for bomb placement"),
            } 
        }

        Ok(())
    }

    fn mutate_neighbors<F>(&mut self, x: &Horizontal, y: &Vertical, mut func: F )
        where F: FnMut(&mut Tile, &mut Tile, (&Horizontal, &Vertical))
    {
        for neighbor_h in x.neighbors().iter()
        {
            for neighbor_w in y.neighbors().iter()
            {
                if let (Some(internal_slot), Some(external_slot))
                    = (self.internal_field.get_mut(neighbor_h, neighbor_w), self.external_field.get_mut(neighbor_h, neighbor_w))
                {
                    func(internal_slot, external_slot, (neighbor_h, neighbor_w));
                }
            }
        }
    }

    fn flag_tile(&mut self, x: &Horizontal, y: &Vertical) -> Result<()>
    {
        match (self.internal_field.get_mut(x, y), self.external_field.get_mut(x, y))
        {
            (Some(internal_tile), Some(external_tile)) =>
            {
                internal_tile.has_flag = true;
                external_tile.has_flag = true;

                self.num_flags = self.num_flags + 1;

                if internal_tile.is_bomb
                {
                    self.num_correct_flags = self.num_correct_flags + 1;
                }

                if self.num_flags == self.num_correct_flags && self.num_correct_flags == self.num_bombs
                {
                    self.state = State::Won;
                }
            },
            _ => bail!("Provided tile x:{}, y:{} was not in the field", x.0, y.0),
        };

        if self.state == State::Won
        {
            self.on_win();
        }

        Ok(())
    }

    fn unflag_tile(&mut self, x: &Horizontal, y: &Vertical) -> Result<()>
    {
        match (self.internal_field.get_mut(x, y), self.external_field.get_mut(x, y))
        {
            (Some(internal_tile), Some(external_tile)) =>
            {
                internal_tile.has_flag = false;
                external_tile.has_flag = false;

                self.num_flags = self.num_flags - 1;

                if internal_tile.is_bomb
                {
                    self.num_correct_flags = self.num_correct_flags - 1;
                }
            },
            _ => bail!("Provided tile x:{}, y:{} was not in the field", x.0, y.0),
        };

        Ok(())
    }

    fn on_loss(&mut self)
    {
        for y in 0..self.height.0
        {
            for x in 0..self.width.0
            {
                let horizontal = Horizontal(x);
                let vertical = Vertical(y);

                match (self.internal_field.get_mut(&horizontal, &vertical), self.external_field.get_mut(&horizontal, &vertical))
                {
                    (Some(internal_tile), Some(external_tile)) =>
                    {
                        if internal_tile.is_bomb
                        {
                            internal_tile.is_shown = true;
                            external_tile.copy_from(internal_tile);
                        }
                    },
                    _ => continue,
                };
            }
        }
    }

    fn on_win(&mut self)
    {
        for y in 0..self.height.0
        {
            for x in 0..self.width.0
            {
                let horizontal = Horizontal(x);
                let vertical = Vertical(y);

                match (self.internal_field.get_mut(&horizontal, &vertical), self.external_field.get_mut(&horizontal, &vertical))
                {
                    (Some(internal_tile), Some(external_tile)) =>
                    {
                        if !internal_tile.is_bomb
                        {
                            internal_tile.is_shown = true;
                            internal_tile.was_clicked = true;
                            external_tile.copy_from(internal_tile);
                        }
                    },
                    _ => continue,
                };
            }
        }
    }

    fn expose_tile(&mut self, x: &Horizontal, y: &Vertical) -> Result<()>
    {
        let mut tiles_to_click = VecDeque::with_capacity(1);
        tiles_to_click.push_back((x.clone(), y.clone()));

        loop
        {
            let (x,y) = match tiles_to_click.pop_front()
            {
                Some(xy) => xy,
                None => break,
            };

            match (self.internal_field.get_mut(&x, &y), self.external_field.get_mut(&x, &y))
            {
                (Some(internal_tile), Some(external_tile)) =>
                {
                    internal_tile.is_shown = true;
                    internal_tile.was_clicked = true;
                    external_tile.copy_from(&internal_tile);

                    if internal_tile.is_bomb
                    {
                        self.state = State::Loss;
                        break;
                    }
                },
                _ => bail!("Provided tile x:{}, y:{} was not in the field", x.0, y.0),
            };

            self.mutate_neighbors(&x, &y, |internal, external, (neighbor_x, neighbor_y)| { 

                internal.is_shown = true;
                external.is_shown = true;
                external.num_bombs_around = internal.num_bombs_around;

                if internal.num_bombs_around == 0 && !internal.was_clicked
                {
                    internal.was_clicked = true;
                    tiles_to_click.push_back((neighbor_x.clone(), neighbor_y.clone()));
                }
            });
        };

        if self.state == State::Loss
        {
            self.on_loss();
        }

        Ok(())
    }

    pub fn handle_action(&mut self, action: Action) -> Result<&State>
    {
        match self.state
        {
            State::New => 
            {
                match action
                {
                    Action::Flag { x_position, y_position } => 
                    {
                        self.initialize_internal_field(&Horizontal(x_position), &Vertical(y_position))?;
                        self.flag_tile(&Horizontal(x_position), &Vertical(y_position))?;

                        self.state = State::InProgress;
                    },
                    Action::Expose { x_position, y_position } =>
                    {
                        self.initialize_internal_field(&Horizontal(x_position), &Vertical(y_position))?;
                        self.expose_tile(&Horizontal(x_position), &Vertical(y_position))?;

                        self.state = State::InProgress;
                    },
                    _ => {},
                }
            },
            State::InProgress =>
            {
                match action
                {
                    Action::Unflag { x_position, y_position } => 
                    { 
                        self.unflag_tile(&Horizontal(x_position), &Vertical(y_position))?;
                    },
                    Action::Flag { x_position, y_position } => 
                    { 
                        self.flag_tile(&Horizontal(x_position), &Vertical(y_position))?;
                    },
                    Action::Expose { x_position, y_position } =>
                    {
                        self.expose_tile(&Horizontal(x_position), &Vertical(y_position))?;
                    },
                    _ => {},
                }
            },
            _ => {},
        };

        Ok(&self.state)
    }

    pub fn get_state(&self) -> &State
    {
        &self.state
    }

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>>
    {
        self.external_field.as_vec_vec()
    }
}

