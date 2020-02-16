## Goals

* fast insertions and updates

## Non-Goals
* fast reads

## Tasks

* example-driven development for tree-interface and visualization
  * [x] task tree with colored progress, proper resizing 
  * [x] title-only
  * [x] bounded progress
  * [x] unbounded progress
  * [x] blocking indicator
    * [x] with optional time-to-unblock
  * [x] tasks overview headline
  * [x] fractional FPS
    * [x] 'last frame at' for FPS < 1.0 in headline
  * [x] unicode support
  * [x] Ring-buffer powered message buffer in tree (done, info, warn, error)
    * [x] display that message buffer in tui
    * [x] overflow handling
  * [x] use future streams as sole interface (one unified stream with Messages)
  * [x] define boundary on the fly using messages (allows incorporating it into other TUIs potentially)
  * [x] allow scrolling through tasks and messages
  * [x] set title dynamically
  * [ ] custom statistics window (data sent via message channel)
* [x] Argh based command-line input
* [x] some benchmarks
* [ ] run example as journey test
* [ ] full documentation and maybe a few smaller examples

## Lessons Learned

* `drop()` is not garantueed to be called when the future returns Ready and is in the futures::executor::ThreadPool
  * Workaround: drop and cleanup explicitly, prone to forgetting it.
  * This is also why `futures::future::abortable()` works (by stopping the polling), but doesn't as cleanup is not performed,
    even though it clearly would be preferred.
  * fix
    * Use a join handle and await it - this will drop the future properly
* `select()` might not work with complex futures - these should then be `boxed()` if `Unpin` isn't implemented.

## Limitations

* The underlying sync data structure, `dashmap`, does not document every use of unsafe
  * I also evaluated `evmap`, which has 25% less uses of unsafe, but a more complex interface.
  * Thus far it seemed 'ok' to use, who knows… we are getting mutable pieces of a hashmap from multiple threads,
    however, we never hand out multiple handles to the same child which should make actual concurrent access to 
    the same key impossible.
* If there are more than 2^16 tasks
  * then
    * running concurrently on a single level of the tree, they start overwriting each other
    * over its lifetime, even though they do not run concurrently, eventually new tasks will seem like old tasks (their ID wrapped around)
  * why
    * on drop, they never decrement a child count used to generate a new ID
  * fix
    * make the id bigger, like u32
    * we should do that once there is a performance test
