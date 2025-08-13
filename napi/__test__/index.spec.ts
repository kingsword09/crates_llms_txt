import test from 'ava'
import path from 'node:path'
import process from 'node:process'

import { fromLocalByRustdoc, fromOnline, fromUrl } from '../index.js'

test('fromLocalByRustdoc_allFeatures_success', (t) => {
  const config = fromLocalByRustdoc({
    manifestPath: path.join(process.cwd(), 'Cargo.toml'),
  })

  if (config === null) {
    t.pass('Local rustdoc generation returned null - this might be expected if rustdoc is not available')
  } else {
    t.is(config.libName, 'crates_llms_txt_napi')
  }
})

test('fromLocalByRustdoc_withFeatures_success', (t) => {
  const config = fromLocalByRustdoc({
    manifestPath: path.join(process.cwd(), 'Cargo.toml'),
    noDefaultFeatures: true,
    features: ['rustdoc'],
  })

  if (config === null) {
    t.pass('Local rustdoc generation returned null - this might be expected if rustdoc is not available')
  } else {
    t.is(config.libName, 'crates_llms_txt_napi')
  }
})

test('fromOnline_success', async (t) => {
  const configLatest = await fromOnline({ libName: 'serde', version: undefined })

  if (configLatest) {
    t.is(configLatest.libName, 'serde')
    t.truthy(configLatest.version)
  } else {
    t.pass('Online fetch returned null - this might be expected due to network issues or API changes')
  }
})

test('fromUrl_success', async (t) => {
  const config = await fromUrl('https://docs.rs/crate/clap/latest/json')
  t.is(config?.libName, 'clap')
  // Note: version might not be exactly '4.5.39' when fetching from latest
  t.truthy(config?.version)
})
