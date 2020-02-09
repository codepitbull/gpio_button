extern crate sysfs_gpio;
extern crate ctrlc;

use std::env;

use std::thread;
use std::thread::sleep;

use std::time::Duration;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Release, SeqCst};

use std::process;
use std::process::{Command, Output};

use sysfs_gpio::{Direction, Pin};

static SHUTDOWN_THREAD_RUNNING:AtomicBool = AtomicBool::new(false);

fn poll(pin_num: u64, delay: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin_num);
    input.with_exported( || {
        input.set_direction(Direction::In)?;
        let mut prev_val: u8 = 255;
        loop {
            let val = input.get_value()?;
            if val != prev_val {
                prev_val = val;
                if val != 0 {
                    if SHUTDOWN_THREAD_RUNNING.compare_and_swap(false, true, SeqCst) == false {
                        //activetae screen by turning on the usb-port
                        let display_on = Command::new("/usr/sbin/uhubctl")
                            .arg("--port")
                            .arg("2")
                            .arg("--a")
                            .arg("on")
                            .output()
                            .expect("failed to execute process");

                        exit_on_error("Failed disabling the screen", &display_on);

                        //Spawn y thread to deactivate screen after the delay
                        thread::spawn(move || {
                            thread::sleep(Duration::from_millis(delay * 1000));
                            let display_off = Command::new("/usr/sbin/uhubctl")
                                .arg("--port")
                                .arg("2")
                                .arg("--a")
                                .arg("off")
                                .output()
                                .expect("failed to execute process");

                            exit_on_error("Failed disabling the screen", &display_off);
                            SHUTDOWN_THREAD_RUNNING.store(false, Release)
                        });
                    }
                }
            }
            sleep(Duration::from_millis(10));
        }
    })
}

fn exit_on_error(message: &str, command_output: &Output) {
    if !command_output.status.success() {
        eprintln!("problem: {}", message);
        eprintln!("stdout: {:?}", String::from_utf8_lossy(&command_output.stdout));
        eprintln!("stderr: {:?}", String::from_utf8_lossy(&command_output.stderr));
        process::exit(0x0100);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        show_usage();
    } else {
        match args[2].parse::<u64>() {
            Ok(delay) => {
                match args[1].parse::<u64>() {
                    Ok(pin) => {
                        match poll(pin, delay) {
                            Ok(()) => println!("Polling Complete!"),
                            Err(err) => println!("Error: {}", err),
                        }
                    }
                    Err(_) => show_usage(),
                }
            }
            Err(_) => show_usage(),
        }
    }
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");
}

fn show_usage() {
    println!("Usage: ./gpio_button <pin> <delayinseconds>");
}