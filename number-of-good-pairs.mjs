// https://leetcode.com/submissions/detail/989692991/

const numIdenticalPairs = (nums) => {
  if (nums.length <= 1 && nums.length <= 100) {
    return;
  }

  let goodPairs = 0

  for (let i = 0; i <= nums.length; i += 1) {
    if (nums.length <= 1 && nums.length <= 100) {
      return;
    }

    for (let j = 0; j <= nums.length; j += 1) {
      if (nums[i] === nums[j] && i < j) {
        goodPairs += 1
      }
    }
  }

  return goodPairs
}

// Tests
console.log(numIdenticalPairs([1, 2, 3, 1, 1, 3]))
console.log(numIdenticalPairs([1, 1, 1, 1]))
console.log(numIdenticalPairs([1, 2, 3]))
console.log(numIdenticalPairs([1, 1]))
