<a name="v4.0.0"></a>

### 4.0.1 (2023-03-16)

Upgrade dependencies and make it compile once again.

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 735 calendar days.
 - 864 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare changelog prior to release ([`232e233`](https://github.com/Byron/crates-io-cli-rs/commit/232e233e914f39484a325ee554db23c7d0b58b0f))
    - Upgrade dependencies and generally make things compile again ([`013c29a`](https://github.com/Byron/crates-io-cli-rs/commit/013c29a2e224315f11b2947eef60f6de94e31d3b))
    - Dependency update ([`6b2e9d8`](https://github.com/Byron/crates-io-cli-rs/commit/6b2e9d83005e286bd8249a234408edb01a4c4504))
</details>

### v4.0.0 (2020-11-02)

* update and upgrade all dependencies
* use latest criner and prodash

<a name="v3.2.0"></a>

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 139 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump major version ([`a2690fa`](https://github.com/Byron/crates-io-cli-rs/commit/a2690fa9fb5f1ad4ad7fcdd8451912d406991d4d))
    - Upgrade to latest criner-cli (including clap-beta) ([`c1083e0`](https://github.com/Byron/crates-io-cli-rs/commit/c1083e0cfd27df57948129e7b4967356b0b1a653))
    - Revert "upgrade to quickerror 2.0" ([`cf280cb`](https://github.com/Byron/crates-io-cli-rs/commit/cf280cbf0e9c805a45ceb363767b62a6d59e1122))
    - Upgrade criner-cli ([`ad5fcf7`](https://github.com/Byron/crates-io-cli-rs/commit/ad5fcf7348863695e1a6ec84d16f3ceada774f97))
    - Upgrade to quickerror 2.0 ([`236ef5e`](https://github.com/Byron/crates-io-cli-rs/commit/236ef5ee82ecc92534632055c7312eb6c4de279a))
    - Dependency update ([`8221276`](https://github.com/Byron/crates-io-cli-rs/commit/8221276b208f45e38eb8628306c4eb2800609c2d))
</details>

### v3.3.0 (2020-06-16)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 20 calendar days.
 - 64 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update dependencies; bump minor version ([`a6cf3df`](https://github.com/Byron/crates-io-cli-rs/commit/a6cf3df3562f4fb6d0cafbb0f3c4e15537b44327))
    - Dependency update ([`7c9415e`](https://github.com/Byron/crates-io-cli-rs/commit/7c9415ea49fe654ef5926650e865b3e1c0fb061d))
    - Optimize include directive ([`2bd2280`](https://github.com/Byron/crates-io-cli-rs/commit/2bd2280d75d2a78226a10331da9b71aed3ec71ad))
</details>

### v3.2.0 (2020-04-12)

* update and upgrade all dependencies
* add an early version of 'criner', which is run with `crates criner mine`

<a name="v2.1.1"></a>

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 22 calendar days.
 - 23 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump minor version for first release with criner ([`939629b`](https://github.com/Byron/crates-io-cli-rs/commit/939629bae79c80a5d4a94f159bc1b825d112e721))
    - Update and upgrade all dependencies ([`4b806ad`](https://github.com/Byron/crates-io-cli-rs/commit/4b806ad081b7d40d04cb3f2622efa39456eb7ae5))
    - Enable relases ([`924af83`](https://github.com/Byron/crates-io-cli-rs/commit/924af83e96219d652ddc76cc7c0bac2528234905))
    - Bye bye travis, we had a good time ([`3ede34c`](https://github.com/Byron/crates-io-cli-rs/commit/3ede34c708ff1a779d0d3c0301135d13b411b629))
    - Don't depend on local criner anymore ([`ca4def4`](https://github.com/Byron/crates-io-cli-rs/commit/ca4def46e20951910ad89077203e74141e3d9fd6))
    - Add github actions, including releases ([`9b945c4`](https://github.com/Byron/crates-io-cli-rs/commit/9b945c4fc48e11ba94129230558782e4d5c87249))
    - Adjust repository to point to new home. ([`2d69dfa`](https://github.com/Byron/crates-io-cli-rs/commit/2d69dfa0267a5d9e582845120ba573be772ee74e))
</details>

### v3.1.1 (2020-03-20)

<csr-id-5e821817191d428d193063907bd96c4334558d79/>
<csr-id-34e11e4a0948ce776728448984c8bd525b6024a0/>

#### Other

 - <csr-id-5e821817191d428d193063907bd96c4334558d79/> futures aren't dropped when you think they are
   So even if abortable() helps to stop polling a future, that doesn't
   mean that it will run drop().
   
   Thus explicit drops have to be placed, C-style.
   To me it seems there is a threaded poller who keeps ahold of the
   future even though it obviously just returned Ready.
 - <csr-id-34e11e4a0948ce776728448984c8bd525b6024a0/> can mix threads and async :)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 342 commits contributed to the release over the course of 46 calendar days.
 - 46 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Allow publishing ([`1a54554`](https://github.com/Byron/crates-io-cli-rs/commit/1a5455478796c643782085fe6220162d3be51a97))
    - Update dependencies ([`32aadb7`](https://github.com/Byron/crates-io-cli-rs/commit/32aadb7fe0dc1acebfe23fbd0b3fc2c640336d28))
    - Fix tests ([`7554d49`](https://github.com/Byron/crates-io-cli-rs/commit/7554d496fee0d8bd0cc6079b52f678b5a4ebfcc3))
    - Don't strip away the top-level criner arguments ([`5f0e42f`](https://github.com/Byron/crates-io-cli-rs/commit/5f0e42f0c31c43ca22faee905149d7ed354ed0ac))
    - Rather leave criner underneath its own subcommand ([`be476b5`](https://github.com/Byron/crates-io-cli-rs/commit/be476b56ed004a8cb3cef8a32b390ae6ae7c73f0))
    - First demo to show that sub-commands can be combined ([`fdd7402`](https://github.com/Byron/crates-io-cli-rs/commit/fdd7402fc1945c5fdf3e6c7823eeb1bb7ffd9a61))
    - Enable all features by default ([`4ee1db3`](https://github.com/Byron/crates-io-cli-rs/commit/4ee1db3c1fbbdca5b5ffb9cdbbd8316a9fe399d2))
    - Remove local criner crate ([`73b3525`](https://github.com/Byron/crates-io-cli-rs/commit/73b35255b294a9161f57f96f0e9a9488e9ba3d95))
    - Use standalone proash ([`7c7802a`](https://github.com/Byron/crates-io-cli-rs/commit/7c7802abffef8d41d6e92aa948385e47f29563af))
    - Fix CLI args ([`4459b8e`](https://github.com/Byron/crates-io-cli-rs/commit/4459b8ef9b225d76c022e4319ab354af493d51d8))
    - Simplify command-line argument parsing ([`360787f`](https://github.com/Byron/crates-io-cli-rs/commit/360787f1adcca4d0a6981b9d9fb6a377341d8db9))
    - Minimize dependencies ([`b3452db`](https://github.com/Byron/crates-io-cli-rs/commit/b3452db0e1a82b19b72812190921e6d54dd886cd))
    - Support for downloads-directory ([`755b764`](https://github.com/Byron/crates-io-cli-rs/commit/755b764a49d9532b983ed3bd1920e59cd17e907d))
    - Lower FPS by default, more doesn't make too much sense and uses more CPU ([`f9e6b29`](https://github.com/Byron/crates-io-cli-rs/commit/f9e6b29f457b28e6783f6e1b89b0cee6c6bd9d0f))
    - Skip download tasks that are done already ([`6f6c80e`](https://github.com/Byron/crates-io-cli-rs/commit/6f6c80e6a1d5da5d7d15535adff344a2ab4b4c14))
    - Fix worker error handling; create hardcoded assets dir automatically ([`2642365`](https://github.com/Byron/crates-io-cli-rs/commit/2642365e42da912e0206162e6287a480f21f8fa1))
    - Spawn processing loop ([`05d22a6`](https://github.com/Byron/crates-io-cli-rs/commit/05d22a6174b78c1cbe60ff1ece3ed0072a05281e))
    - Urls are now controlled by the scheduler; source url is recorded ([`2286a7b`](https://github.com/Byron/crates-io-cli-rs/commit/2286a7be31163658dcc49f7c756e271f5211c073))
    - Now with the ability to store the downloads ([`a9f2ee1`](https://github.com/Byron/crates-io-cli-rs/commit/a9f2ee1519b933dffc882edefd5b63b7a6a28f29))
    - Start writing downloaded data directly ([`dbb785f`](https://github.com/Byron/crates-io-cli-rs/commit/dbb785fcdf8a799f9e23c3eb8d429ebde3735aa1))
    - Allow configuring amount of downloaders ([`52b37fa`](https://github.com/Byron/crates-io-cli-rs/commit/52b37fa08d0c941a9f4308d62b3b9ff7fba39965))
    - And now with proper download progress ([`d229968`](https://github.com/Byron/crates-io-cli-rs/commit/d229968aa4dabbbeb04b0e6535fd3ac89e4bc982))
    - Set prodash version to something usable ([`c09dde7`](https://github.com/Byron/crates-io-cli-rs/commit/c09dde7155af9b5310795fd012f98c6441f25fbb))
    - For now, let's only show messages, not the progress itself ([`1097f46`](https://github.com/Byron/crates-io-cli-rs/commit/1097f4617d4b89f93a3fb93abf118e1a99a314ba))
    - Fix prodash feature toggles ([`92bebce`](https://github.com/Byron/crates-io-cli-rs/commit/92bebced90337201facf99d3e0d3959fced1e682))
    - Add no-gui mode ([`9d9e416`](https://github.com/Byron/crates-io-cli-rs/commit/9d9e4164c0c9adb7589b38e4709301cf0ca14cb6))
    - Make clear which executor is which ([`29ecfb3`](https://github.com/Byron/crates-io-cli-rs/commit/29ecfb3b20446945840a6fd94923ff638bbcdde0))
    - Configure tokio runtime more precisely ([`1655731`](https://github.com/Byron/crates-io-cli-rs/commit/16557312d503808283da7552b4b4800f8d733554))
    - Add tokio runtime for reqwest ([`ccf8fc7`](https://github.com/Byron/crates-io-cli-rs/commit/ccf8fc779c59155e8958d9d95794e7182bbadafe))
    - Trying reqwest, which unfortunately relies on old tokio :/, needs tokio ([`0b1fa61`](https://github.com/Byron/crates-io-cli-rs/commit/0b1fa61e376c7568bdfca70238cbbc791e8a3e3f))
    - Remove surf ([`71ae962`](https://github.com/Byron/crates-io-cli-rs/commit/71ae96242a8ebc2d980ab8e43f5661f2b5dc35c1))
    - First usage of surf client - doesn't look too good ([`7eeed37`](https://github.com/Byron/crates-io-cli-rs/commit/7eeed37080860972d1f5ab9c38dea438c3d29b71))
    - Now tasks are kept track of ([`33261f5`](https://github.com/Byron/crates-io-cli-rs/commit/33261f5f205496cefcc3c5dc2aeece8530a0ce33))
    - Refactor ([`cfd3319`](https://github.com/Byron/crates-io-cli-rs/commit/cfd331917eb95a8b21aca5d5288f0d9953a97084))
    - First clumsy attempt to read and write into a tree… ([`9077747`](https://github.com/Byron/crates-io-cli-rs/commit/9077747433febfbfb4a54fff937dc7391ba9c144))
    - Refactor ([`d3c2c3c`](https://github.com/Byron/crates-io-cli-rs/commit/d3c2c3cfd43ff1bcc8e95bc2608e4303a226f1b8))
    - Refactor ([`68beaa7`](https://github.com/Byron/crates-io-cli-rs/commit/68beaa7fb02cc5697a2c2a9bd5e73db05b8e2fa4))
    - Only one allocation for merge please ([`aaef111`](https://github.com/Byron/crates-io-cli-rs/commit/aaef111f118864e59b7c54ae746dec5707a5f984))
    - Probably finished merging the task state ([`ebe1574`](https://github.com/Byron/crates-io-cli-rs/commit/ebe1574c3604441bae3e1cb24707b33c94ce7b84))
    - Implement task metadata, naively ([`0b8f367`](https://github.com/Byron/crates-io-cli-rs/commit/0b8f3678fcaafe267f1fe15311c41879930b6b0a))
    - First sketch of tasks tree ([`0cc7246`](https://github.com/Byron/crates-io-cli-rs/commit/0cc7246cf2e56b2601ce0cf377a3babf1a4cdb83))
    - Consolidate keying into a trait ([`e020bd5`](https://github.com/Byron/crates-io-cli-rs/commit/e020bd564cdcdb567b6d96973e26b37699ad9d64))
    - A more flexible task data structure ([`d2ba255`](https://github.com/Byron/crates-io-cli-rs/commit/d2ba255ac9f09dfc10598a4ea5665588e1ad639c))
    - Remove isaac - it's not time to choose just yet ([`dc6c660`](https://github.com/Byron/crates-io-cli-rs/commit/dc6c66047e62c6dab54ea48b2b5c4e4e78dddce4))
    - Dependency with Cows ([`b6f43ea`](https://github.com/Byron/crates-io-cli-rs/commit/b6f43ea80ef46da46a80e2d639bd8d03668466fe))
    - Hashmap as Cow ([`8b13f55`](https://github.com/Byron/crates-io-cli-rs/commit/8b13f55d78eaeaba405959b98cd55d7b364d19d6))
    - Use borrowed version of dependencies, too ([`d0149a1`](https://github.com/Byron/crates-io-cli-rs/commit/d0149a1766d3ab0470bfb827ebef57407d746fe2))
    - More Cows, now for Crates - makes a difference! ([`a3f17d0`](https://github.com/Byron/crates-io-cli-rs/commit/a3f17d0b32ffa7f73443e54491539b9985301832))
    - More Cow for crate version ([`ddc6ddc`](https://github.com/Byron/crates-io-cli-rs/commit/ddc6ddcf96394ccd77503660b3482acb8e63acda))
    - Sketch download data structure for persistence ([`adf11fc`](https://github.com/Byron/crates-io-cli-rs/commit/adf11fc29474a3927ecdfc33488a266a773c6560))
    - Prepare use isahc as http client ([`8df063e`](https://github.com/Byron/crates-io-cli-rs/commit/8df063ebba103f4c0c1cef16095a87c85688bd06))
    - No need to reverse crates - they are sorted alphabeticall ([`004348e`](https://github.com/Byron/crates-io-cli-rs/commit/004348ed57a0cde1e566d29a7e4af5d97f9b0ecf))
    - Now with simulated downloads ([`f3bd710`](https://github.com/Byron/crates-io-cli-rs/commit/f3bd710146dd5ea9b00734916a2ad8e32c5f46ac))
    - Dependency update - doesn't fix the issue though ([`b12953d`](https://github.com/Byron/crates-io-cli-rs/commit/b12953df1061815f7e89e34e13c20c7f6bbd33a3))
    - At least now it compiles, but it seems to abort way too early ([`fc5d727`](https://github.com/Byron/crates-io-cli-rs/commit/fc5d7275b9f15acfc383682a14339d56f423bf2d))
    - First stab at doing work scheduling continuously… ([`34126b4`](https://github.com/Byron/crates-io-cli-rs/commit/34126b4c480329a36912be5e8f87c7ae8818575b))
    - Make fetch timer progress transient ([`4c38d2d`](https://github.com/Byron/crates-io-cli-rs/commit/4c38d2d647d9a4c3282a8602dee075f1b5350054))
    - Refactor ([`be30425`](https://github.com/Byron/crates-io-cli-rs/commit/be304258a3ab0bb247f80771dc30896653792f35))
    - Refactor ([`dfea2c8`](https://github.com/Byron/crates-io-cli-rs/commit/dfea2c8e8d05e0939a596116349a98a3a9234550))
    - Improve deadline and error handling ([`aa82bc1`](https://github.com/Byron/crates-io-cli-rs/commit/aa82bc1416c871667232be6f03b8354664c5314c))
    - The first step towards making criner run forever ([`bdcb185`](https://github.com/Byron/crates-io-cli-rs/commit/bdcb185aadffd4a13fcd331b04c3f4f865112593))
    - Now set the name of the download ([`730d1b9`](https://github.com/Byron/crates-io-cli-rs/commit/730d1b99f3d357b09a07b6309bc6a882b5adc0c3))
    - First use of channels to throttle work ([`a9a1344`](https://github.com/Byron/crates-io-cli-rs/commit/a9a13448bc81b8e3892eaf67a62d22a0fef8022b))
    - Refactor ([`fffdf1f`](https://github.com/Byron/crates-io-cli-rs/commit/fffdf1f78acd07a349ceabab40ca5769e0e9173d))
    - Refactor ([`b36f96c`](https://github.com/Byron/crates-io-cli-rs/commit/b36f96c9468f0f8937f0ae0d9b591419c9ef7c27))
    - Refactor ([`e392b48`](https://github.com/Byron/crates-io-cli-rs/commit/e392b487c8aca5aa1779598cbba2bcdb95f56912))
    - Finalized pool setup - this seems to be working ([`bcc1917`](https://github.com/Byron/crates-io-cli-rs/commit/bcc19171f89a455aa04740be24ccfa103d047bb0))
    - For some reason, only the blocking queue makes progress ([`41437d6`](https://github.com/Byron/crates-io-cli-rs/commit/41437d61bf7fbbaa2faa949c38fbfe7a56b62fff))
    - Frame for download tasks, but…they don't seem to be polled ([`ac9c92b`](https://github.com/Byron/crates-io-cli-rs/commit/ac9c92b5198ecdfc8dd7cfea37388c7cacda7165))
    - First sketch of using a task scheduler for crate versions ([`2416a2a`](https://github.com/Byron/crates-io-cli-rs/commit/2416a2a1b46bc80c15d347196a4b0c6f1fe846ca))
    - Progress improvements ([`11fa685`](https://github.com/Byron/crates-io-cli-rs/commit/11fa68533492a9bffa7b701d48264b2046565cc2))
    - Migration to prodash complete! ([`6a5a6cb`](https://github.com/Byron/crates-io-cli-rs/commit/6a5a6cbe6a77a1e3c32fddd9f130ade2e447b3ec))
    - Refator ([`28803c0`](https://github.com/Byron/crates-io-cli-rs/commit/28803c06283c28d6cc5214d0de72cae966adec58))
    - Display statistics ([`00dd372`](https://github.com/Byron/crates-io-cli-rs/commit/00dd3728318a0d6e25958b8fe894bc8e89d55e52))
    - Refactor ([`c1d2165`](https://github.com/Byron/crates-io-cli-rs/commit/c1d2165081a0f2e6f99d68841ab6d7e00907bade))
    - Behaviour is a bit more natural, but gui shutdown is still tricky ([`7c10838`](https://github.com/Byron/crates-io-cli-rs/commit/7c10838db882528d42f13e982390d7a3523eb522))
    - Refactor - now the future doesn't spawn its own pool :D ([`44e6425`](https://github.com/Byron/crates-io-cli-rs/commit/44e64251cd1be7ae6b20867ab94a131f8104c4b4))
    - First bare-bones version of prodash intergration to criner ([`06cc691`](https://github.com/Byron/crates-io-cli-rs/commit/06cc69169166ff546ece2c1ac906e186d68153bb))
    - (cargo-release) start next development iteration 1.0.2-alpha.0 ([`06b5ad0`](https://github.com/Byron/crates-io-cli-rs/commit/06b5ad09b43f93fc571509284382c70845bd3da7))
    - Include README in crate ([`d27e85c`](https://github.com/Byron/crates-io-cli-rs/commit/d27e85c4d6de24b7aaacf9b5ce9006d16b2f8531))
    - Add --speed-multiplier to dashboard ([`910a4d9`](https://github.com/Byron/crates-io-cli-rs/commit/910a4d9e038bbe6ca709e28acd871474a88ffd52))
    - (cargo-release) start next development iteration 1.0.1-alpha.0 ([`bced7a1`](https://github.com/Byron/crates-io-cli-rs/commit/bced7a1358a3d6e3f86df0012ef8a95424964e95))
    - Add asciinema link ([`659d575`](https://github.com/Byron/crates-io-cli-rs/commit/659d57552e576e4999bffcad97950b8f1df8043f))
    - Add license ([`1d1a7ae`](https://github.com/Byron/crates-io-cli-rs/commit/1d1a7ae50f511a052860792c1f589e25a9006490))
    - Improvements to the readme ([`fd17bdd`](https://github.com/Byron/crates-io-cli-rs/commit/fd17bdd8ad4846e9d4b4a2a5d36d7d629df4f5a7))
    - Finally fix centering title boxes ([`0537f79`](https://github.com/Byron/crates-io-cli-rs/commit/0537f791d90b1c692514445ee98989d008b58180))
    - Remove noisy text from dashboard demo ([`33e836e`](https://github.com/Byron/crates-io-cli-rs/commit/33e836e149569339bb3b1e2974f9b37f35c019c9))
    - Finish TUI documentation ([`f45134e`](https://github.com/Byron/crates-io-cli-rs/commit/f45134e11103ad4999fef1be8c49265c745790af))
    - Minimal example for bringing up a tui ([`bbe0912`](https://github.com/Byron/crates-io-cli-rs/commit/bbe091248adb3e8525fae9a3e480d7aec1303a0f))
    - More renaming ([`8422591`](https://github.com/Byron/crates-io-cli-rs/commit/8422591242db5c5d67785aa0449713ee90e88302))
    - First round of documentation fixes after restructuring ([`fd6bf28`](https://github.com/Byron/crates-io-cli-rs/commit/fd6bf28a8e4096bae45749974ab05b5ba725734c))
    - First round of renaming ([`bf6a0dc`](https://github.com/Byron/crates-io-cli-rs/commit/bf6a0dc37612e603d5772b8fb2d0627d595206d0))
    - Document everything in the progress tree ([`949eeeb`](https://github.com/Byron/crates-io-cli-rs/commit/949eeeb2ab3d643f81e749d932219b96f2fa0873))
    - Run cargo check for prodash explicitly ([`23a6510`](https://github.com/Byron/crates-io-cli-rs/commit/23a6510bf065c18badc500767a270bf2292f8318))
    - Show that it's possible to re-init progress ([`f1f080e`](https://github.com/Byron/crates-io-cli-rs/commit/f1f080ecabfad5171c7fc09814b76f439cd673c1))
    - Refactor ([`b81b2e1`](https://github.com/Byron/crates-io-cli-rs/commit/b81b2e11a0baabf9d7e9a107bf444f08b2ea4ae7))
    - Auto-hide info pane if it gets too small ([`11b109e`](https://github.com/Byron/crates-io-cli-rs/commit/11b109e4ff2975adeffc083cf992226081227732))
    - Fix information margin ([`f5baa57`](https://github.com/Byron/crates-io-cli-rs/commit/f5baa5766f59fcd2a2e03a8bcb031ade79dba472))
    - Fix out-of-bounds drawing ([`ac44295`](https://github.com/Byron/crates-io-cli-rs/commit/ac442953c0c1ab54af366ebe946606b1a952c17e))
    - Draw last line as well ([`593e23b`](https://github.com/Byron/crates-io-cli-rs/commit/593e23b442693de4de8d05cec7835b71da44ea38))
    - Pretty decent drawing of statistics ([`ad7678c`](https://github.com/Byron/crates-io-cli-rs/commit/ad7678c821dc66a6c9f0adc34879a0ec312536e5))
    - Support for maximizing info pane ([`1dfa622`](https://github.com/Byron/crates-io-cli-rs/commit/1dfa622a036359b51ddeb7394893447ad92876ef))
    - Now panes don't overlap ([`185a9e0`](https://github.com/Byron/crates-io-cli-rs/commit/185a9e0cc958e589a0d3cfb5eaf1bacbfde5428e))
    - Refactor ([`efbbe5d`](https://github.com/Byron/crates-io-cli-rs/commit/efbbe5d6098f6d5ccbd9770296a6349924267ebb))
    - First message info box drawing ([`8416444`](https://github.com/Byron/crates-io-cli-rs/commit/8416444776a8ee9bda988f814009459dd6082d6b))
    - Prepare for info pane bound computation ([`d43d5e3`](https://github.com/Byron/crates-io-cli-rs/commit/d43d5e35f4e5e5c51f45839ca93a39b89e8a38a3))
    - Dashboard demo now sends statistic ([`f836a1f`](https://github.com/Byron/crates-io-cli-rs/commit/f836a1fea77d20eb53d4cca106086ef9332b0596))
    - Minor improvements ([`1f7b4a3`](https://github.com/Byron/crates-io-cli-rs/commit/1f7b4a358bfb8a9c02c8d1189062b51c8a814279))
    - Help text for message handling ([`2d5a15b`](https://github.com/Byron/crates-io-cli-rs/commit/2d5a15bd096b2f71ecf1b991ff854897ecd053b4))
    - Add message pane help text ([`8ee236d`](https://github.com/Byron/crates-io-cli-rs/commit/8ee236d5661ef62d0e5bda964b4d7092f0637813))
    - Refactor; add help for task navigation ([`aa285b5`](https://github.com/Byron/crates-io-cli-rs/commit/aa285b539a387af579b57af6f8e4a4b6447b965f))
    - Progress overflow handles skips properly ([`f735f31`](https://github.com/Byron/crates-io-cli-rs/commit/f735f3190d4b44770ee6e326b4136772ebfdb78f))
    - Avoid short 'blip' of overflow bar if there is nothing to overflow ([`ef3df0a`](https://github.com/Byron/crates-io-cli-rs/commit/ef3df0a9bd3f894f6d485bc4691187dde5462529))
    - Near-fullscreen message pane support ([`a478694`](https://github.com/Byron/crates-io-cli-rs/commit/a478694e59fa701e59eb9a54511d541c384a5d98))
    - Allow scrolling 10 messages/tasks at a time ([`7e72f3f`](https://github.com/Byron/crates-io-cli-rs/commit/7e72f3ffe26e6469f1cadb674c73a10e69b72ab9))
    - Allow toggling the messages pane ([`4e77da2`](https://github.com/Byron/crates-io-cli-rs/commit/4e77da22f432fbba48e413a398f051792c67a7ff))
    - Rename 'progress-dashboard' to 'prodash' ([`1d30b6c`](https://github.com/Byron/crates-io-cli-rs/commit/1d30b6c174566c56167f6212f0338ab965d33667))
    - Phrase concerns about underlying datastructure ([`a7e0321`](https://github.com/Byron/crates-io-cli-rs/commit/a7e0321915d42f66fb3d14bc0d75a36f37828f9c))
    - Refactor; allow tasks to change their name ([`b95eeba`](https://github.com/Byron/crates-io-cli-rs/commit/b95eebae3acd97dbd8ec8b41eb80a20403684848))
    - Allow back-propagation of offset values to… ([`4158db4`](https://github.com/Byron/crates-io-cli-rs/commit/4158db46907be7a8507c7211b685722e21fd3ee2))
    - Offset sanitization ([`8849397`](https://github.com/Byron/crates-io-cli-rs/commit/88493970622c9c8459a424ac532ef9e30c8ee4c8))
    - Naive offset implementation for tasks ([`dedc963`](https://github.com/Byron/crates-io-cli-rs/commit/dedc96307f94a983dfbefa6dbb6c909b60ab34de))
    - Better overflow message ([`005c683`](https://github.com/Byron/crates-io-cli-rs/commit/005c683af4f03230d2cd1f26cd9cfcd614a71dc9))
    - Offset-aware overflow message calculation ([`686b611`](https://github.com/Byron/crates-io-cli-rs/commit/686b611d9de0fa19585600b5192165689869480d))
    - First naive way of doing message buffer offsets ([`87b3e93`](https://github.com/Byron/crates-io-cli-rs/commit/87b3e93b448462a86c05a0a640591def520661ae))
    - Greatly improve redraw logic ([`c642f3c`](https://github.com/Byron/crates-io-cli-rs/commit/c642f3c5b0b5bb26875137c0f3d8ecfc00284d4a))
    - Refactor ([`feb657e`](https://github.com/Byron/crates-io-cli-rs/commit/feb657ef7732a8efe7ef8513e65e533d4e6024a7))
    - Allow setting the title dynamically ([`fb45a3a`](https://github.com/Byron/crates-io-cli-rs/commit/fb45a3a6dc777e74d2be77a1a6e712403933340f))
    - First wiring of keys for going up and down ([`2210efc`](https://github.com/Byron/crates-io-cli-rs/commit/2210efc15046df5dd7efb37954c424baec9b4189))
    - Bump timeout - it's barely working with 5s in China … ([`10db62c`](https://github.com/Byron/crates-io-cli-rs/commit/10db62c91b06eaaac8454d7ef679a5344f810ebf))
    - Overflow handling for messages ([`88cd180`](https://github.com/Byron/crates-io-cli-rs/commit/88cd180cf5cf02fe3ec1076e275d93a329f66c5f))
    - Add argh-based argument parser ([`800794a`](https://github.com/Byron/crates-io-cli-rs/commit/800794a916d6f6ac1924e7b1a37d1a8bff3ed2c9))
    - Refactor ([`c090d0e`](https://github.com/Byron/crates-io-cli-rs/commit/c090d0e459dfb08538ff104cb8b6426feab171cf))
    - Respect window ratio when computing size ([`2367b7a`](https://github.com/Byron/crates-io-cli-rs/commit/2367b7a7184718639178845c459b134b0d92b705))
    - Implement auto-resizing - should be proportional though ([`e07b35d`](https://github.com/Byron/crates-io-cli-rs/commit/e07b35da5b4f9beea0ac0eeff750533961135638))
    - Lay the ground work for supporting events fed by users ([`39f0317`](https://github.com/Byron/crates-io-cli-rs/commit/39f03173538e3f6f136126b1506323cb736bceac))
    - Use streams and select on them! NEAT ([`07fe2f1`](https://github.com/Byron/crates-io-cli-rs/commit/07fe2f1ceb6daa98883a5fb547dfae488a085321))
    - Add message origin ([`9afea23`](https://github.com/Byron/crates-io-cli-rs/commit/9afea2348d0e6e00dfeebe62cfe2b71be47b9617))
    - An attempt to reduce magic numbers ([`c995ed0`](https://github.com/Byron/crates-io-cli-rs/commit/c995ed0bed0e66857e47631e1f403bd01c55c50d))
    - On average terminals, a smaller message buffer is sufficient ([`4ebd8f0`](https://github.com/Byron/crates-io-cli-rs/commit/4ebd8f0ffaef775fe67996fd25786641f88a68e9))
    - First naive implementation of showing log level… ([`9bf406d`](https://github.com/Byron/crates-io-cli-rs/commit/9bf406d9f9e21946109bf7cacf4fce37906e5bb0))
    - Only show hours minutes seconds ([`6e3459d`](https://github.com/Byron/crates-io-cli-rs/commit/6e3459d35de6ed31ba82521a68603d45ee8da9ab))
    - Refactor ([`74f05ec`](https://github.com/Byron/crates-io-cli-rs/commit/74f05ec906bf1fbb00b14aa8385cb8e320fc8820))
    - Fix crash at a better spot :) ([`180d55a`](https://github.com/Byron/crates-io-cli-rs/commit/180d55abee22234b1f75ba90bec47cc3c8b647da))
    - Fix out of bound access to buffer with multi-block characters ([`e4af4db`](https://github.com/Byron/crates-io-cli-rs/commit/e4af4db81710881fee31be43d010ba8f3382aa10))
    - First naive drawing of time stamps ([`5e1d80e`](https://github.com/Byron/crates-io-cli-rs/commit/5e1d80e15e0e26516eaa4ee2190e7026599f7da9))
    - First primitive message drawing ([`3c16685`](https://github.com/Byron/crates-io-cli-rs/commit/3c16685c5a297270bb9b14f4560f9264f2676447))
    - Seemingly appropriate frame computations ([`e729ea7`](https://github.com/Byron/crates-io-cli-rs/commit/e729ea7422dfe8abb695f7f4ffcea55211611522))
    - Refactor ([`64a5fb8`](https://github.com/Byron/crates-io-cli-rs/commit/64a5fb8a022ea31ab6d2e9088deca428ddddb797))
    - Make internal buffer sizes and capacity accessible ([`b634b2b`](https://github.com/Byron/crates-io-cli-rs/commit/b634b2b65eaef7dc956cda46cb9e37dd325dbd04))
    - Reduce amount of messages kept to what a terminal would typically show… ([`956fadf`](https://github.com/Byron/crates-io-cli-rs/commit/956fadf14112802b0c45fcf3c6a536ac21b55f11))
    - Implement copying all messages… ([`23d174d`](https://github.com/Byron/crates-io-cli-rs/commit/23d174d5feb83969d6d6d3ad8c9c8b9a6e146f76))
    - Now with actually storing messages - it's pretty slow, but… ([`937255a`](https://github.com/Byron/crates-io-cli-rs/commit/937255a549e85d1bd1b3425ba53d7667560240f1))
    - A step closer to storing messages ([`3dc0257`](https://github.com/Byron/crates-io-cli-rs/commit/3dc02575b369df99be9be92dfe7a9225b7d350a6))
    - Sketched interface for sending progress messages ([`40e91a1`](https://github.com/Byron/crates-io-cli-rs/commit/40e91a1142163b181fb2ae93ffe1969c00b449fa))
    - YES - finally, how silly could I be! Mixing coordinate systems… ([`918ff34`](https://github.com/Byron/crates-io-cli-rs/commit/918ff3482778ade5cba0f566032d2f9f227a8c0b))
    - Seemingly working drawing for multi-block-width unicode characters ([`713f30c`](https://github.com/Byron/crates-io-cli-rs/commit/713f30c2bda2d3a14556888138f9b95e9e7b7093))
    - For some reason, titles were offset and needed a new formula… ([`7a4da36`](https://github.com/Byron/crates-io-cli-rs/commit/7a4da36efd4175eb1a990093bb15b94d0dce5a95))
    - Show that multi-width unicode characters are not displayed correctly ([`cdb81b7`](https://github.com/Byron/crates-io-cli-rs/commit/cdb81b7e4c99d73dd8dfab7ddd285f51b16f8458))
    - Bring back 10fps ([`09caca2`](https://github.com/Byron/crates-io-cli-rs/commit/09caca2dc58e054c1c8d516e81cb24e282b06679))
    - Sub-second precision for really really saving energy ([`b6ac025`](https://github.com/Byron/crates-io-cli-rs/commit/b6ac025002194abe2a3266a0f1299dfebd28a6b0))
    - Headline is now right-aligned ([`b5a5b1c`](https://github.com/Byron/crates-io-cli-rs/commit/b5a5b1c6057bc6a7c4294ce5e029bebe84279f61))
    - First usable headline ([`3fbc539`](https://github.com/Byron/crates-io-cli-rs/commit/3fbc5394546a0799a10c4df8df2e1ff40c32e99e))
    - Deny unsafe ([`b16b1e2`](https://github.com/Byron/crates-io-cli-rs/commit/b16b1e2db885cef00fb154afb6f93bf03baf2260))
    - Show ETA when blocked ([`e27f632`](https://github.com/Byron/crates-io-cli-rs/commit/e27f6327ee08feaf773cc3c964de6078e182b05a))
    - Fix crashes with very low vertical or horizontal size ([`ad9b1c1`](https://github.com/Byron/crates-io-cli-rs/commit/ad9b1c1fc4df2b947114173ac9800b3af7910bd3))
    - Support for blocking task (without ETA display) ([`3de027b`](https://github.com/Byron/crates-io-cli-rs/commit/3de027b3576b6dd6284f67eb4661514fbec47e41))
    - Optimize performance ([`f436dff`](https://github.com/Byron/crates-io-cli-rs/commit/f436dffa3c2d4d2d68d9be1e725b06857f69c773))
    - Auto-collapsing tree based on size ([`8e226ae`](https://github.com/Byron/crates-io-cli-rs/commit/8e226aeef4f2cf67a7628c667ac8c64e8a63cd30))
    - Refactor ([`13929c6`](https://github.com/Byron/crates-io-cli-rs/commit/13929c68a7d5a497ab7e77e1c3f11c1f15843961))
    - Refactor ([`89afff2`](https://github.com/Byron/crates-io-cli-rs/commit/89afff2e15752587944854b0852d038af802b287))
    - Refactor ([`8821d4e`](https://github.com/Byron/crates-io-cli-rs/commit/8821d4e9bd335d571f28859e3047df5f9f8131d4))
    - That's the look for overflow ([`e5d1813`](https://github.com/Byron/crates-io-cli-rs/commit/e5d1813d232379dffb86cbca82873beb7e9bf91f))
    - Refactor ([`94569aa`](https://github.com/Byron/crates-io-cli-rs/commit/94569aa35156a42e995438391638cd45e8b02b30))
    - Draw a progress bar for all tasks ([`afa8d32`](https://github.com/Byron/crates-io-cli-rs/commit/afa8d3214c7914d5447533ef672c530b3086985f))
    - Nicer overflow rendering ([`fc67e48`](https://github.com/Byron/crates-io-cli-rs/commit/fc67e48a99f8b4fc4735211acd93159b71bd1578))
    - Rearrange task orders ([`e35a8aa`](https://github.com/Byron/crates-io-cli-rs/commit/e35a8aa1afd7c2f22783e3de3adab35f3750810e))
    - Refactor ([`8abf05f`](https://github.com/Byron/crates-io-cli-rs/commit/8abf05fe15932b20bfa2bf5788bc14004353909c))
    - Refactor ([`160b068`](https://github.com/Byron/crates-io-cli-rs/commit/160b068aa29712c925eea4d7b2e477985ea319fc))
    - Refactor ([`a086ec9`](https://github.com/Byron/crates-io-cli-rs/commit/a086ec9c8cac4e41e52ab0b97ded2d7d5fad1ac6))
    - Refactor ([`3170c2b`](https://github.com/Byron/crates-io-cli-rs/commit/3170c2becbdbe5fb2fe9dac9167f7eaf44253069))
    - Refactor ([`2f4f630`](https://github.com/Byron/crates-io-cli-rs/commit/2f4f630ce500a4fe630b19c5998a86542c6580fc))
    - Refactor ([`fd86630`](https://github.com/Byron/crates-io-cli-rs/commit/fd8663025979feabcf2aefe31e159d0e28daaac0))
    - Split utilities into module ([`e941438`](https://github.com/Byron/crates-io-cli-rs/commit/e9414381bacaae05dcf13055b5cafa2ee7c137d1))
    - Support for title ([`f7bc9be`](https://github.com/Byron/crates-io-cli-rs/commit/f7bc9be95412daec88c9f9026567ac17b3e234e3))
    - Progress bars now work as expected ([`4ec8e59`](https://github.com/Byron/crates-io-cli-rs/commit/4ec8e59c690d176a79e795ecdd2c4358d1a6b000))
    - Some code simplified, but progress bars are off by one :/ ([`511e61f`](https://github.com/Byron/crates-io-cli-rs/commit/511e61f8ca1420e7f1a952cfb54bc465fcbfc3b6))
    - Better handling of lack of entries ([`5e4b064`](https://github.com/Byron/crates-io-cli-rs/commit/5e4b064320dba1d09a97c36dec5fc30716e567fa))
    - Refactor ([`d0c2d52`](https://github.com/Byron/crates-io-cli-rs/commit/d0c2d52e684d2ea50fc6e2cd0f69dedc30a61f03))
    - And this is how it should be done (at least) ([`7690ead`](https://github.com/Byron/crates-io-cli-rs/commit/7690eadcd50b81e2798bb4a0a1d749af0934e89a))
    - Brute-force way of doing elipsis better ([`6a25d3f`](https://github.com/Byron/crates-io-cli-rs/commit/6a25d3ffaeb24e16e939405bd7c5f04b0d6125a6))
    - Fix progress line overdraw ([`3cba011`](https://github.com/Byron/crates-io-cli-rs/commit/3cba011a3920e0e1356091bf1ccb596c1da0b6f4))
    - Better title spacing when window is small ([`9442bd5`](https://github.com/Byron/crates-io-cli-rs/commit/9442bd5f5926d8f62332eed531a0965582d5ba7c))
    - Better, but not perfect, … (elipsis) handling ([`be80b5a`](https://github.com/Byron/crates-io-cli-rs/commit/be80b5a871b3314cb77e2403d29b4d02e21ec295))
    - Add some simple benchmarks ([`a674af5`](https://github.com/Byron/crates-io-cli-rs/commit/a674af528059c94b68b2c9b6d462dc55238a8d11))
    - Speed up execution by optimizing dependencies ([`361871a`](https://github.com/Byron/crates-io-cli-rs/commit/361871a30ba9531deddd4b41ab6767737979fc74))
    - Fix crash bug - Rect::intersection() triggers underflows ([`e9c80b2`](https://github.com/Byron/crates-io-cli-rs/commit/e9c80b216ed80d4068e7bad516cde4203380719a))
    - More crash safety (drawing out of bounds) ([`80d4662`](https://github.com/Byron/crates-io-cli-rs/commit/80d4662e1f373e98000bf9f78f95abd418082230))
    - Refactor ([`66f5c01`](https://github.com/Byron/crates-io-cli-rs/commit/66f5c01067b89bf40367efd424d623ee7de09ff3))
    - Make progress tree configurable ([`227b172`](https://github.com/Byron/crates-io-cli-rs/commit/227b172765d22f14d2ce4128a70def2ab2709eb6))
    - First version of simple unbounded progress ([`ac43e2f`](https://github.com/Byron/crates-io-cli-rs/commit/ac43e2fc8c129c4c593c1281834dcf6dcf41a112))
    - Frame for implementing a pulse-like progress bar… ([`5a32b59`](https://github.com/Byron/crates-io-cli-rs/commit/5a32b592f7a46866708f7ae41b0e0429c41a763c))
    - Green progress bar if progress is 80% or more ([`daa950b`](https://github.com/Byron/crates-io-cli-rs/commit/daa950ba0b9469f5c0c875b7bb81122ca6adbd2c))
    - Draw progress texts with more contrast when ther is a bar ([`4176f42`](https://github.com/Byron/crates-io-cli-rs/commit/4176f4254337a10efca405cd24d2f7f4c1089bf3))
    - Use draw_text… everywhere for visibly improved performance and ease-of-use ([`ea79462`](https://github.com/Byron/crates-io-cli-rs/commit/ea794624bdb4f80a194951d0912fee085b1f1893))
    - Progress bar is not overdrawn by text anymore ([`65af7e7`](https://github.com/Byron/crates-io-cli-rs/commit/65af7e7660e8d2a581b1c853f6c3b3920d1d430f))
    - Properly centered titles with hierarchy indicator ([`2014bdd`](https://github.com/Byron/crates-io-cli-rs/commit/2014bddf9afc7656f8596b0b40977561165188e1))
    - Refactor ([`8bb766e`](https://github.com/Byron/crates-io-cli-rs/commit/8bb766e8b81d4bb1754d501b5f4782c7342fa0c6))
    - Refactor ([`f0cc663`](https://github.com/Byron/crates-io-cli-rs/commit/f0cc66369b8808a4cd9b11208e1825b990ce5251))
    - Update goals ([`e429bdf`](https://github.com/Byron/crates-io-cli-rs/commit/e429bdf1c329a1c41d54003a2986784e36b4f657))
    - Some fixes ([`4cacb47`](https://github.com/Byron/crates-io-cli-rs/commit/4cacb47bd2a197d0d0cb4f0d3445b9cc41201d58))
    - Better title-only mode ([`a3e0267`](https://github.com/Byron/crates-io-cli-rs/commit/a3e0267ac2749eb845d2ecf8d91cf363f2eec166))
    - First version of title-only ([`bb5a44a`](https://github.com/Byron/crates-io-cli-rs/commit/bb5a44acae8924e5cd9fec2bd75703bbe8c9f1c9))
    - Refactor ([`53a18ab`](https://github.com/Byron/crates-io-cli-rs/commit/53a18ab7b920f1d894442702348429fb7269625c))
    - Colored progress bars! ([`c8f80f3`](https://github.com/Byron/crates-io-cli-rs/commit/c8f80f3a6a2e5f8d953cb919ab848eb5b896e296))
    - Nice horizontal overflow handling (good enough) ([`491dfa8`](https://github.com/Byron/crates-io-cli-rs/commit/491dfa803fdcfe2cd3daf800fb0e9769dc82bb72))
    - Prepare for proper column separation ([`86381ec`](https://github.com/Byron/crates-io-cli-rs/commit/86381ec2fd39c29385c52df0b4ca64abf2c72f89))
    - Refactor ([`699882c`](https://github.com/Byron/crates-io-cli-rs/commit/699882c64fe259a02be2017f85f709ba2a1d5283))
    - Refactor ([`181c9bc`](https://github.com/Byron/crates-io-cli-rs/commit/181c9bcd60d12b0651e7578c5932cac54bf4b0af))
    - Refactor ([`d5bfa70`](https://github.com/Byron/crates-io-cli-rs/commit/d5bfa7090c7ed3b24672c7939c027dc6b9721eeb))
    - Fix overflow computation ([`18e213f`](https://github.com/Byron/crates-io-cli-rs/commit/18e213f441ada8561285571850e17e7df1d4119f))
    - Refactor ([`ab30676`](https://github.com/Byron/crates-io-cli-rs/commit/ab30676c5628e2e47db3c01cc38893fc306676f0))
    - Better alignment thanks to precomputation ([`23fee3b`](https://github.com/Byron/crates-io-cli-rs/commit/23fee3b855ac76c717f505ab45d46df0d2787acf))
    - Get the benefit of a vector for pre-computations ([`a104c3b`](https://github.com/Byron/crates-io-cli-rs/commit/a104c3b7a330101c328b14bc54c3f5b4697c296b))
    - Formatting ([`dac5da2`](https://github.com/Byron/crates-io-cli-rs/commit/dac5da2995b79ca5200860312045547b588c5e20))
    - Formatting ([`717a2aa`](https://github.com/Byron/crates-io-cli-rs/commit/717a2aa750bd3289ccdb6f5150f074622312bbbe))
    - A test to see if right-alignment looks better… ([`758e1c2`](https://github.com/Byron/crates-io-cli-rs/commit/758e1c2b6ee3b061c077107065f44e96750c0e43))
    - Now with percentage of overflowing taskst ([`67e6518`](https://github.com/Byron/crates-io-cli-rs/commit/67e65180ebb825450061b3a6e523db4529aa2727))
    - Basic overflow handling ([`14c55e5`](https://github.com/Byron/crates-io-cli-rs/commit/14c55e5bf4fb7010e508ebe0bc7802d66f09624c))
    - Remove floating extra level ([`5ce1e29`](https://github.com/Byron/crates-io-cli-rs/commit/5ce1e296b7d9b8a18ffab26efef3f3291ab5d941))
    - Revert "probably better level lifetime handling" ([`d6bd124`](https://github.com/Byron/crates-io-cli-rs/commit/d6bd1243de6a3996a0bb9fc63b1e7fb03325ee2d))
    - Probably better level lifetime handling ([`70b7696`](https://github.com/Byron/crates-io-cli-rs/commit/70b7696fd5c4ddee0ebcc27af6349c129e51ea1f))
    - More pooled tasks ([`ee3a53a`](https://github.com/Byron/crates-io-cli-rs/commit/ee3a53aaa333d0968bed141cef5824ef324a8af0))
    - Revert "Attempt to use a different way of creating tasks… FAIL :D" ([`a90c552`](https://github.com/Byron/crates-io-cli-rs/commit/a90c552759ff28cfb27834665c2e33a7b389b447))
    - Attempt to use a different way of creating tasks… FAIL :D ([`e6d2113`](https://github.com/Byron/crates-io-cli-rs/commit/e6d2113382792b574fda44f06eca266811d383e6))
    - Now it works as expected! Countless off-by-one errors :D ([`da17405`](https://github.com/Byron/crates-io-cli-rs/commit/da1740523e1d89ad911396b8128668d9657ba388))
    - Closer to 'interesting', but tasks disappear too quickly ([`ea76a83`](https://github.com/Byron/crates-io-cli-rs/commit/ea76a83967f64bdba6fa65c6c70854d7918530eb))
    - First preliminary display of actual data ([`5694c26`](https://github.com/Byron/crates-io-cli-rs/commit/5694c261f7752e55022166314aa357a73159823a))
    - It shows we want the title elsewhere… ([`548bf17`](https://github.com/Byron/crates-io-cli-rs/commit/548bf17da64619d77afee06d12cb846043ce9a90))
    - Refactor ([`eb19a98`](https://github.com/Byron/crates-io-cli-rs/commit/eb19a985f73d87bbf3abd2b16dfc4d8635d20d7c))
    - First visible lines to show something is going on ([`6f3b61c`](https://github.com/Byron/crates-io-cli-rs/commit/6f3b61cf70e021c005e581782d028c86dbbe8cb0))
    - Use TUI-react for a more flexible rendering ([`9fa25d9`](https://github.com/Byron/crates-io-cli-rs/commit/9fa25d9f0797586b3c263890a897e2810632cb9e))
    - First step towards seeing something - need TUI-react for statefulness ([`9bc3530`](https://github.com/Byron/crates-io-cli-rs/commit/9bc35305674d5dfba173660d0657cae82bf3416f))
    - Differentiate between root and progress trees ([`9adba83`](https://github.com/Byron/crates-io-cli-rs/commit/9adba8347cab59729c0c9ea79db49fd7f5882af3))
    - Support for 4 levels of hierarchy ([`9f15bea`](https://github.com/Byron/crates-io-cli-rs/commit/9f15bea01f4c158475bf169bd7acb9f87d83df58))
    - And now it's perfect! ([`1e5c543`](https://github.com/Byron/crates-io-cli-rs/commit/1e5c5434fab847b12a6f60b09886f874f5f52e24))
    - This looks like an acceptable solution ([`dd58cea`](https://github.com/Byron/crates-io-cli-rs/commit/dd58ceaeb4ce8084a1e54cb24616a6db1162a5b4))
    - Remove the 'gui aborted' channel ([`ce4a098`](https://github.com/Byron/crates-io-cli-rs/commit/ce4a09823b15025af11ce59f670aa3dc3b9f4e07))
    - Renaming makes the current solution OK, even though… ([`0973707`](https://github.com/Byron/crates-io-cli-rs/commit/09737072c0242edf6c0708868412744cab5bb281))
    - Let's use the GUI handle again to see when it's shut down ([`301e6fc`](https://github.com/Byron/crates-io-cli-rs/commit/301e6fc8d651056435424675f5a39f3b4845d250))
    - This actually allows to get rid of 'gui done', but… ([`4c70909`](https://github.com/Byron/crates-io-cli-rs/commit/4c709099c51b6bbd247665d6046928fca9218e4c))
    - Now abort-handle works too ([`f36f5c3`](https://github.com/Byron/crates-io-cli-rs/commit/f36f5c3c596c9d853d8415d5205d08c28a5bfd9d))
    - Don't tell GUI on how to stop anymore ([`79ee668`](https://github.com/Byron/crates-io-cli-rs/commit/79ee668cfbc6a5b29414d68d481f469a6eacaea7))
    - With handle, dropping seems to work ([`626ab27`](https://github.com/Byron/crates-io-cli-rs/commit/626ab2786051686aab72cb4de7526a554cec090d))
    - This seems to be needed, too - have to wait for it ([`29eacd3`](https://github.com/Byron/crates-io-cli-rs/commit/29eacd326667a75fbcc0a473e2d4028151e43bb5))
    - Finally, signalling works both ways ([`fecd46f`](https://github.com/Byron/crates-io-cli-rs/commit/fecd46f085e15123121e58604041afdfac3a6580))
    - Futures aren't dropped when you think they are ([`5e82181`](https://github.com/Byron/crates-io-cli-rs/commit/5e821817191d428d193063907bd96c4334558d79))
    - Now with explicit abort signal - lots of code, and doesn't work consistently ([`2a45875`](https://github.com/Byron/crates-io-cli-rs/commit/2a45875d1b5e0a701d4c0e8c40f4de898f5c5204))
    - Refactor ([`9eabad1`](https://github.com/Byron/crates-io-cli-rs/commit/9eabad14e0e9696a9c401b1f2b84dab32309434d))
    - Make GUI abortable with the the normal means ([`77db818`](https://github.com/Byron/crates-io-cli-rs/commit/77db81886fb50ba1099186f60a2b2691b4157c65))
    - Can mix threads and async :) ([`34e11e4`](https://github.com/Byron/crates-io-cli-rs/commit/34e11e4a0948ce776728448984c8bd525b6024a0))
    - GUI can now be aborted, and signals when it registed an abort itself ([`5c1f902`](https://github.com/Byron/crates-io-cli-rs/commit/5c1f9022cf887f9ee27cc82be4937ad8780892dc))
    - First attempt to select on GUI shutdown - fail ([`4da5332`](https://github.com/Byron/crates-io-cli-rs/commit/4da5332ed91acda2f5b8cf610dfc7d170196f69d))
    - Fix bug ([`b70988b`](https://github.com/Byron/crates-io-cli-rs/commit/b70988ba0609966e9d7482b0719c307a94a332f3))
    - Avoid spurious value_mut() failures ([`445c814`](https://github.com/Byron/crates-io-cli-rs/commit/445c814198a6e4b5b1caca3bf0a772bf59e92247))
    - Refactor ([`d9dbe55`](https://github.com/Byron/crates-io-cli-rs/commit/d9dbe55629a9e608bd47239390bc0823552ed168))
    - Now with longer runtimes, and with awful breakage ([`5ed8a7a`](https://github.com/Byron/crates-io-cli-rs/commit/5ed8a7aafe4b7c814c1914d72882ce66a8d8d1f5))
    - Minor cleanup ([`2714651`](https://github.com/Byron/crates-io-cli-rs/commit/2714651e31a9284d6a609a2ae1cd998a1124cf3b))
    - Signals aren't working from the shell in RAW mode :D ([`3e0a18b`](https://github.com/Byron/crates-io-cli-rs/commit/3e0a18bd7bf1c4f4edc51c262a753307e0ce21b5))
    - Refactor ([`4ea705b`](https://github.com/Byron/crates-io-cli-rs/commit/4ea705b660487abda80e08e777ef74b2a21eabe1))
    - Auto-shutdown of GUI after some time to prove… ([`4bc81b2`](https://github.com/Byron/crates-io-cli-rs/commit/4bc81b25b35d441ba9d1ea973043fa9fbe8113c6))
    - Revert "refactor; attempt to use tokio-signals. Needs reactor!" ([`78d7e85`](https://github.com/Byron/crates-io-cli-rs/commit/78d7e85715e4c916f22945daa5750dfaaa179a8c))
    - Refactor; attempt to use tokio-signals. Needs reactor! ([`9a00c64`](https://github.com/Byron/crates-io-cli-rs/commit/9a00c64da987f8757521241820da8d0d264973f9))
    - First frame for TUI gui - needs proper shutdown to reset screen ([`9fccab0`](https://github.com/Byron/crates-io-cli-rs/commit/9fccab07a1894a94242788a810353b1c3c050e0b))
    - It actually DOES seem to do something! Nice ([`700d43c`](https://github.com/Byron/crates-io-cli-rs/commit/700d43c5972b217bc7cd31123f18c53df0a357f3))
    - It does something, let's see what… ([`21e07ed`](https://github.com/Byron/crates-io-cli-rs/commit/21e07ed83d9511c5b31f23de04b7dcf8f3185429))
    - Recursive futures - fair enough! ([`2f31a29`](https://github.com/Byron/crates-io-cli-rs/commit/2f31a2966491febb8b3c20d72e1ecbf91df2f92e))
    - Let's not bother with bigger child indices, or reduction of parent indices ([`9ee8341`](https://github.com/Byron/crates-io-cli-rs/commit/9ee8341f6534204cda8a4083db20b8c0bcf1e857))
    - First sketch of dashboard - a lot more work to be done ([`07ef7f5`](https://github.com/Byron/crates-io-cli-rs/commit/07ef7f5ede3ea3c25fa9e904ef4e4e48c52bc616))
    - Add progress-dashboard ([`13adffd`](https://github.com/Byron/crates-io-cli-rs/commit/13adffd5c3f99e2e8e11206bc1a3f48185a916bc))
    - Let's try size + performance optimized builds ([`e038703`](https://github.com/Byron/crates-io-cli-rs/commit/e0387039518b8f7220c9e1f51711af7d5431f9ac))
    - Refactor ([`24a3ed8`](https://github.com/Byron/crates-io-cli-rs/commit/24a3ed8032f7286c5396cb91b07178fe7cd14a8c))
    - Remove async-std in favor of futures ([`b24ddf4`](https://github.com/Byron/crates-io-cli-rs/commit/b24ddf4b1232fd054fcf2427e9d60ecb7f78c845))
    - Refactor ([`30a5207`](https://github.com/Byron/crates-io-cli-rs/commit/30a5207f6a3dd468849cb9d1095ded999b869bc4))
    - Replace async-std timer with futures delay + select ([`e33dd28`](https://github.com/Byron/crates-io-cli-rs/commit/e33dd2834983205093e7c6b55f7188337d71cd96))
    - Refine task list ([`01d0a96`](https://github.com/Byron/crates-io-cli-rs/commit/01d0a96f4a38a63c5dd119ff883784cb9e35a870))
    - Refactor ([`aaaf321`](https://github.com/Byron/crates-io-cli-rs/commit/aaaf3217efcccbe25e267eb3587dda80dadb741d))
    - Tick a box :D ([`8180e1f`](https://github.com/Byron/crates-io-cli-rs/commit/8180e1f68b62bcabbb704724b2fabb0e3402839c))
    - Refactor ([`b117eb4`](https://github.com/Byron/crates-io-cli-rs/commit/b117eb4c80b0d78104853c656d5cb60a43f86681))
    - First example on how accessing data would look like right now ([`08f2354`](https://github.com/Byron/crates-io-cli-rs/commit/08f2354ecdb0f67fb490b31ca6b2b1229f06b702))
    - Remove plenty of complexity ([`2b8ad49`](https://github.com/Byron/crates-io-cli-rs/commit/2b8ad49484d449516fda65e6edf2f3fbc0473890))
    - Minor refactor ([`ed8e703`](https://github.com/Byron/crates-io-cli-rs/commit/ed8e70358d753943c76c6666eb51a41a8abb8059))
    - TreeAccess starts to become useful ([`6029218`](https://github.com/Byron/crates-io-cli-rs/commit/6029218d87aa3d8559a7ca77c1552808117d6a36))
    - Add useful traits to the model, pre-emptively ([`46038b6`](https://github.com/Byron/crates-io-cli-rs/commit/46038b659a0d19045ba89b17228f878515275bde))
    - 'update(fn)' support for Trees ([`6274813`](https://github.com/Byron/crates-io-cli-rs/commit/62748134aee03f5ebc606cf1693c925c51004694))
    - Make version insertion idempotent ([`b72a117`](https://github.com/Byron/crates-io-cli-rs/commit/b72a11734c708247c0fe7e27298213079e3c7d45))
    - Make CrateVersionTree inherit TreeAccess ([`16aacb3`](https://github.com/Byron/crates-io-cli-rs/commit/16aacb37be67ec22d47690f08bd2c0a387e12a0c))
    - Clean usage of context using new abstraction ([`15e836d`](https://github.com/Byron/crates-io-cli-rs/commit/15e836dfbf5821fd8deeb11c86483ebb704731d5))
    - Drop compression for maximum speed ([`89914e4`](https://github.com/Byron/crates-io-cli-rs/commit/89914e48db675951db2168ba855c9b72b7e4ba6a))
    - Back to normal compression for half size ([`de445f1`](https://github.com/Byron/crates-io-cli-rs/commit/de445f1701e9480f952618ee17758a9d573e23c1))
    - With highest compression factor… ([`5ebc54a`](https://github.com/Byron/crates-io-cli-rs/commit/5ebc54a7578059d62531fba8c8f3f8499c56b7d1))
    - Try out compression stanard level… ([`941cca6`](https://github.com/Byron/crates-io-cli-rs/commit/941cca63c7b587574412a1d8561b840d0aba5507))
    - Remove appveyor ([`14a1398`](https://github.com/Byron/crates-io-cli-rs/commit/14a13988837aa5987ac90e1b19e34869224622f5))
    - Move context into its own tree ([`44c0285`](https://github.com/Byron/crates-io-cli-rs/commit/44c02854a0b2d5595eee16e3667ec7933accb12a))
    - Now the trait is what it should be ([`51c207d`](https://github.com/Byron/crates-io-cli-rs/commit/51c207d95839784948afe0192de03da15e458e42))
    - Now the trait actually works, but could be even better ([`25c90e8`](https://github.com/Byron/crates-io-cli-rs/commit/25c90e84d540362eb2a8de8ee1384258874b6603))
    - Better names ([`dd2f96e`](https://github.com/Byron/crates-io-cli-rs/commit/dd2f96e4fa4f32921c2c679a264a0d9b9da9144b))
    - First version of Trait implemented for the Crates tree ([`7e07bb3`](https://github.com/Byron/crates-io-cli-rs/commit/7e07bb3d65e8ced0fb32d24eaf03c0ef09664dc7))
    - Trait also is hard to handle, it can't ever know the actual type ([`90299f8`](https://github.com/Byron/crates-io-cli-rs/commit/90299f887df07446da8535f19cea9563d31b9274))
    - First trait-based version with default implementation - better ([`d75bb05`](https://github.com/Byron/crates-io-cli-rs/commit/d75bb05c58e7d5a08ce4e36bbff7fa79d210931c))
    - Sketch of purely field based TreeAccess ([`6499115`](https://github.com/Byron/crates-io-cli-rs/commit/64991156087b97e04edbf35aa398bd0c6461f6d9))
    - Update dependencies ([`76d5009`](https://github.com/Byron/crates-io-cli-rs/commit/76d5009609a71d5d724068e723d2b9713437e518))
    - Remove code duplication; populate model module ([`b7aa3b3`](https://github.com/Byron/crates-io-cli-rs/commit/b7aa3b305e60002f8a81251e1f4ae297e24c07c7))
    - Add criner tasks ([`0d7a33d`](https://github.com/Byron/crates-io-cli-rs/commit/0d7a33dc7d2f48dfcf464a415a1d2f014643ac38))
    - Now with support for deltas per run, all stored by date ([`e71298d`](https://github.com/Byron/crates-io-cli-rs/commit/e71298d8b18d310457c418aa888ab951d3fbc881))
    - Support for wallclock time counting ([`cad3c4b`](https://github.com/Byron/crates-io-cli-rs/commit/cad3c4b1ce3c0f63927a07b29070c929a7940867))
    - Add support for global, shared state for statistics ([`3a311dc`](https://github.com/Byron/crates-io-cli-rs/commit/3a311dc01d3db2a6dede2e4e3bafa5ee37a4d49b))
    - Refactor ([`ed70ea0`](https://github.com/Byron/crates-io-cli-rs/commit/ed70ea0b7d38e0a81f744063d1c2da910db884e2))
    - Failed experiment with chunking ([`f5f3ad8`](https://github.com/Byron/crates-io-cli-rs/commit/f5f3ad8066369c48139a3bb37e5bab4199bc1e52))
    - Also keep track of all versions by crate ([`70f1810`](https://github.com/Byron/crates-io-cli-rs/commit/70f1810d9debb67baae8a43afe49cb1d392872d8))
    - For now, force everyone to go through the persistence abstraction ([`92022a7`](https://github.com/Byron/crates-io-cli-rs/commit/92022a72cc46332a8ef4ec91c67dd9bdac2acf55))
    - Mildly absract persistence for type and model safety ([`19d2b82`](https://github.com/Byron/crates-io-cli-rs/commit/19d2b82d53d37138439cff5e84c53a3c58a8f207))
    - Reorganize code - prepare for growth ([`0cd4ab0`](https://github.com/Byron/crates-io-cli-rs/commit/0cd4ab080da9c0fe6866e81c1c1f0cb6cb672e85))
    - Make sure we only act on reasonable deadlines ([`95fb4cb`](https://github.com/Byron/crates-io-cli-rs/commit/95fb4cbe720db5a4ce1400066cb1809aa0355bea))
    - Upgrade to faster crates-index-diff version ([`e9a8ade`](https://github.com/Byron/crates-io-cli-rs/commit/e9a8adebd6647d83af04ae9d6683459f2a1be586))
    - Per-crate task spawning is slower than manual iteration ([`64b8e12`](https://github.com/Byron/crates-io-cli-rs/commit/64b8e12651a20b9fabdc0c020ad30557502ab060))
    - Now with insertion parallelism ([`8773275`](https://github.com/Byron/crates-io-cli-rs/commit/8773275895af8baf4edfb87f9af16373cd99ba1d))
    - Much more precise timeouts thanks to using a stream ([`6a1fd6d`](https://github.com/Byron/crates-io-cli-rs/commit/6a1fd6d9080257cd47ea298c35efae2b29766991))
    - A neat way to enforce timeouts on tasks that would otherwise block ([`c97bdec`](https://github.com/Byron/crates-io-cli-rs/commit/c97bdecbfb8098c3031dcf2a018433dc692336e6))
    - Use the blocking task pool to not block everything else ([`50ca348`](https://github.com/Byron/crates-io-cli-rs/commit/50ca3484e07ec650accf419fb22b87648e941718))
    - Get married to async-std ([`06ff8ba`](https://github.com/Byron/crates-io-cli-rs/commit/06ff8ba040a8f53e77422eb0ea26847db5c7b111))
    - Turn off fast debug builds - slow is good actually ([`0dcbbc1`](https://github.com/Byron/crates-io-cli-rs/commit/0dcbbc1320a9c8fb85cd3687ba18e1f90b9a291e))
    - Fix tests for now ([`65457a3`](https://github.com/Byron/crates-io-cli-rs/commit/65457a3105604d032aad48222d545b0afd704040))
    - More performance thanks to optimized dependencies ([`d08c9fd`](https://github.com/Byron/crates-io-cli-rs/commit/d08c9fdc9c584719059bb1a149ee25e7295b32da))
    - Fix deadline calculation ([`603825a`](https://github.com/Byron/crates-io-cli-rs/commit/603825a87ecb2f35c7ffebb6a75ee1385458c4b0))
    - The first purely blocking implementation of storing meta data in sled ([`155cc5d`](https://github.com/Byron/crates-io-cli-rs/commit/155cc5d0cb0b8224da1e9f4046e9f5498861cb30))
    - Add sled ([`adb91f6`](https://github.com/Byron/crates-io-cli-rs/commit/adb91f6b13eb81b789e8cc28696b690b027c546f))
    - Remove git2 dependency ([`ed6a049`](https://github.com/Byron/crates-io-cli-rs/commit/ed6a04919f1f2fd3b7f22f5e8ceccabd3fc4692b))
    - The most important arguments for 'criner' are now provided via args ([`e90cb9d`](https://github.com/Byron/crates-io-cli-rs/commit/e90cb9dd44e98991f7d45e77667927b301620ca8))
    - First sketch of 'criner', with minimal dependencies (for dev) ([`6396d19`](https://github.com/Byron/crates-io-cli-rs/commit/6396d1953fc095988206e7e308779d60c61fd9ee))
    - Prepare for next release ([`e417a2c`](https://github.com/Byron/crates-io-cli-rs/commit/e417a2c779fbc5009582d4c2bda057a4ff924f0d))
    - (cargo-release) start next development iteration 3.0.2-alpha.0 ([`0b402d4`](https://github.com/Byron/crates-io-cli-rs/commit/0b402d4e35a79377c9dfe9c6a6d779a9f385e21f))
</details>

### v3.0.1 (2020-02-02)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Actually, 3.0.1 should have been the one… ([`d009c75`](https://github.com/Byron/crates-io-cli-rs/commit/d009c757673b843870cb17bf592fdb4d86f89cb1))
    - (cargo-release) start next development iteration 2.2.3-alpha.0 ([`11302b8`](https://github.com/Byron/crates-io-cli-rs/commit/11302b801eac794f8ad1e862cf9baaa6baa68227))
</details>

### v2.2.2 (2020-02-02)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 1 calendar day.
 - 211 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Prepare new release 2.2.2 ([`04d2f38`](https://github.com/Byron/crates-io-cli-rs/commit/04d2f387da24ff9cb81a1c648eaa24e733d3a302))
    - Add disclaimer - let's not fix the old stuff ([`7fab5b9`](https://github.com/Byron/crates-io-cli-rs/commit/7fab5b937a20cc66b1547ce20bd5622a3f6809d9))
    - Fix tests ([`0cb60f3`](https://github.com/Byron/crates-io-cli-rs/commit/0cb60f32344a5bfda6d6da08c3d3288d88d0d050))
    - Re-enable all sub-commands, each one gatable ([`9f325bb`](https://github.com/Byron/crates-io-cli-rs/commit/9f325bbb4dea7fada97e75f84c1657a1b753dc63))
    - Re-add 'list' feature ([`112f8ca`](https://github.com/Byron/crates-io-cli-rs/commit/112f8caf6a8d35f619d8f831b6615d7445b4c466))
    - Protect from accidental publishes ([`05ec345`](https://github.com/Byron/crates-io-cli-rs/commit/05ec345fe1e7d67694eee957addb0ed8cc82dd2b))
    - Control everything with feature toggles. ([`bae17eb`](https://github.com/Byron/crates-io-cli-rs/commit/bae17eb0ec391d35867f80e3b2c9bc37dd7b84ba))
    - Remove cpu pool ([`155c0af`](https://github.com/Byron/crates-io-cli-rs/commit/155c0af0575ed7fd203a5341c7ad61e6aef73615))
    - Alternative impl for recent changes, without cpupool ([`e304e8c`](https://github.com/Byron/crates-io-cli-rs/commit/e304e8ca30bf864a3c17e0209d4453a53e4a4fac))
    - Let 'recent-changes' use struct-opt arguments ([`0fc6c90`](https://github.com/Byron/crates-io-cli-rs/commit/0fc6c9042a77d68397e021859e0162f16169ba6d))
    - Add structopt parsing equivalent to manual declaration ([`1624e4b`](https://github.com/Byron/crates-io-cli-rs/commit/1624e4b9dbb4b0448a2ba73da38d390bd3c776e9))
    - Remove _new suffix ([`2cd3dd6`](https://github.com/Byron/crates-io-cli-rs/commit/2cd3dd607123654e28d0602a116da4bc67db2f6a))
    - Remove rustc serialize ([`6e3274f`](https://github.com/Byron/crates-io-cli-rs/commit/6e3274fe1be9364980d1d3f0a6c812fc79dd25d3))
    - Less code for list by user command; remove rustc-serialize ([`96c23e2`](https://github.com/Byron/crates-io-cli-rs/commit/96c23e23c983757886ff9f334224320a8668d83a))
    - Fix warning; bump patch level ([`36cb987`](https://github.com/Byron/crates-io-cli-rs/commit/36cb987fe8cd401b93e6d2eac8d41c87db69f383))
    - Cargo.lock format update; update all deps without breakage ([`dcafeb9`](https://github.com/Byron/crates-io-cli-rs/commit/dcafeb9d520b85b83a1eb2098a90c22ca8f3cc11))
    - Simplify .gitignore ([`44f9799`](https://github.com/Byron/crates-io-cli-rs/commit/44f9799ff2aba8afa0876067021a9c0e96f88860))
    - Remove asciinema recordings! They end up in the crate, too ([`44e3d83`](https://github.com/Byron/crates-io-cli-rs/commit/44e3d83a4b6ed0f02bd246fb87ead06f2710303d))
    - Smaller packages ([`37e29e9`](https://github.com/Byron/crates-io-cli-rs/commit/37e29e9f7c03abe7fd6192b2dc3fd034e5066ff9))
    - Nicer journey tests; no clog ([`02ebda3`](https://github.com/Byron/crates-io-cli-rs/commit/02ebda364dcf47391f592c5fa9845bab30fe2821))
    - It compiles once again ([`6796c98`](https://github.com/Byron/crates-io-cli-rs/commit/6796c9825901721f11a2124f7424760f9639f125))
    - Edition 2018; fix obvious warnings and errors ([`7fffdb1`](https://github.com/Byron/crates-io-cli-rs/commit/7fffdb15ba1ef7901ef28086c39fbae02e4d2377))
    - Update to latest git-index-diff ([`43f8d58`](https://github.com/Byron/crates-io-cli-rs/commit/43f8d58c6475a465ce2ebcc1a6d30507577f0e2a))
    - Remove extra (and redundant) binary : 'krates' ([`ad5cd56`](https://github.com/Byron/crates-io-cli-rs/commit/ad5cd56eeeb73abf86438b6a701d0888bc8fd82e))
    - Cargo-fmt ([`46aa71f`](https://github.com/Byron/crates-io-cli-rs/commit/46aa71fd2d192e4cf7597a5b7326326eb03cc02b))
    - Add automated help ([`7529a4c`](https://github.com/Byron/crates-io-cli-rs/commit/7529a4cd0d0ad27cad16f7c7f0b1ee10baa5e87f))
</details>

### v2.2.1 (2019-07-05)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 272 calendar days.
 - 272 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`8e057ff`](https://github.com/Byron/crates-io-cli-rs/commit/8e057ff5eb789293ad6e06325954f99bd0154aaf))
    - Update Cargo.lock; add colors ([`468b164`](https://github.com/Byron/crates-io-cli-rs/commit/468b1643ed12d7c74d22045f3e6c79d000c6d0f3))
    - Add proper useragent string to be allowed to make queries again ([`7900172`](https://github.com/Byron/crates-io-cli-rs/commit/7900172d8c19057a9aa25bae17320af864847d20))
    - Update all dependencies to latest minor/patch ([`422c7b5`](https://github.com/Byron/crates-io-cli-rs/commit/422c7b503269a5fd47c57c3dd4b6729e0a2e8160))
    - Potentially fix tests ([`85cb942`](https://github.com/Byron/crates-io-cli-rs/commit/85cb942ad1b829a79f59a46984aaf5fbea7981b5))
</details>

### v2.2.0 (2018-10-05)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 295 calendar days.
 - 295 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version prior to publishing ([`8e47fc6`](https://github.com/Byron/crates-io-cli-rs/commit/8e47fc6f7fb1d48c15bbcc30ff65234041de55a0))
    - Use 'search' as default subcommand ([`3153f95`](https://github.com/Byron/crates-io-cli-rs/commit/3153f951fd27d3a4a318afe81c01260db97e2b02))
    - Update deps ([`665284c`](https://github.com/Byron/crates-io-cli-rs/commit/665284cc1300ae1c384337c57eb6a868b9204374))
    - Run rustfmt on everything ([`99fbd34`](https://github.com/Byron/crates-io-cli-rs/commit/99fbd34222ac02b3722df90d7b005f06b255f26e))
    - Update changelog ([`a557eb1`](https://github.com/Byron/crates-io-cli-rs/commit/a557eb161265d909dc781eded1ef9bf7e905789b))
</details>

### v2.1.1 (2017-12-13)

Fix all deprecation warnings.

<a name="v2.1.0"></a>

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump patch level ([`70a87c4`](https://github.com/Byron/crates-io-cli-rs/commit/70a87c47a691d0263b6cf2875353d0c01ba1077a))
    - Fix all deprecation warnings ([`5dcb448`](https://github.com/Byron/crates-io-cli-rs/commit/5dcb448b583d91bf7eaea93ee235d116c41d59e1))
    - Update using 'cargo update' ([`575c730`](https://github.com/Byron/crates-io-cli-rs/commit/575c7303c5e3b87241df74b4962f459968d132ca))
    - Remove usage of deprecated 'boxed()' ([`8fbdc7a`](https://github.com/Byron/crates-io-cli-rs/commit/8fbdc7af5da0b9cfae77f8bef72ef29f95da7aab))
</details>

### v2.1.0 (2017-12-13)

<csr-id-8c9a671647366a4249b4989e2cc956dd93cdb2e8/>

Add aggregated amount of downloads to 'list by-human <id>'.

<a name="v2.0.1"></a>

#### Bug Fixes

 - <csr-id-513a025fa90c7d68e2a7ef78fa47105cf4e66a56/> now for real
   It seems the macro engine received a bugfix, which caused
   trailing commas to become invalid
 - <csr-id-bc9349d4f0b9cc25f53a43236f59cf0be8a119b8/> downgrade pretty-tables
   row! macro was broken.

#### Chore

 - <csr-id-8c9a671647366a4249b4989e2cc956dd93cdb2e8/> run everything through latest rustfmt

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 173 calendar days.
 - 308 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 2.1 ([`01d8a1c`](https://github.com/Byron/crates-io-cli-rs/commit/01d8a1ce57b96530015705e852dc85ea2b666984))
    - Compute total downloads in 'list by-user -o human' ([`51285e4`](https://github.com/Byron/crates-io-cli-rs/commit/51285e48915bdef41095e39e95606a49bbd7207f))
    - Run latest rustfmt ([`c1415f6`](https://github.com/Byron/crates-io-cli-rs/commit/c1415f6854c6d4ec91666e72f8f1bc1d0ed109e3))
    - Update dependencies ([`7f1e7a7`](https://github.com/Byron/crates-io-cli-rs/commit/7f1e7a7f183a30742add83e814b273e133f60546))
    - Now for real ([`513a025`](https://github.com/Byron/crates-io-cli-rs/commit/513a025fa90c7d68e2a7ef78fa47105cf4e66a56))
    - Downgrade pretty-tables ([`bc9349d`](https://github.com/Byron/crates-io-cli-rs/commit/bc9349d4f0b9cc25f53a43236f59cf0be8a119b8))
    - Run everything through latest rustfmt ([`8c9a671`](https://github.com/Byron/crates-io-cli-rs/commit/8c9a671647366a4249b4989e2cc956dd93cdb2e8))
</details>

### v2.0.1 (2017-02-07)

<csr-id-5c9e2c1b9bf636b593ed37688d7aa6d68f96df54/>

<a name="v2.0.0"></a>

#### Chore

 - <csr-id-5c9e2c1b9bf636b593ed37688d7aa6d68f96df54/> v2.0.1

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 12 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V2.0.1 ([`5c9e2c1`](https://github.com/Byron/crates-io-cli-rs/commit/5c9e2c1b9bf636b593ed37688d7aa6d68f96df54))
    - (imp): "Implemented CTRL-C to exit crates search command" ([`ff374c5`](https://github.com/Byron/crates-io-cli-rs/commit/ff374c57064414aaca7d1335d36ab1cb015a8606))
</details>

### v2.0.0 (2017-01-26)

<csr-id-5408ae092cad3916d5d8d477c00d06f699b3310f/>
<csr-id-271fcf561ac98e8438643bea84a59ace83e2dc59/>
<csr-id-0e8b0e930fa4cecf5f7fe4236973431fb6cf16ac/>
<csr-id-9d4827964fafac6a8ac3b57445ea268c9ba12a93/>
<csr-id-c802a198c6bf22300ef2533efba7db2d8c25aac0/>
<csr-id-a278a55b556b2f01ab34ded394f0bc3ac27f83f8/>
<csr-id-32b017824f1a928fd559c933940cf6667e2a7e8c/>
<csr-id-d358c18b8b5cca6ac5ccd9baf83ea311b2829a38/>
<csr-id-84404912744c8fdecad5c1b62af5dfdfe1a73c8e/>
<csr-id-8fdb54ca82400e6e8b07ee636757c589ebf0af1e/>
<csr-id-36cc73917430cc971f18ccd8f54419e07fab8963/>
<csr-id-2b56cd1f5f87f0c15387f54235408d8a5439bb0f/>

This release comes with vastly improved responsiveness for `crates search`, as well as the first
implementation of the `list` subcommand.

#### Other

 - <csr-id-5408ae092cad3916d5d8d477c00d06f699b3310f/> use app_from_crate! macro
 - <csr-id-271fcf561ac98e8438643bea84a59ace83e2dc59/> --repository is local to recent-changes
   As it is the only command which uses it.
 - <csr-id-0e8b0e930fa4cecf5f7fe4236973431fb6cf16ac/> error handling in paging function
   It seems to be quite production ready now, let's see
   if list by user is happening soon!
 - <csr-id-9d4827964fafac6a8ac3b57445ea268c9ba12a93/> support for paging
   relevant only for terminals with a vertical height greater
   than the max page size, currently 100.
 - <csr-id-c802a198c6bf22300ef2533efba7db2d8c25aac0/> encode search term
   Unfortunatly it relies on a library that is unlikely to
   improve, but at least it looked sufficiently well implemented.
 - <csr-id-a278a55b556b2f01ab34ded394f0bc3ac27f83f8/> ignore tab character
   Usually nothing good happens, so for now it is just ignored
 - <csr-id-32b017824f1a928fd559c933940cf6667e2a7e8c/> use max_items in paged call
 - <csr-id-d358c18b8b5cca6ac5ccd9baf83ea311b2829a38/> paged_remote_call finally compiles
   It ain't pretty, as I had to brutally Arc<Mutex<_>> my way around
   the requirements imposed by Box.
   It would be so nice to have impl Trait!!
 - <csr-id-84404912744c8fdecad5c1b62af5dfdfe1a73c8e/> trying to get a boxed stream from a bunch of futures
 - <csr-id-8fdb54ca82400e6e8b07ee636757c589ebf0af1e/> use utility function for remote call
 - <csr-id-36cc73917430cc971f18ccd8f54419e07fab8963/> add remote_call function
   One step on the way to a paging call

#### New Features

 - <csr-id-4b783bdbe145ae0974c081d3393bcf028e283e8b/> --format for all list outputs
 - <csr-id-a9f9fcdba9311e662cf3c289e261910ed249c180/> first implementation of by-user
   It works, but apparently all crates are returned if the user-id
   is invalid or otherwise not found.

#### Chore

 - <csr-id-2b56cd1f5f87f0c15387f54235408d8a5439bb0f/> changelog in preparation for release

#### Improvements

* **recent-changes:**  --repository is local to recent-changes ([271fcf56](https://github.com/Byron/crates-io-cli-rs/commit/271fcf561ac98e8438643bea84a59ace83e2dc59))
* **search:**
  *  support for paging ([9d482796](https://github.com/Byron/crates-io-cli-rs/commit/9d4827964fafac6a8ac3b57445ea268c9ba12a93))
  *  ignore tab character ([a278a55b](https://github.com/Byron/crates-io-cli-rs/commit/a278a55b556b2f01ab34ded394f0bc3ac27f83f8), closes [#2](https://github.com/Byron/crates-io-cli-rs/issues/2))

#### Features

* **list:**
  * a new subcommand for listing crates by a certain critereons.
  *  --format for all list outputs ([4b783bdb](https://github.com/Byron/crates-io-cli-rs/commit/4b783bdbe145ae0974c081d3393bcf028e283e8b))
  *  first implementation of by-user ([a9f9fcdb](https://github.com/Byron/crates-io-cli-rs/commit/a9f9fcdba9311e662cf3c289e261910ed249c180))



<a name="v1.3.2"></a>

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 13 calendar days.
 - 13 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Use app_from_crate! macro ([`5408ae0`](https://github.com/Byron/crates-io-cli-rs/commit/5408ae092cad3916d5d8d477c00d06f699b3310f))
    - Changelog in preparation for release ([`2b56cd1`](https://github.com/Byron/crates-io-cli-rs/commit/2b56cd1f5f87f0c15387f54235408d8a5439bb0f))
    - --format for all list outputs ([`4b783bd`](https://github.com/Byron/crates-io-cli-rs/commit/4b783bdbe145ae0974c081d3393bcf028e283e8b))
    - --repository is local to recent-changes ([`271fcf5`](https://github.com/Byron/crates-io-cli-rs/commit/271fcf561ac98e8438643bea84a59ace83e2dc59))
    - Remove lib ([`49a13b9`](https://github.com/Byron/crates-io-cli-rs/commit/49a13b986d6b941bf7c443df5edc4164769d10a2))
    - First implementation of by-user ([`a9f9fcd`](https://github.com/Byron/crates-io-cli-rs/commit/a9f9fcdba9311e662cf3c289e261910ed249c180))
    - Implemented by-user, need deserialization code ([`f80d0c0`](https://github.com/Byron/crates-io-cli-rs/commit/f80d0c0a21ee8bb53b5838963a8b30d735dc961f))
    - First sketch of list by-user ([`f8533a7`](https://github.com/Byron/crates-io-cli-rs/commit/f8533a744d780ded1418adc5fbcba83f52d032e7))
    - Error handling in paging function ([`0e8b0e9`](https://github.com/Byron/crates-io-cli-rs/commit/0e8b0e930fa4cecf5f7fe4236973431fb6cf16ac))
    - Add paging function with error handling support ([`0b608f3`](https://github.com/Byron/crates-io-cli-rs/commit/0b608f318d3f17d18a7979d348ba8470f1bddd03))
    - Support for paging ([`9d48279`](https://github.com/Byron/crates-io-cli-rs/commit/9d4827964fafac6a8ac3b57445ea268c9ba12a93))
    - Encode search term ([`c802a19`](https://github.com/Byron/crates-io-cli-rs/commit/c802a198c6bf22300ef2533efba7db2d8c25aac0))
    - Ignore tab character ([`a278a55`](https://github.com/Byron/crates-io-cli-rs/commit/a278a55b556b2f01ab34ded394f0bc3ac27f83f8))
    - Use max_items in paged call ([`32b0178`](https://github.com/Byron/crates-io-cli-rs/commit/32b017824f1a928fd559c933940cf6667e2a7e8c))
    - Paged_remote_call finally compiles ([`d358c18`](https://github.com/Byron/crates-io-cli-rs/commit/d358c18b8b5cca6ac5ccd9baf83ea311b2829a38))
    - Trying to get a boxed stream from a bunch of futures ([`8440491`](https://github.com/Byron/crates-io-cli-rs/commit/84404912744c8fdecad5c1b62af5dfdfe1a73c8e))
    - Use utility function for remote call ([`8fdb54c`](https://github.com/Byron/crates-io-cli-rs/commit/8fdb54ca82400e6e8b07ee636757c589ebf0af1e))
    - Add remote_call function ([`36cc739`](https://github.com/Byron/crates-io-cli-rs/commit/36cc73917430cc971f18ccd8f54419e07fab8963))
    - Revert "chore(cli): remove list for now" ([`d5ffdd3`](https://github.com/Byron/crates-io-cli-rs/commit/d5ffdd32a2b5a260244cc40c27b2b5ccd47cf2fd))
</details>

### v1.3.2 (2017-01-13)

<csr-id-85d49a745e45894efb18ac44c93fd293f05d1cca/>
<csr-id-d72cc2759a6eee744fb38f5d3574c61fb16b38c3/>
<csr-id-d349d978ee617b929f60a99d8dc50a4863c06f7d/>
<csr-id-c4dbef29d4a1f1a341f869e4d3424f5341b23ff3/>
<csr-id-9e4aff7293cac609035627d82629e601f1eaf73e/>
<csr-id-4f8f3d6fcc82e9cc8339d2d17019ddbbe686ed82/>
<csr-id-8d11e2b51a33cbe2d6c8e479c9d68d69c7efe3d2/>
<csr-id-2f115a84dc481fccd9945aca532e419a245dcef3/>
<csr-id-1b00f362827b69319703fa64b79a34e2c7d6d5b9/>
<csr-id-8d013f82dbf85d5895eca72f94674b6a403cfb29/>
<csr-id-fee77ec2d3142f7be29a3c0d1b72209941d81d32/>
<csr-id-329c54b9fc88c8e3f995e09cab6dee78f1a82d61/>
<csr-id-b025eab4ebffa7dbba8fc0c59d93d7312eb0a0ff/>
<csr-id-7d1f8933b51718c17b382fd9ae5ce5b84846694b/>
<csr-id-6ba9d0527d11efa935f3c01f604b06f0fd728ff3/>
<csr-id-1675c1e0ea304d25eb66b9b2550f6f8dc371eb85/>

#### Bug Fixes

* **search:**  prevent failure due to paging size ([961360b0](https://github.com/Byron/crates-io-cli-rs/commit/961360b007122d0be8e942174d866a3fe85a7f5d))
 - <csr-id-961360b007122d0be8e942174d866a3fe85a7f5d/> prevent failure due to paging size
   Crates allows a maximum of 100 results to be shown.
   Terminals could be higher, especially on vertical screens.

#### Improvements

* **bin:**  `krates` is an alternative program name ([329c54b9](https://github.com/Byron/crates-io-cli-rs/commit/329c54b9fc88c8e3f995e09cab6dee78f1a82d61))
* **cargo:**  better keywords ([fee77ec2](https://github.com/Byron/crates-io-cli-rs/commit/fee77ec2d3142f7be29a3c0d1b72209941d81d32))
* **cli:**  allow printing causes ([7d1f8933](https://github.com/Byron/crates-io-cli-rs/commit/7d1f8933b51718c17b382fd9ae5ce5b84846694b))
* **search:**
  *  make clear which search result you see ([1b00f362](https://github.com/Byron/crates-io-cli-rs/commit/1b00f362827b69319703fa64b79a34e2c7d6d5b9))
  *  parallel search processing thanks to spawn! ([8d013f82](https://github.com/Byron/crates-io-cli-rs/commit/8d013f82dbf85d5895eca72f94674b6a403cfb29))



<a name="v1.3.1"></a>

#### Refactor

 - <csr-id-85d49a745e45894efb18ac44c93fd293f05d1cca/> separate actual computation into fn
 - <csr-id-d72cc2759a6eee744fb38f5d3574c61fb16b38c3/> it now uses Results only
 - <csr-id-d349d978ee617b929f60a99d8dc50a4863c06f7d/> thread panics are now propagated
   As well as errors that happen while successfully executing
   in the thread.
 - <csr-id-c4dbef29d4a1f1a341f869e4d3424f5341b23ff3/> correct error handling!
   Now the thread communicates issues too.
   Nice - and doing this as part of code-cleanup
   is a nice thing as well. Not too hard at all.
 - <csr-id-9e4aff7293cac609035627d82629e601f1eaf73e/> just one more hard exit remains
 - <csr-id-4f8f3d6fcc82e9cc8339d2d17019ddbbe686ed82/> setup_future just operates on futures
   Thus there is no logic to prevent stream interruption.
   Instead, said logic goes into the stream itself, which
   is wway cleaner.
 - <csr-id-8d11e2b51a33cbe2d6c8e479c9d68d69c7efe3d2/> non-threaded code now uses errors only
   That way, we can transition into a well-behaved library
   and make the code way more testable.
 - <csr-id-2f115a84dc481fccd9945aca532e419a245dcef3/> pull out key-handler

#### Other

 - <csr-id-1b00f362827b69319703fa64b79a34e2c7d6d5b9/> make clear which search result you see
   If a search fails, one will see the previous result.
   This is now made absolutely clear.
 - <csr-id-8d013f82dbf85d5895eca72f94674b6a403cfb29/> parallel search processing thanks to spawn!
   Now it finally behaves as it should :).
 - <csr-id-fee77ec2d3142f7be29a3c0d1b72209941d81d32/> better keywords
 - <csr-id-329c54b9fc88c8e3f995e09cab6dee78f1a82d61/> `krates` is an alternative program name
 - <csr-id-b025eab4ebffa7dbba8fc0c59d93d7312eb0a0ff/> basic frame to implement various subcommands
   Even though this is a bit overkill considering there really
   is just one of them I am interested in, it's a great exercise
   for me and the typesystem.
 - <csr-id-7d1f8933b51718c17b382fd9ae5ce5b84846694b/> allow printing causes

#### Chore

 - <csr-id-6ba9d0527d11efa935f3c01f604b06f0fd728ff3/> v1.3.2
 - <csr-id-1675c1e0ea304d25eb66b9b2550f6f8dc371eb85/> remove list for now
   It requires more work and prevents a patch-release

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 11 calendar days.
 - 11 days passed between releases.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.3.2 ([`6ba9d05`](https://github.com/Byron/crates-io-cli-rs/commit/6ba9d0527d11efa935f3c01f604b06f0fd728ff3))
    - Remove list for now ([`1675c1e`](https://github.com/Byron/crates-io-cli-rs/commit/1675c1e0ea304d25eb66b9b2550f6f8dc371eb85))
    - Prevent failure due to paging size ([`961360b`](https://github.com/Byron/crates-io-cli-rs/commit/961360b007122d0be8e942174d866a3fe85a7f5d))
    - Make clear which search result you see ([`1b00f36`](https://github.com/Byron/crates-io-cli-rs/commit/1b00f362827b69319703fa64b79a34e2c7d6d5b9))
    - Parallel search processing thanks to spawn! ([`8d013f8`](https://github.com/Byron/crates-io-cli-rs/commit/8d013f82dbf85d5895eca72f94674b6a403cfb29))
    - Better keywords ([`fee77ec`](https://github.com/Byron/crates-io-cli-rs/commit/fee77ec2d3142f7be29a3c0d1b72209941d81d32))
    - `krates` is an alternative program name ([`329c54b`](https://github.com/Byron/crates-io-cli-rs/commit/329c54b9fc88c8e3f995e09cab6dee78f1a82d61))
    - Basic frame to implement various subcommands ([`b025eab`](https://github.com/Byron/crates-io-cli-rs/commit/b025eab4ebffa7dbba8fc0c59d93d7312eb0a0ff))
    - Frame for `crates list by-user <user>` ([`a4f9f5d`](https://github.com/Byron/crates-io-cli-rs/commit/a4f9f5d59dfa0a7c50a852273f63600b9fbd96b8))
    - Allow printing causes ([`7d1f893`](https://github.com/Byron/crates-io-cli-rs/commit/7d1f8933b51718c17b382fd9ae5ce5b84846694b))
    - Separate actual computation into fn ([`85d49a7`](https://github.com/Byron/crates-io-cli-rs/commit/85d49a745e45894efb18ac44c93fd293f05d1cca))
    - It now uses Results only ([`d72cc27`](https://github.com/Byron/crates-io-cli-rs/commit/d72cc2759a6eee744fb38f5d3574c61fb16b38c3))
    - Thread panics are now propagated ([`d349d97`](https://github.com/Byron/crates-io-cli-rs/commit/d349d978ee617b929f60a99d8dc50a4863c06f7d))
    - Correct error handling! ([`c4dbef2`](https://github.com/Byron/crates-io-cli-rs/commit/c4dbef29d4a1f1a341f869e4d3424f5341b23ff3))
    - Just one more hard exit remains ([`9e4aff7`](https://github.com/Byron/crates-io-cli-rs/commit/9e4aff7293cac609035627d82629e601f1eaf73e))
    - Setup_future just operates on futures ([`4f8f3d6`](https://github.com/Byron/crates-io-cli-rs/commit/4f8f3d6fcc82e9cc8339d2d17019ddbbe686ed82))
    - Non-threaded code now uses errors only ([`8d11e2b`](https://github.com/Byron/crates-io-cli-rs/commit/8d11e2b51a33cbe2d6c8e479c9d68d69c7efe3d2))
    - Pull out key-handler ([`2f115a8`](https://github.com/Byron/crates-io-cli-rs/commit/2f115a84dc481fccd9945aca532e419a245dcef3))
</details>

### v1.3.1 (2017-01-01)

<csr-id-dbd55820571bedbf7708a54ceb299f059cef9cba/>
<csr-id-b76c9aad3a5e0fb9b8002c284864748e55fe53ff/>
<csr-id-3874e03a4831868bb169cf65644259401b28bcce/>
<csr-id-4a28121cf9a1a39297659f8d315150f2a5f0274a/>
<csr-id-d238844e621183f2508b9d46c00a75dc63fee265/>
<csr-id-c26d4d741fde091dbf57e87f1259825bee4c9bf6/>
<csr-id-57f7bcb97f408b370ad99e3ac053f7d13b88b6c1/>
<csr-id-71d816a7d836649b3a115e80c381c7055ccf0beb/>
<csr-id-e05e37da6f0310164226121576e0340236a9f9f8/>
<csr-id-798acf3449d97ec7c68d7630e0895ad96b2580de/>
<csr-id-f0e96258b4707e9df7f161f884adf5d023655a66/>
<csr-id-65888661f1f1516f5fa6f8549d0d65b047a18330/>
<csr-id-1f195cc7a8de5850c0cc5344d1fe6079a95e0fd6/>
<csr-id-31b2858481293377b77881a2548aab14eaefa8c6/>

#### Improvements

* **search:**
  *  timeout for curl requests ([798acf34](https://github.com/Byron/crates-io-cli-rs/commit/798acf3449d97ec7c68d7630e0895ad96b2580de))
  *  allow to do nothing in some cases ([f0e96258](https://github.com/Byron/crates-io-cli-rs/commit/f0e96258b4707e9df7f161f884adf5d023655a66))
  *  explicit select-like future drops ([65888661](https://github.com/Byron/crates-io-cli-rs/commit/65888661f1f1516f5fa6f8549d0d65b047a18330))
  *  failed queries don't abort everything anymore ([1f195cc7](https://github.com/Byron/crates-io-cli-rs/commit/1f195cc7a8de5850c0cc5344d1fe6079a95e0fd6))

#### Refactor

 - <csr-id-dbd55820571bedbf7708a54ceb299f059cef9cba/> mv all structs
 - <csr-id-b76c9aad3a5e0fb9b8002c284864748e55fe53ff/> mv DropOutdated into utils
 - <csr-id-3874e03a4831868bb169cf65644259401b28bcce/> no CPU pool needed here
 - <csr-id-4a28121cf9a1a39297659f8d315150f2a5f0274a/> The future we get is just for the sender
   It seems that once it is sent, it's handled by the thread and ...
   we don't see anything.
   
   Or do we?
 - <csr-id-d238844e621183f2508b9d46c00a75dc63fee265/> cleanup worker command enum
 - <csr-id-c26d4d741fde091dbf57e87f1259825bee4c9bf6/> use a multi-match to flatten tree
   This also makes more evident all the states we can be in,
   and generally is easier to handle than a deeply nested structure.
   
   But of course, one has to be a bit more careful about how to handle
   state to make it work, but all relatively straight-forward.
 - <csr-id-57f7bcb97f408b370ad99e3ac053f7d13b88b6c1/> Use custom reducer command
 - <csr-id-71d816a7d836649b3a115e80c381c7055ccf0beb/> move future_setup into own fn
 - <csr-id-e05e37da6f0310164226121576e0340236a9f9f8/> standalone reducer function

#### Other

 - <csr-id-798acf3449d97ec7c68d7630e0895ad96b2580de/> timeout for curl requests
   in theory, that should also cause the future
   to be dropped, but I am not too sure about that.
 - <csr-id-f0e96258b4707e9df7f161f884adf5d023655a66/> allow to do nothing in some cases
   That way we don't refresh the screen unnecessarily
 - <csr-id-65888661f1f1516f5fa6f8549d0d65b047a18330/> explicit select-like future drops
   That way, one can prevent existing curl calls to reach completion.
   It still does look like multiple ones come in in the right order,
   fortunately, but yeah, let's hope the best for now.
 - <csr-id-1f195cc7a8de5850c0cc5344d1fe6079a95e0fd6/> failed queries don't abort everything anymore

#### Chore

 - <csr-id-31b2858481293377b77881a2548aab14eaefa8c6/> v1.3.1

#### Bug Fixes

* **search:**
  *  info about valid characters is now shown ([89dfcd04](https://github.com/Byron/crates-io-cli-rs/commit/89dfcd04bf7e10632676a72d9265056a877a77ee))
  *  show better info if no search was made ([a6f8be20](https://github.com/Byron/crates-io-cli-rs/commit/a6f8be20b4c5b25667eb26566c10146cfee574e9))
*  show better info if no search was made ([a6f8be20](https://github.com/Byron/crates-io-cli-rs/commit/a6f8be20b4c5b25667eb26566c10146cfee574e9))
 - <csr-id-a6f8be20b4c5b25667eb26566c10146cfee574e9/> show better info if no search was made
 - <csr-id-89dfcd04bf7e10632676a72d9265056a877a77ee/> info about valid characters is now shown
   It was shown previously as well, but overwritten by the command
   right away.

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 16 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.3.1 ([`31b2858`](https://github.com/Byron/crates-io-cli-rs/commit/31b2858481293377b77881a2548aab14eaefa8c6))
    - Info about valid characters is now shown ([`89dfcd0`](https://github.com/Byron/crates-io-cli-rs/commit/89dfcd04bf7e10632676a72d9265056a877a77ee))
    - Misc improvements during tw review :) ([`7e0f12f`](https://github.com/Byron/crates-io-cli-rs/commit/7e0f12f40af04b674b03c2914141a2a7714b4b9d))
    - Mv all structs ([`dbd5582`](https://github.com/Byron/crates-io-cli-rs/commit/dbd55820571bedbf7708a54ceb299f059cef9cba))
    - Mv DropOutdated into utils ([`b76c9aa`](https://github.com/Byron/crates-io-cli-rs/commit/b76c9aad3a5e0fb9b8002c284864748e55fe53ff))
    - Timeout for curl requests ([`798acf3`](https://github.com/Byron/crates-io-cli-rs/commit/798acf3449d97ec7c68d7630e0895ad96b2580de))
    - No CPU pool needed here ([`3874e03`](https://github.com/Byron/crates-io-cli-rs/commit/3874e03a4831868bb169cf65644259401b28bcce))
    - The future we get is just for the sender ([`4a28121`](https://github.com/Byron/crates-io-cli-rs/commit/4a28121cf9a1a39297659f8d315150f2a5f0274a))
    - Allow to do nothing in some cases ([`f0e9625`](https://github.com/Byron/crates-io-cli-rs/commit/f0e96258b4707e9df7f161f884adf5d023655a66))
    - Explicit select-like future drops ([`6588866`](https://github.com/Byron/crates-io-cli-rs/commit/65888661f1f1516f5fa6f8549d0d65b047a18330))
    - Cleanup worker command enum ([`d238844`](https://github.com/Byron/crates-io-cli-rs/commit/d238844e621183f2508b9d46c00a75dc63fee265))
    - Failed queries don't abort everything anymore ([`1f195cc`](https://github.com/Byron/crates-io-cli-rs/commit/1f195cc7a8de5850c0cc5344d1fe6079a95e0fd6))
    - Show better info if no search was made ([`a6f8be2`](https://github.com/Byron/crates-io-cli-rs/commit/a6f8be20b4c5b25667eb26566c10146cfee574e9))
    - Use a multi-match to flatten tree ([`c26d4d7`](https://github.com/Byron/crates-io-cli-rs/commit/c26d4d741fde091dbf57e87f1259825bee4c9bf6))
    - Use custom reducer command ([`57f7bcb`](https://github.com/Byron/crates-io-cli-rs/commit/57f7bcb97f408b370ad99e3ac053f7d13b88b6c1))
    - Move future_setup into own fn ([`71d816a`](https://github.com/Byron/crates-io-cli-rs/commit/71d816a7d836649b3a115e80c381c7055ccf0beb))
    - Standalone reducer function ([`e05e37d`](https://github.com/Byron/crates-io-cli-rs/commit/e05e37da6f0310164226121576e0340236a9f9f8))
</details>

<csr-unknown>
show better info if no search was made (https://github.com/Byron/crates-io-cli-rs/commit/a6f8be20b4c5b25667eb26566c10146cfee574e9a6f8be20)<csr-unknown/>

### v1.3.0 (2016-12-30)

<csr-id-1839d7a235f092137ec4793bfac0fbfabe77825f/>
<csr-id-dba238296868ee3fa18da196ac995a9e0839f8fe/>
<csr-id-0e8837808812ac9807005aa96e6683fbd7081592/>
<csr-id-256aa62521781de59bd4a115f294e61fc0e8b620/>
<csr-id-602b937a854ccfbc658bfb2a4f2fc267816ed50b/>
<csr-id-4e788bc2c496ccf6bb10e2b6a8cd8b257d1c2720/>
<csr-id-b87386397f4b5a8a005c72310841e6b17e59c48f/>
<csr-id-9a04731720d90e313828ec92ab2fd29afd17545b/>
<csr-id-ac926039b87c03ebab7c28d27bdfe9c0b29fa96d/>
<csr-id-c945bb31627e6d270ddcb887c767f838924dc58e/>

#### Improvements

* **search and open**
 * Open crates on cates.io 


<a name="v1.2.0"></a>

#### Refactor

 - <csr-id-1839d7a235f092137ec4793bfac0fbfabe77825f/> model state much better
   That way, bugs like that last one can't happen
 - <csr-id-dba238296868ee3fa18da196ac995a9e0839f8fe/> obtain open-information
 - <csr-id-0e8837808812ac9807005aa96e6683fbd7081592/> allow to send commands instead
   This basically offloads all work related to the content
   into a separate thread powered by futures.
 - <csr-id-256aa62521781de59bd4a115f294e61fc0e8b620/> improved state model
   That way, we can more easily decide what to do.

#### Other

 - <csr-id-602b937a854ccfbc658bfb2a4f2fc267816ed50b/> put open indices closer to crate name
 - <csr-id-4e788bc2c496ccf6bb10e2b6a8cd8b257d1c2720/> redraw last search result
   Previously the search would be repeated, which caused a delay.
   Now switching from open to search mode is instantaneous.
 - <csr-id-b87386397f4b5a8a005c72310841e6b17e59c48f/> open a crate with <enter>
   However, there is a bug that can cause the previous search to not
   be shown, yet they are available for selection.
 - <csr-id-9a04731720d90e313828ec92ab2fd29afd17545b/> open now works, basically
   However, it's not yet possible to get non-unique matches.
 - <csr-id-ac926039b87c03ebab7c28d27bdfe9c0b29fa96d/> only allow entering numbers

#### Bug Fixes

 - <csr-id-4fd38536d74ddf6fff941cedf5e4307ed166346f/> correct state handling
   Previously, after opening, one couldn't search anymore.
   This shows that our mode shouldn't be used for that.

#### Chore

 - <csr-id-c945bb31627e6d270ddcb887c767f838924dc58e/> v1.3.0
   [skip ci]

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.3.0 ([`c945bb3`](https://github.com/Byron/crates-io-cli-rs/commit/c945bb31627e6d270ddcb887c767f838924dc58e))
    - Put open indices closer to crate name ([`602b937`](https://github.com/Byron/crates-io-cli-rs/commit/602b937a854ccfbc658bfb2a4f2fc267816ed50b))
    - Model state much better ([`1839d7a`](https://github.com/Byron/crates-io-cli-rs/commit/1839d7a235f092137ec4793bfac0fbfabe77825f))
    - Correct state handling ([`4fd3853`](https://github.com/Byron/crates-io-cli-rs/commit/4fd38536d74ddf6fff941cedf5e4307ed166346f))
    - Redraw last search result ([`4e788bc`](https://github.com/Byron/crates-io-cli-rs/commit/4e788bc2c496ccf6bb10e2b6a8cd8b257d1c2720))
    - Fix inconsistency issues ([`7532dc5`](https://github.com/Byron/crates-io-cli-rs/commit/7532dc576a12c260fa1a99762bbbc8c1c43461e3))
    - Open a crate with <enter> ([`b873863`](https://github.com/Byron/crates-io-cli-rs/commit/b87386397f4b5a8a005c72310841e6b17e59c48f))
    - With the current model return values are impossible ([`9520ada`](https://github.com/Byron/crates-io-cli-rs/commit/9520ada3aba3ae62e30c26e04648d4b7ca378c6a))
    - Transfer of results doesnt work as expected. ([`11ba57d`](https://github.com/Byron/crates-io-cli-rs/commit/11ba57de8bdd8412ad95daf631018d78159ff7ba))
    - Open now works, basically ([`9a04731`](https://github.com/Byron/crates-io-cli-rs/commit/9a04731720d90e313828ec92ab2fd29afd17545b))
    - Inform about upcoming open feature ([`2ae5f11`](https://github.com/Byron/crates-io-cli-rs/commit/2ae5f112b84a47a65cb0cade6aee417ced907cee))
    - Correct way of displaying indices ([`96323fa`](https://github.com/Byron/crates-io-cli-rs/commit/96323fa5ce03f658956d976c479fb2db3ccfa25e))
    - First attempt to draw indices ([`f228233`](https://github.com/Byron/crates-io-cli-rs/commit/f228233374a6a76e2e31136a103bc1cb946fabc7))
    - All machinery to handle number input ([`295bfff`](https://github.com/Byron/crates-io-cli-rs/commit/295bfffbf7e24a5a64541e15c3d6c3219a854647))
    - Only allow entering numbers ([`ac92603`](https://github.com/Byron/crates-io-cli-rs/commit/ac926039b87c03ebab7c28d27bdfe9c0b29fa96d))
    - Obtain open-information ([`dba2382`](https://github.com/Byron/crates-io-cli-rs/commit/dba238296868ee3fa18da196ac995a9e0839f8fe))
    - Allow to send commands instead ([`0e88378`](https://github.com/Byron/crates-io-cli-rs/commit/0e8837808812ac9807005aa96e6683fbd7081592))
    - Improved state model ([`256aa62`](https://github.com/Byron/crates-io-cli-rs/commit/256aa62521781de59bd4a115f294e61fc0e8b620))
</details>

### v1.2.0 (2016-12-30)

<csr-id-3629c64d1703d20fff8a25c8646df255f10f715a/>
<csr-id-b9e764ef5b3be735a05f2aafe1b96b8ea31cab9f/>
<csr-id-9d352672cca9b96e1cb6195bc63526cb13690034/>
<csr-id-88ae68c54287e0565079f0e423a5f0b257c7ea77/>
<csr-id-40a39f767bef00c6f11ec351906c1542ad929486/>
<csr-id-99488daa85b86022243df19edad36492b216f3ec/>
<csr-id-ebe329adea98a3ad29c6ac73576597ce3bd100c5/>
<csr-id-53ac3ec6a6d6a063abfd195469cef3d45051c047/>
<csr-id-b64a2dd8edc4e11900e10d5d6d9d62dfca6faab6/>
<csr-id-f8c0dd5b00a224fd86d8d6eb96a3907e0b6c2c38/>
<csr-id-89b718893bed5349866932940849293ed11c0167/>
<csr-id-4f698a6fb931cd9b7679cbdcef72dcb063b3edad/>
<csr-id-48ac66eec6b3d1677a1ee8fdb44b3dc4a687f976/>
<csr-id-3f48d91677583af663be842f302ab6e6d96f88c0/>
<csr-id-8f3606dbb15bd8043b1a8debe37120d5a8d38df7/>
<csr-id-86512487c58f053d5242875c885b5f8a20f34b23/>
<csr-id-e98897c190579a96d558ad9a96936a6bd3055c52/>
<csr-id-4cd2f3de2652f93e5c4ebf964cd476036e505faa/>
<csr-id-ca792c0bc6dd86758d3d905a6ffca5f60fd59c68/>
<csr-id-0dcef290de1633aa0a0904792b3ee9e41ecde8f5/>
<csr-id-d7f32855a0afb749bebb50d593a50e7a799141f7/>
<csr-id-77d824039759e3f03d4ecf9ca2bf93b71d8c08cc/>
<csr-id-8a2837b9c829811201d6a15a5f11b3ba973cb735/>
<csr-id-d44f874fd0d413afd2e45d3f1682be5711078f7f/>
<csr-id-a6d4e8e8f8bd83ece10d8506bcd85d88514af160/>
<csr-id-f60e6c9161b7aa6ab06d3b5eeb31ef3041cfa927/>
<csr-id-49a139a7ea84da77895e329f32fb0ff9ddc134f4/>
<csr-id-ecb0d70601155f3eb8b22175e98cd87c4dc64006/>
<csr-id-e1e526264db5f695b517abc4d34896933d83a6ae/>
<csr-id-ecdb07f666e501ab4212957ffc68013e998246ae/>
<csr-id-cbbf89865f9b2a3fa90784fa7889a928780d91b9/>
<csr-id-0571ea0caf14156f41d3eeaeb215919d391e6910/>
<csr-id-7cee7869f6f7b5a65eddf4490cec4203248693af/>

#### Features

* **search:**
  *  use `crates search` for an interactive search on crates.io

#### Test

 - <csr-id-3629c64d1703d20fff8a25c8646df255f10f715a/> use the official search URL
   This takes a little longer and nicely shows how connections
   are just dropped.
 - <csr-id-b9e764ef5b3be735a05f2aafe1b96b8ea31cab9f/> thanks to the CpuPool, semantics work now!
   The latter is actually providing its own loop, and thus polls futures
   that are handed to it.
   
   Maybe one wouldn't need a separate thread anymore, lets see!
 - <csr-id-9d352672cca9b96e1cb6195bc63526cb13690034/> one has to run the futures ...
   Lesson learned: one wants to express everything with futures.
   
   However, it's not easy at all to not block the main thread
   if the latter does not use futures.
   One has to spin them somehow without using the own thread.
 - <csr-id-88ae68c54287e0565079f0e423a5f0b257c7ea77/> use mpsc instead
   Current version does not compile because ... hmm.
   I really don't get it.
 - <csr-id-40a39f767bef00c6f11ec351906c1542ad929486/> try using tokio-curl
   However, the loop needs to be spun by someone.
   It appears to be easiest to run it forever in a thread.
   Let's see how the program manages to shut down then.
 - <csr-id-99488daa85b86022243df19edad36492b216f3ec/> A timer-future shows the correct semantics
   However, now it's about getting curl up and running.
   The thread to make progress would handle all IO, which
   technically can't be the main thread as the stdin handling
   is not futurized.
 - <csr-id-ebe329adea98a3ad29c6ac73576597ce3bd100c5/> see how blocking cpu-futures behave
   I think that could already be it, as all we need here is
   some operation which runs on the loop and can thus be aborted.
   
   Thanks to the threadpool, operations make progress even without
   us waiting for it in the main thread, and thus can be dropped at
   any time.
 - <csr-id-53ac3ec6a6d6a063abfd195469cef3d45051c047/> minimal read-line like functionality

#### Refactor

 - <csr-id-b64a2dd8edc4e11900e10d5d6d9d62dfca6faab6/> Dimension fits well in there
 - <csr-id-f8c0dd5b00a224fd86d8d6eb96a3907e0b6c2c38/> move implementation in own module
 - <csr-id-89b718893bed5349866932940849293ed11c0167/> own module for structures
 - <csr-id-4f698a6fb931cd9b7679cbdcef72dcb063b3edad/> make it its own module
 - <csr-id-48ac66eec6b3d1677a1ee8fdb44b3dc4a687f976/> remove url crate
   curl can do the same.
 - <csr-id-3f48d91677583af663be842f302ab6e6d96f88c0/> rustfmt and better arrangement of timer
 - <csr-id-8f3606dbb15bd8043b1a8debe37120d5a8d38df7/> move crate declarations to point of use
   Didn't know that you can do that, and it should help to keep
   them more local.
 - <csr-id-86512487c58f053d5242875c885b5f8a20f34b23/> restructure code into modules
   That way, we can more easily add new subcommand.

#### Other

 - <csr-id-e98897c190579a96d558ad9a96936a6bd3055c52/> saturate width
 - <csr-id-4cd2f3de2652f93e5c4ebf964cd476036e505faa/> compute column width to not exceed space
 - <csr-id-ca792c0bc6dd86758d3d905a6ffca5f60fd59c68/> display a nice table
 - <csr-id-0dcef290de1633aa0a0904792b3ee9e41ecde8f5/> take non-content lines into consideration
 - <csr-id-d7f32855a0afb749bebb50d593a50e7a799141f7/> tui uses heigth dynamically
 - <csr-id-77d824039759e3f03d4ecf9ca2bf93b71d8c08cc/> more consistent TUI
 - <csr-id-8a2837b9c829811201d6a15a5f11b3ba973cb735/> wait for 3 seconds before showing timeout message
 - <csr-id-d44f874fd0d413afd2e45d3f1682be5711078f7f/> more descriptive timeout message

#### Bug Fixes

 - <csr-id-c7bd58acadf99b7c42f8d4c472aedd164c4e1323/> plural 's'
   [skip ci]
 - <csr-id-621d0988f09bf82624a263402eca41f87d3af8c0/> update to latest libgit2
 - <csr-id-96f706268d091f4edd3dc5814aba3065aae16cb0/> descriptions are optional

#### New Features

 - <csr-id-56447d6f8a45bbcf360e25898b4a616c438b6507/> first usable version
   Notably missing:
   * properly use vertical space
* nice table alignment
* jump to crates.io

#### Documentation

 - <csr-id-43aa34ad1ef26a174eb755cb5e319a315fe3dd7b/> feature presentation
   asciinema is great!
   [skip ci]
 - <csr-id-80218b270d46d85cf70c125706b2ef23575b3676/> add appveyor badge
   [skip ci]

#### Chore

 - <csr-id-a6d4e8e8f8bd83ece10d8506bcd85d88514af160/> v1.2.0
 - <csr-id-f60e6c9161b7aa6ab06d3b5eeb31ef3041cfa927/> one more time
 - <csr-id-49a139a7ea84da77895e329f32fb0ff9ddc134f4/> maybe this time
 - <csr-id-ecb0d70601155f3eb8b22175e98cd87c4dc64006/> exclude interactive search on windows
 - <csr-id-e1e526264db5f695b517abc4d34896933d83a6ae/> basic appveyor testing
 - <csr-id-ecdb07f666e501ab4212957ffc68013e998246ae/> test linux only
   cmake was the problem last time, now I remember.
 - <csr-id-cbbf89865f9b2a3fa90784fa7889a928780d91b9/> try to test on osx too
   Linux is currently broken ... .
 - <csr-id-0571ea0caf14156f41d3eeaeb215919d391e6910/> basic usage of termion
   Next step is to put new events into futures.
 - <csr-id-7cee7869f6f7b5a65eddf4490cec4203248693af/> v1.1.1

#### Improvements

* **UX:**  wait for 3 seconds before showing timeout message ([8a2837b9](https://github.com/Byron/crates-io-cli-rs/commit/8a2837b9c829811201d6a15a5f11b3ba973cb735))
* **changes:**  display a nice table ([ca792c0b](https://github.com/Byron/crates-io-cli-rs/commit/ca792c0bc6dd86758d3d905a6ffca5f60fd59c68))

<a name="v1.1.1"></a>

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 39 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.2.0 ([`a6d4e8e`](https://github.com/Byron/crates-io-cli-rs/commit/a6d4e8e8f8bd83ece10d8506bcd85d88514af160))
    - Plural 's' ([`c7bd58a`](https://github.com/Byron/crates-io-cli-rs/commit/c7bd58acadf99b7c42f8d4c472aedd164c4e1323))
    - Feature presentation ([`43aa34a`](https://github.com/Byron/crates-io-cli-rs/commit/43aa34ad1ef26a174eb755cb5e319a315fe3dd7b))
    - One more time ([`f60e6c9`](https://github.com/Byron/crates-io-cli-rs/commit/f60e6c9161b7aa6ab06d3b5eeb31ef3041cfa927))
    - Maybe this time ([`49a139a`](https://github.com/Byron/crates-io-cli-rs/commit/49a139a7ea84da77895e329f32fb0ff9ddc134f4))
    - Exclude interactive search on windows ([`ecb0d70`](https://github.com/Byron/crates-io-cli-rs/commit/ecb0d70601155f3eb8b22175e98cd87c4dc64006))
    - Add appveyor badge ([`80218b2`](https://github.com/Byron/crates-io-cli-rs/commit/80218b270d46d85cf70c125706b2ef23575b3676))
    - Basic appveyor testing ([`e1e5262`](https://github.com/Byron/crates-io-cli-rs/commit/e1e526264db5f695b517abc4d34896933d83a6ae))
    - Update to latest libgit2 ([`621d098`](https://github.com/Byron/crates-io-cli-rs/commit/621d0988f09bf82624a263402eca41f87d3af8c0))
    - Test linux only ([`ecdb07f`](https://github.com/Byron/crates-io-cli-rs/commit/ecdb07f666e501ab4212957ffc68013e998246ae))
    - Saturate width ([`e98897c`](https://github.com/Byron/crates-io-cli-rs/commit/e98897c190579a96d558ad9a96936a6bd3055c52))
    - Compute column width to not exceed space ([`4cd2f3d`](https://github.com/Byron/crates-io-cli-rs/commit/4cd2f3de2652f93e5c4ebf964cd476036e505faa))
    - Descriptions are optional ([`96f7062`](https://github.com/Byron/crates-io-cli-rs/commit/96f706268d091f4edd3dc5814aba3065aae16cb0))
    - Display a nice table ([`ca792c0`](https://github.com/Byron/crates-io-cli-rs/commit/ca792c0bc6dd86758d3d905a6ffca5f60fd59c68))
    - Try to test on osx too ([`cbbf898`](https://github.com/Byron/crates-io-cli-rs/commit/cbbf89865f9b2a3fa90784fa7889a928780d91b9))
    - Dimension fits well in there ([`b64a2dd`](https://github.com/Byron/crates-io-cli-rs/commit/b64a2dd8edc4e11900e10d5d6d9d62dfca6faab6))
    - Take non-content lines into consideration ([`0dcef29`](https://github.com/Byron/crates-io-cli-rs/commit/0dcef290de1633aa0a0904792b3ee9e41ecde8f5))
    - Tui uses heigth dynamically ([`d7f3285`](https://github.com/Byron/crates-io-cli-rs/commit/d7f32855a0afb749bebb50d593a50e7a799141f7))
    - Move implementation in own module ([`f8c0dd5`](https://github.com/Byron/crates-io-cli-rs/commit/f8c0dd5b00a224fd86d8d6eb96a3907e0b6c2c38))
    - Own module for structures ([`89b7188`](https://github.com/Byron/crates-io-cli-rs/commit/89b718893bed5349866932940849293ed11c0167))
    - Make it its own module ([`4f698a6`](https://github.com/Byron/crates-io-cli-rs/commit/4f698a6fb931cd9b7679cbdcef72dcb063b3edad))
    - Remove url crate ([`48ac66e`](https://github.com/Byron/crates-io-cli-rs/commit/48ac66eec6b3d1677a1ee8fdb44b3dc4a687f976))
    - More consistent TUI ([`77d8240`](https://github.com/Byron/crates-io-cli-rs/commit/77d824039759e3f03d4ecf9ca2bf93b71d8c08cc))
    - First usable version ([`56447d6`](https://github.com/Byron/crates-io-cli-rs/commit/56447d6f8a45bbcf360e25898b4a616c438b6507))
    - Decode search results and show the amount of hits ([`ddf6fba`](https://github.com/Byron/crates-io-cli-rs/commit/ddf6fba24ff0d332f1ca6c99ee4d950d067cd1c9))
    - Use the official search URL ([`3629c64`](https://github.com/Byron/crates-io-cli-rs/commit/3629c64d1703d20fff8a25c8646df255f10f715a))
    - Thanks to the CpuPool, semantics work now! ([`b9e764e`](https://github.com/Byron/crates-io-cli-rs/commit/b9e764ef5b3be735a05f2aafe1b96b8ea31cab9f))
    - One has to run the futures ... ([`9d35267`](https://github.com/Byron/crates-io-cli-rs/commit/9d352672cca9b96e1cb6195bc63526cb13690034))
    - Use mpsc instead ([`88ae68c`](https://github.com/Byron/crates-io-cli-rs/commit/88ae68c54287e0565079f0e423a5f0b257c7ea77))
    - Try using tokio-curl ([`40a39f7`](https://github.com/Byron/crates-io-cli-rs/commit/40a39f767bef00c6f11ec351906c1542ad929486))
    - Rustfmt and better arrangement of timer ([`3f48d91`](https://github.com/Byron/crates-io-cli-rs/commit/3f48d91677583af663be842f302ab6e6d96f88c0))
    - A timer-future shows the correct semantics ([`99488da`](https://github.com/Byron/crates-io-cli-rs/commit/99488daa85b86022243df19edad36492b216f3ec))
    - See how blocking cpu-futures behave ([`ebe329a`](https://github.com/Byron/crates-io-cli-rs/commit/ebe329adea98a3ad29c6ac73576597ce3bd100c5))
    - Minimal read-line like functionality ([`53ac3ec`](https://github.com/Byron/crates-io-cli-rs/commit/53ac3ec6a6d6a063abfd195469cef3d45051c047))
    - Move crate declarations to point of use ([`8f3606d`](https://github.com/Byron/crates-io-cli-rs/commit/8f3606dbb15bd8043b1a8debe37120d5a8d38df7))
    - Basic usage of termion ([`0571ea0`](https://github.com/Byron/crates-io-cli-rs/commit/0571ea0caf14156f41d3eeaeb215919d391e6910))
    - Restructure code into modules ([`8651248`](https://github.com/Byron/crates-io-cli-rs/commit/86512487c58f053d5242875c885b5f8a20f34b23))
    - Wait for 3 seconds before showing timeout message ([`8a2837b`](https://github.com/Byron/crates-io-cli-rs/commit/8a2837b9c829811201d6a15a5f11b3ba973cb735))
    - V1.1.1 ([`7cee786`](https://github.com/Byron/crates-io-cli-rs/commit/7cee7869f6f7b5a65eddf4490cec4203248693af))
    - More descriptive timeout message ([`d44f874`](https://github.com/Byron/crates-io-cli-rs/commit/d44f874fd0d413afd2e45d3f1682be5711078f7f))
</details>

### v1.1.1 (2016-12-28)

#### Improvements

* **cli:**  more descriptive timeout message ([d44f874f](https://github.com/Byron/crates-io-cli-rs/commit/d44f874fd0d413afd2e45d3f1682be5711078f7f))



<a name="v1.1.0"></a>

### v1.1.0 (2016-12-28)

<csr-id-185ee003cc7f1f8bc742f5f121d468318a0de10e/>
<csr-id-47748a6616d0589aa0326e21be479b308a15f4c8/>

#### Improvements

* **cli:**  try to implement timeout with futures ([185ee003](https://github.com/Byron/crates-io-cli-rs/commit/185ee003cc7f1f8bc742f5f121d468318a0de10e))



<a name="v1.0.2"></a>

#### Other

 - <csr-id-185ee003cc7f1f8bc742f5f121d468318a0de10e/> try to implement timeout with futures
   This version will properly select either the timeout future or
   the computation.
   However, right now it seems impossible to keep the computation
   running.

#### New Features

 - <csr-id-5187ae51c10e539ede401e2ee2e83cf9d9551732/> inform about long-running computation
   In case we take a while to perform our computation, we will now report
   that we are probably checking out a big repository.

#### Chore

 - <csr-id-47748a6616d0589aa0326e21be479b308a15f4c8/> v1.1.0

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.1.0 ([`47748a6`](https://github.com/Byron/crates-io-cli-rs/commit/47748a6616d0589aa0326e21be479b308a15f4c8))
    - Inform about long-running computation ([`5187ae5`](https://github.com/Byron/crates-io-cli-rs/commit/5187ae51c10e539ede401e2ee2e83cf9d9551732))
    - Try to implement timeout with futures ([`185ee00`](https://github.com/Byron/crates-io-cli-rs/commit/185ee003cc7f1f8bc742f5f121d468318a0de10e))
</details>

### v1.0.2 (2016-12-28)

#### Features

* **async:**  inform about long-running computation ([5187ae51](https://github.com/Byron/crates-io-cli-rs/commit/5187ae51c10e539ede401e2ee2e83cf9d9551732))



<a name="v1.0.1"></a>

### v1.0.1 (2016-12-26)

<csr-id-508e9589933e10d9c9e5fc33fb08390f3ab93e55/>
<csr-id-b7a39ad8ef68adb81b2d8a7e552cb0a2a73f7d5b/>

#### Other

 - <csr-id-508e9589933e10d9c9e5fc33fb08390f3ab93e55/> add version badge
   [skip ci]

#### Chore

 - <csr-id-b7a39ad8ef68adb81b2d8a7e552cb0a2a73f7d5b/> v1.0.1

#### Bug Fixes

* **cargo:**  remove unnecessary documentation link ([8de7d263](https://github.com/Byron/crates-io-cli-rs/commit/8de7d263241c5061578f5aaf6d99e4e9c77a72e4))
 - <csr-id-8de7d263241c5061578f5aaf6d99e4e9c77a72e4/> remove unnecessary documentation link

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.0.1 ([`b7a39ad`](https://github.com/Byron/crates-io-cli-rs/commit/b7a39ad8ef68adb81b2d8a7e552cb0a2a73f7d5b))
    - Add version badge ([`508e958`](https://github.com/Byron/crates-io-cli-rs/commit/508e9589933e10d9c9e5fc33fb08390f3ab93e55))
    - Remove unnecessary documentation link ([`8de7d26`](https://github.com/Byron/crates-io-cli-rs/commit/8de7d263241c5061578f5aaf6d99e4e9c77a72e4))
</details>

### v1.0.0 (2016-12-26)

<csr-id-88753b7f24a779d76853aea4cd5af1be1e43825c/>
<csr-id-6032bf7d286872518f00938b7156a52ef017179f/>
<csr-id-30b779fbd4e7c804434314259719e1e0a9290c36/>
<csr-id-9597811afa5f37aa3ca3642ce35b14454167bc51/>
<csr-id-b0a6468477ca43e748cb39fd5efad764897e78d8/>
<csr-id-76ea4378719ef394af5577cec3f67eb58f9c29db/>
<csr-id-52072b1f567de5dfc4937447286255c7964b36aa/>
<csr-id-b498107f30c7670b3d4f4dc456ceee2206329994/>

#### Chore

 - <csr-id-88753b7f24a779d76853aea4cd5af1be1e43825c/> v1.0.0
 - <csr-id-6032bf7d286872518f00938b7156a52ef017179f/> fix makefile for linux
   For some reason cargo cannot be found. Interesting,
   it seemed to have worked in the crates-index-diff project.
 - <csr-id-30b779fbd4e7c804434314259719e1e0a9290c36/> initial test setup
 - <csr-id-9597811afa5f37aa3ca3642ce35b14454167bc51/> first commit

#### New Features

 - <csr-id-222deaf027dcd24a6804154f474cd07f6c618325/> --output=json
   default is 'human' for the previous display style.
 - <csr-id-abe1c888806070740edaf7e474d584c3c2561a6d/> show changes in human-readable format

#### Bug Fixes

 - <csr-id-4be7a802856f117f0c9bdcb5599994bdc8727d84/> create repository directory accordingly
   Empty dirs are OK with git fortunately.

#### Other

 - <csr-id-b0a6468477ca43e748cb39fd5efad764897e78d8/> better advertisement
 - <csr-id-76ea4378719ef394af5577cec3f67eb58f9c29db/> first basic flags and subcommand

#### Refactor

 - <csr-id-52072b1f567de5dfc4937447286255c7964b36aa/> fmt::Write !== io::Write
   It's a well known fact, but ... I didn't see it in the message :).
   Sometimes one is a little blind.
 - <csr-id-b498107f30c7670b3d4f4dc456ceee2206329994/> put subcommand into own function
   Unification of error handling is a bit more difficult this time
   around, as the success type is different each time.

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.0.0 ([`88753b7`](https://github.com/Byron/crates-io-cli-rs/commit/88753b7f24a779d76853aea4cd5af1be1e43825c))
    - Better advertisement ([`b0a6468`](https://github.com/Byron/crates-io-cli-rs/commit/b0a6468477ca43e748cb39fd5efad764897e78d8))
    - Fmt::Write !== io::Write ([`52072b1`](https://github.com/Byron/crates-io-cli-rs/commit/52072b1f567de5dfc4937447286255c7964b36aa))
    - Put subcommand into own function ([`b498107`](https://github.com/Byron/crates-io-cli-rs/commit/b498107f30c7670b3d4f4dc456ceee2206329994))
    - Create repository directory accordingly ([`4be7a80`](https://github.com/Byron/crates-io-cli-rs/commit/4be7a802856f117f0c9bdcb5599994bdc8727d84))
    - --output=json ([`222deaf`](https://github.com/Byron/crates-io-cli-rs/commit/222deaf027dcd24a6804154f474cd07f6c618325))
    - Fix makefile for linux ([`6032bf7`](https://github.com/Byron/crates-io-cli-rs/commit/6032bf7d286872518f00938b7156a52ef017179f))
    - Show changes in human-readable format ([`abe1c88`](https://github.com/Byron/crates-io-cli-rs/commit/abe1c888806070740edaf7e474d584c3c2561a6d))
    - First basic flags and subcommand ([`76ea437`](https://github.com/Byron/crates-io-cli-rs/commit/76ea4378719ef394af5577cec3f67eb58f9c29db))
    - Initial test setup ([`30b779f`](https://github.com/Byron/crates-io-cli-rs/commit/30b779fbd4e7c804434314259719e1e0a9290c36))
    - First commit ([`9597811`](https://github.com/Byron/crates-io-cli-rs/commit/9597811afa5f37aa3ca3642ce35b14454167bc51))
</details>

