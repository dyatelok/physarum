use crate::world::consts::DECAY_RATE;
use crate::world::consts::HEIGHT;
use crate::world::consts::PHEROMONE_LIMIT;
use crate::world::consts::WIDTH;

use crate::world::consts::VIRIDIS;

use rand::Rng;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    heat: f32,
}

impl Cell {
    fn new(pheromone_level: f32) -> Self {
        Self {
            heat: pheromone_level.min(PHEROMONE_LIMIT),
        }
    }
    fn decay(&mut self) {
        self.heat = (self.heat.min(PHEROMONE_LIMIT) - DECAY_RATE).max(0.0);
    }
    fn color(&self) -> [u8; 4] {
        let i = (self.heat / PHEROMONE_LIMIT * 512.0) as usize;
        VIRIDIS[i]
    }
}

struct Grid {
    state: bool,
    grid0: [[Cell; WIDTH as usize]; HEIGHT as usize],
    grid1: [[Cell; WIDTH as usize]; HEIGHT as usize],
}

impl Grid {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut grid = [[Cell::new(0.0); WIDTH as usize]; HEIGHT as usize];

        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                let rn: f32 = rng.gen();
                grid[i][j] = Cell::new(rn * PHEROMONE_LIMIT);
            }
        }

        Self {
            state: false,
            grid0: grid,
            grid1: [[Cell::new(0.0); WIDTH as usize]; HEIGHT as usize],
        }
    }
    fn get_grid(&self) -> &[[Cell; WIDTH as usize]; HEIGHT as usize] {
        //if !self.state {
        &self.grid0
        /*} else {
            &self.grid1
        }*/
    }
    fn get_mutgrid(&mut self) -> &mut [[Cell; WIDTH as usize]; HEIGHT as usize] {
        //if !self.state {
        &mut self.grid0
        /*} else {
            &mut self.grid1
        }*/
    }
    fn update(&mut self) {
        let grid = self.get_mutgrid();
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                grid[i][j].decay();
            }
        }
    }
}

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct World {
    do_render_agents: bool,
    do_render_pheromone: bool,
    grid: Grid,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    /*pub fn new() -> Self {
        Self {
            do_render_agents: false,
            do_render_pheromone: true,
        }
    }*/

    pub fn new() -> Self {
        Self {
            do_render_agents: false,
            do_render_pheromone: true,
            grid: Grid::new(),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {
        //self.grid.update();
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        let grid = self.grid.get_grid();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as usize;
            let y = (i / WIDTH as usize) as usize;

            let rgba = grid[y][x].color();

            pixel.copy_from_slice(&rgba);
        }
    }
}
