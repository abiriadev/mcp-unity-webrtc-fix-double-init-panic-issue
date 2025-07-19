use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;
pub use jitsi_xmpp_parsers::jingle::{Action, Jingle};
use jitsi_xmpp_parsers::{
  jingle::{Content, Description, Transport},
  jingle_dtls_srtp::Fingerprint,
  jingle_ice_udp::Transport as IceUdpTransport,
  jingle_rtp::Description as RtpDescription,
  jingle_ssma::{Group as SourceGroup, Parameter as SourceParameter, Source},
};
pub use sdp::SessionDescription;
use sdp::{direction::Direction, extmap::ExtMap, MediaDescription};
use xmpp_parsers::{
  hashes::Algo,
  jingle::{ContentId, Creator, Senders, SessionId},
  jingle_dtls_srtp::Setup,
  jingle_grouping::{Content as GroupContent, Group},
  jingle_ice_udp::Type as CandidateType,
  jingle_rtcp_fb::RtcpFb,
  jingle_rtp::{Channels, Parameter, PayloadType, RtcpMux},
  jingle_rtp_hdrext::{RtpHdrext, Senders as RtpHdrextSenders},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("invalid Jingle IQ")]
  InvalidJingle,
  #[error("invalid JID")]
  InvalidJid,
  #[error("unknown error")]
  Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait SessionDescriptionJingleConversionsExt {
  fn try_from_jingle(jingle: &Jingle) -> Result<SessionDescription>;
  fn try_to_jingle(
    &self,
    action: Action,
    session_id: &str,
    initiator: &str,
    responder: &str,
  ) -> Result<Jingle>;
  fn add_sources_from_jingle(&mut self, jingle: &Jingle) -> Result<()>;
  fn remove_sources_from_jingle(&mut self, jingle: &Jingle) -> Result<()>;
}

fn group_sources(description: &RtpDescription) -> Vec<(Option<&SourceGroup>, Vec<&Source>)> {
  let sources: HashMap<u32, &Source> = description
    .ssrcs
    .iter()
    .map(|source| (source.id, source))
    .collect();

  let mut sources_by_group = vec![];

  sources_by_group.extend(
    sources
      .values()
      .filter(|source| {
        !description.ssrc_groups.iter().any(|group| {
          group
            .sources
            .iter()
            .find(|group_source| group_source.id == source.id)
            .is_some()
        })
      })
      .copied()
      .map(|source| (None, vec![source])),
  );

  sources_by_group.extend(description.ssrc_groups.iter().map(|group| {
    (
      Some(group),
      group
        .sources
        .iter()
        .filter_map(|source| sources.get(&source.id))
        .copied()
        .collect(),
    )
  }));

  sources_by_group.sort_by_key(|(_, source)| {
    !source
      .iter()
      .map(|source| &source.parameters)
      .flatten()
      .any(|parameter| {
        parameter.name == "msid"
          && parameter
            .value
            .as_ref()
            .map(|value| value.starts_with("mixedmslabel "))
            .unwrap_or_default()
      })
  });

  sources_by_group
}

fn msid_for_sources<'a>(sources: &'a [&'a Source]) -> Option<&'a str> {
  sources
    .iter()
    .filter_map(|source| {
      source
        .parameters
        .iter()
        .filter_map(|param| {
          (param.name == "msid")
            .then(|| param.value.as_deref())
            .flatten()
        })
        .next()
    })
    .next()
}

fn populate_media_description_from_sources(
  mut md: MediaDescription,
  sources: &[&Source],
  maybe_group: Option<&SourceGroup>,
) -> MediaDescription {
  if let Some(msid) = msid_for_sources(sources) {
    md = md.with_value_attribute("msid".into(), msid.into());
  }

  if sources
    .first()
    .and_then(|source| source.info.as_ref())
    .map(|info| info.owner.as_str())
    == Some("jvb")
  {
    md = md.with_property_attribute("sendrecv".into());
  }
  else {
    md = md.with_property_attribute("sendonly".into());
  }

  for source in sources {
    for param in &source.parameters {
      md = md.with_value_attribute(
        "ssrc".into(),
        format!(
          "{} {}{}",
          source.id,
          param.name,
          param
            .value
            .as_ref()
            .map(|value| format!(":{}", value))
            .unwrap_or_default(),
        ),
      );
    }
  }

  if let Some(group) = maybe_group {
    md = md.with_value_attribute(
      "ssrc-group".into(),
      format!(
        "{} {}",
        group.semantics,
        group.sources.iter().map(|source| source.id).join(" ")
      ),
    );
  }

  md
}

impl SessionDescriptionJingleConversionsExt for SessionDescription {
  fn try_from_jingle(jingle: &Jingle) -> Result<SessionDescription> {
    let mut sd = SessionDescription::new_jsep_session_description(false)
      .with_value_attribute("msid-semantic".into(), " WMS *".into());

    let mut mid = 0;

    for content in &jingle.contents {
      if let Some(Description::Rtp(description)) = &content.description {
        let sources_by_group = group_sources(&description);

        for (maybe_group, sources) in sources_by_group {
          let mut md =
            MediaDescription::new_jsep_media_description(description.media.clone(), vec![]);

          for pt in &description.payload_types {
            md = md.with_codec(
              pt.id,
              pt.name.as_ref().ok_or(Error::InvalidJingle)?.clone(),
              pt.clockrate.ok_or(Error::InvalidJingle)?,
              if pt.channels.0 == 1 {
                0
              }
              else {
                pt.channels.0.into()
              },
              pt.parameters
                .iter()
                .map(|param| {
                  format!(
                    "{}{}",
                    (!param.name.is_empty())
                      .then(|| format!("{}=", param.name))
                      .unwrap_or_default(),
                    param.value,
                  )
                })
                .join(";"),
            );

            md = md.with_value_attribute("rtcp".into(), "1 IN IP4 0.0.0.0".into());

            for rtcp_fb in &pt.rtcp_fbs {
              md = md.with_value_attribute(
                "rtcp-fb".into(),
                format!(
                  "{} {}{}",
                  pt.id,
                  rtcp_fb.type_,
                  rtcp_fb
                    .subtype
                    .as_ref()
                    .map(|subtype| format!(" {}", subtype))
                    .unwrap_or_default(),
                ),
              );
            }
          }

          for hdrext in &description.hdrexts {
            md = md.with_extmap(ExtMap {
              value: hdrext.id.try_into().map_err(|_| Error::InvalidJingle)?,
              uri: Some(hdrext.uri.parse().map_err(|_| Error::InvalidJingle)?),
              direction: Direction::Unspecified,
              ext_attr: None,
            });
          }

          if let Some(Transport::IceUdp(transport)) = &content.transport {
            if let Some(setup) = transport
              .fingerprint
              .as_ref()
              .and_then(|fingerprint| fingerprint.setup.as_ref())
            {
              md = md.with_value_attribute(
                "setup".into(),
                match setup {
                  Setup::Active => "active",
                  Setup::Passive => "passive",
                  Setup::Actpass => "actpass",
                }
                .into(),
              );
            }
          }

          md = md.with_value_attribute("mid".into(), mid.to_string());
          md = populate_media_description_from_sources(md, &sources, maybe_group);

          if let Some(Transport::IceUdp(transport)) = &content.transport {
            if let (Some(ufrag), Some(pwd)) = (&transport.ufrag, &transport.pwd) {
              md = md.with_ice_credentials(ufrag.into(), pwd.into());
            }

            if let Some(fingerprint) = &transport.fingerprint {
              md = md.with_fingerprint(
                // RFC 4572 section 5
                // https://www.iana.org/assignments/hash-function-text-names/hash-function-text-names.xhtml
                match &fingerprint.hash {
                  Algo::Sha_1 => "sha-1",
                  Algo::Sha_256 => "sha-256",
                  Algo::Sha_512 => "sha-512",
                  Algo::Unknown(algo)
                    if ["sha-224", "sha-384", "shake128", "shake256", "md5", "md2"]
                      .contains(&algo.as_str()) =>
                  {
                    &algo
                  },
                  _ => return Err(Error::InvalidJingle),
                }
                .into(),
                fingerprint
                  .value
                  .iter()
                  .map(|byte| format!("{:02X}", byte))
                  .join(":"),
              );
            }

            for candidate in &transport.candidates {
              // https://datatracker.ietf.org/doc/html/rfc8839#section-5.1
              let mut candidate_str = format!(
                "{} {} {} {} {} {} typ {}",
                candidate.foundation,
                candidate.component,
                candidate.protocol,
                candidate.priority,
                candidate.ip,
                candidate.port,
                candidate.type_,
              );
              match candidate.type_ {
                CandidateType::Host => {},
                CandidateType::Prflx | CandidateType::Srflx | CandidateType::Relay => {
                  candidate_str.push_str(&format!(
                    "{}{}",
                    candidate
                      .rel_addr
                      .map(|raddr| format!(" raddr {}", raddr))
                      .unwrap_or_default(),
                    candidate
                      .rel_port
                      .map(|rport| format!(" rport {}", rport))
                      .unwrap_or_default(),
                  ));
                },
              };
              candidate_str.push_str(&format!(" generation {}", candidate.generation));
              md = md.with_candidate(candidate_str);
            }
          }

          if description.rtcp_mux.is_some() {
            md = md.with_property_attribute("rtcp-mux".into());
          }

          sd = sd.with_media(md);
          mid += 1;
        }
      }
    }

    if let Some(group) = &jingle.group {
      sd = sd.with_value_attribute(
        "group".into(),
        format!("{} {}", group.semantics, (0..mid).join(" "),),
      );
    }

    Ok(sd)
  }

  fn try_to_jingle(
    &self,
    action: Action,
    session_id: &str,
    initiator: &str,
    responder: &str,
  ) -> Result<Jingle> {
    let mut jingle = Jingle::new(action, SessionId(session_id.into()))
      .with_initiator(initiator.parse().map_err(|_| Error::InvalidJid)?)
      .with_responder(responder.parse().map_err(|_| Error::InvalidJid)?);

    let mut contents: HashMap<&str, Content> = HashMap::new();

    for md in &self.media_descriptions {
      let content = match contents.entry(md.media_name.media.as_str()) {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => {
          let mut description = RtpDescription::new(md.media_name.media.clone());

          description.ssrc = md
            .attributes
            .iter()
            .filter(|attribute| attribute.key == "ssrc")
            .next()
            .and_then(|attribute| {
              let mut parts = attribute.value.as_ref()?.split(' ');
              Some(parts.next().unwrap().into())
            });

          for rtpmap in md
            .attributes
            .iter()
            .filter(|attribute| attribute.key == "rtpmap")
          {
            if let Some(value) = &rtpmap.value {
              let mut parts = value.splitn(2, ' ');
              let id: u8 = parts
                .next()
                .unwrap()
                .parse()
                .map_err(|_| Error::InvalidJingle)?;
              let mut parts = parts.next().ok_or(Error::InvalidJingle)?.split('/');
              let mut pt = PayloadType {
                id,
                name: Some(parts.next().unwrap().into()),
                clockrate: Some(
                  parts
                    .next()
                    .ok_or(Error::InvalidJingle)?
                    .parse()
                    .map_err(|_| Error::InvalidJingle)?,
                ),
                channels: Channels(
                  parts
                    .next()
                    .map(|channels| channels.parse())
                    .transpose()
                    .map_err(|_| Error::InvalidJingle)?
                    .unwrap_or(1),
                ),
                ptime: None,
                maxptime: None,
                parameters: vec![],
                rtcp_fbs: vec![],
              };

              for fmtp in md
                .attributes
                .iter()
                .filter(|attribute| attribute.key == "fmtp")
              {
                if let Some(value) = &fmtp.value {
                  let mut parts = value.splitn(2, ' ');
                  let fmtp_id: u8 = parts
                    .next()
                    .unwrap()
                    .parse()
                    .map_err(|_| Error::InvalidJingle)?;
                  if fmtp_id == id {
                    let parameters = parts.next().ok_or(Error::InvalidJingle)?.split(';');
                    for parameter in parameters {
                      if let Some((name, value)) = parameter.split_once('=') {
                        pt.parameters.push(Parameter {
                          name: name.into(),
                          value: value.into(),
                        });
                      }
                    }
                  }
                }
              }

              for rtcp_fb in md
                .attributes
                .iter()
                .filter(|attribute| attribute.key == "rtcp-fb")
              {
                if let Some(value) = &rtcp_fb.value {
                  let mut parts = value.splitn(3, ' ');
                  let rtcp_fb_id: u8 = parts
                    .next()
                    .unwrap()
                    .parse()
                    .map_err(|_| Error::InvalidJingle)?;
                  if rtcp_fb_id == id {
                    pt.rtcp_fbs.push(RtcpFb {
                      type_: parts.next().ok_or(Error::InvalidJingle)?.into(),
                      subtype: parts.next().map(Into::into),
                    });
                  }
                }
              }

              description.payload_types.push(pt);
            }
          }

          if md.attribute("rtcp-mux").is_some() {
            description.rtcp_mux = Some(RtcpMux);
          }

          for extmap in md
            .attributes
            .iter()
            .filter(|attribute| attribute.key == "extmap")
          {
            if let Some(value) = &extmap.value {
              let mut parts = value.splitn(2, ' ');
              let id = parts
                .next()
                .unwrap()
                .parse()
                .map_err(|_| Error::InvalidJingle)?;
              let uri = parts.next().ok_or(Error::InvalidJingle)?;
              description.hdrexts.push(RtpHdrext {
                id,
                uri: uri.into(),
                senders: RtpHdrextSenders::Both,
              });
            }
          }

          let mut transport = IceUdpTransport::new();

          if let Some(maybe_ufrag) = md.attribute("ice-ufrag") {
            transport.ufrag = maybe_ufrag.map(Into::into);
          }

          if let Some(maybe_pwd) = md.attribute("ice-pwd") {
            transport.pwd = maybe_pwd.map(Into::into);
          }

          if let Some(maybe_fingerprint) = md.attribute("fingerprint") {
            transport.fingerprint = maybe_fingerprint
              .and_then(|fingerprint| {
                fingerprint.split_once(' ').map(|(hash, value)| {
                  Fingerprint::from_colon_separated_hex(
                    match md
                      .attribute("setup")
                      .flatten()
                      .ok_or(Error::InvalidJingle)?
                    {
                      "passive" => Setup::Passive,
                      "active" => Setup::Active,
                      "actpass" => Setup::Actpass,
                      _ => return Err(Error::InvalidJingle),
                    },
                    hash,
                    value,
                  )
                  .map_err(|_| Error::InvalidJingle)
                })
              })
              .transpose()?;
          }

          entry.insert(
            Content::new(Creator::Responder, ContentId(md.media_name.media.clone()))
              .with_senders(if md.attribute("sendrecv").is_some() {
                Senders::Both
              }
              else if md.attribute("sendonly").is_some() {
                Senders::Responder
              }
              else if md.attribute("recvonly").is_some() {
                Senders::Initiator
              }
              else {
                Senders::None
              })
              .with_description(description)
              .with_transport(transport),
          )
        },
      };

      if let Some(Description::Rtp(description)) = &mut content.description {
        let mut ssrcs: HashMap<u32, Vec<SourceParameter>> = HashMap::new();

        for ssrc in md
          .attributes
          .iter()
          .filter(|attribute| attribute.key == "ssrc")
        {
          if let Some(value) = &ssrc.value {
            let mut parts = value.splitn(2, ' ');
            let parameters = ssrcs
              .entry(
                parts
                  .next()
                  .unwrap()
                  .parse()
                  .map_err(|_| Error::InvalidJingle)?,
              )
              .or_default();
            if let Some(parameter) = parts.next() {
              let mut parts = parameter.splitn(2, ':');
              parameters.push(SourceParameter {
                name: parts.next().unwrap().into(),
                value: parts.next().map(Into::into),
              });
            }
          }
        }

        for parameters in ssrcs.values_mut() {
          if !parameters.iter().any(|parameter| parameter.name == "msid") {
            if let Some(msid) = md
              .attributes
              .iter()
              .find(|attribute| attribute.key == "msid")
            {
              parameters.push(SourceParameter {
                name: "msid".into(),
                value: msid.value.clone(),
              });
            }
          }
        }

        description
          .ssrcs
          .extend(ssrcs.into_iter().map(|(id, parameters)| Source {
            id,
            parameters,
            info: None,
          }));

        for ssrc_group in md
          .attributes
          .iter()
          .filter(|attribute| attribute.key == "ssrc-group")
        {
          if let Some(value) = &ssrc_group.value {
            let mut parts = value.split(' ');
            description.ssrc_groups.push(SourceGroup {
              semantics: parts
                .next()
                .unwrap()
                .parse()
                .map_err(|_| Error::InvalidJingle)?,
              sources: parts
                .map(|id| {
                  Ok(Source {
                    id: id.parse().map_err(|_| Error::InvalidJingle)?,
                    parameters: vec![],
                    info: None,
                  })
                })
                .collect::<Result<_>>()?,
            });
          }
        }
      }
    }

    if let Some(group) = self.attribute("group") {
      let mut parts = group.splitn(2, ' ');
      jingle.group = Some(Group {
        semantics: parts
          .next()
          .unwrap()
          .parse()
          .map_err(|_| Error::InvalidJingle)?,
        // TODO
        contents: contents
          .keys()
          .map(|&name| GroupContent {
            name: ContentId(name.into()),
          })
          .collect(),
      });
    }

    jingle.contents = contents.into_values().collect();

    Ok(jingle)
  }

  fn add_sources_from_jingle(&mut self, jingle: &Jingle) -> Result<()> {
    let mut mid = self.media_descriptions.len();
    for content in &jingle.contents {
      if let Some(Description::Rtp(description)) = &content.description {
        if let Some(template_md) = self
          .media_descriptions
          .iter()
          .find(|md| md.media_name.media == description.media)
        {
          let mut template_md = template_md.clone();
          template_md.attributes.retain(|attribute| {
            !["ssrc", "ssrc-group", "mid", "msid", "sendrecv", "sendonly"]
              .contains(&&*attribute.key)
          });

          let sources_by_group = group_sources(&description);
          for (maybe_group, sources) in sources_by_group {
            let mut md = template_md.clone();

            md = md.with_value_attribute("mid".into(), mid.to_string());
            md = populate_media_description_from_sources(md, &sources, maybe_group);

            self.media_descriptions.push(md);
            mid += 1;
          }
        }
      }
    }
    Ok(())
  }

  fn remove_sources_from_jingle(&mut self, jingle: &Jingle) -> Result<()> {
    unimplemented!()
  }
}
