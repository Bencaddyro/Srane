use egui::ColorImage;

use crate::{
    config::{
        Settings, MAX_AGENT_N, MAX_AGENT_SPEED, MAX_AGENT_TURN, MAX_SENSOR_ANGLE,
        MAX_SENSOR_DISTANCE, MAX_SENSOR_SIZE, MAX_SIZE_X, MAX_SIZE_Y, MAX_TRAIL_DECAY,
        MAX_TRAIL_DIFFUSE, MAX_TRAIL_WEIGHT,
    },
    gpu::{gpu_all, gpu_decay, gpu_diffuse, gpu_move},
    simulation::{
        cpu_deposit, cpu_diffuse_decay, cpu_move, cpu_sense_rotate, Agent, Agents, TrailMap,
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
    gpu: bool,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::default();
        let mut agents = Vec::new();
        agents.resize_with(MAX_AGENT_N as usize, || {
            Agent::new(settings.size_x, settings.size_y)
        });
        MyEguiApp {
            settings,
            textury: None,
            image: ColorImage::new(
                [MAX_SIZE_X as usize, MAX_SIZE_Y as usize],
                egui::Color32::DARK_GRAY,
            ),
            trail_map: vec![0.0; (MAX_SIZE_X * MAX_SIZE_Y) as usize],
            agents,
            running: true,
            gpu: false,
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
            ui.checkbox(&mut self.gpu, "Enable GPU render");
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

            ui.separator();
            ui.label("Spawn Settings");
            if ui.add(egui::Button::new("Random Agent")).clicked() {
                let mut agents = Vec::new();
                agents.resize_with(MAX_AGENT_N as usize, || {
                    Agent::new(self.settings.size_x, self.settings.size_y)
                });
                self.agents = agents;
            };
            ui.add(
                egui::Slider::new(&mut self.settings.spawn_radius, 0_f64..=MAX_SIZE_Y as f64)
                    .text("spawn_radius"),
            );

            if ui.add(egui::Button::new("Random Circle Agent")).clicked() {
                let mut agents = Vec::new();
                agents.resize_with(MAX_AGENT_N as usize, || Agent::new_circle(&self.settings));
                self.agents = agents;
            };
            if ui.add(egui::Button::new("Random Star Agent")).clicked() {
                let mut agents = Vec::new();
                agents.resize_with(MAX_AGENT_N as usize, || Agent::new_star(&self.settings));
                self.agents = agents;
            };
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
                let value = self.trail_map[(x + MAX_SIZE_X * y) as usize];
                current
                    [((x + MAX_SIZE_X * y) * 4) as usize..((x + MAX_SIZE_X * y) * 4 + 3) as usize]
                    .copy_from_slice(&[value as u8; 3]);
            }
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            if self.gpu {
                // cpu_sense_rotate(&self.trail_map, &mut self.agents, &self.settings);

                gpu_move(&mut self.agents, &self.settings).unwrap();

                cpu_deposit(&self.agents, &mut self.trail_map, &self.settings);

                // Diffuse & Decay
                // gpu_all(&mut self.trail_map, &self.settings).unwrap();
            } else {
                cpu_sense_rotate(&self.trail_map, &mut self.agents, &self.settings);

                cpu_move(&mut self.agents, &self.settings);

                cpu_deposit(&self.agents, &mut self.trail_map, &self.settings);

                // Diffuse
                cpu_diffuse_decay(&mut self.trail_map, &self.settings);
            }
        }

        self.draw_map();

        self.left_panel(ctx);

        self.central_panel(ctx);
    }
}
