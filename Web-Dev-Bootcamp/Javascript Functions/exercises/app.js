function rollDie(numSides) {
    randomNumber = Math.floor(Math.random() * numSides) +1;
    return randomNumber
}

let diceRoll = rollDie(12)
console.log(diceRoll)

// isShortsWeather
function isShortsWeather(temp){
    if (temp >= 75){
        return true;
    }else {
        return false;
    }
}
// Last Element
function lastElement(arr) {
  if (arr.length === 0) {
    return null;
  } else {
    return arr[arr.length - 1];
  }
}


// Capitalize
function capitalize(str) {
    let firstLetter = str[0].toUpperCase(0)
    let newStr = str.slice(1)
    return firstLetter + newStr
}


// Sum Array
function sumArray(nums) {
  let total = 0;
  for (let num of nums) {
    total += num;
  }
  return total;
}

// Days of the Week
function returnDay(num) {
  const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
  if (num < 1 || num > 7) {
    return null;
  } else {
    return days[num - 1];
  }
}