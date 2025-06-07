import test from 'ava'
import path from 'node:path'
import process from 'node:process'

import {
  getLlmsConfigByRustdocAllFeatures,
  getLlmsConfigByRustdocFeatures,
  getLlmsConfigOnlineByCratesName,
  getLlmsConfigOnlineByUrl,
} from '../index.js'

test('getLlmsConfigByRustdocAllFeatures_success', (t) => {
  const config = getLlmsConfigByRustdocAllFeatures('stable', path.join(process.cwd(), 'Cargo.toml'))

  t.is(config?.libName, 'crates_llms_txt_napi')
})

test('getLlmsConfigByRustdocFeatures_success', (t) => {
  const config = getLlmsConfigByRustdocFeatures('stable', path.join(process.cwd(), 'Cargo.toml'), true, null)

  t.is(config?.libName, 'crates_llms_txt_napi')
})

test('getLlmsConfigOnlineByCratesName_success', async (t) => {
  const config = await getLlmsConfigOnlineByCratesName('clap', '4.5.39')
  t.is(config?.libName, 'clap')
  t.is(config?.version, '4.5.39')
})

test('getLlmsConfigOnlineByUrl_success', async (t) => {
  const config = await getLlmsConfigOnlineByUrl('https://docs.rs/crate/clap/latest/json')
  t.is(config?.libName, 'clap')
  t.is(config?.version, '4.5.39')
})
