use xmltree::EmitterConfig;

pub fn format_xml(xml: &str) -> Result<String, ()> {
	let mut v = Vec::with_capacity(xml.len());
	xmltree::Element::parse(xml.as_bytes())
		.map_err(|_| ())?
		.write_with_config(
			&mut v,
			EmitterConfig::new()
				.indent_string("    ")
				.write_document_declaration(false)
				.perform_indent(true),
		)
		.map_err(|_| ())?;
	Ok(String::from_utf8(v).map_err(|_| ())?)
}

pub fn format_xml_unwrap(xml: &str) -> String {
	format_xml(xml).expect("unwrap xml: ({xml})")
}

pub fn format_xml_with_indent(xml: &str, indent: &str) -> Result<String, ()> {
	let x = format_xml(xml).map_err(|_| ())?;

	Ok(x.lines()
		.map(|l| format!("{indent}{l}"))
		.collect::<Vec<_>>()
		.join("\n"))
}

pub fn format_xml_with_indent_unwrap(xml: &str, indent: &str) -> String {
	format_xml_with_indent(&xml, indent).expect("unwrap indented xml: ({xml})")
}
