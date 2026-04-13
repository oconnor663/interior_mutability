# Interior Mutability in Rust

- "guard" cells (give out `&mut T`)
    - [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
    - [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
    - [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- "once" cells (only give out `&T`)
    - [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html)
    - [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)
    - [`OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html)
    - [`LazyCell`](https://doc.rust-lang.org/std/cell/struct.LazyCell.html)
- "copy" cells (don't give out references)
    - [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/)
    - [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html)
- inside all of these
    - [`UnsafeCell`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)
