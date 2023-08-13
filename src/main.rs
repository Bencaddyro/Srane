use std::cmp::{min,max};
// use image::{RgbImage, ImageBuffer, Rgb};
use rand::Rng;
//use std::{thread, time};
use eframe::egui;
use arr_macro::arr;

struct Agent {
    pos_x: f64,
    pos_y: f64,
    angle: f64,
}

impl Agent {
    fn new(size_x: usize, size_y: usize) -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            pos_x: rng.gen::<f64>() * size_x as f64,
            pos_y: rng.gen::<f64>() * size_y as f64,
            angle: rng.gen::<f64>() * 2.0 * 3.14159,
        }
    }
    /*
fn new_agent() -> Agent {
    let mut rng = rand::thread_rng();
    match AGENT_SPAWN {
        1 => Agent{
                pos_x: rng.gen::<f64>() * IMAGE_X as f64,
                pos_y: rng.gen::<f64>() * IMAGE_Y as f64,
                angle: rng.gen::<f64>() * 2.0 * 3.14159,
            },
        2 => {
            let distance = rng.gen::<f64>() * SPAWN_SIZE;
            let angle = rng.gen::<f64>() * 2.0 * 3.14159;
            Agent{
                pos_x: IMAGE_X as f64 / 2.0 + angle.cos() * distance,
                pos_y: IMAGE_Y as f64 / 2.0 + angle.sin() * distance,
                angle: angle + 3.14159,
            }},
        _ => Agent{
                pos_x: IMAGE_X as f64 / 2.0,
                pos_y: IMAGE_Y as f64 / 2.0,
                angle: rng.gen::<f64>() * 2.0 * 3.14159,
            },
    }
}
*/

}

const MAX_X: usize = 500;
const MAX_Y: usize = 500;
const MAX_AGENT: usize = 500;

struct MyEguiApp {
    // Simulations Settings
    size_x: usize,          // 320
    size_y: usize,          // 180
    // Agents Settings
    agent_n: usize,         // 250
    agent_speed: f64,       // 1
    agent_turn: f64,        // 35
    // Spawn Settings
    // const AGENT_SPAWN: usize = 1;
    // const SPAWN_SIZE: f64 = 100.0;
    // Sensor Settings
    sensor_angle: f64,      // 35
    sensor_distance: f64,   // 3.5
    sensor_size: usize,     // 1
    // Trail Settings
    trail_weight: f64,      // 255
    trail_decay: f64,       // 1.8
    trail_diffuse: f64,     // 0.07

    // Buffer var
    rgb_buffer: [u8; 3 * MAX_X * MAX_Y],
    trail_map: [[f64; MAX_Y]; MAX_X],
    agents: [Agent; MAX_AGENT],

    // State var
    running: bool,
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Srane Render", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))))
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        MyEguiApp {
            // Simulations Settings
            size_x: 500,//320,
            size_y: 400,//180,
            // Agents Settings
            agent_n: 10,
            agent_speed: 1.0,
            agent_turn: 35.0,
            // Spawn Settings
            // const AGENT_SPAWN: usize = 1;
            // const SPAWN_SIZE: f64 = 100.0;
            // Sensor Settings
            sensor_angle: 35.0,
            sensor_distance: 3.5,
            sensor_size: 1,
            // Trail Settings
            trail_weight: 255.0,
            trail_decay: 1.8,
            trail_diffuse: 0.07,
            // Core var
            rgb_buffer: [0; 3 * MAX_X * MAX_Y],
            trail_map: [[0.0; MAX_Y]; MAX_X],
            agents: arr![Agent::new(320, 180); 500],
            running: true,
        }
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            ui.label("Simulation Settings");
            ui.add(egui::Slider::new(&mut self.size_x, 1..=MAX_X).text("size_x"));
            ui.add(egui::Slider::new(&mut self.size_y, 1..=MAX_Y).text("size_y"));
            if self.running {
                if ui.add(egui::Button::new("Pause")).clicked() { self.running = false };
            } else {
                if ui.add(egui::Button::new("Run")).clicked() { self.running = true };
            }
            if ui.add(egui::Button::new("Reset")).clicked() { () }; //TODO
            ui.separator();
            ui.label("Agents Settings");
            ui.add(egui::Slider::new(&mut self.agent_n, 1..=MAX_AGENT).text("agent_n"));
            ui.add(egui::Slider::new(&mut self.agent_speed, 0.0..=3.0).text("agent_speed"));
            ui.add(egui::Slider::new(&mut self.agent_turn, 0.0..=360.0).text("agent_turn"));
            if ui.add(egui::Button::new("Default")).clicked() { () }; //TODO
            ui.separator();
            ui.label("Spawn Settings");
            ui.label("TODO");
            ui.separator();
            ui.label("Sensor Settings");
            ui.add(egui::Slider::new(&mut self.sensor_angle, 0.0..=360.0).text("sensor_angle"));
            ui.add(egui::Slider::new(&mut self.sensor_distance, 0.0..=10.0).text("sensor_distance"));
            ui.add(egui::Slider::new(&mut self.sensor_size, 0..=5).text("sensor_size"));
            if ui.add(egui::Button::new("Default")).clicked() { () }; //TODO
            ui.separator();
            ui.label("Trail Settings");
            ui.add(egui::Slider::new(&mut self.trail_weight, 0.0..=360.0).text("trail_weight"));
            ui.add(egui::Slider::new(&mut self.trail_decay, 0.0..=10.0).text("trail_decay"));
            ui.add(egui::Slider::new(&mut self.trail_diffuse, 0.0..=1.0).text("trail_diffuse"));
            if ui.add(egui::Button::new("Default")).clicked() { () }; //TODO
            ui.separator();
            if self.running { ui.add(egui::Spinner::new()); };
        });
    }

    fn central_panel(&self, ctx: &egui::Context) {
        let image = egui::ColorImage::from_rgb([self.size_x, self.size_y], &(self.rgb_buffer[0..3*(self.size_x*self.size_y)]));
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = ui.ctx().load_texture(
                    "image",
                    egui::ImageData::Color(image),
                    Default::default(),
                );
            let size = texture.size_vec2();
            ui.image(&texture, size);
            });
    }

    fn update_agents(&mut self) {
        let mut rng = rand::thread_rng();
        for agent in &mut self.agents[0..self.agent_n] {
            // Sense
            let weight_forward = sense(self.trail_map, &agent, 0.0, self.sensor_distance, self.sensor_size);
            let weight_left = sense(self.trail_map, &agent, self.sensor_angle, self.sensor_distance, self.sensor_size);
            let weight_right = sense(self.trail_map, &agent, -self.sensor_angle, self.sensor_distance, self.sensor_size);
            let random_steer_strength = rng.gen::<f64>();

            // Rotate
            // Keep forward
            if weight_forward > weight_left && weight_forward > weight_right {
                ();
            }
            // Random turn
            else if weight_forward < weight_left && weight_forward < weight_right {
                agent.angle += (random_steer_strength - 0.5) * 2.0 * self.agent_turn * 3.14159 / 180_f64;
            }
            // Turn right
            else if weight_right > weight_left {
                agent.angle -= random_steer_strength * self.agent_turn * 3.14159 / 180_f64;
            }
            // Turn left
            else if weight_left > weight_right {
                agent.angle += random_steer_strength * self.agent_turn * 3.14159 / 180_f64;
            }

            // Move
            let (x, y) = (agent.angle.cos() * self.agent_speed, agent.angle.sin() * self.agent_speed);
            agent.pos_x += x;
            agent.pos_y += y;

            // Check Collision
            if agent.pos_x < 0.0 || agent.pos_x >= self.size_x as f64 || agent.pos_y < 0.0 || agent.pos_y >= self.size_y as f64 {
                agent.pos_x = ((self.size_x-1) as f64).min(0_f64.max(agent.pos_x));
                agent.pos_y = ((self.size_y-1) as f64).min(0_f64.max(agent.pos_y));
                agent.angle = rng.gen::<f64>() * 2.0 * 3.14159;
            }
        }
    }

    fn draw_agents(&mut self) {
        for agent in &self.agents[0..self.agent_n] {
            let x = agent.pos_x.floor() as usize;
            let y = agent.pos_y.floor() as usize;
            self.trail_map[x][y] = self.trail_weight;
        }
    }

    fn diffuse(&mut self) {
        let source = self.trail_map.clone();

        for x in 0..MAX_X {
        for y in 0..MAX_Y {
            // Diffuse
            let mut sum = 0.0;
            for offset_x in [-1, 0, 1] {
            for offset_y in [-1, 0, 1] {
                let pick_x = min(max(x as isize +offset_x, 0), MAX_X as isize -1) as usize;
                let pick_y = min(max(y as isize +offset_y, 0), MAX_Y as isize -1) as usize;
                sum += source[pick_x][pick_y];
            }}
            sum = sum / 9.0;
            self.trail_map[x][y] = self.trail_map[x][y] * (1.0 - self.trail_diffuse) + sum * self.trail_diffuse;

            // Decay
            self.trail_map[x][y] = 0_f64.max(self.trail_map[x][y] - self.trail_decay);
        }}
    }

    fn draw_map(&mut self){
        for (x, row) in self.trail_map[0..self.size_x].iter().enumerate() {
            for (y, value) in row[0..self.size_y].iter().enumerate() {
                self.rgb_buffer[(x+self.size_x*y)*3] = *value as u8;
                self.rgb_buffer[(x+self.size_x*y)*3+1] = *value as u8;
                self.rgb_buffer[(x+self.size_x*y)*3+2] = *value as u8;
            }
        }
    }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            self.update_agents();
            self.draw_agents();
            self.diffuse();
        }
        self.draw_map();
        self.left_panel(ctx);
        self.central_panel(ctx);
   }
}

fn sense(trail_map: [[f64; MAX_Y]; MAX_X], agent: &Agent, angle_offset: f64, distance_offset: f64, size: usize) -> f64 {
    let angle = agent.angle + angle_offset * 3.14159 / 180_f64;
    let (x, y) = (agent.pos_x + distance_offset * angle.cos(), agent.pos_y + distance_offset * angle.sin());
    let mut sum = 0.0;

    for offset_x in -(size as isize)..size as isize {
        for offset_y in -(size as isize)..size as isize {
            let pick_x = min(max(x.round() as isize + offset_x, 0), MAX_X as isize -1) as usize;
            let pick_y = min(max(y.round() as isize + offset_y, 0), MAX_Y as isize -1) as usize;
            sum += trail_map[pick_x][pick_y];
        }
    }
    sum
}
