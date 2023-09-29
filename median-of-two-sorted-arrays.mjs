// https://leetcode.com/submissions/detail/997925322/

const findMedianSortedArrays = (nums1, nums2) => {
  if (!(nums1.length >= 0 && nums1.length <= 1000)
    && !(nums2.length >= 0 && nums2.length <= 1000)) {
    return
  }

  if (!(nums1.length + nums2.length >= 1)
    && !(nums1.length + nums2.length <= 2000)) {
    return
  }

  for (let i = 0; i < nums1.length; i += 1) {
    if (!(nums1[i] >= -Math.pow(10, 6)
      && nums1[i] <= Math.pow(10, 6))) {
      return
    }
  }

  for (let i = 0; i < nums2.length; i += 1) {
    if (!(nums2[i] >= -Math.pow(10, 6)
      && (nums2[i] <= Math.pow(10, 6)))) {
      return
    }
  }

  const sortFn = (a, b) => a - b

  const merged = nums1
    .concat(nums2)
    .sort(sortFn)

  const mid = merged.length / 2

  return merged.length % 2 === 0
    ? (merged[mid - 1] + merged[mid]) / 2
    : merged[Math.floor(mid)]
}

console.log(findMedianSortedArrays([], [1]))
