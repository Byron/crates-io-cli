<a name="v1.3.2"></a>
### v1.3.2 (2017-01-13)


#### Bug Fixes

* **search:**  prevent failure due to paging size ([961360b0](https://github.com/Byron/crates-io-cli-rs/commit/961360b007122d0be8e942174d866a3fe85a7f5d))

#### Improvements

* **bin:**  `krates` is an alternative program name ([329c54b9](https://github.com/Byron/crates-io-cli-rs/commit/329c54b9fc88c8e3f995e09cab6dee78f1a82d61))
* **cargo:**  better keywords ([fee77ec2](https://github.com/Byron/crates-io-cli-rs/commit/fee77ec2d3142f7be29a3c0d1b72209941d81d32))
* **cli:**  allow printing causes ([7d1f8933](https://github.com/Byron/crates-io-cli-rs/commit/7d1f8933b51718c17b382fd9ae5ce5b84846694b))
* **search:**
  *  make clear which search result you see ([1b00f362](https://github.com/Byron/crates-io-cli-rs/commit/1b00f362827b69319703fa64b79a34e2c7d6d5b9))
  *  parallel search processing thanks to spawn! ([8d013f82](https://github.com/Byron/crates-io-cli-rs/commit/8d013f82dbf85d5895eca72f94674b6a403cfb29))



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



