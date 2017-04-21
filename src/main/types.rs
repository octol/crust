// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use common::Uid;
use mio::Token;
use net2::TcpBuilder;
use std::net::SocketAddr;

// ========================================================================================
//                                     ConnectionId
// ========================================================================================
#[derive(Debug, Clone, Copy)]
pub struct ConnectionId {
    pub active_connection: Option<Token>,
    pub currently_handshaking: usize,
}

// ========================================================================================
//                                   ConnectionInfoResult
// ========================================================================================
/// The result of a `Service::prepare_contact_info` call.
#[derive(Debug)]
pub struct ConnectionInfoResult<UID> {
    /// The token that was passed to `prepare_connection_info`.
    pub result_token: u32,
    /// The new contact info, if successful.
    pub result: ::Res<PrivConnectionInfo<UID>>,
}

// ========================================================================================
//                                     PrivConnectionInfo
// ========================================================================================
/// Contact info generated by a call to `Service::prepare_contact_info`.
#[derive(Debug)]
pub struct PrivConnectionInfo<UID> {
    #[doc(hidden)]
    pub id: UID,
    #[doc(hidden)]
    pub for_direct: Vec<SocketAddr>,
    #[doc(hidden)]
    pub for_hole_punch: Vec<SocketAddr>,
    #[doc(hidden)]
    pub hole_punch_socket: Option<TcpBuilder>,
}

impl<UID: Uid> PrivConnectionInfo<UID> {
    /// Use private connection info to create public connection info that can be shared with the
    /// peer.
    pub fn to_pub_connection_info(&self) -> PubConnectionInfo<UID> {
        PubConnectionInfo {
            for_hole_punch: self.for_hole_punch.clone(),
            for_direct: self.for_direct.clone(),
            id: self.id,
        }
    }
}

// ========================================================================================
//                                     PubConnectionInfo
// ========================================================================================
/// Contact info used to connect to another peer.
#[derive(Debug, Serialize, Deserialize)]
pub struct PubConnectionInfo<UID> {
    #[doc(hidden)]
    pub id: UID,
    #[doc(hidden)]
    pub for_hole_punch: Vec<SocketAddr>,
    #[doc(hidden)]
    pub for_direct: Vec<SocketAddr>,
}

impl<UID: Uid> PubConnectionInfo<UID> {
    /// Returns the `UID` of the node that created this connection info.
    pub fn id(&self) -> UID {
        self.id
    }
}
