extern crate roadrunner;

pub fn run(input: &str) -> String {
    let engine = roadrunner::engine::Engine::new();
    engine.run(input)
}
