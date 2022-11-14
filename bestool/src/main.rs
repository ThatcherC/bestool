mod cmds;
mod serial_monitor;

use clap::Parser;
use crate::cmds::{cmd_list_serial_ports, cmd_serial_port_monitor};

// BES2300 programming utility for better cross platform support
// This is completely reverse engineered at this point; there ~may~ will be bugs

/* Key commands:
* - Write binary
* - Read binary
* - List serial ports / Serial Monitor
*/

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "bestool")]
#[command(bin_name = "bestool")]
enum BesTool {
    ListSerialPorts(ListSerialPorts),
    SerialMonitor(SerialMonitor),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct ListSerialPorts {}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct SerialMonitor {
    serial_port_path: String,
    #[arg(short, long, default_value_t = 115200)]
    baud_rate: u32,
}

fn main() {
    match BesTool::parse() {
        BesTool::ListSerialPorts(_) => cmd_list_serial_ports(),
        BesTool::SerialMonitor(args) =>{
            cmd_serial_port_monitor(args.serial_port_path,args.baud_rate);
        },
    }
}
