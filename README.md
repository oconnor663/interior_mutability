# Interior Mutability in Rust

- "guard" cells (give out `&mut T`)
    - [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
    - [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
        - Notice that [`impl Sync for Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E) and [`impl Sync for RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html#impl-Sync-for-RwLock%3CT%3E) have different bounds.
    - [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
        - [with thread-locals](https://github.com/rust-lang/rust/blob/17584a181979f04f2aaad867332c22db1caa511a/compiler/rustc_middle/src/ty/impls_ty.rs#L23-L29)
        - [with "reentrant" locks](https://github.com/rust-lang/rust/blob/17584a181979f04f2aaad867332c22db1caa511a/library/std/src/io/stdio.rs#L612)
- "once" cells (only give out `&T`)
    - [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html)
    - [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)
    - [`OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html)
    - [`LazyCell`](https://doc.rust-lang.org/std/cell/struct.LazyCell.html)
- "copy" cells (don't give out references at all)
    - [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html)
        - I lied, there's a [`.get_mut()`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.get_mut) method, and it's perfectly safe.
        - If you think that's weird, take a look at [`.from_mut()`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.from_mut).
    - [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/)
- [`UnsafeCell`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)
