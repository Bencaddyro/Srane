use core::f64::consts::PI;
use ocl::OclPrm;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use tracing::debug;

use crate::config::{Settings, MAX_SIZE_X, MAX_SIZE_Y};

#[derive(Clone, Debug, PartialEq, Default, Copy)]
pub struct Agent {
    pos_x: f64,
    pos_y: f64,
    angle: f64,
}



unsafe impl OclPrm for Agent {

}

pub type Agents = Vec<Agent>;
pub type TrailMap = Vec<f64>;

impl Agent {
    pub fn new(size_x: u32, size_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            pos_x: rng.gen::<f64>() * size_x as f64,
            pos_y: rng.gen::<f64>() * size_y as f64,
            angle: rng.gen::<f64>() * 2_f64 * PI,
        }
    }

    pub fn new_circle(settings: &Settings) -> Agent {
        let mut rng = rand::thread_rng();
        let angle = rng.gen::<f64>() * 2_f64 * PI;
        let radius = rng.gen::<f64>() * settings.spawn_radius;

        let pos_x = settings.size_x as f64 / 2_f64 + angle.cos() * radius;
        let pos_y = settings.size_y as f64 / 2_f64 + angle.sin() * radius;

        Agent {
            pos_x,
            pos_y,
            angle: angle + PI,
        }
    }

    pub fn new_star(settings: &Settings) -> Agent {
        let mut rng = rand::thread_rng();
        let angle = rng.gen::<f64>() * 2_f64 * PI;
        let pos_x = settings.size_x as f64 / 2_f64;
        let pos_y = settings.size_y as f64 / 2_f64;

        Agent {
            pos_x,
            pos_y,
            angle,
        }
    }
}



fn agent_sense(trail_map: &TrailMap, agent: &Agent, sensor_angle: f64, settings: &Settings) -> f64 {
    let angle = agent.angle + sensor_angle.to_radians();
    let (x, y) = (
        agent.pos_x + settings.sensor_distance * angle.cos(),
        agent.pos_y + settings.sensor_distance * angle.sin(),
    );
    let mut sum = 0.0;

    for offset_x in -(settings.sensor_size as isize)..settings.sensor_size as isize {
        for offset_y in -(settings.sensor_size as isize)..settings.sensor_size as isize {
            let pick_x = min(
                max(x.round() as isize + offset_x, 0),
                MAX_SIZE_X as isize - 1,
            ) as usize;
            let pick_y = min(
                max(y.round() as isize + offset_y, 0),
                MAX_SIZE_Y as isize - 1,
            ) as usize;
            sum += trail_map[pick_x + MAX_SIZE_X as usize * pick_y];
        }
    }
    sum
}

/// Step 1&2: Sense & Rotate
pub fn cpu_sense_rotate(trail_map: &TrailMap, agents: &mut Agents, settings: &Settings) {
    let mut rng = thread_rng();

    for agent in &mut agents[0..settings.agent_n as usize] {
        // Sense
        let (weight_forward, weight_left, weight_right) = (
            agent_sense(trail_map, agent, 0.0, settings),
            agent_sense(trail_map, agent, settings.sensor_angle, settings),
            agent_sense(trail_map, agent, -settings.sensor_angle, settings),
        );
        let random_steer_strength = rng.gen::<f64>();

        // Rotate
        // Keep forward
        if weight_forward > weight_left && weight_forward > weight_right {
            // Nothing to do
        }
        // Random turn
        else if weight_forward < weight_left && weight_forward < weight_right {
            agent.angle +=
                ((random_steer_strength - 0.5) * 2_f64 * settings.agent_turn).to_radians();
        }
        // Turn right
        else if weight_right > weight_left {
            agent.angle -= (random_steer_strength * settings.agent_turn).to_radians();
        }
        // Turn left
        else if weight_left > weight_right {
            agent.angle += (random_steer_strength * settings.agent_turn).to_radians();
        }
    }
}

/// Step 3: Move
pub fn cpu_move(agents: &mut Agents, settings: &Settings) {
    let mut rng = thread_rng();
    for agent in &mut agents[0..settings.agent_n as usize] {
        agent.pos_x += agent.angle.cos() * settings.agent_speed;
        agent.pos_y += agent.angle.sin() * settings.agent_speed;

        // Check Collision
        if agent.pos_x < 0.0
            || agent.pos_x >= settings.size_x as f64
            || agent.pos_y < 0.0
            || agent.pos_y >= settings.size_y as f64
        {
            debug!(
                "Bonk {} {} [{} ; {}]",
                agent.pos_x >= settings.size_x as f64,
                agent.pos_y >= settings.size_y as f64,
                agent.pos_x,
                agent.pos_y
            );
            agent.pos_x = agent.pos_x.max(0_f64).min(settings.size_x as f64 - 1_f64);
            agent.pos_y = agent.pos_y.max(0_f64).min(settings.size_y as f64 - 1_f64);
            agent.angle = rng.gen::<f64>() * 2_f64 * PI;
            debug!("Corrected [{} ; {}]", agent.pos_x, agent.pos_y);
        }
    }
}

/// Step 4: Deposit
pub fn cpu_deposit(agents: &Agents, trail_map: &mut TrailMap, settings: &Settings) {
    for agent in &agents[0..settings.agent_n as usize] {
        let x = agent.pos_x.floor() as usize;
        let y = agent.pos_y.floor() as usize;
        trail_map[x + MAX_SIZE_X as usize * y] = settings.trail_weight;
    }
}

/// Step 5&6: Diffuse & Decay
pub fn cpu_diffuse_decay(trail_map: &mut TrailMap, settings: &Settings) {
    let source = trail_map.clone();
    for y in 0..settings.size_y {
        for x in 0..settings.size_x {
            // Diffuse
            let mut sum = 0.0;
            for offset_x in [-1, 0, 1] {
                for offset_y in [-1, 0, 1] {
                    let pick_x =
                        min(max(x as isize + offset_x, 0), MAX_SIZE_X as isize - 1) as usize;
                    let pick_y =
                        min(max(y as isize + offset_y, 0), MAX_SIZE_Y as isize - 1) as usize;
                    sum += source[pick_x + MAX_SIZE_X as usize * pick_y];
                }
            }
            sum /= 9.0;
            trail_map[(x + MAX_SIZE_X * y) as usize] *= 1.0 - settings.trail_diffuse;
            trail_map[(x + MAX_SIZE_X * y) as usize] += sum * settings.trail_diffuse;

            // Decay
            trail_map[(x + MAX_SIZE_X * y) as usize] =
                0_f64.max(trail_map[(x + MAX_SIZE_X * y) as usize] - settings.trail_decay);
        }
    }
}
