use auto_domain_blocker::file;

fn main() {
    let path = "./fixtures/hosts.txt";
    println!("{}", file::read_file_into_string(path).unwrap());
}

