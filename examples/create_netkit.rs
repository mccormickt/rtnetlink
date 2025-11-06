// SPDX-License-Identifier: MIT

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<(), String> {
    use rtnetlink::{new_connection, LinkNetkit};
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
        return Ok(());
    }
    let if_name = &args[1];
    let peer_name = &args[2];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    handle
        .link()
        .add(LinkNetkit::new(if_name, peer_name).build())
        .execute()
        .await
        .map_err(|e| format!("{}", e))
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example create_netkit -- <interface> <peer_interface>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

    cargo build --example create_netkit

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./create_netkit <interface> <peer_interface>"
    );
}
