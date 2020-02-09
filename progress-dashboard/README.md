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
  * [ ] blocking indicator
    * [ ] with optional time-to-unblock
  * [ ] fractional FPS
  * [ ] use future streams as sole interface (one unified stream with Messages)
  * [ ] tasks overview headline
    * [ ] FPS count
    * [ ] 'last frame at' for FPS < 1.0
  * [ ] define boundary on the fly using messages (allows incorporating it into other TUIs potentially)
  * [ ] custom statistics window (data sent via message channel)
    * [ ] overflow handling
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

* If there are more than 2^16 tasks
  * then
    * running concurrently on a single level of the tree, they start overwriting each other
    * over its lifetime, even though they do not run concurrently, eventually new tasks will seem like old tasks (their ID wrapped around)
  * why
    * on drop, they never decrement a child count used to generate a new ID
  * fix
    * make the id bigger, like u32
    * we should do that once there is a performance test
