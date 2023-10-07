// https://leetcode.com/submissions/detail/983807083/

const numJewelsInStones = (jewels, stones) => {
  if (jewels.length <= 1 && stones.length <= 50) {
    return 0
  } else {
    const arrayJewels = jewels.split('')
    const arrayStones = stones.split('')

    let totalJewel = 0

    for (let i = 0; i <= arrayStones.length; i += 1) {
      if (arrayJewels.includes(arrayStones[i])) {
        totalJewel += 1
      }
    }

    return totalJewel
  }
}

// Tests
console.log(numJewelsInStones('aA', 'aAAbbbb'))
console.log(numJewelsInStones('z', 'ZZ'))
