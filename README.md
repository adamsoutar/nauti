<p align="center">
  <img src='./img/logo.png' height="170" />
</p>

<p align="center">
  <b>The fastest way to rid your strings of naughty words, supercharged by Rust</b>
</p>

## The What

nauti is a profanity filter/swear censor for NodeJS that's designed to be faster than other modules on offer.

Most npm package profanity filters use JS regexes to clean serially, **nauti uses native Rust code & parallelism** across all cores to achieve awesome performance.

## The How

For the sake of keeping the readme nice and clean, let's pretend 'Nasty' and 'Ugly' are swear words

```js
const nauti = require('nauti')

const dodgyStrings = ['Perfectly fine', 'A nasty phrase', 'Very ugly words']
const cleanedStrings = nauti.cleanStrings(dodgyStrings)

// ['Perfectly fine', 'A n#$%y phrase', 'Very u!@y words']

const dodgyLeaderboard = [{ score: 10, nickname: 'Adam' }, { score: 5, nickname: 'Nasty name' }]
const cleanedObjects = nauti.cleanObjects(dodgyLeaderboard, 'nickname')

// [{ score: 10, nickname: 'Adam' }, { score: 5, nickname: 'N&!%y name' }]
```

## The Why

nauti was developed to filter the leaderboard for a game I developed.

Once the leaderboard grew to around fifteen-thousand records, `bad-words-plus` - which I was using - began to take an unreasonable amount of time to filter players' usernames. Players are bound to abandon the leaderboard viewer if it takes almost half a minute to load!

nauti filters that same board four-hundred-and-seven times faster on a 16 thread CPU

```
bad-words-plus: 25.295s
nauti: 62.113ms
```
