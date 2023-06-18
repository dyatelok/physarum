use crate::sim::consts::HEIGHT;
use crate::sim::consts::PHEROMONE_LIMIT;
use crate::sim::consts::WIDTH;

use crate::sim::consts::GRADIENT_BOUND;

use crate::sim::consts::FORCED_ROT;
use crate::sim::consts::WOBBLE;

use crate::sim::consts::TRAIL;

use crate::sim::consts::AGENTS_N;

use crate::sim::consts::DIFFUSION;

use crate::sim::consts::ROT_M_AV;
use crate::sim::consts::ROT_M_DIS;

use crate::sim::consts::SPEED_AV;
use crate::sim::consts::SPEED_DIS;

use crate::sim::consts::ROT_S_AV;
use crate::sim::consts::ROT_S_DIS;

use crate::sim::consts::DIS_S_AV;
use crate::sim::consts::DIS_S_DIS;

use crate::sim::consts::C_LEVEL_AV;
use crate::sim::consts::C_LEVEL_DIS;

use crate::sim::consts::C_DURATION_AV;
use crate::sim::consts::C_DURATION_DIS;

use crate::sim::consts::VIRIDIS;

const TAU: f32 = std::f32::consts::TAU;

use rand::Rng;

use rayon::prelude::*;

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
    fn color(&self) -> [u8; 4] {
        let i = ((self.heat / GRADIENT_BOUND * 511.0).floor() as usize).min(511);
        VIRIDIS[i]
    }
    fn flush(&mut self) {
        self.heat = 0.0;
    }
    fn add(&mut self, add: f32) {
        self.heat = (self.heat + add).min(PHEROMONE_LIMIT);
    }
}

struct Grid {
    state: bool,
    grid0: Vec<Vec<Cell>>,
    grid1: Vec<Vec<Cell>>,
    neighbors: Vec<Vec<[[(usize, usize); 3]; 3]>>,
}

impl Grid {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::new(0.0); WIDTH as usize]; HEIGHT as usize];

        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                let rn: f32 = rng.gen();
                grid[i][j].heat = 0.0 * rn * PHEROMONE_LIMIT;
            }
        }

        let mut neighbors = vec![vec![[[(0, 0); 3]; 3]; WIDTH as usize]; HEIGHT as usize];

        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                let x = j;
                let xp = (x + 1) % WIDTH as usize;
                let xm = (x + WIDTH as usize - 1) % WIDTH as usize;
                let y = i;
                let yp = (y + 1) % HEIGHT as usize;
                let ym = (y + HEIGHT as usize - 1) % HEIGHT as usize;

                neighbors[i][j] = [
                    [(ym, xm), (ym, x), (ym, xp)],
                    [(y, xm), (y, x), (y, xp)],
                    [(yp, xm), (yp, x), (yp, xp)],
                ];
            }
        }

        Self {
            state: true,
            grid0: grid,
            grid1: vec![vec![Cell::new(0.0); WIDTH as usize]; HEIGHT as usize],
            neighbors,
        }
    }
    fn get_grid(&self) -> &Vec<Vec<Cell>> {
        if !self.state {
            &self.grid0
        } else {
            &self.grid1
        }
    }
    /*fn get_mutgrid(&mut self) -> &mut Vec<Vec<Cell>> {
        if !self.state {
            &mut self.grid0
        } else {
            &mut self.grid1
        }
    }*/
    fn get_mutgrid_gridr_neighbors(
        &mut self,
    ) -> (
        &mut Vec<Vec<Cell>>,
        &Vec<Vec<Cell>>,
        &Vec<Vec<[[(usize, usize); 3]; 3]>>,
    ) {
        if !self.state {
            (&mut self.grid0, &self.grid1, &self.neighbors)
        } else {
            (&mut self.grid1, &self.grid0, &self.neighbors)
        }
    }
    fn update(&mut self) {
        let (grid, prev, neighbors) = self.get_mutgrid_gridr_neighbors();
        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                for l in 0..2 {
                    for k in 0..2 {
                        grid[neighbors[i][j][l][k].0][neighbors[i][j][l][k].1]
                            .add(prev[i][j].heat * DIFFUSION[l][k]);
                    }
                }
            }
        }
        self.state = !self.state;
    }
}

struct Agent {
    cord: (f32, f32),
    dir: f32,
    rot_m: f32,      //move rotation
    speed: f32,      //movement speed
    rot_s: f32,      //sensor rotation
    dis_s: f32,      //sensoe distance
    c_level: f32,    //pheromone level to make him crazy
    c_duration: u16, //crazy duration in moves
    is_crazy: bool,  // is crazy
    c_timer: u16,    // crazy timer
}

impl Agent {
    fn new_n(n: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut res: Vec<Self> = vec![];
        for _ in 0..n {
            let mut cord_x: f32 = rng.gen();
            cord_x *= WIDTH as f32;
            if cord_x < 0.0 {
                cord_x += WIDTH as f32;
            }
            if cord_x >= WIDTH as f32 {
                cord_x -= WIDTH as f32;
            }
            let mut cord_y: f32 = rng.gen();
            cord_y *= HEIGHT as f32;
            if cord_y < 0.0 {
                cord_y += HEIGHT as f32;
            }
            if cord_y >= HEIGHT as f32 {
                cord_y -= HEIGHT as f32;
            }
            let mut dir: f32 = rng.gen();
            dir *= TAU;
            let mut rot_m: f32 = rng.gen();
            rot_m = ROT_M_AV + (rot_m * 2.0 - 1.0) * ROT_M_DIS;

            let mut speed: f32 = rng.gen();
            speed = SPEED_AV + (speed * 2.0 - 1.0) * SPEED_DIS;
            let mut rot_s: f32 = rng.gen();
            rot_s = ROT_S_AV + (rot_s * 2.0 - 1.0) * ROT_S_DIS;
            let mut dis_s: f32 = rng.gen();
            dis_s = DIS_S_AV + (dis_s * 2.0 - 1.0) * DIS_S_DIS;
            let mut c_level: f32 = rng.gen();
            c_level = C_LEVEL_AV + (c_level * 2.0 - 1.0) * C_LEVEL_DIS;
            let c_dur: f32 = rng.gen();
            let c_duration = (C_DURATION_AV as i32
                + ((c_dur * 2.0 - 1.0) * C_DURATION_DIS as f32) as i32)
                as u16;
            res.push(Self {
                cord: (cord_x, cord_y),
                dir,
                rot_m,
                speed,
                rot_s,
                dis_s,
                c_level,
                c_duration,
                is_crazy: false,
                c_timer: 0,
            });
        }
        res
    }
    fn pos(p: (f32, f32), r: f32, d: f32, l: f32) -> (f32, f32) {
        let mut ans = p;
        let rot = r + d;
        ans = (ans.0 + l * rot.cos(), ans.1 + l * rot.sin());

        if ans.0 < 0.0 {
            ans.0 += WIDTH as f32;
        }
        if ans.0 >= WIDTH as f32 {
            ans.0 -= WIDTH as f32;
        }
        if ans.1 < 0.0 {
            ans.1 += HEIGHT as f32;
        }
        if ans.1 >= HEIGHT as f32 {
            ans.1 -= HEIGHT as f32;
        }

        ans
    }
    fn step(agents: &mut Vec<Self>, grid: &mut Vec<Vec<Cell>>, prev: &Vec<Vec<Cell>>) {
        let phers: Vec<(usize, usize)> = agents
            .par_iter_mut()
            .map(|a| {
                let mut rng = rand::thread_rng();
                //leave pheromone on grid
                //grid[a.cord.1.floor() as usize][a.cord.0.floor() as usize].add(TRAIL);

                //read data from sensors
                let sen_r_c = Agent::pos(a.cord, a.dir, a.rot_s, a.dis_s);
                let sen_m_c = Agent::pos(a.cord, a.dir, 0.0, a.dis_s);
                let sen_l_c = Agent::pos(a.cord, a.dir, -a.rot_s, a.dis_s);

                let sen_r = prev[sen_r_c.1.floor() as usize][sen_r_c.0.floor() as usize].heat;
                let sen_m = prev[sen_m_c.1.floor() as usize][sen_m_c.0.floor() as usize].heat;
                let sen_l = prev[sen_l_c.1.floor() as usize][sen_l_c.0.floor() as usize].heat;
                //move according to the data
                if !a.is_crazy {
                    if sen_m >= sen_r && sen_m >= sen_l {
                    } else {
                        if sen_r >= sen_l {
                            a.dir += a.rot_m;
                        } else {
                            a.dir -= a.rot_m;
                        }
                    }
                    a.cord = Agent::pos(a.cord, a.dir, 0.0, a.speed);
                    if sen_m > a.c_level {
                        a.is_crazy = true;
                        a.c_timer = a.c_duration;
                    }
                } else {
                    if sen_m <= sen_r && sen_m <= sen_l {
                    } else {
                        if sen_r <= sen_l {
                            a.dir += a.rot_m;
                        } else {
                            a.dir -= a.rot_m;
                        }
                    }
                    a.cord = Agent::pos(a.cord, a.dir, 0.0, a.speed);
                    a.c_timer -= 1;
                    if a.c_timer == 0 {
                        a.is_crazy = false;
                    }
                }
                //add forced parameters
                let rn: f32 = rng.gen();
                a.dir += (rn * 2.0 - 1.0) * WOBBLE;
                a.dir += FORCED_ROT;
                (a.cord.1.floor() as usize, a.cord.0.floor() as usize)
            })
            .collect();
        phers.iter().for_each(|(y, x)| {
            grid[*y][*x].add(TRAIL);
        });
    }
}

pub struct World {
    do_render_agents: bool,
    do_render_pheromone: bool,
    grid: Grid,
    agents: Vec<Agent>,
}

impl World {
    pub fn new() -> Self {
        Self {
            do_render_agents: false,
            do_render_pheromone: true,
            grid: Grid::new(),
            agents: Agent::new_n(AGENTS_N),
        }
    }

    pub fn update(&mut self) {
        let (grid, prev, _) = self.grid.get_mutgrid_gridr_neighbors();

        for i in 0..HEIGHT as usize {
            for j in 0..WIDTH as usize {
                grid[i][j].flush();
            }
        }

        Agent::step(&mut self.agents, grid, prev);

        self.grid.update();
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