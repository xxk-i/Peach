use mdns_sd::{ServiceDaemon, ServiceInfo};
use tauri::async_runtime::Receiver;

pub async fn start(mut discover_rx: Receiver<&str>) {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = "_peach._tcp.local.";
    let instance_name = "macos-peach";
    let ip = "";
    let host_name = "peach.local.";
    let port = 7778;
    let properties = [("property_1", "test"), ("property_2", "1234")];

    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        ip,
        port,
        &properties[..],
    )
    .unwrap()
    .enable_addr_auto();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service)
        .expect("Failed to register our service");

    while let Some(message) = discover_rx.recv().await {
        if message.eq("terminate") {
            // Gracefully shutdown the daemon
            mdns.shutdown().unwrap();
        }
    }
}
