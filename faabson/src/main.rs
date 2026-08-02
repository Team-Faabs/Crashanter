use core_dump::protocol::robot_command_frame::RobotCommandFrame;
use std::error::Error;
use std::mem::transmute;
use std::time::Duration;
use tokio::time::sleep;

#[repr(C)]
struct TeensyMsg {
  first: u8,
  vx: i16,
  vy: i16,
  omega: i16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut args = std::env::args();
  args.next(); // skip program name

  let Some(robot_id) = args.next() else {
    panic!("Missing robot ID argument");
  };

  let robot_id: u8 = robot_id.parse()?;

  let Some(addr) = args.next() else {
    panic!("Missing address argument");
  };

  let mut teensy_vid = 5824;

  if let Some(vid) = args.next() {
    let vid: u16 = vid.parse()?;
    teensy_vid = vid;
  }

  let mut teensy_pid = 1158;

  if let Some(pid) = args.next() {
    let pid: u16 = pid.parse()?;
    teensy_pid = pid;
  }

  let socket = tokio::net::UdpSocket::bind(addr).await?;

  let mut buf = [0u8; 1024];

  let open_hid = async || {
    let mut backoff_ms = 100;
    loop {
      let device_res = match hidapi::HidApi::new() {
        Ok(api) => match api.open(teensy_vid, teensy_pid) {
          Ok(dev) => Ok(dev),
          Err(e) => Err(format!("Failed to open HID device for Teensy: {}", e)),
        },
        Err(e) => Err(format!("Failed to initialize HID API: {}", e)),
      };

      let dev = match device_res {
        Ok(dev) => dev,
        Err(msg) => {
          eprintln!("{}", msg);
          sleep(Duration::from_millis(backoff_ms)).await;
          backoff_ms = (backoff_ms * 2).min(5000);
          continue;
        }
      };

      if let Err(e) = dev.set_blocking_mode(false) {
        eprintln!("Failed to set Teensy HID device to nonblocking mode: {}", e);
        // Drop device and retry after backoff
        sleep(Duration::from_millis(backoff_ms)).await;
        backoff_ms = (backoff_ms * 2).min(5000);
        continue;
      }

      eprintln!(
        "Teensy HID device connected (vid=0x{:04x}, pid=0x{:04x})",
        teensy_vid, teensy_pid
      );

      return dev;
    }
  };

  let mut hid = open_hid().await;

  loop {
    match socket.recv_from(&mut buf).await {
      Ok((mut len, _)) => {
        if let Some(mut msg) = RobotCommandFrame::decode_slice(&buf[..len]) {
          loop {
            match socket.try_recv_from(&mut buf) {
              Ok((l, _)) => {
                len = l;

                if let Some(new_msg) = RobotCommandFrame::decode_slice(&buf[..len]) {
                  msg = new_msg;
                }
              }
              Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
              Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
              }
            }
          }

          //TODO: handle crc

          let Some(cmd) = msg.commands.get(robot_id as usize) else {
            continue;
          };

          let teensy_msg = TeensyMsg {
            first: 0,
            vx: cmd.vx_mmps,
            vy: cmd.vy_mmps,
            omega: cmd.omega_mradps,
          };

          //surely safe, I promise!
          let array = unsafe { transmute::<TeensyMsg, [u8; 8]>(teensy_msg) };

          match hid.write(&array) {
            Ok(_) => {}
            Err(e) => {
              eprintln!("Error writing to Teensy HID device: {}", e);
              hid = open_hid().await;
            }
          }
        }
      }
      Err(e) => {
        eprintln!("Error receiving data: {}", e);
      }
    }
  }
}
