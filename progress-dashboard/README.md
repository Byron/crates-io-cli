## Goals

* fast insertions and updates

## Non-Goals
* fast reads

## Tasks

* example-driven development for tree-interface and visualization
  * [ ] blocking indicator
  * [ ] unbounded progress
  * [ ] tasks-done list
  * [ ] tasks overview headline
  * [ ] custom statistics window
* [ ] document test-strategy (at least)


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
