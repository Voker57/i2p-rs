use nom::{alphanumeric, alt, do_parse, named, separated_list, space, tag_s, take_till_s};

fn is_space(chr: char) -> bool {
	chr == ' ' || chr == '\t'
}

fn is_next_line(chr: char) -> bool {
	chr == '\n'
}

fn is_space_or_next_line(chr: char) -> bool {
	is_space(chr) || is_next_line(chr)
}

fn is_double_quote(chr: char) -> bool {
	chr == '\"'
}

named!(quoted_value <&str, &str>,
	do_parse!(
			 tag_s!("\"")                  >>
		val: take_till_s!(is_double_quote) >>
			 tag_s!("\"")                  >>
		(val)
	)
);

named!(value <&str, &str>, take_till_s!(is_space_or_next_line));

named!(key_value <&str, (&str, &str)>,
	do_parse!(
		key: alphanumeric               >>
			 tag_s!("=")                >>
		val: alt!(quoted_value | value) >>
		(key, val)
	)
);

named!(keys_and_values<&str, Vec<(&str, &str)> >, separated_list!(space, key_value));

named!(pub sam_hello <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag_s!("HELLO REPLY ") >>
		opts: keys_and_values        >>
			  tag_s!("\n")           >>
		(opts)
	)
);

named!(pub sam_session_status <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag_s!("SESSION STATUS ") >>
		opts: keys_and_values           >>
			  tag_s!("\n")              >>
		(opts)
	)
);

named!(pub sam_stream_status <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag_s!("STREAM STATUS ") >>
		opts: keys_and_values          >>
			  tag_s!("\n")             >>
		(opts)
	)
);

named!(pub sam_naming_reply <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag_s!("NAMING REPLY ") >>
		opts: keys_and_values         >>
			  tag_s!("\n")            >>
		(opts)
	)
);

named!(pub sam_dest_reply <&str, Vec<(&str, &str)> >,
	do_parse!(
			  tag_s!("DEST REPLY ") >>
		opts: keys_and_values       >>
			  tag_s!("\n")          >>
		(opts)
	)
);

#[cfg(test)]
mod tests {
	use nom::ErrorKind;

	#[test]
	fn hello() {
		use crate::parsers::sam_hello;

		assert_eq!(
			sam_hello("HELLO REPLY RESULT=OK VERSION=3.1\n"),
			Ok(("", vec![("RESULT", "OK"), ("VERSION", "3.1")]))
		);
		assert_eq!(
			sam_hello("HELLO REPLY RESULT=NOVERSION\n"),
			Ok(("", vec![("RESULT", "NOVERSION")]))
		);
		assert_eq!(
			sam_hello("HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Something failed\"\n"),
			Ok((
				"",
				vec![("RESULT", "I2P_ERROR"), ("MESSAGE", "Something failed")]
			))
		);
	}

	#[test]
	fn session_status() {
		use crate::parsers::sam_session_status;

		assert_eq!(
			sam_session_status("SESSION STATUS RESULT=OK DESTINATION=privkey\n"),
			Ok(("", vec![("RESULT", "OK"), ("DESTINATION", "privkey")]))
		);
		assert_eq!(
			sam_session_status("SESSION STATUS RESULT=DUPLICATED_ID\n"),
			Ok(("", vec![("RESULT", "DUPLICATED_ID")]))
		);
	}

	#[test]
	fn stream_status() {
		use crate::parsers::sam_stream_status;

		assert_eq!(
			sam_stream_status("STREAM STATUS RESULT=OK\n"),
			Ok(("", vec![("RESULT", "OK")]))
		);
		assert_eq!(
			sam_stream_status(
				"STREAM STATUS RESULT=CANT_REACH_PEER MESSAGE=\"Can't reach peer\"\n"
			),
			Ok((
				"",
				vec![
					("RESULT", "CANT_REACH_PEER"),
					("MESSAGE", "Can't reach peer")
				]
			))
		);
	}

	#[test]
	fn naming_reply() {
		use crate::parsers::sam_naming_reply;

		assert_eq!(
			sam_naming_reply("NAMING REPLY RESULT=OK NAME=name VALUE=dest\n"),
			Ok((
				"",
				vec![("RESULT", "OK"), ("NAME", "name"), ("VALUE", "dest")]
			))
		);
		assert_eq!(
			sam_naming_reply("NAMING REPLY RESULT=KEY_NOT_FOUND\n"),
			Ok(("", vec![("RESULT", "KEY_NOT_FOUND")]))
		);

		assert_eq!(
			sam_naming_reply("NAMINGREPLY RESULT=KEY_NOT_FOUND\n")
				.unwrap_err()
				.into_error_kind(),
			ErrorKind::Tag
		);
		assert_eq!(
			sam_naming_reply("NAMING  REPLY RESULT=KEY_NOT_FOUND\n")
				.unwrap_err()
				.into_error_kind(),
			ErrorKind::Tag
		);
	}

	#[test]
	fn dest_reply() {
		use crate::parsers::sam_dest_reply;

		assert_eq!(
			sam_dest_reply("DEST REPLY PUB=foo PRIV=foobar\n"),
			Ok(("", vec![("PUB", "foo"), ("PRIV", "foobar")]))
		);
	}
}
