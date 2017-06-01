pub struct Diagnostics<'a> {
	current_method:&'a str
}

impl Diagnostics<'a> {

	pub fn new<'a>() -> Diagnostics<'a> {
		Diagnostics {
			current_method:"none"
		}
	}
	
	pub fn enter_method<S: Into<String>>(&mut self, name:S) {
	    self.current_method = name.into()
	}

    pub fn exit_method<S: Into<String>>(&mut self, name:S) {
	
	}
}

