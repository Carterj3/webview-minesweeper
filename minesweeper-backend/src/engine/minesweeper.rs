#[derive(Deserialize, Serialize, Debug)]
pub enum Tile
{
    Unknown,
    Flag,
    Bomb,
    Exposed {num_bombs_around: u8},
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Action
{
    Flag { x_position: u16, y_position: u16 },
    Expose { x_position: u16, y_position: u16 },
    Reset,
    Start { width: u16, height: u16, num_bombs: u16 },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Minesweeper 
{
    /// Starts initially empty so that _after_ the first move it can be populated
    field: Vec<Tile>,

    width: u16,
    height: u16,
    num_bombs: u16,
}

impl Minesweeper
{
    pub fn new(width: u16, height: u16, num_bombs: u16) -> Minesweeper
    {
        Minesweeper {
            field: Vec::with_capacity((width * height) as usize),

            width,
            height,
            num_bombs,
        }
    }

    pub fn get_tiles(&self) -> (u16, &[Tile])
    {
        (self.width, &self.field)
    }
}

