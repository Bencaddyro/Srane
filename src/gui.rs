use egui::ColorImage;

use crate::{
    config::{
        Settings, MAX_AGENT_N, MAX_AGENT_SPEED, MAX_AGENT_TURN, MAX_SENSOR_ANGLE,
        MAX_SENSOR_DISTANCE, MAX_SENSOR_SIZE, MAX_SIZE_X, MAX_SIZE_Y, MAX_TRAIL_DECAY,
        MAX_TRAIL_DIFFUSE, MAX_TRAIL_WEIGHT,
    },
    simulation::{
        agents_move, agents_sense_rotate, map_deposit, map_diffuse_decay, Agent, Agents, TrailMap,
    },
};

pub struct MyEguiApp {
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

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::default();
        let mut agents = Vec::new();
        agents.resize_with(MAX_AGENT_N, || Agent::new(settings.size_x, settings.size_y));
        MyEguiApp {
            settings,
            textury: None,
            image: ColorImage::new([MAX_SIZE_X, MAX_SIZE_Y], egui::Color32::DARK_GRAY),
            trail_map: vec![0.0; MAX_SIZE_X * MAX_SIZE_Y],
            agents,
            running: true,
        }
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            ui.label("Simulation Settings");
            ui.add(egui::Slider::new(&mut self.settings.size_x, 1..=MAX_SIZE_X).text("size_x"));
            ui.add(egui::Slider::new(&mut self.settings.size_y, 1..=MAX_SIZE_Y).text("size_y"));
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
            ui.add(egui::Slider::new(&mut self.settings.agent_n, 1..=MAX_AGENT_N).text("agent_n"));
            ui.add(
                egui::Slider::new(&mut self.settings.agent_speed, 0.0..=MAX_AGENT_SPEED)
                    .text("agent_speed"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.agent_turn, 0.0..=MAX_AGENT_TURN)
                    .text("agent_turn"),
            );
            if ui.add(egui::Button::new("Default")).clicked() {
                self.settings.default_agents()
            };
            if ui.add(egui::Button::new("Reset Agent")).clicked() {
                let mut agents = Vec::new();
                agents.resize_with(MAX_AGENT_N, || {
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
                egui::Slider::new(&mut self.settings.sensor_angle, 0.0..=MAX_SENSOR_ANGLE)
                    .text("sensor_angle"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.settings.sensor_distance,
                    0.0..=MAX_SENSOR_DISTANCE,
                )
                .text("sensor_distance"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.sensor_size, 0..=MAX_SENSOR_SIZE)
                    .text("sensor_size"),
            );
            if ui.add(egui::Button::new("Default")).clicked() {
                self.settings.default_sensor()
            };
            ui.separator();
            ui.label("Trail Settings");
            ui.add(
                egui::Slider::new(&mut self.settings.trail_weight, 0.0..=MAX_TRAIL_WEIGHT)
                    .text("trail_weight"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.trail_decay, 0.0..=MAX_TRAIL_DECAY)
                    .text("trail_decay"),
            );
            ui.add(
                egui::Slider::new(&mut self.settings.trail_diffuse, 0.0..=MAX_TRAIL_DIFFUSE)
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

        for y in 0..self.settings.size_y {
            for x in 0..self.settings.size_x {
                let value = self.trail_map[x + MAX_SIZE_X * y];
                current[(x + MAX_SIZE_X * y) * 4..(x + MAX_SIZE_X * y) * 4 + 3]
                    .copy_from_slice(&[value as u8; 3]);
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
