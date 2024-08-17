use core::f64::consts::PI;
use rand::Rng;
use std::cmp::{max, min};

use crate::{MAX_X, MAX_Y};

#[derive(Clone, Debug)]
pub struct Agent {
    pos_x: f64,
    pos_y: f64,
    angle: f64,
}
impl Agent {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            pos_x: rng.gen::<f64>() * size_x as f64,
            pos_y: rng.gen::<f64>() * size_y as f64,
            angle: rng.gen::<f64>() * 2.0 * PI,
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
// fn update_agents(&mut self) {
//         let mut rng = rand::thread_rng();
//         for agent in &mut self.agents[0..self.agent_n] {
//             // Sense
//             let weight_forward = sense(self.trail_map, &agent, 0.0, self.sensor_distance, self.sensor_size);
//             let weight_left = sense(self.trail_map, &agent, self.sensor_angle, self.sensor_distance, self.sensor_size);
//             let weight_right = sense(self.trail_map, &agent, -self.sensor_angle, self.sensor_distance, self.sensor_size);
//             let random_steer_strength = rng.gen::<f64>();
//
//             // Rotate
//             // Keep forward
//             if weight_forward > weight_left && weight_forward > weight_right {
//                 ();
//             }
//             // Random turn
//             else if weight_forward < weight_left && weight_forward < weight_right {
//                 agent.angle += (random_steer_strength - 0.5) * 2.0 * self.agent_turn * 3.14159 / 180_f64;
//             }
//             // Turn right
//             else if weight_right > weight_left {
//                 agent.angle -= random_steer_strength * self.agent_turn * 3.14159 / 180_f64;
//             }
//             // Turn left
//             else if weight_left > weight_right {
//                 agent.angle += random_steer_strength * self.agent_turn * 3.14159 / 180_f64;
//             }
//
//             // Move
//             let (x, y) = (agent.angle.cos() * self.agent_speed, agent.angle.sin() * self.agent_speed);
//             agent.pos_x += x;
//             agent.pos_y += y;
//
//             // Check Collision
//             if agent.pos_x < 0.0 || agent.pos_x >= self.size_x as f64 || agent.pos_y < 0.0 || agent.pos_y >= self.size_y as f64 {
//                 agent.pos_x = ((self.size_x-1) as f64).min(0_f64.max(agent.pos_x));
//                 agent.pos_y = ((self.size_y-1) as f64).min(0_f64.max(agent.pos_y));
//                 agent.angle = rng.gen::<f64>() * 2.0 * 3.14159;
//             }
//         }
//     }
//
//     fn draw_agents(&mut self) {
//         for agent in &self.agents[0..self.agent_n] {
//             let x = agent.pos_x.floor() as usize;
//             let y = agent.pos_y.floor() as usize;
//             self.trail_map[x][y] = self.trail_weight;
//         }
//     }
//
//     fn diffuse(&mut self) {
//         let source = self.trail_map.clone();
//
//         for x in 0..MAX_X {
//         for y in 0..MAX_Y {
//             // Diffuse
//             let mut sum = 0.0;
//             for offset_x in [-1, 0, 1] {
//             for offset_y in [-1, 0, 1] {
//                 let pick_x = min(max(x as isize +offset_x, 0), MAX_X as isize -1) as usize;
//                 let pick_y = min(max(y as isize +offset_y, 0), MAX_Y as isize -1) as usize;
//                 sum += source[pick_x][pick_y];
//             }}
//             sum = sum / 9.0;
//             self.trail_map[x][y] = self.trail_map[x][y] * (1.0 - self.trail_diffuse) + sum * self.trail_diffuse;
//
//             // Decay
//             self.trail_map[x][y] = 0_f64.max(self.trail_map[x][y] - self.trail_decay);
//         }}
//     }

fn sense(
    trail_map: [[f64; MAX_Y]; MAX_X],
    agent: &Agent,
    angle_offset: f64,
    distance_offset: f64,
    size: usize,
) -> f64 {
    let angle = agent.angle + angle_offset * PI / 180_f64;
    let (x, y) = (
        agent.pos_x + distance_offset * angle.cos(),
        agent.pos_y + distance_offset * angle.sin(),
    );
    let mut sum = 0.0;

    for offset_x in -(size as isize)..size as isize {
        for offset_y in -(size as isize)..size as isize {
            let pick_x = min(max(x.round() as isize + offset_x, 0), MAX_X as isize - 1) as usize;
            let pick_y = min(max(y.round() as isize + offset_y, 0), MAX_Y as isize - 1) as usize;
            sum += trail_map[pick_x][pick_y];
        }
    }
    sum
}
