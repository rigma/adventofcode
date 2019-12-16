#!/usr/bin/env node

let searchSet = []

// First, we'll create the main password search set by selecting the right number
for (let number = 245182; number <= 790572; ++number) {
  let work = number.toString(10)
    .split('')
    .map(digit => Number.parseInt(digit))

  let doubleDigit = false
  let growing = true

  for (let i = 0; i < 5; ++i) {
    // We're checking that digits are growing
    growing = growing && work[i] <= work[i + 1]

    // Then we'll check if the double digits critera is respected
    if (i > 0) {
      doubleDigit = doubleDigit || work[i] == work[i - 1] || work[i] == work[i + 1]
    }
  }

  // If growing and doubleDigit are true, then we include the number in the search set
  if (growing && doubleDigit) {
    searchSet.push(work)
  }
}

// Then, we'll remove the number that contains no double digits groups
searchSet = searchSet.filter(number => {
  return number.reduce((output, d1) => {
    return output || number.reduce((count, d2) => d1 === d2 ? count + 1 : count, 0) === 2
  }, false)
})

// Displaying answer to the user
console.log(searchSet.length)
