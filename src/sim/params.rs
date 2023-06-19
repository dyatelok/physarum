const PI: f32 = std::f32::consts::PI;

use rand::Rng;

pub struct Params {
    pub do_update_world: bool,
    pub do_render_pheromone: bool,
    pub do_render_agents: bool,
    pub trail: f32,
    pub gradient_bound: f32,
    pub pheromone_limit: f32,
    pub wobble: f32,
    pub forced_rot: f32,
    pub rot_m_av: f32,
    pub rot_m_dis: f32,
    pub speed_av: f32,
    pub speed_dis: f32,
    pub rot_s_av: f32,
    pub rot_s_dis: f32,
    pub dis_s_av: f32,
    pub dis_s_dis: f32,
    pub c_level_av: f32,
    pub c_level_dis: f32,
    pub c_duration_av: u16,
    pub c_duration_dis: u16,
}

impl Params {
    pub fn new() -> Self {
        Self {
            do_update_world: false,

            do_render_pheromone: true,
            do_render_agents: false,

            trail: 100.0,

            gradient_bound: 1000.0,
            pheromone_limit: 1000000.0,

            wobble: PI * 0.04,
            forced_rot: 0.02,

            rot_m_av: PI / 7.0,
            rot_m_dis: PI / 32.0,

            speed_av: 2.7,
            speed_dis: 0.5,

            rot_s_av: PI / 2.0,
            rot_s_dis: PI / 12.0,

            dis_s_av: 4.0,
            dis_s_dis: 1.3,

            c_level_av: 10000.0,
            c_level_dis: 1000.0,

            c_duration_av: 300,
            c_duration_dis: 50,
        }
    }
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        self.do_update_world = true;

        self.rot_m_av = rng.gen::<f32>() * PI / 2.0;
        self.rot_m_dis = self.rot_m_av / 10.0;

        self.speed_av = rng.gen::<f32>() * 2.0 + 0.5;
        self.speed_dis = self.speed_av / 10.0;

        self.rot_s_av = rng.gen::<f32>() * PI / 2.0;
        self.rot_s_dis = self.rot_s_av / 10.0;

        self.dis_s_av = rng.gen::<f32>() * 1.0 + 1.0;
        self.dis_s_dis = self.dis_s_av / 10.0;

        self.c_level_av = rng.gen::<f32>() * 40000.0;
        self.c_level_dis = self.c_level_av / 10.0;

        self.c_duration_av = (rng.gen::<f32>() * 500.0) as u16;
        self.c_duration_dis = self.c_duration_av / 10;
    }
}
