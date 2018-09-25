pub fn collatz(n: u64) -> Option<u64> {
  if n == 1 {
    return Some(0);
  }
  if n == 0 {
    return None;
  }
  let mut num_of_steps = 1;
  let mut num = n;
  loop {
    if num == 1 {
      return Some(num_of_steps - 1);
    }
    println!("{}", num);
    match num % 2 {
      0 => num = num / 2,
      _ => num = 3 * num + 1,
    }
    num_of_steps += 1;
  }
}
