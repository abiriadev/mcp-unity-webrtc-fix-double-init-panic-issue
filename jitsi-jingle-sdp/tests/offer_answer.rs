use std::io::Cursor;

use jitsi_jingle_sdp::SessionDescriptionJingleConversionsExt;
use jitsi_xmpp_parsers::jingle::{Action, Jingle};
use minidom::Element;
use pretty_assertions::assert_eq;
use sdp::SessionDescription;

#[test]
fn session_initiate_to_offer() {
  let element: Element = SESSION_INITIATE.parse().unwrap();
  let jingle = Jingle::try_from(element).unwrap();

  let sdp = SessionDescription::try_from_jingle(&jingle).unwrap();

  assert_eq!(OFFER, sdp.marshal());
}

#[test]
fn answer_to_session_accept() {
  let sdp = SessionDescription::unmarshal(&mut Cursor::new(ANSWER)).unwrap();
  let jingle = sdp
    .try_to_jingle(
      Action::SessionAccept,
      "5rpgqh5rbclco",
      "respectivearticlesholdoverly@conference.avstack.onavstack.net/focus",
      "f7e91a40-063d-4f11-9423-b31f5228d8e9@avstack.onavstack.net/3GQxy6tBgrIe",
    )
    .unwrap();

  let element: Element = SESSION_ACCEPT.parse().unwrap();
  let expected_jingle = Jingle::try_from(element).unwrap();

  assert_eq!(expected_jingle, jingle);
}

const SESSION_INITIATE: &str = r#"<jingle initiator='focus@auth.avstack.onavstack.net/focus' action='session-initiate' xmlns='urn:xmpp:jingle:1' sid='5rpgqh5rbclco'>
  <content creator='initiator' senders='both' name='audio'>
    <description xmlns='urn:xmpp:jingle:apps:rtp:1' media='audio' maxptime='60'>
      <payload-type name='opus' channels='2' clockrate='48000' id='111'>
        <parameter value='10' name='minptime' />
        <parameter value='1' name='useinbandfec' />
        <rtcp-fb type='transport-cc' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
      </payload-type>
      <payload-type id='103' clockrate='16000' name='ISAC' />
      <payload-type id='104' clockrate='32000' name='ISAC' />
      <payload-type id='126' clockrate='8000' name='telephone-event' />
      <rtp-hdrext xmlns='urn:xmpp:jingle:apps:rtp:rtp-hdrext:0' id='1' uri='urn:ietf:params:rtp-hdrext:ssrc-audio-level' />
      <rtp-hdrext xmlns='urn:xmpp:jingle:apps:rtp:rtp-hdrext:0' id='5' uri='http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01' />
      <rtcp-mux />
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='959822806' name='jvb-a0'>
        <ssrc-info owner='jvb' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='mixedmslabel mixedlabelaudio0' name='msid' />
      </source>
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='4173594801'>
        <ssrc-info owner='respectivearticlesholdoverly@conference.avstack.onavstack.net/a31b53fe' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='a31b53fe-audio-2 0215c3ee-69cb-4f4d-b0be-a2a0f9d13c6f-2' name='msid' />
      </source>
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='2769410349'>
        <ssrc-info owner='respectivearticlesholdoverly@conference.avstack.onavstack.net/e4a15029' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='e4a15029-audio-2 4984f4b0-e293-4a95-bb28-e3c7f4bb85c5-2' name='msid' />
      </source>
    </description>
    <transport pwd='2skqeu0fthmib2q5kc7ml66phk' ufrag='8un1l1gc8oissf' xmlns='urn:xmpp:jingle:transports:ice-udp:1'>
      <web-socket xmlns='http://jitsi.org/protocol/colibri' url='wss://meet.avstack.io/colibri-ws/sin-2482603c.media.avstack.net/4079b55e1160f1d4/f7e91a40?pwd=2skqeu0fthmib2q5kc7ml66phk' />
      <rtcp-mux />
      <fingerprint setup='actpass' hash='sha-256' xmlns='urn:xmpp:jingle:apps:dtls:0' required='false'>9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20</fingerprint>
      <candidate type='host' protocol='udp' foundation='1' priority='2130706431' network='0' component='1' port='10000' ip='2406:da18:f72:6d01:703c:126:854c:2111' id='61e9feff7f7228a004fb9dbba' generation='0' />
      <candidate type='host' protocol='udp' foundation='2' priority='2130706431' network='0' component='1' port='10000' ip='10.228.1.45' id='3952ff197f7228a00319d7397' generation='0' />
      <candidate type='srflx' protocol='udp' component='1' rel-addr='10.228.1.45' foundation='3' priority='1677724415' network='0' rel-port='10000' port='10000' ip='13.229.216.23' id='44e672f77f7228a00349f4a81' generation='0' />
    </transport>
  </content>
  <content creator='initiator' senders='both' name='video'>
    <description xmlns='urn:xmpp:jingle:apps:rtp:1' media='video'>
      <payload-type id='100' clockrate='90000' name='VP8'>
        <rtcp-fb type='ccm' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='fir' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='pli' />
        <parameter value='800' name='x-google-start-bitrate' />
        <rtcp-fb type='transport-cc' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
      </payload-type>
      <payload-type id='107' clockrate='90000' name='H264'>
        <rtcp-fb type='ccm' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='fir' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='pli' />
        <parameter value='800' name='x-google-start-bitrate' />
        <rtcp-fb type='transport-cc' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
        <parameter value='42e01f;level-asymmetry-allowed=1;packetization-mode=1;' name='profile-level-id' />
      </payload-type>
      <payload-type id='101' clockrate='90000' name='VP9'>
        <rtcp-fb type='ccm' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='fir' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='pli' />
        <parameter value='800' name='x-google-start-bitrate' />
        <rtcp-fb type='transport-cc' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
      </payload-type>
      <payload-type id='96' clockrate='90000' name='rtx'>
        <parameter value='100' name='apt' />
        <rtcp-fb type='ccm' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='fir' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='pli' />
      </payload-type>
      <payload-type id='97' clockrate='90000' name='rtx'>
        <parameter value='101' name='apt' />
        <rtcp-fb type='ccm' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='fir' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' />
        <rtcp-fb type='nack' xmlns='urn:xmpp:jingle:apps:rtp:rtcp-fb:0' subtype='pli' />
      </payload-type>
      <payload-type id='99' clockrate='90000' name='rtx'>
        <parameter value='107' name='apt' />
      </payload-type>
      <rtp-hdrext xmlns='urn:xmpp:jingle:apps:rtp:rtp-hdrext:0' id='3' uri='http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time' />
      <rtp-hdrext xmlns='urn:xmpp:jingle:apps:rtp:rtp-hdrext:0' id='5' uri='http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01' />
      <rtcp-mux />
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='1351679567' name='jvb-v0'>
        <ssrc-info owner='jvb' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='mixedmslabel mixedlabelvideo0' name='msid' />
      </source>
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='2339496998'>
        <ssrc-info owner='respectivearticlesholdoverly@conference.avstack.onavstack.net/a31b53fe' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='a31b53fe-video-2 38dd1275-10f6-4835-925b-c065203f7e8a-2' name='msid' />
      </source>
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='1726149289'>
        <ssrc-info owner='respectivearticlesholdoverly@conference.avstack.onavstack.net/a31b53fe' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='a31b53fe-video-2 38dd1275-10f6-4835-925b-c065203f7e8a-2' name='msid' />
      </source>
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='2735266332'>
        <ssrc-info owner='respectivearticlesholdoverly@conference.avstack.onavstack.net/e4a15029' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='e4a15029-video-2 abac7b09-2fb7-447d-b978-77bb793ed793-2' name='msid' />
      </source>
      <source xmlns='urn:xmpp:jingle:apps:rtp:ssma:0' ssrc='2639030607'>
        <ssrc-info owner='respectivearticlesholdoverly@conference.avstack.onavstack.net/e4a15029' xmlns='http://jitsi.org/jitmeet' />
        <parameter xmlns='urn:xmpp:jingle:apps:rtp:1' value='e4a15029-video-2 abac7b09-2fb7-447d-b978-77bb793ed793-2' name='msid' />
      </source>
      <ssrc-group semantics='FID' xmlns='urn:xmpp:jingle:apps:rtp:ssma:0'>
        <source ssrc='2339496998' />
        <source ssrc='1726149289' />
      </ssrc-group>
      <ssrc-group semantics='FID' xmlns='urn:xmpp:jingle:apps:rtp:ssma:0'>
        <source ssrc='2735266332' />
        <source ssrc='2639030607' />
      </ssrc-group>
    </description>
    <transport pwd='2skqeu0fthmib2q5kc7ml66phk' ufrag='8un1l1gc8oissf' xmlns='urn:xmpp:jingle:transports:ice-udp:1'>
      <web-socket xmlns='http://jitsi.org/protocol/colibri' url='wss://meet.avstack.io/colibri-ws/sin-2482603c.media.avstack.net/4079b55e1160f1d4/f7e91a40?pwd=2skqeu0fthmib2q5kc7ml66phk' />
      <rtcp-mux />
      <fingerprint setup='actpass' hash='sha-256' xmlns='urn:xmpp:jingle:apps:dtls:0' required='false'>9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20</fingerprint>
      <candidate type='host' protocol='udp' foundation='1' priority='2130706431' network='0' component='1' port='10000' ip='2406:da18:f72:6d01:703c:126:854c:2111' id='61e9feff7f7228a004fb9dbba' generation='0' />
      <candidate type='host' protocol='udp' foundation='2' priority='2130706431' network='0' component='1' port='10000' ip='10.228.1.45' id='3952ff197f7228a00319d7397' generation='0' />
      <candidate type='srflx' protocol='udp' component='1' rel-addr='10.228.1.45' foundation='3' priority='1677724415' network='0' rel-port='10000' port='10000' ip='13.229.216.23' id='44e672f77f7228a00349f4a81' generation='0' />
    </transport>
  </content>
  <group semantics='BUNDLE' xmlns='urn:xmpp:jingle:apps:grouping:0'>
    <content name='audio' />
    <content name='video' />
  </group>
  <bridge-session xmlns='http://jitsi.org/protocol/focus' id='0a4d44cc-3c29-4290-889f-b14b7a191464' region='sin' />
</jingle>"#;

const OFFER: &str = r#"v=0
o=- 1662446564308 3 IN IP4 0.0.0.0
s=-
t=0 0
a=msid-semantic: WMS *
a=group:BUNDLE 0 1 2 3 4 5
m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 126
c=IN IP4 0.0.0.0
a=rtpmap:111 opus/48000/2
a=rtpmap:103 ISAC/16000
a=rtpmap:104 ISAC/32000
a=rtpmap:126 telephone-event/8000
a=fmtp:111 minptime=10;useinbandfec=1
a=rtcp:1 IN IP4 0.0.0.0
a=rtcp-fb:111 transport-cc
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=setup:actpass
a=mid:0
a=msid:mixedmslabel mixedlabelaudio0
a=sendrecv
a=ice-ufrag:8un1l1gc8oissf
a=ice-pwd:2skqeu0fthmib2q5kc7ml66phk
a=fingerprint:sha-256 9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20
a=candidate:1 1 udp 2130706431 2406:da18:f72:6d01:703c:126:854c:2111 10000 typ host generation 0
a=candidate:2 1 udp 2130706431 10.228.1.45 10000 typ host generation 0
a=candidate:3 1 udp 1677724415 13.229.216.23 10000 typ srflx raddr 10.228.1.45 rport 10000 generation 0
a=ssrc:959822806 msid:mixedmslabel mixedlabelaudio0
a=rtcp-mux
m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 126
c=IN IP4 0.0.0.0
a=rtpmap:111 opus/48000/2
a=rtpmap:103 ISAC/16000
a=rtpmap:104 ISAC/32000
a=rtpmap:126 telephone-event/8000
a=fmtp:111 minptime=10;useinbandfec=1
a=rtcp:1 IN IP4 0.0.0.0
a=rtcp-fb:111 transport-cc
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=setup:actpass
a=mid:1
a=msid:a31b53fe-audio-2 0215c3ee-69cb-4f4d-b0be-a2a0f9d13c6f-2
a=sendonly
a=ice-ufrag:8un1l1gc8oissf
a=ice-pwd:2skqeu0fthmib2q5kc7ml66phk
a=fingerprint:sha-256 9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20
a=candidate:1 1 udp 2130706431 2406:da18:f72:6d01:703c:126:854c:2111 10000 typ host generation 0
a=candidate:2 1 udp 2130706431 10.228.1.45 10000 typ host generation 0
a=candidate:3 1 udp 1677724415 13.229.216.23 10000 typ srflx raddr 10.228.1.45 rport 10000 generation 0
a=ssrc:4173594801 msid:a31b53fe-audio-2 0215c3ee-69cb-4f4d-b0be-a2a0f9d13c6f-2
a=rtcp-mux
m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 126
c=IN IP4 0.0.0.0
a=rtpmap:111 opus/48000/2
a=rtpmap:103 ISAC/16000
a=rtpmap:104 ISAC/32000
a=rtpmap:126 telephone-event/8000
a=fmtp:111 minptime=10;useinbandfec=1
a=rtcp:1 IN IP4 0.0.0.0
a=rtcp-fb:111 transport-cc
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=setup:actpass
a=mid:2
a=msid:e4a15029-audio-2 4984f4b0-e293-4a95-bb28-e3c7f4bb85c5-2
a=sendonly
a=ice-ufrag:8un1l1gc8oissf
a=ice-pwd:2skqeu0fthmib2q5kc7ml66phk
a=fingerprint:sha-256 9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20
a=candidate:1 1 udp 2130706431 2406:da18:f72:6d01:703c:126:854c:2111 10000 typ host generation 0
a=candidate:2 1 udp 2130706431 10.228.1.45 10000 typ host generation 0
a=candidate:3 1 udp 1677724415 13.229.216.23 10000 typ srflx raddr 10.228.1.45 rport 10000 generation 0
a=ssrc:2769410349 msid:e4a15029-audio-2 4984f4b0-e293-4a95-bb28-e3c7f4bb85c5-2
a=rtcp-mux
m=video 9 UDP/TLS/RTP/SAVPF 100 107 101 96 97 99
c=IN IP4 0.0.0.0
a=rtpmap:100 VP8/90000
a=rtpmap:107 H264/90000
a=rtpmap:101 VP9/90000
a=rtpmap:96 rtx/90000
a=rtpmap:97 rtx/90000
a=rtpmap:99 rtx/90000
a=fmtp:100 x-google-start-bitrate=800
a=fmtp:107 x-google-start-bitrate=800;profile-level-id=42e01f;level-asymmetry-allowed=1;packetization-mode=1;
a=fmtp:101 x-google-start-bitrate=800
a=fmtp:96 apt=100
a=fmtp:97 apt=101
a=fmtp:99 apt=107
a=rtcp:1 IN IP4 0.0.0.0
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=rtcp-fb:100 transport-cc
a=rtcp-fb:107 ccm fir
a=rtcp-fb:107 nack
a=rtcp-fb:107 nack pli
a=rtcp-fb:107 transport-cc
a=rtcp-fb:101 ccm fir
a=rtcp-fb:101 nack
a=rtcp-fb:101 nack pli
a=rtcp-fb:101 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtcp-fb:97 ccm fir
a=rtcp-fb:97 nack
a=rtcp-fb:97 nack pli
a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=setup:actpass
a=mid:3
a=msid:mixedmslabel mixedlabelvideo0
a=sendrecv
a=ice-ufrag:8un1l1gc8oissf
a=ice-pwd:2skqeu0fthmib2q5kc7ml66phk
a=fingerprint:sha-256 9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20
a=candidate:1 1 udp 2130706431 2406:da18:f72:6d01:703c:126:854c:2111 10000 typ host generation 0
a=candidate:2 1 udp 2130706431 10.228.1.45 10000 typ host generation 0
a=candidate:3 1 udp 1677724415 13.229.216.23 10000 typ srflx raddr 10.228.1.45 rport 10000 generation 0
a=ssrc:1351679567 msid:mixedmslabel mixedlabelvideo0
a=rtcp-mux
m=video 9 UDP/TLS/RTP/SAVPF 100 107 101 96 97 99
c=IN IP4 0.0.0.0
a=rtpmap:100 VP8/90000
a=rtpmap:107 H264/90000
a=rtpmap:101 VP9/90000
a=rtpmap:96 rtx/90000
a=rtpmap:97 rtx/90000
a=rtpmap:99 rtx/90000
a=fmtp:100 x-google-start-bitrate=800
a=fmtp:107 x-google-start-bitrate=800;profile-level-id=42e01f;level-asymmetry-allowed=1;packetization-mode=1;
a=fmtp:101 x-google-start-bitrate=800
a=fmtp:96 apt=100
a=fmtp:97 apt=101
a=fmtp:99 apt=107
a=rtcp:1 IN IP4 0.0.0.0
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=rtcp-fb:100 transport-cc
a=rtcp-fb:107 ccm fir
a=rtcp-fb:107 nack
a=rtcp-fb:107 nack pli
a=rtcp-fb:107 transport-cc
a=rtcp-fb:101 ccm fir
a=rtcp-fb:101 nack
a=rtcp-fb:101 nack pli
a=rtcp-fb:101 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtcp-fb:97 ccm fir
a=rtcp-fb:97 nack
a=rtcp-fb:97 nack pli
a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=setup:actpass
a=mid:4
a=msid:a31b53fe-video-2 38dd1275-10f6-4835-925b-c065203f7e8a-2
a=sendonly
a=ice-ufrag:8un1l1gc8oissf
a=ice-pwd:2skqeu0fthmib2q5kc7ml66phk
a=fingerprint:sha-256 9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20
a=candidate:1 1 udp 2130706431 2406:da18:f72:6d01:703c:126:854c:2111 10000 typ host generation 0
a=candidate:2 1 udp 2130706431 10.228.1.45 10000 typ host generation 0
a=candidate:3 1 udp 1677724415 13.229.216.23 10000 typ srflx raddr 10.228.1.45 rport 10000 generation 0
a=ssrc:2339496998 msid:a31b53fe-video-2 38dd1275-10f6-4835-925b-c065203f7e8a-2
a=ssrc:1726149289 msid:a31b53fe-video-2 38dd1275-10f6-4835-925b-c065203f7e8a-2
a=ssrc-group:FID 2339496998 1726149289
a=rtcp-mux
m=video 9 UDP/TLS/RTP/SAVPF 100 107 101 96 97 99
c=IN IP4 0.0.0.0
a=rtpmap:100 VP8/90000
a=rtpmap:107 H264/90000
a=rtpmap:101 VP9/90000
a=rtpmap:96 rtx/90000
a=rtpmap:97 rtx/90000
a=rtpmap:99 rtx/90000
a=fmtp:100 x-google-start-bitrate=800
a=fmtp:107 x-google-start-bitrate=800;profile-level-id=42e01f;level-asymmetry-allowed=1;packetization-mode=1;
a=fmtp:101 x-google-start-bitrate=800
a=fmtp:96 apt=100
a=fmtp:97 apt=101
a=fmtp:99 apt=107
a=rtcp:1 IN IP4 0.0.0.0
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=rtcp-fb:100 transport-cc
a=rtcp-fb:107 ccm fir
a=rtcp-fb:107 nack
a=rtcp-fb:107 nack pli
a=rtcp-fb:107 transport-cc
a=rtcp-fb:101 ccm fir
a=rtcp-fb:101 nack
a=rtcp-fb:101 nack pli
a=rtcp-fb:101 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtcp-fb:97 ccm fir
a=rtcp-fb:97 nack
a=rtcp-fb:97 nack pli
a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=setup:actpass
a=mid:5
a=msid:e4a15029-video-2 abac7b09-2fb7-447d-b978-77bb793ed793-2
a=sendonly
a=ice-ufrag:8un1l1gc8oissf
a=ice-pwd:2skqeu0fthmib2q5kc7ml66phk
a=fingerprint:sha-256 9F:21:D6:E6:40:52:7E:10:25:24:B9:A1:E4:AD:E0:03:B4:9A:1A:91:5F:D0:EA:89:D7:7B:E3:8B:92:16:3D:20
a=candidate:1 1 udp 2130706431 2406:da18:f72:6d01:703c:126:854c:2111 10000 typ host generation 0
a=candidate:2 1 udp 2130706431 10.228.1.45 10000 typ host generation 0
a=candidate:3 1 udp 1677724415 13.229.216.23 10000 typ srflx raddr 10.228.1.45 rport 10000 generation 0
a=ssrc:2735266332 msid:e4a15029-video-2 abac7b09-2fb7-447d-b978-77bb793ed793-2
a=ssrc:2639030607 msid:e4a15029-video-2 abac7b09-2fb7-447d-b978-77bb793ed793-2
a=ssrc-group:FID 2735266332 2639030607
a=rtcp-mux"#;

const SESSION_ACCEPT: &str = r#"<jingle action="session-accept" initiator="respectivearticlesholdoverly@conference.avstack.onavstack.net/focus" responder="f7e91a40-063d-4f11-9423-b31f5228d8e9@avstack.onavstack.net/3GQxy6tBgrIe" sid="5rpgqh5rbclco" xmlns="urn:xmpp:jingle:1">
  <group semantics="BUNDLE" xmlns="urn:xmpp:jingle:apps:grouping:0">
    <content name="audio" />
    <content name="video" />
  </group>
  <content creator="responder" name="audio" senders="both">
    <description media="audio" ssrc="721497566" xmlns="urn:xmpp:jingle:apps:rtp:1">
      <payload-type channels="2" clockrate="48000" id="111" name="opus">
        <parameter name="minptime" value="10" />
        <parameter name="useinbandfec" value="1" />
        <rtcp-fb type="transport-cc" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
      </payload-type>
      <payload-type channels="1" clockrate="16000" id="103" name="ISAC" />
      <payload-type channels="1" clockrate="32000" id="104" name="ISAC" />
      <payload-type channels="1" clockrate="8000" id="126" name="telephone-event" />
      <source ssrc="721497566" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-audio-1 4a1ff5d6-a2a3-4bc0-94ac-d156ec0f04d3-1" />
      </source>
      <rtcp-mux />
      <rtp-hdrext id="1" uri="urn:ietf:params:rtp-hdrext:ssrc-audio-level" xmlns="urn:xmpp:jingle:apps:rtp:rtp-hdrext:0" />
      <rtp-hdrext id="5" uri="http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01" xmlns="urn:xmpp:jingle:apps:rtp:rtp-hdrext:0" />
    </description>
    <transport pwd="ObUbT6uj3zlm0ZqFvRIR5ktm" ufrag="pn49" xmlns="urn:xmpp:jingle:transports:ice-udp:1">
      <fingerprint hash="sha-256" setup="active" xmlns="urn:xmpp:jingle:apps:dtls:0">27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C</fingerprint>
    </transport>
  </content>
  <content creator="responder" name="video" senders="both">
    <description media="video" ssrc="1208574038" xmlns="urn:xmpp:jingle:apps:rtp:1">
      <payload-type channels="1" clockrate="90000" id="100" name="VP8">
        <parameter name="x-google-start-bitrate" value="800" />
        <rtcp-fb subtype="fir" type="ccm" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb subtype="pli" type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="transport-cc" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
      </payload-type>
      <payload-type channels="1" clockrate="90000" id="96" name="rtx">
        <parameter name="apt" value="100" />
        <rtcp-fb subtype="fir" type="ccm" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb subtype="pli" type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
      </payload-type>
      <payload-type channels="1" clockrate="90000" id="101" name="VP9">
        <parameter name="x-google-start-bitrate" value="800" />
        <rtcp-fb subtype="fir" type="ccm" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb subtype="pli" type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="transport-cc" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
      </payload-type>
      <payload-type channels="1" clockrate="90000" id="97" name="rtx">
        <parameter name="apt" value="101" />
        <rtcp-fb subtype="fir" type="ccm" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb subtype="pli" type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
      </payload-type>
      <payload-type channels="1" clockrate="90000" id="107" name="H264">
        <parameter name="level-asymmetry-allowed" value="1" />
        <parameter name="packetization-mode" value="1" />
        <parameter name="profile-level-id" value="42e01f" />
        <parameter name="x-google-start-bitrate" value="800" />
        <rtcp-fb subtype="fir" type="ccm" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb subtype="pli" type="nack" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
        <rtcp-fb type="transport-cc" xmlns="urn:xmpp:jingle:apps:rtp:rtcp-fb:0" />
      </payload-type>
      <payload-type channels="1" clockrate="90000" id="99" name="rtx">
        <parameter name="apt" value="107" />
      </payload-type>
      <source ssrc="1208574038" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-video-1 1e12a73d-1462-4c3a-8cf3-d3711120ca74-1" />
      </source>
      <source ssrc="316749150" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-video-1 1e12a73d-1462-4c3a-8cf3-d3711120ca74-1" />
      </source>
      <source ssrc="1624223237" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-video-1 1e12a73d-1462-4c3a-8cf3-d3711120ca74-1" />
      </source>
      <source ssrc="2944900311" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-video-1 1e12a73d-1462-4c3a-8cf3-d3711120ca74-1" />
      </source>
      <source ssrc="487584165" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-video-1 1e12a73d-1462-4c3a-8cf3-d3711120ca74-1" />
      </source>
      <source ssrc="1494453719" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <parameter name="msid" value="f7e91a40-video-1 1e12a73d-1462-4c3a-8cf3-d3711120ca74-1" />
      </source>
      <ssrc-group semantics="FID" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <source ssrc="1208574038" />
        <source ssrc="316749150" />
      </ssrc-group>
      <ssrc-group semantics="SIM" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <source ssrc="1208574038" />
        <source ssrc="1624223237" />
        <source ssrc="2944900311" />
      </ssrc-group>
      <ssrc-group semantics="FID" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <source ssrc="1624223237" />
        <source ssrc="487584165" />
      </ssrc-group>
      <ssrc-group semantics="FID" xmlns="urn:xmpp:jingle:apps:rtp:ssma:0">
        <source ssrc="2944900311" />
        <source ssrc="1494453719" />
      </ssrc-group>
      <rtcp-mux />
      <rtp-hdrext id="3" uri="http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time" xmlns="urn:xmpp:jingle:apps:rtp:rtp-hdrext:0" />
      <rtp-hdrext id="5" uri="http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01" xmlns="urn:xmpp:jingle:apps:rtp:rtp-hdrext:0" />
    </description>
    <transport pwd="ObUbT6uj3zlm0ZqFvRIR5ktm" ufrag="pn49" xmlns="urn:xmpp:jingle:transports:ice-udp:1">
      <fingerprint hash="sha-256" setup="active" xmlns="urn:xmpp:jingle:apps:dtls:0">27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C</fingerprint>
    </transport>
  </content>
</jingle>"#;

const ANSWER: &str = r#"v=0
o=- 457839365782951393 2 IN IP4 127.0.0.1
s=-
t=0 0
a=group:BUNDLE 0 1 2 3 4 5
a=msid-semantic: WMS
m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 126
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:pn49
a=ice-pwd:ObUbT6uj3zlm0ZqFvRIR5ktm
a=ice-options:trickle
a=fingerprint:sha-256 27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C
a=setup:active
a=mid:0
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=sendrecv
a=msid:- 4a1ff5d6-a2a3-4bc0-94ac-d156ec0f04d3
a=rtcp-mux
a=rtpmap:111 opus/48000/2
a=rtcp-fb:111 transport-cc
a=fmtp:111 minptime=10;useinbandfec=1
a=rtpmap:103 ISAC/16000
a=rtpmap:104 ISAC/32000
a=rtpmap:126 telephone-event/8000
a=ssrc:721497566 cname:dSc2HNsYuQ+Phbpe
m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 126
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:pn49
a=ice-pwd:ObUbT6uj3zlm0ZqFvRIR5ktm
a=ice-options:trickle
a=fingerprint:sha-256 27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C
a=setup:active
a=mid:1
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=recvonly
a=rtcp-mux
a=rtpmap:111 opus/48000/2
a=rtcp-fb:111 transport-cc
a=fmtp:111 minptime=10;useinbandfec=1
a=rtpmap:103 ISAC/16000
a=rtpmap:104 ISAC/32000
a=rtpmap:126 telephone-event/8000
m=audio 9 UDP/TLS/RTP/SAVPF 111 103 104 126
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:pn49
a=ice-pwd:ObUbT6uj3zlm0ZqFvRIR5ktm
a=ice-options:trickle
a=fingerprint:sha-256 27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C
a=setup:active
a=mid:2
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=recvonly
a=rtcp-mux
a=rtpmap:111 opus/48000/2
a=rtcp-fb:111 transport-cc
a=fmtp:111 minptime=10;useinbandfec=1
a=rtpmap:103 ISAC/16000
a=rtpmap:104 ISAC/32000
a=rtpmap:126 telephone-event/8000
m=video 9 UDP/TLS/RTP/SAVPF 100 96 101 97 107 99
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:pn49
a=ice-pwd:ObUbT6uj3zlm0ZqFvRIR5ktm
a=ice-options:trickle
a=fingerprint:sha-256 27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C
a=setup:active
a=mid:3
a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=sendrecv
a=msid:- 1e12a73d-1462-4c3a-8cf3-d3711120ca74
a=rtcp-mux
a=rtpmap:100 VP8/90000
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=rtcp-fb:100 transport-cc
a=fmtp:100 x-google-start-bitrate=800
a=rtpmap:96 rtx/90000
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=fmtp:96 apt=100
a=rtpmap:101 VP9/90000
a=rtcp-fb:101 ccm fir
a=rtcp-fb:101 nack
a=rtcp-fb:101 nack pli
a=rtcp-fb:101 transport-cc
a=fmtp:101 x-google-start-bitrate=800
a=rtpmap:97 rtx/90000
a=rtcp-fb:97 ccm fir
a=rtcp-fb:97 nack
a=rtcp-fb:97 nack pli
a=fmtp:97 apt=101
a=rtpmap:107 H264/90000
a=rtcp-fb:107 ccm fir
a=rtcp-fb:107 nack
a=rtcp-fb:107 nack pli
a=rtcp-fb:107 transport-cc
a=fmtp:107 ;level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f;x-google-start-bitrate=800
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=107
a=ssrc-group:FID 1208574038 316749150
a=ssrc:1208574038 cname:dSc2HNsYuQ+Phbpe
a=ssrc:316749150 cname:dSc2HNsYuQ+Phbpe
m=video 9 UDP/TLS/RTP/SAVPF 100 107 101 96 97 99
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:pn49
a=ice-pwd:ObUbT6uj3zlm0ZqFvRIR5ktm
a=ice-options:trickle
a=fingerprint:sha-256 27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C
a=setup:active
a=mid:4
a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=recvonly
a=rtcp-mux
a=rtpmap:100 VP8/90000
a=rtcp-fb:100 transport-cc
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=rtpmap:107 H264/90000
a=rtcp-fb:107 transport-cc
a=rtcp-fb:107 ccm fir
a=rtcp-fb:107 nack
a=rtcp-fb:107 nack pli
a=fmtp:107 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f
a=rtpmap:101 VP9/90000
a=rtcp-fb:101 transport-cc
a=rtcp-fb:101 ccm fir
a=rtcp-fb:101 nack
a=rtcp-fb:101 nack pli
a=fmtp:101 profile-id=0
a=rtpmap:96 rtx/90000
a=fmtp:96 apt=100
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=101
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=107
m=video 9 UDP/TLS/RTP/SAVPF 100 107 101 96 97 99
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:pn49
a=ice-pwd:ObUbT6uj3zlm0ZqFvRIR5ktm
a=ice-options:trickle
a=fingerprint:sha-256 27:23:19:CB:18:88:3D:8F:AB:2C:51:8D:86:70:96:BA:6A:56:A2:20:1A:AE:7F:80:16:27:2A:67:16:3C:56:8C
a=setup:active
a=mid:5
a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:5 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=recvonly
a=rtcp-mux
a=rtpmap:100 VP8/90000
a=rtcp-fb:100 transport-cc
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=rtpmap:107 H264/90000
a=rtcp-fb:107 transport-cc
a=rtcp-fb:107 ccm fir
a=rtcp-fb:107 nack
a=rtcp-fb:107 nack pli
a=fmtp:107 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f
a=rtpmap:101 VP9/90000
a=rtcp-fb:101 transport-cc
a=rtcp-fb:101 ccm fir
a=rtcp-fb:101 nack
a=rtcp-fb:101 nack pli
a=fmtp:101 profile-id=0
a=rtpmap:96 rtx/90000
a=fmtp:96 apt=100
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=101
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=107
"#;
