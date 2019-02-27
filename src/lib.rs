pub fn collatz(n: u64) -> Option<u64> {
  match n {
    1 => Some(0),
    0 => None,
    _ => {
      let num_of_steps = (0..)
        .try_fold((n, 0), |(num, num_of_steps), _| match num {
          1 => Err(num_of_steps),
          x if x % 2 == 0 => Ok((num / 2, num_of_steps + 1)),
          _ => Ok((3 * num + 1, num_of_steps + 1)),
        })
        .err()
        .unwrap();

      Some(num_of_steps)
    }
  }
}
