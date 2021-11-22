#[allow(dead_code)]
pub struct HostFile {
    // location point to host file's file path
    location: String,
    // contents contain buffer of the host file
    contents: String,
}

impl HostFile {
    // new read conents from given file path and return a HostFile instance if
    // read success.
    pub fn new(file_path: &'static str) -> Result<HostFile, String> {
        let contents = std::fs::read(file_path);
        let contents = match contents {
            Ok(contents) => match String::from_utf8(contents) {
                Ok(buffer) => buffer,
                Err(err) => {
                    return Err(format!(
                        "File {} contains invalid contents: {}",
                        file_path, err
                    ))
                }
            },
            Err(err) => return Err(format!("Read file '{}': {:?}'", file_path, err)),
        };

        Ok(HostFile {
            contents,
            location: file_path.to_string(),
        })
    }

    // cat return a copy of the file inner contents
    pub fn cat(&self) -> &str {
        &self.contents
    }

    // which return the file path of this host file
    pub fn which(&self) -> &str {
        &self.location
    }
}
