# TODO List
- [x] Teensy Reconnect
- [ ] CP Reconnect
- [x] Self.Velo (Vec2U)
- [x] Self.Orientation (Degree)


- [ ] Logic


- [ ] Orca tuning



- vIMU
- motor PID / drive response
- Sensors
  - IMU
  - Gyro
  - Magnetometer => see with motors, recalibrate, fall back to gyro
  - Battery
  - (UWB + AoA?)
  - Encoder
- kicker
- dribbler
- Sensor fusion / EKF / KF

- Camera Ball detection
- stabilization / active tipping detection & prevention
- pattern detection board

- Recv Packets
  - WiFi / SDR balancing


Position senses:
- Vision @ 60-120Hz
- UWB @ 500-1000Hz
- Accelerometer @ 1000Hz
- Encoder @ ?

Angle senses:
- Vision @ 60-120Hz
- UWB / AoA @ 500-1000Hz
- Gyro @ 1000Hz
- Magnetometer @ 100Hz
- Encoder @ ?