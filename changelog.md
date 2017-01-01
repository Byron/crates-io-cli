<a name="v1.3.1"></a>
### v1.3.1 (2017-01-01)


#### Improvements

* **search:**
  *  timeout for curl requests ([798acf34](https://github.com/Byron/crates-io-cli-rs/commit/798acf3449d97ec7c68d7630e0895ad96b2580de))
  *  allow to do nothing in some cases ([f0e96258](https://github.com/Byron/crates-io-cli-rs/commit/f0e96258b4707e9df7f161f884adf5d023655a66))
  *  explicit select-like future drops ([65888661](https://github.com/Byron/crates-io-cli-rs/commit/65888661f1f1516f5fa6f8549d0d65b047a18330))
  *  failed queries don't abort everything anymore ([1f195cc7](https://github.com/Byron/crates-io-cli-rs/commit/1f195cc7a8de5850c0cc5344d1fe6079a95e0fd6))

#### Bug Fixes

* **search:**
  *  info about valid characters is now shown ([89dfcd04](https://github.com/Byron/crates-io-cli-rs/commit/89dfcd04bf7e10632676a72d9265056a877a77ee))
  *  show better info if no search was made ([a6f8be20](https://github.com/Byron/crates-io-cli-rs/commit/a6f8be20b4c5b25667eb26566c10146cfee574e9))



<a name="v1.3.0"></a>
## v1.3.0 (2016-12-30)


#### Improvements

* **search and open**
 * Open crates on cates.io 


<a name="v1.2.0"></a>
## v1.2.0 (2016-12-30)

#### Features

* **search:**
  *  use `crates search` for an interactive search on crates.io

#### Improvements

* **UX:**  wait for 3 seconds before showing timeout message ([8a2837b9](https://github.com/Byron/crates-io-cli-rs/commit/8a2837b9c829811201d6a15a5f11b3ba973cb735))
* **changes:**  display a nice table ([ca792c0b](https://github.com/Byron/crates-io-cli-rs/commit/ca792c0bc6dd86758d3d905a6ffca5f60fd59c68))

<a name="v1.1.1"></a>
### v1.1.1 (2016-12-28)


#### Improvements

* **cli:**  more descriptive timeout message ([d44f874f](https://github.com/Byron/crates-io-cli-rs/commit/d44f874fd0d413afd2e45d3f1682be5711078f7f))



<a name="v1.1.0"></a>
## v1.1.0 (2016-12-28)

#### Improvements

* **cli:**  try to implement timeout with futures ([185ee003](https://github.com/Byron/crates-io-cli-rs/commit/185ee003cc7f1f8bc742f5f121d468318a0de10e))



<a name="v1.0.2"></a>
### v1.0.2 (2016-12-28)

#### Features

* **async:**  inform about long-running computation ([5187ae51](https://github.com/Byron/crates-io-cli-rs/commit/5187ae51c10e539ede401e2ee2e83cf9d9551732))



<a name="v1.0.1"></a>
### v1.0.1 (2016-12-26)


#### Bug Fixes

* **cargo:**  remove unnecessary documentation link ([8de7d263](https://github.com/Byron/crates-io-cli-rs/commit/8de7d263241c5061578f5aaf6d99e4e9c77a72e4))



