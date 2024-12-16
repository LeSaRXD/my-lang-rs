use std::{collections::HashMap, fmt::Display};

pub(crate) fn hashmap_to_string<K: Display, V: Display>(map: &HashMap<K, V>) -> String {
	format!(
		"{{\n{}\n}}",
		map.iter()
			.map(|(ident, value)| format!("{ident}: {value}"))
			.collect::<Box<[_]>>()
			.join(",\n")
	)
}

pub(crate) fn iter_to_string<I, V: Display>(it: I) -> String
where
	I: IntoIterator<Item = V>,
{
	format!(
		"[\n{}\n]",
		it.into_iter()
			.map(|v| v.to_string())
			.collect::<Box<[_]>>()
			.join(",\n")
	)
}
