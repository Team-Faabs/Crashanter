use core_dump::vec::types::{Vec2, Vec3};


pub struct DataFrame {
  pub sensors: Sensors,
  pub commands: RobotCommand,
  pub odometry: OdometryInput,
}

pub struct OdometryInput {
  pub radio: RobotOdometry,
  pub vision: Option<RobotOdometry>,
}

pub struct RobotOdometry {
  pub position: Vec2<f32>,
  pub orientation: Option<i16>, // might be none for the Radio as it might not be able to sense AoA

  pub velocity: Option<Vec2<f32>>,
  pub angular_velocity: Option<f32>,
}

pub struct RobotCommand {
  pub cmd: DriveCommand,
  pub kick_speed: u8,
  pub dribbler_speed: u8,
  pub flags: u8,
}

pub enum DriveCommand {
  Velocity {
    vel: Vec2<f32>,
    drive_dir: f32,
    omega: f32,
  },
  Position {
    pos: Vec2<f32>,
    drive_dir: f32,
    omega: f32,
  }
}

pub struct Stamped<T> {
  pub timestamp_us: u64,
  pub value: T,
}



pub struct Sensors {
  pub imu: Stamped<Imu>,
  pub compass: Option<Vec3<f32>>,
  pub encoder: DriveEncoder,
  pub camera: Camera,
  pub kicker: KickerStatus,
}

pub struct Imu {
  pub acc: Vec3<f32>,
  pub angular: Vec3<f32>,
}

pub struct DriveEncoder {
  pub fl: Encoder,
  pub fr: Encoder,
  pub bl: Encoder,
  pub br: Encoder,
}

pub struct Encoder {
  pub rpm: f32,
  pub ticks: u32,
  pub forward_ticks: i32,
}

pub enum KickerStatus {
  Ready,
  Error,
  Charging,
  Overvoltage,
}

pub struct Camera {
  
}