use udev;
use uinput::Device;
use uinput::event::{keyboard, Press, Release};

use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::time::Duration;
use std::thread::sleep;

const PACKET_LEN: usize = 20;

// G keys
// [17, 255, 8,  0, X]    - G1-G8
// [17, 255, 8,  0, 0, 1] - G9
// [17, 255, 9,  0, X]    - M1-M3
// [17, 255, 10, 0, 1]    - MR

// Media keys
// [2, 8] - Play/Pause
// [2, 4] - Stop
// [2, 2] - Back
// [2, 1] - Next

// Volume
// [2, 64] - Mute
// [2, 16] - Scroll up
// [2, 32] - Scroll down

fn main() {
    let mut enumerator = udev::Enumerator::new().unwrap();
    enumerator.match_subsystem("hidraw").unwrap();
    let mut device0 = true;

    for device in enumerator.scan_devices().unwrap() {
        if device.syspath().to_str().unwrap().contains("046D:C335") {
            // Need interface 1, not 0. AFAIK there is no way to get that information, so just skip
            // first found device.
            if device0 {
                device0 = false;
                continue;
            }

            let mut f = OpenOptions::new().read(true).write(true)
                .open(device.devnode().unwrap()).expect("Unable to open /dev/hidraw*");

            if !unmap_g_f_keys(&mut f, true) {
                eprintln!("Unable to send data to keyboard");
                return;
            }

            let mut udevice = uinput::default().unwrap()
                .name("G910-macros").unwrap()
                .event(uinput::event::Keyboard::All).unwrap()
                .create().unwrap();

            let sleep_time = Duration::from_millis(50);

            loop {
                let mut buf = [0; PACKET_LEN];
                let status = f.read(&mut buf);

                match status {
                    Ok(len) => {
                        if len > 0 {
                            check_keypress(&buf, &mut udevice);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading from hidraw: {}", e);
                    }
                }

                sleep(sleep_time);
            }
        }
    }
    eprintln!("Keyboard not detected");
}

fn unmap_g_f_keys(kb: &mut File, unmap: bool) -> bool {
    let val = unmap as u8;
    let mut packet = vec![0x11, 0xff, 0x08, 0x2e, val];
    packet.resize(PACKET_LEN, 0);
    kb.write(&packet).is_ok()
}

fn check_keypress(buf: &[u8], device: &mut Device) {
    if buf[0] == 0x11 && buf[1] == 0xff && buf[3] != 46 {
        press_gkey(buf, device);
    }

    if buf[0] == 0x2 {
        press_media(buf, device);
    }

    device.synchronize().unwrap();
}

fn press_gkey(buf: &[u8], device: &mut Device) {
    match buf[2] {
        8 => {
            keypress(device, &keyboard::Key::F13, buf[4], 0b1);         // G1
            keypress(device, &keyboard::Key::F14, buf[4], 0b10);        // G2
            keypress(device, &keyboard::Key::F15, buf[4], 0b100);       // G3
            keypress(device, &keyboard::Key::F16, buf[4], 0b1000);      // G4
            keypress(device, &keyboard::Key::F17, buf[4], 0b10000);     // G5
            keypress(device, &keyboard::Key::F18, buf[4], 0b100000);    // G6
            keypress(device, &keyboard::Key::F19, buf[4], 0b1000000);   // G7
            keypress(device, &keyboard::Key::F20, buf[4], 0b10000000);  // G8
            keypress(device, &keyboard::Key::F21, buf[5], 0b1);         // G9
        },
        9 => {
            keypress(device, &keyboard::Key::F22, buf[4], 0b1);         // M1
            keypress(device, &keyboard::Key::F23, buf[4], 0b10);        // M2
            keypress(device, &keyboard::Key::F24, buf[4], 0b100);       // M3
        },
        10 => {
            keypress(device, &keyboard::Misc::Calc, buf[4], 0b1);       // MR
        },
        _ => {}
    }
}

fn press_media(buf: &[u8], device: &mut Device) {
    keypress(device, &keyboard::Misc::NextSong,     buf[1], 0b1);
    keypress(device, &keyboard::Misc::PreviousSong, buf[1], 0b10);
    keypress(device, &keyboard::Misc::StopCD,       buf[1], 0b100);
    keypress(device, &keyboard::Misc::PlayPause,    buf[1], 0b1000);
    keypress(device, &keyboard::Misc::VolumeUp,     buf[1], 0b10000);
    keypress(device, &keyboard::Misc::VolumeDown,   buf[1], 0b100000);
    keypress(device, &keyboard::Misc::Mute,         buf[1], 0b1000000);
}

fn keypress<T: Press + Release>(device: &mut Device, key: &T, byte: u8, flag: u8) {
    if byte & flag != 0 {
        device.click(key).unwrap();
    }
}
