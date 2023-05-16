import test from 'ava'

import { sum } from '../index.js'

test('sum from native', (t) => {
  const obj = { value: 1 }
  t.is(sum(obj, 2), 5)
})
