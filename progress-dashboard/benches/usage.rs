use criterion::*;

use progress_dashboard::{Config, MessageLevel, TreeRoot};

fn usage(c: &mut Criterion) {
    fn small_tree() -> TreeRoot {
        Config {
            initial_capacity: 10,
            message_buffer_capacity: 2,
        }
        .create()
    };
    c.benchmark_group("TreeRoot::add_child")
        .throughput(Throughput::Elements(4))
        .bench_function(
            "add children to build a tree of tasks and clear them (in drop)",
            |b| {
                let root = small_tree();
                b.iter(|| {
                    let mut c = root.add_child("1");
                    let _one = c.add_child("1");
                    let _two = c.add_child("2");
                    let _three = c.add_child("3");
                });
            },
        );
    c.benchmark_group("Tree::set")
        .throughput(Throughput::Elements(5))
        .bench_function("set tree 5 times", |b| {
            let root = small_tree();
            let mut progress = root.add_child("the one");
            progress.init(Some(20), Some("element"));
            b.iter(|| {
                progress.set(1);
                progress.set(2);
                progress.set(3);
                progress.set(4);
                progress.set(5);
            });
        });
    c.benchmark_group("Tree::message")
        .throughput(Throughput::Elements(1))
        .bench_function(
            "send one message with a full message buffer (worst case performance)",
            |b| {
                let root = small_tree();
                let mut progress = root.add_child("the one");
                progress.init(Some(20), Some("element"));
                b.iter(|| {
                    progress.message(MessageLevel::Success, "for testing");
                });
            },
        );
}

criterion_group!(benches, usage);
criterion_main!(benches);
