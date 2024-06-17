use url::Url;
use std::error::Error;
use std::fs;

fn is_url(path: &str) -> bool {
    match Url::parse(path) {
        Ok(url) => match url.scheme() {
            "http" => true,
            "https" => true,
            _ => false,
        },
        Err(_) => false,
    }
}

fn download_url(path: &str) -> reqwest::Result<String> {
    let client = reqwest::blocking::Client::new();
    client.get(path)
        .header(reqwest::header::ACCEPT, "text/markdown")
        .send()?
        .error_for_status()?
        .text()
}

pub fn get_by_path(path: &str) -> Result<String, Box<dyn Error>> {
    match is_url(path) {
        true => Ok(download_url(path)?),
        false => Ok(fs::read_to_string(path)?),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_url() {
        assert_eq!(is_url(""), false);
        assert_eq!(is_url("file.md"), false);
        assert_eq!(is_url("/path/to/file.md"), false);
        assert_eq!(is_url("../path/to/file.md"), false);

        assert_eq!(is_url("http://foo/bar"), true);
        assert_eq!(is_url("https://foo/bar"), true);
        assert_eq!(is_url("ftp://foo/bar"), false);
    }

    #[test]
    fn test_download_url() {
        let range5 = download_url("http://httpbin.org/range/5");
        assert!(range5.is_ok());
        assert_eq!(range5.unwrap(), "abcde");

        let not_an_url = download_url("not_an_url");
        assert!(not_an_url.is_err());

        let not_found = download_url("http://httpbin.org/status/404");
        assert!(not_found.is_err());
    }
}
