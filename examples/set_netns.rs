// SPDX-License-Identifier: MIT

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<(), String> {
    use rtnetlink::{new_connection, LinkUnspec, NetworkNamespace};
    use std::{env, os::fd::AsRawFd};

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
        return Ok(());
    }
    let if_name = &args[1];
    let ns_name = &args[2];

    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    let ns_fd = NetworkNamespace::from_name(ns_name.to_string())
        .map_err(|e| format!("{}", e))?;

    handle
        .link()
        .set(
            LinkUnspec::new_with_name(if_name)
                .setns_by_fd(ns_fd.as_raw_fd())
                .build(),
        )
        .execute()
        .await
        .map_err(|e| format!("{}", e))
}

fn usage() {
    eprintln!(
        "usage:
    cargo run --example set_netns -- <interface> <ns_name>

Note that you need to run this program as root. Instead of running cargo as root,
build the example normally:

cargo build --example set_netns

Then find the binary in the target directory:

    cd ../target/debug/example ; sudo ./set_netns <interface> <ns_name>"
    );
}
