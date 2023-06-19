const PI: f32 = std::f32::consts::PI;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
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
    pub save_to: String,
    pub load_from: String,
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

            save_to: String::new(),
            load_from: String::new(),
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
    fn write_to(name: String, content: String) -> std::io::Result<()> {
        let mut file = File::create(name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    pub fn save(&mut self) {
        let save_to = "sim_params/".to_string() + self.save_to.as_str() + ".json";
        self.save_to = String::new();
        self.load_from = String::new();
        let content = json!(self).to_string();
        match Params::write_to(save_to.clone(), content) {
            Ok(()) => {
                println!("Saved current parameters to {}", save_to);
            }
            Err(a) => {
                println!("Failed to write to the file: {}", a);
            }
        }
    }
    fn read_from(name: String) -> String {
        let mut file = match File::open(name.clone()) {
            Ok(file) => file,
            Err(a) => {
                panic!("Failed to read from file: {}", a);
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                println!("Loaded parameters from: {}", name);
            }
            Err(a) => {
                panic!("Failed to read from file: {}", a);
            }
        }
        contents
    }
    pub fn load(&mut self) {
        let load_from = "sim_params/".to_string() + self.load_from.as_str() + ".json";
        let content = Params::read_from(load_from.clone());
        let params: Params = match serde_json::from_str(content.as_str()) {
            Ok(a) => a,
            Err(e) => {
                panic!("Failed to convert json to Params: {}", e);
            }
        };
        *self = params;
        self.do_update_world = true;
    }
}
