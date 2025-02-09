use tergo_lib::tergo_format;

wit_bindgen::generate!({
    world: "tergo"
});

struct Tergo;

impl Guest for Tergo {
    fn format(code: String) -> Result<String, String> {
        simple_logger::init_with_env().map_err(|err| format!("Error initializing logger: {:?}", err))?;
        tergo_format(&code, None)
    }
}

export!(Tergo);
