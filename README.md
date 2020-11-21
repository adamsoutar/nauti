<p align="center">
  <img src='./img/logo.png' height="170" />
</p>

<p align="center">
  <b>The fastest way to rid your strings of naughty words, supercharged by Rust</b>
</p>

## The What

nauti is a profanity filter/swear censor for NodeJS that's designed to be faster than other modules on offer.

## The How

For the sake of keeping the readme nice and clean, let's pretend 'Nasty' and 'Ugly' are swear words

```js
const nauti = require('nauti')

const dodgyStrings = ['Perfectly fine', 'A nasty phrase', 'Very ugly words']
nauti.cleanStrings(dodgyStrings)

// ['Perfectly fine', 'A ***** phrase', 'Very **** words']

const dodgyLeaderboard = [{ score: 10, nickname: 'Adam' }, { score: 5, name: 'Nasty name' }]
nauti.cleanObjects(dodgyLeaderboard, 'nickname')

// [{ score: 10, nickname: 'Adam' }, { score: 5, name: '***** name' }]
```

## The Why

nauti was developed to filter the leaderboard for a game I developed.

I was originally using `bad-words-plus`, but when the leaderboard grew to around fifteen thousand objects, filtering the `nickname` attribute was taking 11 seconds! Players are likely to abandon the leaderboard viewer if it loads for 11 seconds.

nauti filters that same board over five and a half times faster

```
bad-words-plus: 10.954s
nauti: 2.345s
```
