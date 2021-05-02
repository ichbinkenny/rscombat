use pcap::Device;
use std::process::Command;


pub struct PacketInterface {
    controller: i32,
    process_id: u32,
    port_numbers: Vec<u32>,
}

impl PacketInterface {
    pub fn new() -> PacketInterface {
        let intf = PacketInterface {
            port_numbers: vec![],
            controller: -1,
            process_id: 0,
        };
        intf
    }
    fn get_controller(&self) -> i32 {
        self.controller
    }
    fn set_controller(&mut self, ctl: i32) {
        self.controller = ctl;
    }
    fn set_process_id(&mut self, pid: u32) {
       self.process_id = pid; 
    }
    fn get_process_id(&self) -> u32 {
        self.process_id
    }
    fn detect_process(pname: &str) -> Option(u32) {
       // Currently this only supports linux! Maybe one day we can do windows too!
       let pid = Command::new("pgrep")
           .arg(pname)
           .output();
       // Check that the PID is not for the launcher
       if Ok(pid) {
            let pinfo = Command::new("cat")
                .arg(format!("/proc/{}/status", pid))
                .output();
            
       }
       None
    }
    fn get_ports(&self) -> Vec<u32> {
        return self.port_numbers.clone()
    }
    fn set_ports(&mut self, ports: Vec<u32>) {
        self.port_numbers = ports;
    }
    fn add_port(&mut self, port: u32) {
        self.port_numbers.push(port);
    }
}

