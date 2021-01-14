fn main() {
    let r = git2::Repository::open(".").unwrap();
    let b = r.blame_file(std::path::Path::new("README.md"), None).unwrap();
    let h = b.get_index(0).unwrap();
    let s = h.final_signature();
    let _boom = s.to_string();
}
