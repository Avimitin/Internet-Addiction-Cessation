#[allow(dead_code)]
pub struct HostFile {
    // location point to host file's file path
    pub location: String,
    // contents contain buffer of the host file
    pub contents: String,
    // bound_index store the start and end bound of the generated contents
    pub bound_index: Option<(usize, usize)>,
}

use anyhow::{bail, Context, Result};
impl HostFile {
    // new read conents from given file path and return a HostFile instance if
    // read success.
    pub fn new(file_path: &str) -> Result<HostFile> {
        let contents = std::fs::read_to_string(file_path)
            .with_context(|| format!("could not read file {}", file_path))?;
        let bound_index = HostFile::find_bound_index(&contents);
        Ok(HostFile {
            contents,
            bound_index,
            location: file_path.to_string(),
        })
    }

    fn find_bound_index(input: &str) -> Option<(usize, usize)> {
        let mut bound = (0, 0);
        let mut inside: bool = false;
        for (i, line) in input.lines().enumerate() {
            if !line.contains("## <!-- auto domain blocker -->") {
                continue;
            }

            if inside {
                bound.1 = i;
                return Some(bound);
            }

            bound.0 = i;
            inside = true;
        }

        None
    }

    fn remove_blocks(&self) -> String {
        // return all contents when there is no bound exist
        if self.bound_index.is_none() {
            return self.contents.clone();
        }

        let mut s = String::new();
        let (i, j) = self.bound_index.unwrap();

        for (cur, line) in self.contents.lines().enumerate() {
            if cur < i || cur > j {
                s.push_str(line);
                s.push('\n');
            }
        }

        s
    }

    pub fn recover(&self) -> Result<()> {
        let orig_content = self.remove_blocks();
        std::fs::write(&self.location, &orig_content).with_context(|| {
            format!("Fail to write new contents `{}` when remove", orig_content)
        })?;

        Ok(())
    }

    pub fn generate(&mut self, cfg: &crate::config::Config) -> Result<()> {
        // Do not generate domains when bound founded
        if self.bound_index.is_some() {
            bail!("URL is already blocked");
        }

        let domains = cfg.build_domains();

        let should_be_block = domains.into_iter().fold(
            "## <!-- auto domain blocker -->".to_string(),
            |before, current| format!("{before}\n0.0.0.0 {current}"),
        );

        self.contents.push_str(&should_be_block);
        self.contents
            .push_str("\n## <!-- auto domain blocker -->\n");

        std::fs::write(&self.location, &self.contents)
            .with_context(|| format!("Write contents into {} fail", &self.location))?;

        Ok(())
    }
}

#[test]
fn test_exclude_domains() {
    let hf = HostFile::new("./fixtures/hosts.txt").unwrap();
    let s = hf.remove_blocks();
    assert_eq!("127.0.0.1 localhost\n\n", s);
}
