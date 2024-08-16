// use std::cmp::{min,max};
// use image::{RgbImage, ImageBuffer, Rgb};
// use rand::Rng;
//use std::{thread, time};
use eframe::egui;

mod simulation;

use simulation::Agent;

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;
const MAX_AGENT: usize = 1000;


struct Settings {
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
}

impl Settings {
    fn new() -> Self {
        Settings {
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
        }
    }
}


struct MyEguiApp {
    // Simulation settings
    settings: Settings,

    // Buffer var
    image: egui::ColorImage,
    trail_map: Vec<Vec<f64>>,
    agents: Vec<Agent>,

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
            settings: Settings::new(),
            // Core var
            image: egui::ColorImage::new([500,400], egui::Color32::DARK_GRAY),
            trail_map: vec![ vec![ 0.0 ; MAX_Y] ; MAX_X ],
            agents: vec![ Agent::new(320, 180) ; MAX_AGENT],
            running: true,
        }
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            ui.label("Simulation Settings");
            ui.add(egui::Slider::new(&mut self.settings.size_x, 1..=MAX_X).text("size_x"));
            ui.add(egui::Slider::new(&mut self.settings.size_y, 1..=MAX_Y).text("size_y"));
            if self.running {
                if ui.add(egui::Button::new("Pause")).clicked() { self.running = false };
            } else {
                if ui.add(egui::Button::new("Run")).clicked() { self.running = true };
            }
            if ui.add(egui::Button::new("Reset")).clicked() { () }; //TODO
            ui.separator();
            ui.label("Agents Settings");
            ui.add(egui::Slider::new(&mut self.settings.agent_n, 1..=MAX_AGENT).text("agent_n"));
            ui.add(egui::Slider::new(&mut self.settings.agent_speed, 0.0..=3.0).text("agent_speed"));
            ui.add(egui::Slider::new(&mut self.settings.agent_turn, 0.0..=360.0).text("agent_turn"));
            if ui.add(egui::Button::new("Default")).clicked() { () }; //TODO
            ui.separator();
            ui.label("Spawn Settings");
            ui.label("TODO");
            ui.separator();
            ui.label("Sensor Settings");
            ui.add(egui::Slider::new(&mut self.settings.sensor_angle, 0.0..=360.0).text("sensor_angle"));
            ui.add(egui::Slider::new(&mut self.settings.sensor_distance, 0.0..=10.0).text("sensor_distance"));
            ui.add(egui::Slider::new(&mut self.settings.sensor_size, 0..=5).text("sensor_size"));
            if ui.add(egui::Button::new("Default")).clicked() { () }; //TODO
            ui.separator();
            ui.label("Trail Settings");
            ui.add(egui::Slider::new(&mut self.settings.trail_weight, 0.0..=360.0).text("trail_weight"));
            ui.add(egui::Slider::new(&mut self.settings.trail_decay, 0.0..=10.0).text("trail_decay"));
            ui.add(egui::Slider::new(&mut self.settings.trail_diffuse, 0.0..=1.0).text("trail_diffuse"));
            if ui.add(egui::Button::new("Default")).clicked() { () }; //TODO
            ui.separator();
            if self.running { ui.add(egui::Spinner::new()); };
        });
    }

    fn central_panel(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = ui.ctx().load_texture(
                    "image",
                    egui::ImageData::Color(self.image.clone()),
                    Default::default(),
                );
            let size = texture.size_vec2();
            ui.image(&texture, size);
            });
    }



    fn draw_map(&mut self){
        let mut buffer = [0; 3 * MAX_X * MAX_Y ];

        for (x, row) in self.trail_map[0..self.settings.size_x].iter().enumerate() {
            for (y, value) in row[0..self.settings.size_y].iter().enumerate() {
                buffer[(x+self.settings.size_x*y)*3] = *value as u8;
                buffer[(x+self.settings.size_x*y)*3+1] = *value as u8;
                buffer[(x+self.settings.size_x*y)*3+2] = *value as u8;
            }
        }
        self.image = egui::ColorImage::from_rgb([self.settings.size_x, self.settings.size_y], &(buffer[0..3*(self.settings.size_x*self.settings.size_y)]));

    }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            // TODO
            // Update internal buffer by calling external function ! (soon gpu one)
        }
        self.draw_map();
        self.left_panel(ctx);
        self.central_panel(ctx);
   }
}


