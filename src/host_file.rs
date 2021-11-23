#[allow(dead_code)]
pub struct HostFile {
    // location point to host file's file path
    location: String,
    // contents contain buffer of the host file
    contents: String,
    // bound_index store the start and end bound of the generated contents
    bound_index: Option<(u32, u32)>,
}

#[allow(dead_code)]
use std::error::Error;
impl HostFile {
    // new read conents from given file path and return a HostFile instance if
    // read success.
    pub fn new(file_path: &'static str) -> Result<HostFile, Box<dyn Error>> {
        let contents = std::fs::read_to_string(file_path)?;
        let bound_index = HostFile::find_bound_index(&contents);
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

    fn find_bound_index(input: &String) -> Option<(u32, u32)> {
        let mut bound = (0, 0);
        let mut i: u32 = 0;
        let mut inside: bool = false;
        for line in input.lines() {
            i+=1;
            if line.contains("## <!-- auto domain blocker -->") {
                if !inside {
                    bound.0 = i;
                    inside = true;
                } else {
                    bound.1 = i;
                    return Some(bound);
                }
            }
        }

        return None;
    }

    pub fn read_bound_index(&self) -> Option<(u32, u32)> {
        self.bound_index
    }

    fn exclude_domains(&self) -> String {
        let mut s = String::new();
        // return all contents when there is no bound exist
        if let None = self.read_bound_index() {
            return self.contents.clone();
        }

        let (i, j) = self.read_bound_index().unwrap();

        let mut cur = 1;
        for line in self.contents.lines() {
            if cur < i || cur > j {
                s.push_str(line);
                s.push('\n');
            }
            cur+=1;
        }

        return s;
    }

    pub fn remove(&self) -> Result<(), Box<dyn Error>> {
        let orig_content = self.exclude_domains();
        std::fs::write(self.which(), orig_content)
            .expect(format!("Write {} fail", self.which()).as_str());

        Ok(())
    }
}

#[test]
fn test_exclude_domains() {
    let hf = HostFile::new("./fixtures/hosts.txt").unwrap();
    let s = hf.exclude_domains();
    assert_eq!("127.0.0.1 localhost", s);
}

