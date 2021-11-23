#[allow(dead_code)]
pub struct HostFile {
    // location point to host file's file path
    location: String,
    // contents contain buffer of the host file
    contents: String,
    // bound_index store the start and end bound of the generated contents
    bound_index: (i32, i32),
}

#[allow(dead_code)]
use std::error::Error;
impl HostFile {
    // new read conents from given file path and return a HostFile instance if
    // read success.
    pub fn new(file_path: &'static str) -> Result<HostFile, Box<dyn Error>> {
        let contents = std::fs::read_to_string(file_path)?;
        let bound_index = HostFile::read_bound_index(&contents);
        Ok(HostFile {
            contents,
            bound_index,
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

    fn read_bound_index(input: &String) -> (i32, i32) {
        let mut bound = (0, 0);
        let mut i: i32 = 0;
        let mut inside: bool = false;
        for line in input.lines() {
            i+=1;
            if line.contains("## <!-- auto domain blocker --->") {
                if !inside {
                    bound.0 = i;
                    inside = true;
                } else {
                    bound.1 = i;
                    return bound;
                }
            }
        }

        return bound;
    }
}
