use eframe::egui;
use egui::ColorImage;

mod simulation;
use simulation::{
    agents_move, agents_sense_rotate, map_deposit, map_diffuse_decay, Agent, Agents, TrailMap,
};

const MAX_X: usize = 1920;
const MAX_Y: usize = 1080;
const MAX_AGENT: usize = 9999;

/// Default settings
const SIZE_X: usize = 512;
const SIZE_Y: usize = 512;
const AGENT_N: usize = 6000;
const AGENT_SPEED: f64 = 1_f64;
const AGENT_TURN: f64 = 35_f64;
const SENSOR_ANGLE: f64 = 35_f64;
const SENSOR_DISTANCE: f64 = 3.5;
const SENSOR_SIZE: usize = 1;
const TRAIL_WEIGHT: f64 = 255_f64;
const TRAIL_DECAY: f64 = 1.8;
const TRAIL_DIFFUSE: f64 = 0.07;

struct Settings {
    /// Simulations settings
    size_x: usize,
    size_y: usize,
    /// Agents settings
    agent_n: usize,
    agent_speed: f64,
    agent_turn: f64,
    // Sensor Settings
    sensor_angle: f64,
    sensor_distance: f64,
    sensor_size: usize,
    // Trail Settings
    trail_weight: f64,
    trail_decay: f64,
    trail_diffuse: f64,
}

impl Settings {
    fn default_agents(&mut self) {
        self.agent_n = AGENT_N;
        self.agent_speed = AGENT_SPEED;
        self.agent_turn = AGENT_TURN;
    }
    fn default_sensor(&mut self) {
        self.sensor_angle = SENSOR_ANGLE;
        self.sensor_distance = SENSOR_DISTANCE;
        self.sensor_size = SENSOR_SIZE;
    }
    fn default_trail(&mut self) {
        self.trail_weight = TRAIL_WEIGHT;
        self.trail_decay = TRAIL_DECAY;
        self.trail_diffuse = TRAIL_DIFFUSE;
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size_x: SIZE_X,
            size_y: SIZE_Y,
            agent_n: AGENT_N,
            agent_speed: AGENT_SPEED,
            agent_turn: AGENT_TURN,
            sensor_angle: SENSOR_ANGLE,
            sensor_distance: SENSOR_DISTANCE,
            sensor_size: SENSOR_SIZE,
            trail_weight: TRAIL_WEIGHT,
            trail_decay: TRAIL_DECAY,
            trail_diffuse: TRAIL_DIFFUSE,
        }
    }
}

struct MyEguiApp {
    // Simulation settings
    settings: Settings,
    // Buffer var
    textury: Option<egui::TextureHandle>,
    image: ColorImage,
    trail_map: TrailMap,
    agents: Agents,
    // State var
    running: bool,
}

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Srane Render",
        options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::default();
        let mut agents = Vec::new();
        agents.resize_with(MAX_AGENT, || Agent::new(settings.size_x, settings.size_y));
        MyEguiApp {
            settings,
            textury: None,
            image: ColorImage::new([MAX_X, MAX_Y], egui::Color32::DARK_GRAY),
            trail_map: vec![vec![0.0; MAX_X]; MAX_Y],
            agents,
            running: true,
        }
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            ui.label("Simulation Settings");
            ui.add(egui::Slider::new(&mut self.settings.size_x, 1..=MAX_X).text("size_x"));
            ui.add(egui::Slider::new(&mut self.settings.size_y, 1..=MAX_Y).text("size_y"));
            if self.running {
                if ui.add(egui::Button::new("Pause")).clicked() {
                    self.running = false
                };
            } else if ui.add(egui::Button::new("Run")).clicked() {
                self.running = true
            };
            if ui.add(egui::Button::new("Reset")).clicked() {
                self.settings = Settings::default()
            };
            ui.separator();
            ui.label("Agents Settings");
            ui.add(egui::Slider::new(&mut self.settings.agent_n, 1..=MAX_AGENT).text("agent_n"));
            ui.add(
                egui::Slider::new(&mut self.settings.agent_speed, 0.0..=3.0).text("agent_speed"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.agent_turn, 0.0..=360.0).text("agent_turn"),
            );
            if ui.add(egui::Button::new("Default")).clicked() {
                self.settings.default_agents()
            };
            if ui.add(egui::Button::new("Reset Agent")).clicked() {
                let mut agents = Vec::new();
                agents.resize_with(MAX_AGENT, || {
                    Agent::new(self.settings.size_x, self.settings.size_y)
                });
                self.agents = agents;
            };
            // ui.separator();
            // ui.label("Spawn Settings");
            // ui.label("TODO");
            ui.separator();
            ui.label("Sensor Settings");
            ui.add(
                egui::Slider::new(&mut self.settings.sensor_angle, 0.0..=360.0)
                    .text("sensor_angle"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.sensor_distance, 0.0..=10.0)
                    .text("sensor_distance"),
            );
            ui.add(egui::Slider::new(&mut self.settings.sensor_size, 0..=5).text("sensor_size"));
            if ui.add(egui::Button::new("Default")).clicked() {
                self.settings.default_sensor()
            };
            ui.separator();
            ui.label("Trail Settings");
            ui.add(
                egui::Slider::new(&mut self.settings.trail_weight, 0.0..=360.0)
                    .text("trail_weight"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.trail_decay, 0.0..=10.0).text("trail_decay"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.trail_diffuse, 0.0..=1.0)
                    .text("trail_diffuse"),
            );
            if ui.add(egui::Button::new("Default")).clicked() {
                self.settings.default_trail()
            };
            ui.separator();
            if self.running {
                ui.add(egui::Spinner::new());
            };
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture: &mut egui::TextureHandle = self.textury.get_or_insert_with(|| {
                // Load the texture only once.
                ui.ctx()
                    .load_texture("mainframe", egui::ColorImage::example(), Default::default())
            });

            texture.set(self.image.clone(), Default::default());
            ui.image((texture.id(), texture.size_vec2()));
        });
    }

    fn draw_map(&mut self) {
        let current = self.image.as_raw_mut();
        for (y, row) in self.trail_map[0..self.settings.size_y].iter().enumerate() {
            for (x, value) in row[0..self.settings.size_x].iter().enumerate() {
                current[(x + MAX_X * y) * 4..(x + MAX_X * y) * 4 + 3]
                    .copy_from_slice(&[*value as u8; 3]);
            }
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            agents_sense_rotate(&self.trail_map, &mut self.agents, &self.settings);
            agents_move(&mut self.agents, &self.settings);
            map_deposit(&self.agents, &mut self.trail_map, &self.settings);
            map_diffuse_decay(&mut self.trail_map, &self.settings);
        }
        self.draw_map();
        self.left_panel(ctx);
        self.central_panel(ctx);
    }
}
