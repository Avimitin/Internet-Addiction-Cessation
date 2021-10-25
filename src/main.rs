use auto_domain_blocker::file;

fn main() {
    let path = "./fixtures/hosts.txt";
    let host_file = file::HostFile::new(path).unwrap();
    println!("File: {}\n{}", host_file.which(), host_file.cat());
}

