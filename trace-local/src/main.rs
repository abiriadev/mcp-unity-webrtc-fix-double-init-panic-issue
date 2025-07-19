use std::{
	env::var,
	ffi::{c_char, c_void},
	mem::transmute,
	str::FromStr,
	sync::atomic::AtomicPtr,
};

use jitsi_meet_signalling_c::{
	jitsi_connection_connect, jitsi_connection_join, Agent, ColibriMessage,
	Context, Participant,
};
use minidom::Element;
use tokio::runtime::Runtime;
use tracing::{info, Level};
use tracing_subscriber::fmt;
use xmpp_parsers::ns;

macro_rules! cstr {
	($str:literal) => {
		std::ffi::CString::new($str).unwrap()
	};
}

struct MockContext {
	runtime: Runtime,
}

unsafe fn mkctx() -> *mut Context {
	Box::into_raw(Box::new(transmute::<_, Context>(
		MockContext {
			runtime: Runtime::new().unwrap(),
		},
	)))
}

unsafe fn frctx(ctx: *mut Context) {
	if !ctx.is_null() {
		drop(Box::from_raw(ctx))
	}
}

#[repr(C)]
struct MockAgent {
	opaque: AtomicPtr<c_void>,
	participant_joined: Option<extern fn(*mut c_void, *mut Participant)>,
	participant_left: Option<extern fn(*mut c_void, *mut Participant)>,
	colibri_message_received:
		Option<extern fn(*mut c_void, *mut ColibriMessage)>,
	offer_received: Option<extern fn(*mut c_void, *const c_char, bool)>,
	source_added: Option<extern fn(*mut c_void, *const c_char)>,
	session_terminate: Option<extern fn(*mut c_void)>,
}

fn main() {
	println!("Hello, world!");
	fmt()
		.with_max_level(
			var("RUST_LOG")
				.ok()
				.and_then(|s| Level::from_str(&s).ok())
				.unwrap_or(Level::DEBUG),
		)
		.pretty()
		.init();

	// let wsurl = cstr!("wss://abiria-lt-arch.local/xmpp-websocket");
	// let wsurl = cstr!("wss://nextlab.duckdns.org/xmpp-websocket");
	let wsurl = cstr!("wss://meet.jit.si/xmpp-websocket");
	// let xmpp = cstr!("meet.jitsi");
	let xmpp = cstr!("meet.jit.si");

	let ctx = unsafe { mkctx() };

	let conn_p = unsafe {
		jitsi_connection_connect(
			ctx,
			wsurl.as_ptr(),
			xmpp.as_ptr(),
			false,
		)
	};

	let ptr = &mut 5 as *mut _ as *mut c_void;
	let agent = unsafe {
		transmute::<_, Agent>(MockAgent {
			opaque: AtomicPtr::new(ptr),
			participant_joined: None,
			participant_left: None,
			colibri_message_received: None,
			offer_received: None,
			source_added: None,
			session_terminate: None,
		})
	};

	info!("jitsi_connection_join 111111111 ====================");

	let _conf_p = unsafe {
		jitsi_connection_join(
			ctx,
			conn_p,
			cstr!("nativ-confe-21381").as_ptr(),
			cstr!("Abiria").as_ptr(),
			agent,
		)
	};

	unsafe { frctx(ctx) };
}

fn _a() {
	let a = Element::builder(
		"jitsi_participant_codecType",
		ns::DEFAULT_NS,
	)
	.append("a")
	.build();
	let mut buf = vec![];
	a.write_to(&mut buf);

	println!("a: {}", String::from_utf8_lossy(&buf));
}
