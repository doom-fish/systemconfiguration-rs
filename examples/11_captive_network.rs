use systemconfiguration::CaptiveNetwork;

fn main() {
    let interfaces = CaptiveNetwork::supported_interfaces();
    println!("supported_interfaces={interfaces:?}");
}
