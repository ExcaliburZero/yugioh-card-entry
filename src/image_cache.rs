extern crate url;

use std::fs;
use std::io::Write;

use url::*;

pub struct ImageCache {
    pub cache_path: String,
}

impl ImageCache {
    fn get_image_path(&self, image_filename: &str) -> String {
        format!("{}/{}", self.cache_path, image_filename)
    }

    fn download_image(&self, image_url: &str, image_filepath: &str) {
        let mut file = fs::File::create(image_filepath).unwrap();
        let response = reqwest::blocking::get(image_url).unwrap();

        file.write_all(&response.bytes().unwrap()).unwrap();
    }

    pub fn get_image(&self, image_url: &str) -> Result<String, String> {
        let image_filename: String = Url::parse(image_url)
            .unwrap()
            .path_segments()
            .unwrap()
            .last()
            .unwrap()
            .to_string();

        let image_path = self.get_image_path(&image_filename);

        if fs::metadata(&image_path).is_err() {
            self.download_image(image_url, &image_path);
        }

        Ok(image_path)
    }
}
