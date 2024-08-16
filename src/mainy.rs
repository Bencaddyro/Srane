use std::cmp::{min,max};
use image::{RgbImage, ImageBuffer, Rgb};
use rand::Rng;
use std::{thread, time};

#[derive(Debug)]
struct Agent {
    pos_x: f64,
    pos_y: f64,
    angle: f64,
}

// Simulation Settings


const STEP: usize = 1500;

const N_AGENT: usize = 250;
const IMAGE_X: u32 = 320;
const IMAGE_Y: u32 = 180;

// const N_AGENT: usize = 250*2;
// const IMAGE_X: u32 = 640;
// const IMAGE_Y: u32 = 360;

// Trail Settings
const TRAIL_WEIGHT: f64 = 255.0;
const DECAY_RATE: f64 = 1.8;
const DIFFUSE_RATE: f64 = 0.07;

// Agent Settings
const AGENT_SPAWN: usize = 1;
const SPAWN_SIZE: f64 = 100.0;
const MOVE_SPEED: f64 = 1.0;
const TURN_SPEED: f64 = 35.0 * 3.14159 / 180.0;

const SENSOR_ANGLE_SPACING: f64 = 35.0 * 3.14159 / 180.0;
const SENSOR_OFFSET_DST: f64 = 3.5;
const SENSOR_SIZE: isize = 1;

use eframe::egui;





fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Srane Render", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))))
}

#[derive(Default)]
struct MyEguiApp {
    size_x: usize,
    size_y: usize,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            ui.add(egui::DragValue::new(&mut self.size_x).speed(0.1));
            ui.add(egui::DragValue::new(&mut self.size_y).speed(0.1));
        });
    }

    fn central_panel(&self, ctx: &egui::Context) {

        let buffer = egui::ColorImage::new( [self.size_x, self.size_y], egui::Color32::LIGHT_GRAY );
        egui::CentralPanel::default().show(ctx, |ui| {

            let mut texture = ui.ctx().load_texture(
                    "logo",
                    egui::ImageData::Color(buffer),
                    Default::default(),
                );

            let size = texture.size_vec2();
            ui.image(&mut texture, size);

            ui.label(format!("The checkbox is {}.", self.size_x));
            });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.left_panel(ctx);
        self.central_panel(ctx);
   }
}

fn mainy() {
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_X, IMAGE_Y);
    let mut trail_map: Vec<Vec<f64>> = vec![vec![0.0; IMAGE_Y as usize];IMAGE_X as usize];
    let mut agents = init_agents();

    // Loop
    // for step in 0..STEP {
    loop { let step = 0;
        // Sense - Rotate - Move - Deposit - Diffuse - Decay
        agents = update_agents(&trail_map, agents);
        trail_map = draw_agents(trail_map, &agents);
        trail_map = diffuse(trail_map);

        buffer = draw_map(&trail_map, buffer);

        match buffer.save(format!("image_{step:04}.png")) {
            Err(e) => eprintln!("Error writing file: {}", e),
            Ok(()) => println!("Step {step}/{STEP} done !"),
        };
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn init_agents() -> Vec<Agent> {
    let mut agents: Vec<Agent> = Vec::new();
    for _ in 0..N_AGENT {
        agents.push(new_agent());
    }
    agents
}

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

fn draw_map(trail_map: &Vec<Vec<f64>>, mut buffer: RgbImage) -> RgbImage {
    for (x, row) in trail_map.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            let g = max(0,min(255, *value as u8));
            // let rb = (*value * 50.0 / 255.0) as u8;
            let pixel = Rgb([g,g,g]);
            // if println!)
            buffer.put_pixel(x as u32, y as u32, pixel);

        }
    }
    buffer
}

fn sense(trail_map: &Vec<Vec<f64>>, agent: &Agent, offset: f64) -> f64 {
    let angle = agent.angle + offset;
    let (pos_x, pos_y) = (agent.pos_x + SENSOR_OFFSET_DST * angle.cos(), agent.pos_y + SENSOR_OFFSET_DST * angle.sin());

    let x = ((IMAGE_X as f64 - 1.0).min(0_f64.max(pos_x))) as u32;
    let y = ((IMAGE_Y as f64 - 1.0).min(0_f64.max(pos_y))) as u32;

    let mut sum = 0.0;

    for offset_x in -SENSOR_SIZE..SENSOR_SIZE {
        for offset_y in -SENSOR_SIZE..SENSOR_SIZE {
            let pick_x = min(max(x as isize +offset_x, 0), IMAGE_X as isize -1) as usize;
            let pick_y = min(max(y as isize +offset_y, 0), IMAGE_Y as isize -1) as usize;
            sum += trail_map[pick_x][pick_y];
        }
    }
    sum
}

fn update_agents(trail_map: &Vec<Vec<f64>>, agents: Vec<Agent>) -> Vec<Agent> {
    let len_x = trail_map.len();
    let len_y = trail_map[0].len();
    let mut new_agents = Vec::new();
    let mut rng = rand::thread_rng();

    for agent in agents{
        let mut angle = agent.angle;

        // Sense
        let weight_forward = sense(trail_map, &agent, 0.0);
        let weight_left = sense(trail_map, &agent, SENSOR_ANGLE_SPACING);
        let weight_right = sense(trail_map, &agent, -SENSOR_ANGLE_SPACING);

        let random_steer_strength = rng.gen::<f64>();

        // Update angle

        if weight_forward > weight_left && weight_forward > weight_right {
            angle += 0.0;
        }
        else if weight_forward < weight_left && weight_forward < weight_right {
           angle += (random_steer_strength - 0.5) * 2.0 * TURN_SPEED;
        }
        // Turn right
        else if weight_right > weight_left {
            angle -= random_steer_strength * TURN_SPEED;
        }
        // Turn left
        else if weight_left > weight_right {
            angle += random_steer_strength * TURN_SPEED;
        }

        // Move
        let (x, y) = (angle.cos() * MOVE_SPEED, angle.sin() * MOVE_SPEED);
        let mut pos_x = agent.pos_x + x;
        let mut pos_y = agent.pos_y + y;

        // Check Collision
        if pos_x < 0.0 || pos_x >= len_x as f64 || pos_y < 0.0 || pos_y >= len_y as f64 {
            pos_x = (len_x as f64 - 1.0).min(0_f64.max(pos_x));
            pos_y = (len_y as f64 - 1.0).min(0_f64.max(pos_y));
            angle = rng.gen::<f64>() * 2.0 * 3.14159;
        }

        let a= Agent{pos_x, pos_y, angle};
        new_agents.push(a);
    }
    new_agents
}

fn diffuse(mut trail_map: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let source = trail_map.clone();
    let len_x = trail_map.len();
    let len_y = trail_map[0].len();

    for x in 0..len_x {
        for y in 0..len_y {
            //3x3 blur
            let mut sum = 0.0;
            for offset_x in [-1, 0, 1] {
                for offset_y in [-1, 0, 1] {
                    let pick_x = min(max(x as isize +offset_x, 0), IMAGE_X as isize -1) as usize;
                    let pick_y = min(max(y as isize +offset_y, 0), IMAGE_Y as isize -1) as usize;
                    sum += source[pick_x][pick_y];
                }
            }
            sum = sum / 9.0;
            let new_trail = trail_map[x][y] * (1.0 - DIFFUSE_RATE) + sum * DIFFUSE_RATE;

            // decay
            let diffused_trail = 0.0_f64.max(new_trail - DECAY_RATE);
            trail_map[x][y] = diffused_trail;
        }
    }
    trail_map
}

fn draw_agents(mut trail_map: Vec<Vec<f64>>, agents: &Vec<Agent>) -> Vec<Vec<f64>> {
    let len_x = trail_map.len();
    let len_y = trail_map[0].len();
    for agent in agents {
        let x = (len_x as f64 - 1.0).min(0_f64.max(agent.pos_x)) as usize;
        let y = (len_y as f64 - 1.0).min(0_f64.max(agent.pos_y)) as usize;
        trail_map[x][y] = TRAIL_WEIGHT;
    }
    trail_map
}
