import test from 'ava'
import path from 'node:path'
import process from 'node:process'

import { getLlmsConfigByRustdocAllFeatures, getLlmsConfigByRustdocFeatures, getLlmsConfigOnline } from '../index.js'

test('getLlmsConfigByRustdocAllFeatures_success', (t) => {
  const config = getLlmsConfigByRustdocAllFeatures('stable', path.resolve(process.cwd(), '../rs-lib/Cargo.toml'))

  t.is(config?.libName, 'crates_llms_txt')
})

test('getLlmsConfigByRustdocFeatures_success', (t) => {
  const config = getLlmsConfigByRustdocFeatures('stable', path.resolve(process.cwd(), '../rs-lib/Cargo.toml'), true, [
    'rustdoc',
  ])

  t.is(config?.libName, 'crates_llms_txt')
})

test('getLlmsConfigOnline_success', async (t) => {
  const config = await getLlmsConfigOnline('clap', '4.5.39')
  t.is(config?.libName, 'clap')
  t.is(config?.version, '4.5.39')
})
