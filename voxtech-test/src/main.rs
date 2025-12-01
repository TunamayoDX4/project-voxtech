pub mod sector;
use sector::Sector;

pub mod block_server;
pub mod types;

fn main() {
  let sec = [
    Sector::new([0, 0, 0]),
    Sector::new([128, 256, 512]),
    Sector::new([123, 456, 789]),
    Sector::new([123456789, 456789123, 789123456]),
    Sector::new([
      01234567890123456789,
      4567890123456789012,
      7890123456789012345,
    ]),
  ];

  let mut buf = String::new();
  for sec in sec.iter() {
    sec
      .generate_name(&mut buf)
      .unwrap();
    println!("{}", buf);
    buf.clear();
  }
}
