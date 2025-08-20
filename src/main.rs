use std::{
    ffi::OsStr,
    process::{Command, ExitCode},
    thread,
    time::Duration,
};

use clap::Parser;

const STEP: Duration = Duration::from_secs(60);

fn main() -> ExitCode {
    let pomo = Pomo::parse();

    loop {
        pomo.clone().start();
    }
}

#[derive(Parser, Clone, Copy)]
#[command(version, about)]
/// Pomodoro timer, everything is in minutes
struct Pomo {
    #[arg(short, default_value_t = 25)]
    work_time: u64,
    #[arg(short, default_value_t = 5)]
    short_break: u64,
    #[arg(short, default_value_t = 20)]
    long_break: u64,
    /// Cycles before the long break (0 or 1 will disable long breaks)
    #[arg(short, default_value_t = 4)]
    cycles: u64,
}

impl Pomo {
    fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "POMO\nW:{}\nS:{}\nL:{}\nC:{}",
            self.work_time, self.short_break, self.long_break, self.cycles,
        );
    }

    fn start(&mut self) {
        self.print();
        self.work();
    }

    fn work(&mut self) {
        let work_time = self.work_time;
        let short_break = self.short_break;
        let long_break = self.long_break;
        let use_cycles = self.cycles != 0 && self.cycles != 1;
        while self.cycles > 0 || !use_cycles {
            send_notif("start working");
            self.work_time = work_time;
            self.short_break = short_break;
            self.long_break = long_break;
            while self.work_time > 0 {
                self.print();
                thread::sleep(STEP);
                self.work_time -= 1;
            }
            if use_cycles {
                self.cycles -= 1;
            }
            if self.cycles == 0 && use_cycles {
                send_notif("take a long break");
                while self.long_break > 0 {
                    self.print();
                    thread::sleep(STEP);
                    self.long_break -= 1;
                }
            } else {
                send_notif("take a short break");
                while self.short_break > 0 {
                    self.print();
                    thread::sleep(STEP);
                    self.short_break -= 1;
                }
            }
        }
    }
}

fn send_notif<S>(msg: S)
where
    S: AsRef<OsStr>,
{
    Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("-a")
        .arg("pomo")
        .arg(msg)
        .output()
        .expect("notify-send not found!");
}
