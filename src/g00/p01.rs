// Two sum
use std::collections::HashMap;

struct Solution {}

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rem = HashMap::<i32, usize>::new();

    nums.iter().enumerate().for_each(|(i, x)| {
      rem.insert(target - x, i);
    });

    nums.iter().enumerate().find_map(|(j, y)| {
      let i = rem.get(y)?;
      if *i == j { None } else { Some(vec![j as i32, *i as i32]) }
    }).unwrap().into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn two_sum_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
  }

  #[test]
  fn two_sum_2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
  }

  #[test]
  fn two_sum_3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
  }

}