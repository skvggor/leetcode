// https://leetcode.com/submissions/detail/984756605/

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */

const twoSum = (nums, target) => {
  if (nums.length < 2 && nums.length > Math.pow(10, 4)) {
    return;
  }

  for (let i = 0; i <= nums.length; i += 1) {
    if (nums[i] < Math.pow(-10, 9) && nums[i] > Math.pow(10, 9)) {
      return;
    }
  }

  if (target < Math.pow(-10, 9) && target > Math.pow(10, 9)) {
    return;
  }

  let result = []

  for (let i = 0; i <= nums.length; i += 1) {
    for (let j = 0; j <= nums.length; j += 1) {
      if (i !== j) {
        if (nums[i] + nums[j] === target) {
          result.push(i)
          result.push(j)

          return result
        }
      }
    }
  }
}
