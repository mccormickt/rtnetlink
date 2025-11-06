// SPDX-License-Identifier: MIT

use crate::{
    link::LinkMessageBuilder,
    packet_route::link::{
        InfoData, InfoKind, InfoNetkit, NetkitMode, NetkitPolicy, NetkitScrub,
    },
    LinkUnspec,
};

/// Represent netkit device.
/// Example code on creating a netkit device
/// ```no_run
/// use rtnetlink::{new_connection, LinkNetkit};
/// #[tokio::main]
/// async fn main() -> Result<(), String> {
///     let (connection, handle, _) = new_connection().unwrap();
///     tokio::spawn(connection);
///
///     handle
///         .link()
///         .add(LinkNetkit::new("nk0", "nk1").build())
///         .execute()
///         .await
///         .map_err(|e| format!("{e}"))
/// }
/// ```
///
/// Please check LinkMessageBuilder::<LinkNetkit> for more detail.
#[derive(Debug)]
pub struct LinkNetkit;

impl LinkNetkit {
    pub fn new(name: &str, peer: &str) -> LinkMessageBuilder<Self> {
        LinkMessageBuilder::<LinkNetkit>::new(name, peer)
    }
}

impl LinkMessageBuilder<LinkNetkit> {
    /// Create [LinkMessageBuilder] for netkit
    pub fn new(name: &str, peer: &str) -> Self {
        LinkMessageBuilder::<LinkNetkit>::new_with_info_kind(InfoKind::Netkit)
            .name(name.to_string())
            .peer(peer)
    }

    pub fn append_info_data(mut self, info: InfoNetkit) -> Self {
        if let InfoData::Netkit(infos) = self
            .info_data
            .get_or_insert_with(|| InfoData::Netkit(Vec::new()))
        {
            infos.push(info);
        }
        self
    }

    pub fn peer(self, peer: &str) -> Self {
        let peer_msg = LinkMessageBuilder::<LinkUnspec>::new()
            .name(peer.to_string())
            .build();
        self.append_info_data(InfoNetkit::Peer(peer_msg))
    }

    pub fn primary(self, primary: bool) -> Self {
        self.append_info_data(InfoNetkit::Primary(primary))
    }

    pub fn policy(self, policy: NetkitPolicy) -> Self {
        self.append_info_data(InfoNetkit::Policy(policy))
    }

    pub fn peer_policy(self, peer_policy: NetkitPolicy) -> Self {
        self.append_info_data(InfoNetkit::PeerPolicy(peer_policy))
    }

    pub fn mode(self, mode: NetkitMode) -> Self {
        self.append_info_data(InfoNetkit::Mode(mode))
    }

    pub fn scrub(self, scrub: NetkitScrub) -> Self {
        self.append_info_data(InfoNetkit::Scrub(scrub))
    }

    pub fn peer_scrub(self, scrub: NetkitScrub) -> Self {
        self.append_info_data(InfoNetkit::PeerScrub(scrub))
    }

    pub fn headroom(self, headroom: u16) -> Self {
        self.append_info_data(InfoNetkit::Headroom(headroom))
    }

    pub fn tailroom(self, tailroom: u16) -> Self {
        self.append_info_data(InfoNetkit::Tailroom(tailroom))
    }
}
