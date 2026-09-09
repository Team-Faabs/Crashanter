use core_dump::{protocol::robot_command_wire::RobotCommandWire, vec::types::{Vec2, Vec3}};




pub struct DataFrame {
  pub sensors: Sensors,
  pub commands: RobotCommandWire,
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



pub struct Sensors {
  pub imu: ImuData,
  pub compass: Option<Vec3<f32>>,
  pub encoder: Encoder,
  pub camera: Camera,
}

pub struct ImuData {
  pub a: Imu,
  pub b: Imu,
}

pub struct Imu {
  pub acc: Vec3<f32>,
  pub angular: Vec3<f32>,
}

pub struct Encoder {
  
}

pub struct Camera {
  
}