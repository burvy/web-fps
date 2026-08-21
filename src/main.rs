fn main() {
    let digest = std::fs::read_to_string("digest.txt")
        .expect("no generated 'digest.txt' was found in my directory")
        .trim()
        .to_string();
    game::run(digest)
}
