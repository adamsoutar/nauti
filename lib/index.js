var addon = require('../native');

let modes = ["Arcade", "Ass Crisis", "Sandbox", "Campaign"]

addon.filterStrings(modes)
console.log(modes)

const leaderboard = [
  {
    nickname: 'Richard D James'
  },
  {
    nickname: 'Yukimi Nagano'
  },
  {
    nickname: 'Ass is a bad word'
  },
  {
    nickname: 'Ashnikko'
  }
]

addon.filterObjects(leaderboard, 'nickname')
console.log(leaderboard)
