import test from 'ava'

import { sum, distance } from '../index.js'

test('sum from native', (t) => {
  const obj = { value: 1 }
  t.is(sum(obj, 2), 5)
})

test('distance from native', (t) => {
  t.is(distance([3, 4], [0, 0]), 5)
})
