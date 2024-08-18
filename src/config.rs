
pub const MAX_X: usize = 1920;
pub const MAX_Y: usize = 1080;
pub const MAX_AGENT: usize = 9999;

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

pub struct Settings {
    /// Simulations settings
    pub size_x: usize,
    pub size_y: usize,
    /// Agents settings
    pub agent_n: usize,
    pub agent_speed: f64,
    pub agent_turn: f64,
    // Sensor Settings
    pub sensor_angle: f64,
    pub sensor_distance: f64,
    pub sensor_size: usize,
    // Trail Settings
    pub trail_weight: f64,
    pub trail_decay: f64,
    pub trail_diffuse: f64,
}

impl Settings {
    pub fn default_agents(&mut self) {
        self.agent_n = AGENT_N;
        self.agent_speed = AGENT_SPEED;
        self.agent_turn = AGENT_TURN;
    }
    pub fn default_sensor(&mut self) {
        self.sensor_angle = SENSOR_ANGLE;
        self.sensor_distance = SENSOR_DISTANCE;
        self.sensor_size = SENSOR_SIZE;
    }
    pub fn default_trail(&mut self) {
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
