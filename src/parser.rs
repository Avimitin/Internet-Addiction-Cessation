pub fn read_section(input: &str) -> String {
    let mut section = String::new();

    let mut mark = HaveMarker::new();
    for line in input.lines() {
        if line.contains("## <!-- auto domain blocker --->") {
            mark.toggle();
            continue;
        }

        if mark.have() {
            section.push_str(line.clone());
            section.push('\n');
        }
    } // line iterate

    section
}

struct HaveMarker {
    have: bool,
}

impl HaveMarker {
    fn new() -> HaveMarker {
        HaveMarker {
            have: false,
        }
    }

    fn toggle(&mut self) {
        self.have = !self.have;
    }

    fn have(&self) -> bool {
        self.have
    }
}
