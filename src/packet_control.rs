use pcap::Device;
use std::process::Command;
use druid::Data;
use std::str;
use std::sync::RwLock;

pub struct PacketInterface {
   pub controller: i32,
   pub process_id: RwLock<u32>,
   pub port_numbers: Vec<u32>,
}

impl PacketInterface {
    pub fn new() -> PacketInterface {
        let intf = PacketInterface {
            port_numbers: vec![],
            controller: -1,
            process_id: RwLock::new(0),
        };
        intf
    }
    pub fn get_controller(&self) -> i32 {
        self.controller
    }
    fn set_controller(&mut self, ctl: i32) {
        self.controller = ctl;
    }
    fn set_process_id(&self, pid: u32) {
       let mut proc_id = self.process_id.write().unwrap();
       *proc_id = pid;
    }
    pub fn get_process_id(&self) -> u32 {
       return *self.process_id.read().unwrap();
    }
    pub fn detect_process(&self, pname: &str, app_name: &str) {
       // Currently this only supports linux! Maybe one day we can do windows too!
       let pid = Command::new("pgrep")
           .arg(pname)
           .output();
       // Check that the PID is not for the launcher
       match pid {
            Ok(res) => { 
                match res.status.success() {
                   true => {
                       let pid = match str::from_utf8(&res.stdout) {
                            Ok(id) => id.trim(),
                            _ => "-1",
                       };
                        println!("Proc id: {:?}.\n Checking to ensure this is not a launcher...", pid);
                        self.check_pid_app_name(pid, app_name);
                   },
                   false => {
                        println!("Failed to lookup process for FFXIV. Is the game running?");
                   },
                }
            },
            _ => { println!("Failure"); }
       }
    }
    fn check_pid_app_name(&self, pid: &str, app_name: &str) {
        let res = Command::new("cat")
            .arg(format!("/proc/{}/status", pid))
            .output();
        match res {
            Ok(info) => { 
                if info.status.success() {
                    let proc_info = match str::from_utf8(&info.stdout) {
                        Ok(inf) => inf,
                        _ => "",
                    };
                    if proc_info.len() > 0 {
                        let mut is_game = false;
                        for line in proc_info.lines() {
                            if line.starts_with("Name:\t") {
                                let prog_name = &line[6..];
                                println!("Potential name is: {}", prog_name);
                                if prog_name == app_name {
                                    is_game = true;
                                    break;
                                }
                            }
                        }
                        if is_game {
                            let pidnum: u32 = pid.parse().unwrap();
                            self.set_process_id(pidnum);
                        }
                    }
                    else {
                        println!("Error occurred while reading proc file.");
                    }
                }
                else {
                    println!("Failed to find process file.");
                }
            },
            _ => { println!("Invalid id, or process no longer exists."); }
        }
    }
    fn get_ports(&self) -> Vec<u32> {
        return self.port_numbers.clone()
    }
    fn set_ports(&mut self, ports: Vec<u32>) {
        self.port_numbers = ports;
        self.port_numbers.sort();
    }
    fn add_port(&mut self, port: u32) {
        self.port_numbers.push(port);
        self.port_numbers.sort();
    }
}

impl Clone for PacketInterface {
    fn clone(&self) -> Self {
        let mut res = PacketInterface::new();
        res.controller = self.controller;
        res.process_id = RwLock::new(self.get_process_id());
        res.port_numbers = self.port_numbers.clone();
        res
    }
}

impl Data for PacketInterface {
    fn same(&self, other: &Self) -> bool {
        let mut sameness = true;
        sameness &= self.controller == other.controller;
        sameness &= *self.process_id.read().unwrap() == *other.process_id.read().unwrap();
        sameness &= self.port_numbers.len() == other.port_numbers.len();
        sameness &= self.port_numbers.eq(&other.port_numbers);
        return sameness;
    }
}
