use clap::Parser;
use rawsocket::RawCapture;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct SnifferCli {
    /// Interface to capture
    #[arg(long, short = 'i')]
    interface: String,
    /// Port to capture
    #[arg(short = 'p', long = "port")]
    ports: Vec<u16>,
}

#[tokio::main]
async fn main() {
    let args = SnifferCli::parse();
    let raw_capture = RawCapture::from_interface_name(&args.interface).unwrap();
    raw_capture
        .set_filter(rawsocket::filter::build_tcp_port_filter(&args.ports))
        .unwrap();
    while let Ok(packet) = raw_capture.next().await {
        println!("Packet: {:?}", packet);
    }
}
