use std::{
	fs::{copy, create_dir_all},
	path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};

macro_rules! error {
	($($msg:expr),*) => {
		{
			eprintln!($($msg),*);
			std::process::exit(1);
		}
    };
}

#[derive(Clone, Copy, ValueEnum)]
enum Os {
	Linux,
	Windows,
	WindowsMsvc,
	Osx,
	OsxM1,
}

impl Os {
	#[allow(unused)]
	const fn is_windows(&self) -> bool {
		matches!(self, Self::Windows | Self::WindowsMsvc)
	}

	#[allow(unused)]
	const fn is_osx(&self) -> bool { matches!(self, Self::Osx | Self::OsxM1) }

	const fn prefix(&self) -> &'static str {
		match self {
			Self::Linux | Self::Osx | Self::OsxM1 => "lib",
			Self::Windows | Self::WindowsMsvc => "",
		}
	}

	const fn suffix(&self) -> &'static str {
		match self {
			Self::Linux => ".so",
			Self::Windows | Self::WindowsMsvc => ".dll",
			Self::Osx | Self::OsxM1 => ".dylib",
		}
	}

	fn name_to_dll_name(&self, name: &str) -> String {
		format!(
			"{}{name}{}",
			self.prefix(),
			self.suffix()
		)
	}

	fn name_to_plugin_name(&self, name: &str, prefix: Option<&str>) -> String {
		format!(
			"{}lib{name}{}",
			prefix.unwrap_or(""),
			self.suffix()
		)
	}

	const fn target_triple(&self) -> &'static str {
		match self {
			Self::Linux => "x86_64-unknown-linux-gnu",
			Self::Windows => "x86_64-pc-windows-gnu",
			Self::WindowsMsvc => "x86_64-pc-windows-msvc",
			Self::Osx => "x86_64-apple-darwin",
			Self::OsxM1 => "aarch64-apple-darwin",
		}
	}

	const fn release_or_debug(is_release: bool) -> &'static str {
		if is_release {
			"release"
		} else {
			"debug"
		}
	}

	fn target_path(&self, is_release: bool) -> PathBuf {
		let mut pbf = PathBuf::new();
		pbf.push("./target");
		pbf.push(self.target_triple());
		pbf.push(Self::release_or_debug(is_release));
		pbf
	}

	fn dll_path(&self, name: &str, is_release: bool) -> PathBuf {
		self.target_path(is_release)
			.join(self.name_to_dll_name(name))
	}

	fn plugin_path<P>(
		&self,
		name: &str,
		plugin_dir: P,
		prefix: Option<&str>,
	) -> PathBuf
	where
		P: AsRef<Path>,
	{
		plugin_dir
			.as_ref()
			.join(self.name_to_plugin_name(name, prefix))
	}
}

#[derive(Parser)]
struct Dll {
	/// Name of the artifact
	name: String,

	/// Target to copy
	#[arg(
		short = 't',
		long = "os",
		value_enum,
		default_value_t = Os::Linux
	)]
	os: Os,

	/// Whether or not to use release mode binary
	#[arg(short = 'r', long = "release")]
	is_release: bool,

	/// Path to unity native plugins directory
	plugin_path: PathBuf,

	/// Make parent directories as needed
	#[arg(short, long)]
	parents: bool,

	/// Prefix to append to the final DLL name
	#[arg(short = 'o', long)]
	prefix: Option<String>,
}

impl Dll {
	#[allow(unused)]
	fn new<T, P>(name: T, os: Os, plugin_path: P) -> Self
	where
		T: AsRef<str>,
		P: AsRef<Path>, {
		Self {
			name: name.as_ref().to_owned(),
			os,
			is_release: true,
			plugin_path: plugin_path.as_ref().to_path_buf(),
			parents: false,
			prefix: None,
		}
	}

	fn dll_path(&self) -> PathBuf {
		self.os
			.dll_path(&self.name, self.is_release)
	}

	fn plugin_path(&self) -> PathBuf {
		self.os.plugin_path(
			&self.name,
			self.plugin_path.as_path(),
			self.prefix.as_deref(),
		)
	}
}

fn main() {
	let dll = Dll::parse();

	let s = dll.dll_path();
	let t = dll.plugin_path();

	match s.try_exists() {
		Ok(true) => (),
		Ok(false) => error!("{s:?} does not exist or is broken symbolic link"),
		Err(e) => match e.kind() {
			k => error!("unexpected error: {k} {s:?}"),
		},
	}

	let tp = match t.parent() {
		Some(p) => p,
		None => error!("you cannot target root"),
	};
	match tp.try_exists() {
		Ok(true) => (),
		Ok(false) if dll.parents => {
			eprintln!("automatically generate path to {tp:?}");
			create_dir_all(tp).unwrap()
		},
		Ok(false) => error!(
			"{:?} does not exist or is broken symbolic link",
			tp
		),
		Err(e) => match e.kind() {
			k => error!("unexpected error: {k} {:?}", tp),
		},
	}

	if matches!(t.try_exists(), Ok(true)) {
		if t.is_dir() {
			error!("{t:?} already exists and is directory")
		} else if t.is_file() {
			eprintln!("{t:?} will be overwritten");
		} else {
			error!("unkown type of existing item: {t:?}");
		}
	}

	copy(s, t).unwrap();
}
