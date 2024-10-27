use tergo_lib::tergo_format;

wit_bindgen::generate!({
    world: "tergo"
});

struct Tergo;

impl Guest for Tergo {
    fn format(code: String) -> Result<String, String> {
        tergo_format(&code, None)
    }
}

export!(Tergo);
