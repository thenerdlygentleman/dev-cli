use clap::Parser;
use evdev::Device;
use iocraft::prelude::*;

fn list_device(device: Device) {
    let _keys = device.supported_keys();
    let _events = device.supported_events();

    println!("keys are {:#?}", _keys);
    println!("events are {:#?}", _events);
}

#[derive(Parser)]
#[command(name = "dev-cli")]
#[command(version = "0.1.0")]
#[command(about = "cli for evdev", long_about = None)]
struct Args {
    #[arg(short, long)]
    device: String,

    #[arg(short, long, default_value_t = false)]
    list: bool,
}

fn main() {
    element! {
        Box(
            border_style: BorderStyle::Round,
            border_color: Color::Blue,
        ) {
            Text(content: "Welcome to the dev-cli")
        }
    }
    .print();
    let args = Args::parse();

    let mut _device = Device::open(args.device).expect("Device opened");

    if args.list {
        list_device(_device);
    }
}
