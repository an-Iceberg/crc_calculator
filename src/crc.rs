pub fn create_tailbits(data_bits: Vec<u8>, generator_polynomial: Vec<u8>) -> Vec<u8>
{

  if *generator_polynomial.first().unwrap() != 1
  {
    panic!("Generator polynomial must start with a 1");
  }

  let mut tail_bits: Vec<u8> = vec![0; generator_polynomial.len()];

  for index in 0..=(generator_polynomial.len() + data_bits.len() - 2)
  {
    // Shifts in one bit from the data bits
    tail_bits.drain(0..1);
    tail_bits.push(match data_bits.get(index)
    {
      Some(bit) => *bit,
      None => 0
    });

    // If the first bit is not a 0
    if *tail_bits.first().unwrap() == 1
    {
      // Then xor all elements with the generator polynomial
      tail_bits = tail_bits
        .iter()
        .zip(generator_polynomial.iter())
        .map(|(&tail_bit, &polynomial_bit)| tail_bit ^ polynomial_bit)
        .collect();
    }
  }

  tail_bits.drain(0..1);
  return tail_bits;
}
