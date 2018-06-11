extern crate roadrunner;

pub fn run(input: &str) -> String {
    let mut engine = roadrunner::engine::Engine::new();
    engine.run(input).unwrap()
}
