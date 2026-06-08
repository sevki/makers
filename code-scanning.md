# Code Scanning Todo

Generated from `code-scanning.json`.

## Progress

Resolved on `main` (verified `cargo build` produces **0 warnings**, the
`raw_ref_op` feature is removed from `src/lib.rs`, and clippy is clean for the
relevant lints):

- `unused_mut`, `unused_assignments`, `unused_attributes`, `unused_unsafe`
- `clippy::empty_line_after_outer_attr`, `clippy::needless_else`
- `stable_features` / `E0554` (`raw_ref_op`)

`rust/access-invalid-pointer` fixed directly: `#1372` (`read.rs` cmdleft `;`
split) and `#1387` (`read.rs` also_make loop, now iterated via `as_ref()`).

Still open: the remaining 81 `rust/access-invalid-pointer` findings (raw
pointer derefs needing per-site `as_ref`/`as_mut`/bounds-check restructuring),
and the 6 `E0425` `VaListImpl` errors (CodeQL build-environment artifacts — the
type resolves under the project's nightly `c_variadic` feature).

## Summary By Rule

| Rule | Count | Severity | Security | Description |
|---|---:|---|---|---|
| `rust/access-invalid-pointer` | 83 | `error` | `high` | Dereferencing an invalid or dangling pointer causes undefined behavior and may result in memory corruption. |
| `unused_mut` | 70 | `warning` | `` | `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default |
| `clippy::empty_line_after_outer_attr` | 30 | `warning` | `` | for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr `#[warn(clippy::empty_line_after_outer_attr)]` on by default |
| `unused_assignments` | 8 | `warning` | `` | `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default |
| `E0425` | 6 | `error` | `` |  |
| `unused_attributes` | 6 | `warning` | `` | externally exported functions are functions with `#[no_mangle]`, `#[export_name]`, or `#[linkage]` `#[warn(unused_attributes)]` (part of `#[warn(unused)]`) on by default |
| `actions/missing-workflow-permissions` | 5 | `warning` | `medium` | Workflows should contain explicit permissions to restrict the scope of the default GITHUB_TOKEN. |
| `unused_unsafe` | 5 | `warning` | `` | `#[warn(unused_unsafe)]` (part of `#[warn(unused)]`) on by default |
| `E0554` | 1 | `error` | `` | the feature `raw_ref_op` has been stable since `1.82.0` and no longer requires an attribute to enable |
| `clippy::needless_else` | 1 | `warning` | `` | for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_else `#[warn(clippy::needless_else)]` on by default |
| `stable_features` | 1 | `warning` | `` | `#[warn(stable_features)]` on by default |

## Summary By File

| File | Count |
|---|---:|
| `src/read.rs` | 48 |
| `src/main.rs` | 21 |
| `src/variable.rs` | 18 |
| `src/rule.rs` | 15 |
| `src/hash.rs` | 14 |
| `src/remake.rs` | 13 |
| `src/file.rs` | 12 |
| `src/misc.rs` | 12 |
| `src/implicit.rs` | 9 |
| `src/job.rs` | 9 |
| `src/dir.rs` | 7 |
| `src/function.rs` | 7 |
| `src/output.rs` | 6 |
| `src/vpath.rs` | 5 |
| `.github/workflows/ci.yml` | 4 |
| `src/ar.rs` | 4 |
| `src/commands.rs` | 3 |
| `src/strcache.rs` | 3 |
| `src/expand.rs` | 2 |
| `src/lib.rs` | 2 |
| `.github/workflows/mutants.yml` | 1 |
| `src/default.rs` | 1 |

## Checklist

### `rust/access-invalid-pointer` (83)

Severity: `error`. Security: `high`.

Dereferencing an invalid or dangling pointer causes undefined behavior and may result in memory corruption.

#### `src/read.rs` (25)

- [ ] [#1363](https://github.com/sevki/makers/security/code-scanning/1363) `src/read.rs:650` - This operation dereferences a pointer that may be invalid.
- [ ] [#1366](https://github.com/sevki/makers/security/code-scanning/1366) `src/read.rs:1049` - This operation dereferences a pointer that may be invalid.
- [ ] [#1368](https://github.com/sevki/makers/security/code-scanning/1368) `src/read.rs:1691` - This operation dereferences a pointer that may be invalid.
- [ ] [#1369](https://github.com/sevki/makers/security/code-scanning/1369) `src/read.rs:1698` - This operation dereferences a pointer that may be invalid.
- [ ] [#1370](https://github.com/sevki/makers/security/code-scanning/1370) `src/read.rs:1736` - This operation dereferences a pointer that may be invalid.
- [ ] [#1371](https://github.com/sevki/makers/security/code-scanning/1371) `src/read.rs:1845` - This operation dereferences a pointer that may be invalid.
- [ ] [#1375](https://github.com/sevki/makers/security/code-scanning/1375) `src/read.rs:1911` - This operation dereferences a pointer that may be invalid.
- [x] [#1372](https://github.com/sevki/makers/security/code-scanning/1372) `src/read.rs:1954` - This operation dereferences a pointer that may be invalid.
- [ ] [#1384](https://github.com/sevki/makers/security/code-scanning/1384) `src/read.rs:3080` - This operation dereferences a pointer that may be invalid.
- [ ] [#1385](https://github.com/sevki/makers/security/code-scanning/1385) `src/read.rs:3228` - This operation dereferences a pointer that may be invalid.
- [ ] [#1386](https://github.com/sevki/makers/security/code-scanning/1386) `src/read.rs:3297` - This operation dereferences a pointer that may be invalid.
- [x] [#1387](https://github.com/sevki/makers/security/code-scanning/1387) `src/read.rs:3309` - This operation dereferences a pointer that may be invalid.
- [ ] [#1389](https://github.com/sevki/makers/security/code-scanning/1389) `src/read.rs:3340` - This operation dereferences a pointer that may be invalid.
- [ ] [#1390](https://github.com/sevki/makers/security/code-scanning/1390) `src/read.rs:3344` - This operation dereferences a pointer that may be invalid.
- [ ] [#1391](https://github.com/sevki/makers/security/code-scanning/1391) `src/read.rs:3355` - This operation dereferences a pointer that may be invalid.
- [ ] [#1392](https://github.com/sevki/makers/security/code-scanning/1392) `src/read.rs:3357` - This operation dereferences a pointer that may be invalid.
- [ ] [#1393](https://github.com/sevki/makers/security/code-scanning/1393) `src/read.rs:3375` - This operation dereferences a pointer that may be invalid.
- [ ] [#1394](https://github.com/sevki/makers/security/code-scanning/1394) `src/read.rs:3466` - This operation dereferences a pointer that may be invalid.
- [ ] [#1395](https://github.com/sevki/makers/security/code-scanning/1395) `src/read.rs:3499` - This operation dereferences a pointer that may be invalid.
- [ ] [#1396](https://github.com/sevki/makers/security/code-scanning/1396) `src/read.rs:3501` - This operation dereferences a pointer that may be invalid.
- [ ] [#1399](https://github.com/sevki/makers/security/code-scanning/1399) `src/read.rs:3836` - This operation dereferences a pointer that may be invalid.
- [ ] [#1400](https://github.com/sevki/makers/security/code-scanning/1400) `src/read.rs:3846` - This operation dereferences a pointer that may be invalid.
- [ ] [#1401](https://github.com/sevki/makers/security/code-scanning/1401) `src/read.rs:4083` - This operation dereferences a pointer that may be invalid.
- [ ] [#1404](https://github.com/sevki/makers/security/code-scanning/1404) `src/read.rs:4151` - This operation dereferences a pointer that may be invalid.
- [ ] [#1406](https://github.com/sevki/makers/security/code-scanning/1406) `src/read.rs:4296` - This operation dereferences a pointer that may be invalid.

#### `src/variable.rs` (8)

- [ ] [#1380](https://github.com/sevki/makers/security/code-scanning/1380) `src/variable.rs:371` - This operation dereferences a pointer that may be invalid.
- [ ] [#1383](https://github.com/sevki/makers/security/code-scanning/1383) `src/variable.rs:465` - This operation dereferences a pointer that may be invalid.
- [ ] [#1388](https://github.com/sevki/makers/security/code-scanning/1388) `src/variable.rs:689` - This operation dereferences a pointer that may be invalid.
- [ ] [#1397](https://github.com/sevki/makers/security/code-scanning/1397) `src/variable.rs:893` - This operation dereferences a pointer that may be invalid.
- [ ] [#1398](https://github.com/sevki/makers/security/code-scanning/1398) `src/variable.rs:1086` - This operation dereferences a pointer that may be invalid.
- [ ] [#1402](https://github.com/sevki/makers/security/code-scanning/1402) `src/variable.rs:1792` - This operation dereferences a pointer that may be invalid.
- [ ] [#1403](https://github.com/sevki/makers/security/code-scanning/1403) `src/variable.rs:1815` - This operation dereferences a pointer that may be invalid.
- [ ] [#1405](https://github.com/sevki/makers/security/code-scanning/1405) `src/variable.rs:1892` - This operation dereferences a pointer that may be invalid.

#### `src/main.rs` (7)

- [ ] [#1353](https://github.com/sevki/makers/security/code-scanning/1353) `src/main.rs:2408` - This operation dereferences a pointer that may be invalid.
- [ ] [#1354](https://github.com/sevki/makers/security/code-scanning/1354) `src/main.rs:2443` - This operation dereferences a pointer that may be invalid.
- [ ] [#1355](https://github.com/sevki/makers/security/code-scanning/1355) `src/main.rs:2455` - This operation dereferences a pointer that may be invalid.
- [ ] [#1356](https://github.com/sevki/makers/security/code-scanning/1356) `src/main.rs:2472` - This operation dereferences a pointer that may be invalid.
- [ ] [#1357](https://github.com/sevki/makers/security/code-scanning/1357) `src/main.rs:2489` - This operation dereferences a pointer that may be invalid.
- [ ] [#1358](https://github.com/sevki/makers/security/code-scanning/1358) `src/main.rs:2503` - This operation dereferences a pointer that may be invalid.
- [ ] [#1359](https://github.com/sevki/makers/security/code-scanning/1359) `src/main.rs:2527` - This operation dereferences a pointer that may be invalid.

#### `src/misc.rs` (6)

- [x] [#1346](https://github.com/sevki/makers/security/code-scanning/1346) `src/misc.rs:118` - This operation dereferences a pointer that may be invalid.
- [x] [#1347](https://github.com/sevki/makers/security/code-scanning/1347) `src/misc.rs:407` - This operation dereferences a pointer that may be invalid.
- [x] [#1348](https://github.com/sevki/makers/security/code-scanning/1348) `src/misc.rs:565` - This operation dereferences a pointer that may be invalid.
- [x] [#1349](https://github.com/sevki/makers/security/code-scanning/1349) `src/misc.rs:568` - This operation dereferences a pointer that may be invalid.
- [x] [#1350](https://github.com/sevki/makers/security/code-scanning/1350) `src/misc.rs:575` - This operation dereferences a pointer that may be invalid.
- [x] [#1351](https://github.com/sevki/makers/security/code-scanning/1351) `src/misc.rs:842` - This operation dereferences a pointer that may be invalid.

#### `src/file.rs` (5)

- [ ] [#1333](https://github.com/sevki/makers/security/code-scanning/1333) `src/file.rs:1005` - This operation dereferences a pointer that may be invalid.
- [ ] [#1334](https://github.com/sevki/makers/security/code-scanning/1334) `src/file.rs:1167` - This operation dereferences a pointer that may be invalid.
- [ ] [#1335](https://github.com/sevki/makers/security/code-scanning/1335) `src/file.rs:1200` - This operation dereferences a pointer that may be invalid.
- [ ] [#1336](https://github.com/sevki/makers/security/code-scanning/1336) `src/file.rs:1214` - This operation dereferences a pointer that may be invalid.
- [ ] [#1338](https://github.com/sevki/makers/security/code-scanning/1338) `src/file.rs:1729` - This operation dereferences a pointer that may be invalid.

#### `src/hash.rs` (5)

- [ ] [#1329](https://github.com/sevki/makers/security/code-scanning/1329) `src/hash.rs:189` - This operation dereferences a pointer that may be invalid.
- [ ] [#1330](https://github.com/sevki/makers/security/code-scanning/1330) `src/hash.rs:201` - This operation dereferences a pointer that may be invalid.
- [ ] [#1407](https://github.com/sevki/makers/security/code-scanning/1407) `src/hash.rs:258` - This operation dereferences a pointer that may be invalid.
- [ ] [#1408](https://github.com/sevki/makers/security/code-scanning/1408) `src/hash.rs:260` - This operation dereferences a pointer that may be invalid.
- [ ] [#1332](https://github.com/sevki/makers/security/code-scanning/1332) `src/hash.rs:399` - This operation dereferences a pointer that may be invalid.

#### `src/rule.rs` (5)

- [ ] [#1364](https://github.com/sevki/makers/security/code-scanning/1364) `src/rule.rs:268` - This operation dereferences a pointer that may be invalid.
- [ ] [#1367](https://github.com/sevki/makers/security/code-scanning/1367) `src/rule.rs:408` - This operation dereferences a pointer that may be invalid.
- [ ] [#1373](https://github.com/sevki/makers/security/code-scanning/1373) `src/rule.rs:811` - This operation dereferences a pointer that may be invalid.
- [ ] [#1374](https://github.com/sevki/makers/security/code-scanning/1374) `src/rule.rs:841` - This operation dereferences a pointer that may be invalid.
- [ ] [#1376](https://github.com/sevki/makers/security/code-scanning/1376) `src/rule.rs:887` - This operation dereferences a pointer that may be invalid.

#### `src/implicit.rs` (4)

- [ ] [#1337](https://github.com/sevki/makers/security/code-scanning/1337) `src/implicit.rs:295` - This operation dereferences a pointer that may be invalid.
- [ ] [#1339](https://github.com/sevki/makers/security/code-scanning/1339) `src/implicit.rs:1233` - This operation dereferences a pointer that may be invalid.
- [ ] [#1340](https://github.com/sevki/makers/security/code-scanning/1340) `src/implicit.rs:1323` - This operation dereferences a pointer that may be invalid.
- [ ] [#1341](https://github.com/sevki/makers/security/code-scanning/1341) `src/implicit.rs:1438` - This operation dereferences a pointer that may be invalid.

#### `src/job.rs` (4)

- [ ] [#1342](https://github.com/sevki/makers/security/code-scanning/1342) `src/job.rs:942` - This operation dereferences a pointer that may be invalid.
- [ ] [#1343](https://github.com/sevki/makers/security/code-scanning/1343) `src/job.rs:1086` - This operation dereferences a pointer that may be invalid.
- [ ] [#1345](https://github.com/sevki/makers/security/code-scanning/1345) `src/job.rs:2044` - This operation dereferences a pointer that may be invalid.
- [ ] [#1352](https://github.com/sevki/makers/security/code-scanning/1352) `src/job.rs:2645` - This operation dereferences a pointer that may be invalid.

#### `src/remake.rs` (4)

- [x] [#1361](https://github.com/sevki/makers/security/code-scanning/1361) `src/remake.rs:292` - This operation dereferences a pointer that may be invalid.
- [x] [#1362](https://github.com/sevki/makers/security/code-scanning/1362) `src/remake.rs:433` - This operation dereferences a pointer that may be invalid.
- [x] [#1365](https://github.com/sevki/makers/security/code-scanning/1365) `src/remake.rs:915` - This operation dereferences a pointer that may be invalid.
- [x] [#1377](https://github.com/sevki/makers/security/code-scanning/1377) `src/remake.rs:1573` - This operation dereferences a pointer that may be invalid.

#### `src/vpath.rs` (4)

- [x] [#1378](https://github.com/sevki/makers/security/code-scanning/1378) `src/vpath.rs:182` - This operation dereferences a pointer that may be invalid.
- [x] [#1379](https://github.com/sevki/makers/security/code-scanning/1379) `src/vpath.rs:398` - This operation dereferences a pointer that may be invalid.
- [x] [#1381](https://github.com/sevki/makers/security/code-scanning/1381) `src/vpath.rs:450` - This operation dereferences a pointer that may be invalid.
- [x] [#1382](https://github.com/sevki/makers/security/code-scanning/1382) `src/vpath.rs:460` - This operation dereferences a pointer that may be invalid.

#### `src/ar.rs` (2)

- [ ] [#1326](https://github.com/sevki/makers/security/code-scanning/1326) `src/ar.rs:394` - This operation dereferences a pointer that may be invalid.
- [ ] [#1327](https://github.com/sevki/makers/security/code-scanning/1327) `src/ar.rs:414` - This operation dereferences a pointer that may be invalid.

#### `src/function.rs` (2)

- [ ] [#1331](https://github.com/sevki/makers/security/code-scanning/1331) `src/function.rs:476` - This operation dereferences a pointer that may be invalid.
- [ ] [#1344](https://github.com/sevki/makers/security/code-scanning/1344) `src/function.rs:2932` - This operation dereferences a pointer that may be invalid.

#### `src/expand.rs` (1)

- [ ] [#1328](https://github.com/sevki/makers/security/code-scanning/1328) `src/expand.rs:313` - This operation dereferences a pointer that may be invalid.

#### `src/output.rs` (1)

- [ ] [#1360](https://github.com/sevki/makers/security/code-scanning/1360) `src/output.rs:353` - This operation dereferences a pointer that may be invalid.

### `unused_mut` (70)

Severity: `warning`. Security: ``.

`#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

#### `src/read.rs` (14)

- [x] [#119](https://github.com/sevki/makers/security/code-scanning/119) `src/read.rs:542` - variable does not need to be mutable
- [x] [#120](https://github.com/sevki/makers/security/code-scanning/120) `src/read.rs:574` - variable does not need to be mutable
- [x] [#121](https://github.com/sevki/makers/security/code-scanning/121) `src/read.rs:928` - variable does not need to be mutable
- [x] [#122](https://github.com/sevki/makers/security/code-scanning/122) `src/read.rs:1477` - variable does not need to be mutable
- [x] [#123](https://github.com/sevki/makers/security/code-scanning/123) `src/read.rs:1565` - variable does not need to be mutable
- [x] [#124](https://github.com/sevki/makers/security/code-scanning/124) `src/read.rs:2158` - variable does not need to be mutable
- [x] [#125](https://github.com/sevki/makers/security/code-scanning/125) `src/read.rs:2748` - variable does not need to be mutable
- [x] [#126](https://github.com/sevki/makers/security/code-scanning/126) `src/read.rs:2751` - variable does not need to be mutable
- [x] [#127](https://github.com/sevki/makers/security/code-scanning/127) `src/read.rs:3067` - variable does not need to be mutable
- [x] [#128](https://github.com/sevki/makers/security/code-scanning/128) `src/read.rs:3274` - variable does not need to be mutable
- [x] [#129](https://github.com/sevki/makers/security/code-scanning/129) `src/read.rs:3340` - variable does not need to be mutable
- [x] [#130](https://github.com/sevki/makers/security/code-scanning/130) `src/read.rs:3356` - variable does not need to be mutable
- [x] [#131](https://github.com/sevki/makers/security/code-scanning/131) `src/read.rs:3575` - variable does not need to be mutable
- [x] [#132](https://github.com/sevki/makers/security/code-scanning/132) `src/read.rs:3609` - variable does not need to be mutable

#### `src/hash.rs` (8)

- [x] [#94](https://github.com/sevki/makers/security/code-scanning/94) `src/hash.rs:81` - variable does not need to be mutable
- [x] [#95](https://github.com/sevki/makers/security/code-scanning/95) `src/hash.rs:136` - variable does not need to be mutable
- [x] [#96](https://github.com/sevki/makers/security/code-scanning/96) `src/hash.rs:213` - variable does not need to be mutable
- [x] [#97](https://github.com/sevki/makers/security/code-scanning/97) `src/hash.rs:255` - variable does not need to be mutable
- [x] [#98](https://github.com/sevki/makers/security/code-scanning/98) `src/hash.rs:269` - variable does not need to be mutable
- [x] [#99](https://github.com/sevki/makers/security/code-scanning/99) `src/hash.rs:295` - variable does not need to be mutable
- [x] [#156](https://github.com/sevki/makers/security/code-scanning/156) `src/hash.rs:320` - variable does not need to be mutable
- [x] [#101](https://github.com/sevki/makers/security/code-scanning/101) `src/hash.rs:380` - variable does not need to be mutable

#### `src/dir.rs` (7)

- [x] [#76](https://github.com/sevki/makers/security/code-scanning/76) `src/dir.rs:133` - variable does not need to be mutable
- [x] [#77](https://github.com/sevki/makers/security/code-scanning/77) `src/dir.rs:459` - variable does not need to be mutable
- [x] [#78](https://github.com/sevki/makers/security/code-scanning/78) `src/dir.rs:605` - variable does not need to be mutable
- [x] [#79](https://github.com/sevki/makers/security/code-scanning/79) `src/dir.rs:606` - variable does not need to be mutable
- [x] [#80](https://github.com/sevki/makers/security/code-scanning/80) `src/dir.rs:843` - variable does not need to be mutable
- [x] [#81](https://github.com/sevki/makers/security/code-scanning/81) `src/dir.rs:871` - variable does not need to be mutable
- [x] [#82](https://github.com/sevki/makers/security/code-scanning/82) `src/dir.rs:897` - variable does not need to be mutable

#### `src/main.rs` (6)

- [x] [#110](https://github.com/sevki/makers/security/code-scanning/110) `src/main.rs:2132` - variable does not need to be mutable
- [x] [#111](https://github.com/sevki/makers/security/code-scanning/111) `src/main.rs:2335` - variable does not need to be mutable
- [x] [#112](https://github.com/sevki/makers/security/code-scanning/112) `src/main.rs:2352` - variable does not need to be mutable
- [x] [#113](https://github.com/sevki/makers/security/code-scanning/113) `src/main.rs:2391` - variable does not need to be mutable
- [x] [#114](https://github.com/sevki/makers/security/code-scanning/114) `src/main.rs:2913` - variable does not need to be mutable
- [x] [#115](https://github.com/sevki/makers/security/code-scanning/115) `src/main.rs:2949` - variable does not need to be mutable

#### `src/rule.rs` (6)

- [x] [#136](https://github.com/sevki/makers/security/code-scanning/136) `src/rule.rs:170` - variable does not need to be mutable
- [x] [#137](https://github.com/sevki/makers/security/code-scanning/137) `src/rule.rs:429` - variable does not need to be mutable
- [x] [#138](https://github.com/sevki/makers/security/code-scanning/138) `src/rule.rs:668` - variable does not need to be mutable
- [x] [#139](https://github.com/sevki/makers/security/code-scanning/139) `src/rule.rs:773` - variable does not need to be mutable
- [x] [#140](https://github.com/sevki/makers/security/code-scanning/140) `src/rule.rs:826` - variable does not need to be mutable
- [x] [#141](https://github.com/sevki/makers/security/code-scanning/141) `src/rule.rs:858` - variable does not need to be mutable

#### `src/variable.rs` (6)

- [x] [#145](https://github.com/sevki/makers/security/code-scanning/145) `src/variable.rs:213` - variable does not need to be mutable
- [x] [#148](https://github.com/sevki/makers/security/code-scanning/148) `src/variable.rs:616` - variable does not need to be mutable
- [x] [#149](https://github.com/sevki/makers/security/code-scanning/149) `src/variable.rs:815` - variable does not need to be mutable
- [x] [#150](https://github.com/sevki/makers/security/code-scanning/150) `src/variable.rs:914` - variable does not need to be mutable
- [x] [#153](https://github.com/sevki/makers/security/code-scanning/153) `src/variable.rs:1901` - variable does not need to be mutable
- [x] [#154](https://github.com/sevki/makers/security/code-scanning/154) `src/variable.rs:2026` - variable does not need to be mutable

#### `src/job.rs` (5)

- [x] [#104](https://github.com/sevki/makers/security/code-scanning/104) `src/job.rs:1011` - variable does not need to be mutable
- [x] [#105](https://github.com/sevki/makers/security/code-scanning/105) `src/job.rs:1256` - variable does not need to be mutable
- [x] [#106](https://github.com/sevki/makers/security/code-scanning/106) `src/job.rs:1336` - variable does not need to be mutable
- [x] [#107](https://github.com/sevki/makers/security/code-scanning/107) `src/job.rs:1337` - variable does not need to be mutable
- [x] [#108](https://github.com/sevki/makers/security/code-scanning/108) `src/job.rs:1676` - variable does not need to be mutable

#### `src/file.rs` (4)

- [x] [#84](https://github.com/sevki/makers/security/code-scanning/84) `src/file.rs:501` - variable does not need to be mutable
- [x] [#85](https://github.com/sevki/makers/security/code-scanning/85) `src/file.rs:502` - variable does not need to be mutable
- [x] [#86](https://github.com/sevki/makers/security/code-scanning/86) `src/file.rs:607` - variable does not need to be mutable
- [x] [#87](https://github.com/sevki/makers/security/code-scanning/87) `src/file.rs:1178` - variable does not need to be mutable

#### `src/strcache.rs` (3)

- [x] [#142](https://github.com/sevki/makers/security/code-scanning/142) `src/strcache.rs:87` - variable does not need to be mutable
- [x] [#143](https://github.com/sevki/makers/security/code-scanning/143) `src/strcache.rs:98` - variable does not need to be mutable
- [x] [#144](https://github.com/sevki/makers/security/code-scanning/144) `src/strcache.rs:166` - variable does not need to be mutable

#### `src/ar.rs` (2)

- [x] [#71](https://github.com/sevki/makers/security/code-scanning/71) `src/ar.rs:288` - variable does not need to be mutable
- [x] [#72](https://github.com/sevki/makers/security/code-scanning/72) `src/ar.rs:290` - variable does not need to be mutable

#### `src/commands.rs` (2)

- [x] [#73](https://github.com/sevki/makers/security/code-scanning/73) `src/commands.rs:255` - variable does not need to be mutable
- [x] [#74](https://github.com/sevki/makers/security/code-scanning/74) `src/commands.rs:678` - variable does not need to be mutable

#### `src/implicit.rs` (2)

- [x] [#102](https://github.com/sevki/makers/security/code-scanning/102) `src/implicit.rs:347` - variable does not need to be mutable
- [x] [#103](https://github.com/sevki/makers/security/code-scanning/103) `src/implicit.rs:1468` - variable does not need to be mutable

#### `src/default.rs` (1)

- [x] [#75](https://github.com/sevki/makers/security/code-scanning/75) `src/default.rs:492` - variable does not need to be mutable

#### `src/function.rs` (1)

- [x] [#88](https://github.com/sevki/makers/security/code-scanning/88) `src/function.rs:1288` - variable does not need to be mutable

#### `src/output.rs` (1)

- [x] [#117](https://github.com/sevki/makers/security/code-scanning/117) `src/output.rs:351` - variable does not need to be mutable

#### `src/remake.rs` (1)

- [x] [#133](https://github.com/sevki/makers/security/code-scanning/133) `src/remake.rs:1337` - variable does not need to be mutable

#### `src/vpath.rs` (1)

- [x] [#155](https://github.com/sevki/makers/security/code-scanning/155) `src/vpath.rs:256` - variable does not need to be mutable

### `clippy::empty_line_after_outer_attr` (30)

Severity: `warning`. Security: ``.

for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#empty_line_after_outer_attr `#[warn(clippy::empty_line_after_outer_attr)]` on by default

#### `src/read.rs` (8)

- [x] [#47](https://github.com/sevki/makers/security/code-scanning/47) `src/read.rs:420-421` - empty line after outer attribute
- [x] [#46](https://github.com/sevki/makers/security/code-scanning/46) `src/read.rs:422` - empty line after outer attribute
- [x] [#49](https://github.com/sevki/makers/security/code-scanning/49) `src/read.rs:425-426` - empty line after outer attribute
- [x] [#48](https://github.com/sevki/makers/security/code-scanning/48) `src/read.rs:427` - empty line after outer attribute
- [x] [#51](https://github.com/sevki/makers/security/code-scanning/51) `src/read.rs:430-431` - empty line after outer attribute
- [x] [#50](https://github.com/sevki/makers/security/code-scanning/50) `src/read.rs:432` - empty line after outer attribute
- [x] [#53](https://github.com/sevki/makers/security/code-scanning/53) `src/read.rs:435-436` - empty line after outer attribute
- [x] [#52](https://github.com/sevki/makers/security/code-scanning/52) `src/read.rs:437` - empty line after outer attribute

#### `src/main.rs` (6)

- [x] [#38](https://github.com/sevki/makers/security/code-scanning/38) `src/main.rs:561-562` - empty line after outer attribute
- [x] [#37](https://github.com/sevki/makers/security/code-scanning/37) `src/main.rs:563` - empty line after outer attribute
- [x] [#40](https://github.com/sevki/makers/security/code-scanning/40) `src/main.rs:566-567` - empty line after outer attribute
- [x] [#39](https://github.com/sevki/makers/security/code-scanning/39) `src/main.rs:568` - empty line after outer attribute
- [x] [#42](https://github.com/sevki/makers/security/code-scanning/42) `src/main.rs:576-577` - empty line after outer attribute
- [x] [#41](https://github.com/sevki/makers/security/code-scanning/41) `src/main.rs:578` - empty line after outer attribute

#### `src/remake.rs` (6)

- [x] [#55](https://github.com/sevki/makers/security/code-scanning/55) `src/remake.rs:165-166` - empty line after outer attribute
- [x] [#54](https://github.com/sevki/makers/security/code-scanning/54) `src/remake.rs:167` - empty line after outer attribute
- [x] [#57](https://github.com/sevki/makers/security/code-scanning/57) `src/remake.rs:170-171` - empty line after outer attribute
- [x] [#56](https://github.com/sevki/makers/security/code-scanning/56) `src/remake.rs:172` - empty line after outer attribute
- [x] [#59](https://github.com/sevki/makers/security/code-scanning/59) `src/remake.rs:175-176` - empty line after outer attribute
- [x] [#58](https://github.com/sevki/makers/security/code-scanning/58) `src/remake.rs:177` - empty line after outer attribute

#### `src/misc.rs` (4)

- [x] [#160](https://github.com/sevki/makers/security/code-scanning/160) `src/misc.rs:104-105` - empty line after outer attribute
- [x] [#159](https://github.com/sevki/makers/security/code-scanning/159) `src/misc.rs:106` - empty line after outer attribute
- [x] [#44](https://github.com/sevki/makers/security/code-scanning/44) `src/misc.rs:109-110` - empty line after outer attribute
- [x] [#43](https://github.com/sevki/makers/security/code-scanning/43) `src/misc.rs:111` - empty line after outer attribute

#### `src/rule.rs` (4)

- [x] [#61](https://github.com/sevki/makers/security/code-scanning/61) `src/rule.rs:145-146` - empty line after outer attribute
- [x] [#60](https://github.com/sevki/makers/security/code-scanning/60) `src/rule.rs:147` - empty line after outer attribute
- [x] [#63](https://github.com/sevki/makers/security/code-scanning/63) `src/rule.rs:150-151` - empty line after outer attribute
- [x] [#62](https://github.com/sevki/makers/security/code-scanning/62) `src/rule.rs:152` - empty line after outer attribute

#### `src/implicit.rs` (2)

- [x] [#36](https://github.com/sevki/makers/security/code-scanning/36) `src/implicit.rs:226-227` - empty line after outer attribute
- [x] [#35](https://github.com/sevki/makers/security/code-scanning/35) `src/implicit.rs:228` - empty line after outer attribute

### `unused_assignments` (8)

Severity: `warning`. Security: ``.

`#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

#### `src/function.rs` (4)

- [x] [#89](https://github.com/sevki/makers/security/code-scanning/89) `src/function.rs:2896` - value assigned to `next` is never read
- [x] [#90](https://github.com/sevki/makers/security/code-scanning/90) `src/function.rs:2899` - value assigned to `next` is never read
- [x] [#91](https://github.com/sevki/makers/security/code-scanning/91) `src/function.rs:2923` - value assigned to `next_0` is never read
- [x] [#92](https://github.com/sevki/makers/security/code-scanning/92) `src/function.rs:2926` - value assigned to `next_0` is never read

#### `src/remake.rs` (2)

- [x] [#134](https://github.com/sevki/makers/security/code-scanning/134) `src/remake.rs:2100` - value assigned to `p` is never read
- [x] [#135](https://github.com/sevki/makers/security/code-scanning/135) `src/remake.rs:2149` - value assigned to `p` is never read

#### `src/variable.rs` (2)

- [x] [#151](https://github.com/sevki/makers/security/code-scanning/151) `src/variable.rs:1814` - value assigned to `s` is never read
- [x] [#152](https://github.com/sevki/makers/security/code-scanning/152) `src/variable.rs:1825-1828` - value assigned to `s` is never read

### `E0425` (6)

Severity: `error`. Security: ``.

#### `src/output.rs` (4)

- [ ] [#3](https://github.com/sevki/makers/security/code-scanning/3) `src/output.rs:424` - cannot find type `VaListImpl` in module `::core::ffi`
- [ ] [#4](https://github.com/sevki/makers/security/code-scanning/4) `src/output.rs:479` - cannot find type `VaListImpl` in module `::core::ffi`
- [ ] [#5](https://github.com/sevki/makers/security/code-scanning/5) `src/output.rs:548` - cannot find type `VaListImpl` in module `::core::ffi`
- [ ] [#6](https://github.com/sevki/makers/security/code-scanning/6) `src/output.rs:616` - cannot find type `VaListImpl` in module `::core::ffi`

#### `src/misc.rs` (2)

- [ ] [#1](https://github.com/sevki/makers/security/code-scanning/1) `src/misc.rs:297` - cannot find type `VaListImpl` in module `::core::ffi`
- [ ] [#2](https://github.com/sevki/makers/security/code-scanning/2) `src/misc.rs:636` - cannot find type `VaListImpl` in module `::core::ffi`

### `unused_attributes` (6)

Severity: `warning`. Security: ``.

externally exported functions are functions with `#[no_mangle]`, `#[export_name]`, or `#[linkage]` `#[warn(unused_attributes)]` (part of `#[warn(unused)]`) on by default

#### `src/file.rs` (3)

- [x] [#65](https://github.com/sevki/makers/security/code-scanning/65) `src/file.rs:374` - `#[inline]` is ignored on externally exported functions
- [x] [#66](https://github.com/sevki/makers/security/code-scanning/66) `src/file.rs:379` - `#[inline]` is ignored on externally exported functions
- [x] [#67](https://github.com/sevki/makers/security/code-scanning/67) `src/file.rs:384` - `#[inline]` is ignored on externally exported functions

#### `src/main.rs` (2)

- [x] [#69](https://github.com/sevki/makers/security/code-scanning/69) `src/main.rs:556` - `#[inline]` is ignored on externally exported functions
- [x] [#70](https://github.com/sevki/makers/security/code-scanning/70) `src/main.rs:571` - `#[inline]` is ignored on externally exported functions

#### `src/implicit.rs` (1)

- [x] [#68](https://github.com/sevki/makers/security/code-scanning/68) `src/implicit.rs:221` - `#[inline]` is ignored on externally exported functions


### `unused_unsafe` (5)

Severity: `warning`. Security: ``.

`#[warn(unused_unsafe)]` (part of `#[warn(unused)]`) on by default

#### `src/variable.rs` (2)

- [x] [#146](https://github.com/sevki/makers/security/code-scanning/146) `src/variable.rs:346` - unnecessary `unsafe` block
- [x] [#147](https://github.com/sevki/makers/security/code-scanning/147) `src/variable.rs:355` - unnecessary `unsafe` block

#### `src/expand.rs` (1)

- [x] [#83](https://github.com/sevki/makers/security/code-scanning/83) `src/expand.rs:134` - unnecessary `unsafe` block

#### `src/hash.rs` (1)

- [x] [#93](https://github.com/sevki/makers/security/code-scanning/93) `src/hash.rs:76` - unnecessary `unsafe` block

#### `src/read.rs` (1)

- [x] [#118](https://github.com/sevki/makers/security/code-scanning/118) `src/read.rs:448` - unnecessary `unsafe` block

### `E0554` (1)

Severity: `error`. Security: ``.

the feature `raw_ref_op` has been stable since `1.82.0` and no longer requires an attribute to enable

#### `src/lib.rs` (1)

- [x] [#7](https://github.com/sevki/makers/security/code-scanning/7) `src/lib.rs:3` - `#![feature]` may not be used on the stable release channel

### `clippy::needless_else` (1)

Severity: `warning`. Security: ``.

for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_else `#[warn(clippy::needless_else)]` on by default

#### `src/commands.rs` (1)

- [x] [#8](https://github.com/sevki/makers/security/code-scanning/8) `src/commands.rs:230-231` - this `else` branch is empty

### `stable_features` (1)

Severity: `warning`. Security: ``.

`#[warn(stable_features)]` on by default

#### `src/lib.rs` (1)

- [x] [#64](https://github.com/sevki/makers/security/code-scanning/64) `src/lib.rs:5` - the feature `raw_ref_op` has been stable since 1.82.0 and no longer requires an attribute to enable
