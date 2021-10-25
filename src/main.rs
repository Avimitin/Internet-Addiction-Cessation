use auto_domain_blocker::file;

fn main() {
    let path = "./fixtures/hosts.txt";
    let host_file = file::HostFile::new(path);
    println!("{}", host_file.unwrap().cat());
}

