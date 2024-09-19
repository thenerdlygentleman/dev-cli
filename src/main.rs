use clap::Parser;
use evdev::Device;

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
    let args = Args::parse();

    let mut _device = Device::open(args.device).expect("Device opened");

    if args.list {
        list_device(_device);
    }
}
