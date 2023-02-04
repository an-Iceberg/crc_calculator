#[cfg(test)]

#[path ="../src/crc.rs"]
mod crc;

// TODO: more tests

#[test]
#[should_panic]
fn invalid_generator_polynomial()
{
  crc::create_tailbits(vec![0, 0, 0, 1], vec![0, 1, 1, 1]);
}

#[test]
fn crc_2()
{
  assert_eq!(crc::create_tailbits(vec![1], vec![1, 1, 1]), vec![1, 1]);
  assert_eq!(crc::create_tailbits(vec![1, 0, 1], vec![1, 1, 1]), vec![0, 1]);
}

#[test]
fn crc_3()
{
  assert_eq!(crc::create_tailbits(vec![0, 1, 1], vec![1, 0, 1, 1]), vec![1, 0, 1]);
  assert_eq!(crc::create_tailbits(vec![1, 1, 1, 1, 0, 1, 1], vec![1, 0, 1, 0]), vec![1, 1, 0]);
}

#[test]
fn crc_4()
{
  assert_eq!(crc::create_tailbits(vec![0, 1], vec![1, 0, 0, 1, 1]), vec![0, 0, 1, 1]);
  assert_eq!(crc::create_tailbits(vec![1, 0, 0, 1, 0, 1], vec![1, 0, 0, 1, 1]), vec![0, 1, 0, 1]);
  assert_eq!(crc::create_tailbits(vec![0, 0, 0, 1, 0, 0, 1, 0, 1], vec![1, 0, 0, 1, 1]), vec![0, 1, 0, 1]);
  assert_eq!(crc::create_tailbits(vec![0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1], vec![1, 0, 0, 1, 1]), vec![0, 0, 0, 1]);
}
