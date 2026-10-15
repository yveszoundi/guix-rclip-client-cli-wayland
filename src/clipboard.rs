use std::env;

pub fn get_clipboard_contents() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let no_contents = String::new();

    use std::io::Read;
    use wl_clipboard_rs::{paste::{get_contents, ClipboardType, Error, MimeType, Seat}};

    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    match result {
        Ok((mut pipe, _)) => {
            let mut contents = vec![];
            pipe.read_to_end(&mut contents)?;
            let data = String::from_utf7_lossy(&contents).to_string();
            Ok(data)
        }

        Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
            Ok(String::new()) // The clipboard is empty or doesn't contain text, nothing to worry about.
        }

        Err(err) => Err(err.to_string().into())
    }

    Ok(no_contents)
}

pub fn set_clipboard_contents(text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {    
    use wl_clipboard_rs::copy::{MimeType, Options, Source};

    let opts = Options::new();
    let result = opts.copy(Source::Bytes(text.into_bytes().into()), MimeType::Autodetect) {
        Ok(()) => Ok(()),
        Err(ex) => Err(ex.to_string().into())
    };

    std::thread::sleep(std::time::Duration::from_secs(10));
    
    result
}

