#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty = 0,
    Wall1 = 1,  // Red brick wall
    Wall2 = 2,  // Blue stone wall
    Wall3 = 3,  // Green wood wall
    Wall4 = 4,  // Yellow metal wall
    Exit = 5,   // Exit point
    Hazard = 6, // Damage zone
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<CellType>>,
}

impl Map {
    pub fn new(level: usize) -> Self {
        match level {
            0 => Self::create_level_1(),
            1 => Self::create_level_2(),
            2 => Self::create_level_3(),
            _ => Self::create_level_1(),
        }
    }

    fn create_level_1() -> Self {
        let data = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 1],
            vec![1, 0, 2, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 2, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 6, 6, 0, 0, 0, 0, 0, 0, 6, 6, 0, 0, 1],
            vec![1, 0, 0, 6, 6, 0, 0, 0, 0, 0, 0, 6, 6, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        
        Self {
            width: 16,
            height: 13,
            data: data.into_iter().map(|row| {
                row.into_iter().map(|cell| match cell {
                    0 => CellType::Empty,
                    1 => CellType::Wall1,
                    2 => CellType::Wall2,
                    3 => CellType::Wall3,
                    4 => CellType::Wall4,
                    5 => CellType::Exit,
                    6 => CellType::Hazard,
                    _ => CellType::Empty,
                }).collect()
            }).collect(),
        }
    }

    fn create_level_2() -> Self {
        let data = vec![
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            vec![2, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2],
            vec![2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2],
            vec![2, 0, 1, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 1, 0, 2],
            vec![2, 0, 0, 0, 0, 0, 3, 6, 6, 3, 0, 0, 0, 0, 0, 2],
            vec![2, 0, 0, 0, 0, 0, 3, 6, 6, 3, 0, 0, 0, 0, 0, 2],
            vec![2, 0, 1, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 1, 0, 2],
            vec![2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2],
            vec![2, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            vec![2, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        ];
        
        Self {
            width: 16,
            height: 13,
            data: data.into_iter().map(|row| {
                row.into_iter().map(|cell| match cell {
                    0 => CellType::Empty,
                    1 => CellType::Wall1,
                    2 => CellType::Wall2,
                    3 => CellType::Wall3,
                    4 => CellType::Wall4,
                    5 => CellType::Exit,
                    6 => CellType::Hazard,
                    _ => CellType::Empty,
                }).collect()
            }).collect(),
        }
    }

    fn create_level_3() -> Self {
        let data = vec![
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
            vec![4, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 4],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
            vec![4, 0, 2, 0, 2, 0, 2, 6, 2, 0, 2, 0, 2, 0, 2, 4],
            vec![4, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 4],
            vec![4, 0, 1, 0, 1, 0, 1, 6, 1, 0, 1, 0, 1, 0, 1, 4],
            vec![4, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 4],
            vec![4, 0, 2, 0, 2, 0, 2, 6, 2, 0, 2, 0, 2, 0, 2, 4],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
            vec![4, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 0, 3, 4],
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 4],
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
        ];
        
        Self {
            width: 16,
            height: 13,
            data: data.into_iter().map(|row| {
                row.into_iter().map(|cell| match cell {
                    0 => CellType::Empty,
                    1 => CellType::Wall1,
                    2 => CellType::Wall2,
                    3 => CellType::Wall3,
                    4 => CellType::Wall4,
                    5 => CellType::Exit,
                    6 => CellType::Hazard,
                    _ => CellType::Empty,
                }).collect()
            }).collect(),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> CellType {
        if x >= self.width || y >= self.height {
            return CellType::Wall1;
        }
        self.data[y][x]
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        match self.get_cell(x, y) {
            CellType::Wall1 | CellType::Wall2 | CellType::Wall3 | CellType::Wall4 => true,
            _ => false,
        }
    }

    pub fn is_exit(&self, x: usize, y: usize) -> bool {
        self.get_cell(x, y) == CellType::Exit
    }

    pub fn is_hazard(&self, x: usize, y: usize) -> bool {
        self.get_cell(x, y) == CellType::Hazard
    }

    pub fn get_wall_color(&self, cell_type: CellType) -> u32 {
        match cell_type {
            CellType::Wall1 => 0xFF4444, // Red brick
            CellType::Wall2 => 0x4444FF, // Blue stone
            CellType::Wall3 => 0x44FF44, // Green wood
            CellType::Wall4 => 0xFFFF44, // Yellow metal
            CellType::Exit => 0xFF00FF,   // Magenta exit
            CellType::Hazard => 0xFF8800, // Orange hazard
            _ => 0x888888,                // Gray default
        }
    }
}