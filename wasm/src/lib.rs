witx_bindgen_rust::import!("../imports.witx");
witx_bindgen_rust::export!("../exports.witx");

use exports::Error;
use std::io::{self, Read};
use witx_bindgen_rust::Handle;

// Helper macro that's like `println!` in Rust, only goes to our custom `log`
// function we're importing from the environment.
macro_rules! log {
	($($t:tt)*) => (imports::log(&format!($($t)*)))
}

struct Exports;
impl exports::Exports for Exports {}

pub struct Tarball {
    files: Vec<(String, String)>,
}

#[witx_bindgen_rust::async_trait(?Send)]
impl exports::Tarball for Tarball {
    // Note the `async` on this method, which allows processing between two
    // `await` points, the `fetch` and the reading of the body.
    async fn fetch(url: String) -> Result<Handle<Tarball>, exports::Error> {
        init();
        log!("fetching {}", url);
        let response = imports::fetch(&url).await?;

        log!("checking status code");
        if response.status() != 200 {
            let mut msg = format!(
                "expected 200 ok got {} - {}",
                response.status(),
                response.status_text(),
            );
            let headers = response.headers().entries();
            if headers.len() > 0 {
                msg.push_str("\nresponse headers:\n");
                for (name, values) in headers {
                    msg.push_str("\t");
                    msg.push_str(&name);
                    msg.push_str(": ");
                    msg.push_str(&values.join(", "));
                    msg.push_str("\n");
                }
            }
            return Err(Error::BadStatus(msg));
        }

        log!("waiting for the body to arrive");
        let body = response.body().await;

        log!("decompressing and iterating the tarball");
        let input = flate2::read::GzDecoder::new(&body[..]);
        let mut archive = tar::Archive::new(input);
        let mut files = Vec::new();
        for entry in archive.entries()? {
            let mut entry = entry?;
            let name = entry.path()?;
            let name = name
                .to_str()
                .ok_or_else(|| Error::NotUtf8(format!("tarball path name not utf-8: {:?}", name)))?
                .to_string();
            let mut contents = String::new();
            entry.read_to_string(&mut contents)?;
            files.push((name, contents));
        }
        Ok(Tarball { files }.into())
    }

    fn files(&self) -> Vec<String> {
        self.files.iter().map(|pair| pair.0.clone()).collect()
    }

    fn contents(&self, i: u32) -> String {
        self.files[i as usize].1.clone()
    }
}

impl From<imports::Error> for Error {
    fn from(err: imports::Error) -> Error {
        match err {
            imports::Error::Failure(s) => Error::Io(s),
            imports::Error::Aborted => Error::Io("request aborted".to_string()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err.to_string())
    }
}

fn init() {
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            imports::log_err(&info.to_string());
        }))
    });
}
