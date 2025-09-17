pub enum ChannelData {
	Write(String),
	Terminate,
}

impl From<&str> for ChannelData {
	fn from(value: &str) -> Self {
		Self::Write(value.to_string())
	}
}

impl From<String> for ChannelData {
	fn from(value: String) -> Self {
		Self::Write(value)
	}
}
