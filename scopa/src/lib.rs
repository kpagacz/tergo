wit_bindgen::generate!({
    world: "tergo"
});

struct Tergo;

impl Guest for Tergo {
    fn format(_code: String) -> Result<String, String> {
        Ok("Hi formatted code".to_owned())
    }
}

export!(Tergo);
