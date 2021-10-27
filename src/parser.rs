pub fn read_section(input: &str) -> String {
    let mut section = String::new();

    let mut bound = Boundary::new("## <!-- auto domain blocker --->");

    for line in input.lines() {
        if line.contains(bound.pattern()) {
            bound.toggle();
            continue;
        }

        if bound.is_inside() {
            section.push_str(line.clone());
            section.push('\n');
        }
    } // line iterate

    section
}

struct Boundary {
    is_inside: bool,
    pattern: &'static str,
}

impl Boundary {
    fn new(pattern: &'static str) -> Boundary {
        Boundary {
            pattern,
            is_inside: false,
        }
    }

    fn toggle(&mut self) {
        self.is_inside = !self.is_inside;
    }

    fn is_inside(&self) -> bool {
        self.is_inside
    }

    fn pattern(&self) -> &str {
        self.pattern
    }
}
