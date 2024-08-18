use core::f64::consts::PI;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use tracing::debug;

use crate::config::{Settings, MAX_SIZE_X, MAX_SIZE_Y};

#[derive(Clone, Debug)]
pub struct Agent {
    pos_x: f64,
    pos_y: f64,
    angle: f64,
}

pub type Agents = Vec<Agent>;
pub type TrailMap = Vec<Vec<f64>>;

impl Agent {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            pos_x: rng.gen::<f64>() * size_x as f64,
            pos_y: rng.gen::<f64>() * size_y as f64,
            angle: rng.gen::<f64>() * 2_f64 * PI,
        }
    }
    /* Spawn
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
            sum += trail_map[pick_y][pick_x];
        }
    }
    sum
}

/// Step 1&2: Sense & Rotate
pub fn agents_sense_rotate(trail_map: &TrailMap, agents: &mut Agents, settings: &Settings) {
    let mut rng = thread_rng();

    for agent in &mut agents[0..settings.agent_n] {
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
pub fn agents_move(agents: &mut Agents, settings: &Settings) {
    let mut rng = thread_rng();
    for agent in &mut agents[0..settings.agent_n] {
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
pub fn map_deposit(agents: &Agents, trail_map: &mut TrailMap, settings: &Settings) {
    for agent in &agents[0..settings.agent_n] {
        let x = agent.pos_x.floor() as usize;
        let y = agent.pos_y.floor() as usize;
        trail_map[y][x] = settings.trail_weight;
    }
}

/// Step 5&6: Diffuse & Decay
pub fn map_diffuse_decay(trail_map: &mut TrailMap, settings: &Settings) {
    let source = trail_map.clone();
    for (y, row) in trail_map.iter_mut().enumerate().take(settings.size_y) {
        for (x, trail_xy) in row.iter_mut().enumerate().take(settings.size_x) {
            // Diffuse
            let mut sum = 0.0;
            for offset_x in [-1, 0, 1] {
                for offset_y in [-1, 0, 1] {
                    let pick_x =
                        min(max(x as isize + offset_x, 0), MAX_SIZE_X as isize - 1) as usize;
                    let pick_y =
                        min(max(y as isize + offset_y, 0), MAX_SIZE_Y as isize - 1) as usize;
                    sum += source[pick_y][pick_x];
                }
            }
            sum /= 9.0;
            *trail_xy *= 1.0 - settings.trail_diffuse;
            *trail_xy += sum * settings.trail_diffuse;

            // Decay
            *trail_xy = 0_f64.max(*trail_xy - settings.trail_decay);
        }
    }
}
