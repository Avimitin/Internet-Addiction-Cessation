use auto_domain_blocker::file::host_file;

fn main() {
    let path = "./fixtures/hosts.txt";
    let host_file = host_file::HostFile::new(path).unwrap();
    println!("File: {}\n{}", host_file.which(), host_file.cat());
}

