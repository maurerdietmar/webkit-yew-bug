# Webkit yew bug reproducer

Reproducer for a bug running yew with webkit.

Inspirted by: https://github.com/jrouaix/wasm_pbm

We test on Debian bookworm using the epiphany-browser.

## To run the example:

    trunk serve

Then open http://locahost:8080 on your webkit based browser and press reload until you get a crash.

It sometimes required up to 50 reloads to trigger the bug.

Note: Everything runs well inside Firefox or Chrome.

## Obseverd bug (see bugs.txt for details) using Webkit

```
RuntimeError: call_indirect to a null table entry (near '...e__h9a7711c915d393d5(arg0, arg1, addHeap...')
RuntimeError: Out of bounds memory access (near '...e__h9a7711c915d393d5(arg0, arg1, addHeap...')
RuntimeError: Unreachable code should not be executed (near '...e__h9a7711c915d393d5(arg0, arg1, addHeap...')
Unhandled Promise Rejection: RuntimeError: Unreachable code should not be executed (evaluating 'wasm.__wbindgen_start()')
panicked at 'assertion failed: `(left != right)`
panicked at 'assertion failed: buckets.is_power_of_two()', /home/dietmar/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.14.3/src/raw/mod.rs:1744:9
panicked at 'attempt to subtract with overflow', /home/dietmar/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.14.3/src/raw/mod.rs:4268:9
panicked at 'called `Result::unwrap()` on an `Err` value: LayoutError', /usr/src/rustc-1.70.0/library/alloc/src/rc.rs:1474:41
panicked at 'assertion failed: self.bucket_mask < Group::WIDTH', /home/dietmar/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.14.3/src/raw/mod.rs:1872:13
[Error] panicked at 'Went past end of probe sequence', /home/dietmar/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.14.3/src/raw/mod.rs:177:9
panicked at 'Hash table capacity overflow', /home/dietmar/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.14.3/src/raw/mod.rs:86:40
```