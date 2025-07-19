use xmpp_parsers::{
  jingle_ice_udp::Candidate,
  ns::{JINGLE_DTLS, JINGLE_ICE_UDP},
};

use crate::{jingle_dtls_srtp::Fingerprint, ns::JITSI_COLIBRI};

// generate_element!(
//   /// Wrapper element for an ICE-UDP transport.
//   #[derive(Default)]
//   Transport, "transport", JINGLE_ICE_UDP,
//   attributes: [
//     /// A Password as defined in ICE-CORE.
//     pwd: Option<String> = "pwd",
//
//     /// A User Fragment as defined in ICE-CORE.
//     ufrag: Option<String> = "ufrag",
//   ],
//   children: [
//     /// List of candidates for this ICE-UDP session.
//     candidates: Vec<Candidate> = ("candidate", JINGLE_ICE_UDP) => Candidate,
//
//     /// Fingerprint of the key used for the DTLS handshake.
//     fingerprint: Option<Fingerprint> = ("fingerprint", JINGLE_DTLS) => Fingerprint,
//
//     /// Details of the Colibri WebSocket
//     web_socket: Option<WebSocket> = ("web-socket", JITSI_COLIBRI) => WebSocket
//   ]
// );

/// Wrapper element for an ICE-UDP transport.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transport {
    /// A Password as defined in ICE-CORE.
    pub pwd: Option<String>,
    /// A User Fragment as defined in ICE-CORE.
    pub ufrag: Option<String>,
    /// List of candidates for this ICE-UDP session.
    pub candidates: Vec<Candidate>,
    /// Fingerprint of the key used for the DTLS handshake.
    pub fingerprint: Option<Fingerprint>,
    /// Details of the Colibri WebSocket
    pub web_socket: Option<WebSocket>,
}
impl std::convert::TryFrom<xmpp_parsers::Element> for Transport {
    type Error = xmpp_parsers::Error;
    fn try_from(elem: xmpp_parsers::Element) -> Result<Transport, xmpp_parsers::Error> {
        if !elem.is("transport", JINGLE_ICE_UDP) {
            return Err(
                xmpp_parsers::Error::ParseError("This is not a transport element."),
            );
        }
        let mut candidates = Vec::new();
        let mut fingerprint = None;
        let mut web_socket = None;
        for _child in elem.children() {
            if _child.is("candidate", JINGLE_ICE_UDP) {
                candidates.push(Candidate::try_from(_child.clone())?);
                continue;
            }
            if _child.is("fingerprint", JINGLE_DTLS) {
                if fingerprint.is_some() {
                    return Err(
                        xmpp_parsers::Error::ParseError(
                            "Element transport must not have more than one fingerprint child.",
                        ),
                    );
                }
                fingerprint = Some(Fingerprint::try_from(_child.clone())?);
                continue;
            }
            if _child.is("web-socket", JITSI_COLIBRI) {
                if web_socket.is_some() {
                    // return Err(
                    //     xmpp_parsers::Error::ParseError(
                    //         "Element transport must not have more than one web-socket child.",
                    //     ),
                    // );
                    continue;
                }
                web_socket = Some(WebSocket::try_from(_child.clone())?);
                continue;
            }
        }
        Ok(Transport {
            pwd: match elem.attr("pwd") {
                Some(value) => Some(value.parse()?),
                None => None,
            },
            ufrag: match elem.attr("ufrag") {
                Some(value) => Some(value.parse()?),
                None => None,
            },
            candidates: candidates,
            fingerprint: fingerprint,
            web_socket: web_socket,
        })
    }
}
impl From<Transport> for xmpp_parsers::Element {
    fn from(elem: Transport) -> xmpp_parsers::Element {
        let mut builder = xmpp_parsers::Element::builder("transport", JINGLE_ICE_UDP);
        builder = builder.attr("pwd", elem.pwd);
        builder = builder.attr("ufrag", elem.ufrag);
        builder = builder.append_all(elem.candidates.into_iter());
        builder = builder
            .append_all(
                elem
                    .fingerprint
                    .map(|elem| ::minidom::Node::Element(
                        xmpp_parsers::Element::from(elem),
                    )),
            );
        builder = builder
            .append_all(
                elem
                    .web_socket
                    .map(|elem| ::minidom::Node::Element(
                        xmpp_parsers::Element::from(elem),
                    )),
            );
        builder.build()
    }
}

impl Transport {
  /// Create a new ICE-UDP transport.
  pub fn new() -> Transport {
    Default::default()
  }

  /// Add a candidate to this transport.
  pub fn add_candidate(mut self, candidate: Candidate) -> Self {
    self.candidates.push(candidate);
    self
  }

  /// Set the DTLS-SRTP fingerprint of this transport.
  pub fn with_fingerprint(mut self, fingerprint: Fingerprint) -> Self {
    self.fingerprint = Some(fingerprint);
    self
  }
}

generate_element!(
  /// Colibri WebSocket details
  WebSocket, "web-socket", JITSI_COLIBRI,
  attributes: [
      /// The WebSocket URL
      url: Required<String> = "url",
  ]
);
