const { rawCleanStrings } = require('../native')

// This may seem weird, but we serialise the arr into a string & parse it again
// in Rust. If we had to interact with the real JS heap, we couldn't use multi
// threading
function cleanStrings (stringArray) {
  const arrJson = JSON.stringify(stringArray)
  return rawCleanStrings(arrJson)
}

module.exports = { cleanStrings }
