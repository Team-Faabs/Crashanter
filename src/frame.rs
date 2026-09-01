use core_dump::{protocol::robot_command_wire::RobotCommandWire, vec::types::{Vec2, Vec3}};




pub struct DataFrame {
  pub sensors: Sensors,
  pub commands: RobotCommandWire,
}

pub struct Sensors {
  pub imu: ImuData,
  pub compass: Option<Vec3<f64>>,
  pub encoder: Encoder,
  pub camera: Camera,
}

pub struct ImuData {
  pub a: Imu,
  pub b: Imu,
}

pub struct Imu {
  pub acc: Vec3<f64>,
  pub angular: Vec3<f64>,
}

pub struct Encoder {
  
}

pub struct Camera {
  
}