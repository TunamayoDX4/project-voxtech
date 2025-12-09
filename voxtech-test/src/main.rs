pub mod block_server;
pub mod types;

pub mod world;

fn main() {
  let world = world::World::initialize("New world").unwrap();
}
