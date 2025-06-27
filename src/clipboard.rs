use std::env;

pub fn get_clipboard_contents() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let no_contents = String::new();

    use std::io::Read;
    use wl_clipboard_rs::{paste::{get_contents, ClipboardType, Error, MimeType, Seat}};

    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    match result {
        Ok((mut pipe, _)) => {
            let mut contents = Vec::with_capacity(1024);

            if pipe.read_to_end(&mut contents).is_ok() {
                return Ok(String::from_utf8(contents).unwrap());
            }
        }

        Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
            // The clipboard is empty or doesn't contain text, nothing to worry about.
        }

        Err(err) => {}
    }

    Ok(no_contents)
}

pub fn set_clipboard_contents(text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {    
    use wl_clipboard_rs::copy::{MimeType, Options, Source};

    let opts = Options::new();
    let _ = opts.copy(Source::Bytes(text.into_bytes().into()), MimeType::Text);


    Ok(())
}

