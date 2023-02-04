mod crc;

fn main()
{
  println!("\n  Rust Playground\n");

  /*
  let data_bits: Vec<u8> = vec![0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1];
  let generator_polynomial: Vec<u8> = vec![1, 0, 0, 1, 1];
  */

  let data_bits: Vec<u8> = vec![0, 1, 1];
  let generator_polynomial: Vec<u8> = vec![1, 0, 1, 1];

  let tail_bits = crc::create_tailbits(data_bits, generator_polynomial);

  println!("Tail Bits: {:?}", tail_bits);
}
