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
- [`UnsafeCell`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)
    - [mutable aliasing in Godbolt][godbolt]
- "copy" cells (don't give out references at all)
    - [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html)
        - [with thread-locals](https://github.com/rust-lang/rust/blob/17584a181979f04f2aaad867332c22db1caa511a/library/std/src/thread/spawnhook.rs#L12)
        - I lied about references; there's a [`.get_mut()`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.get_mut) method, and it's perfectly safe.
        - If you think that's weird, take a look at [`.from_mut()`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.from_mut).
        - [Godbolt again][godbolt_cell]
    - [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/)

[godbolt]: <https://godbolt.org/#z:OYLghAFBqd5TKALEBjA9gEwKYFFMCWALugE4A0BIEAZgQDbYB2AhgLbYgDkAjF%2BTXRMiAZVQtGIAMwAGAUKIBVAM7YACgA9uM/gCsQPcvRZNQAfQupyqxqiIEh1ZpgDC6egFc2TEACZyzgAyBEzYAHJeAEbYpCAALOQADujKxA5Mbp7efkkpaULBoRFs0bEJNth26SJELKREmV4%2B/hVVQjV1RIXhUTHx1rX1jdktg10hPSV9cQCU1ugepKicXCFEANSC6BAYTMobawBU6xrk60frAJ4z6wCkAOwAQrcyAILrH%2BuHl3e%2Bz1IAES%2BGluUmeb0%2B61I2CIiyYwNB4NeDwBXDm9G4AFZ%2BD4uDpyOhuC51soFktsL8pHxyERtGi5gBrECYuQYrhxfhsZlyXH4wlcfjKEByWl4tHkOCwFAYNiJBgxSjUGVyxixVCoYymXy%2BHhyOj0IgxIUQSJ08iREJ1S7cakW1ikS4AeUiukqoupMo4wkdTHo1rF5BwkQ8wBcEnoQt4/BwbBMwEkAcI0KqADdsJH8dgNJUPIabfw1tg2fj6ARIqQrW4cGaiKQCFyo%2BQ06RIilsADsLHTKXTHS5jRjMBlAA1AjYADujsSzHz8mEYgknFkc6Uqk0Zv0hk15ksRjLQsgc3QiXsQkjAFpHVJ%2BOhm3WcAeIHNWqefBBnMMfDx/EEJsVSsyuSpK%2Bn4GP4yTAek3T/n0mLWEWbrVGMoHfvBtivh09TQb0sRwfsnQoaMnTYVMuHPmSywGOiWI4ma/InAAHAAbOeTFxOsGpxus2oAHQ8DxMjrBA%2BDEGQlI8DM/CijoMxzEg2AsDgsRPuQTIskY3AcuQvI3twgrCjSfbUVwvicsyTE8VImIAJxsdZ36%2BAxMj3DI/g6QSemGWKcySgg8AQNKWB4IQJAUFQtDyqwHCzoI87iJIy6xau6haAG%2Bj%2BNuICkB4%2BxoYhjjvkwrjuE0OS/kUOGARB%2BQZCVIxATVJEAXhCFtEwmENHVzR5W1HVNbBAwEV1OT4Vhf6VZi5GLJRXC3L4Ui3JijwhD22AQKEzYzItAIvK8iQeJEmzwlsEBaL8TEEFI/hXCA51sLm5xXTc56grgj2%2BHcTy7ZC3y/P8QKHCCYLfZ8gO7Si4psti2l0dwABKOUbKS00UnNV1SUZ5DyYpfQqWprKaWZ6nufy%2Bkin2EqIAFaDoLK8phUqtMqn0pA8NZcR6gwhqkMapoBnaVqzgLDrOq6dizp6zBED6fpmkGIZhvQEazjGcYJviSaIWmGb8FmOZ5o2hbFvwpblpWQU1nWDbUs2raqB2XbxiEoDeQIg4jmOk7Tri1JJQuCV6goKgpRuOSZdluWm4%2BR4nukF7EseRDnowab0KCALXgSd4EA%2B/lRZw2AhekTYSB4Ky%2BK5HKyT1r5OEVKEBEV/WxIY1UgcNrd5K%2BzcGDXSFDVkX59%2B0Yw94Yo2dYPvcT2PU3kn4xnQyT3BQojqDrPx7MCUJImhZSviSV5MlyQpSnUIy3IaeytEBqT1gGdJ9JX6Z5BcnEUg8WxMhWTIDH3PcbNfDWQYjDW%2BnlH7VyZLqAmXBM7LwFEfJ%2BzZUiODiEAA>

[godbolt_cell]: <https://godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(filename:'1',fontScale:24,fontUsePx:'0',j:2,lang:rust,selection:(endColumn:2,endLineNumber:19,positionColumn:2,positionLineNumber:19,selectionStartColumn:2,selectionStartLineNumber:19,startColumn:2,startLineNumber:19),source:'%23%5Binline(never)%5D%0Apub+fn+foo1(x:+%26i32,+y:+%26mut+i32)+-%3E+i32+%7B%0A++++*y+%2B%3D+*x%3B%0A++++*x%0A%7D%0A%0Ause+std::cell::Cell%3B%0A%0A%23%5Binline(never)%5D%0Apub+fn+foo2(x:+%26Cell%3Ci32%3E,+y:+%26Cell%3Ci32%3E)+-%3E+i32+%7B%0A++++y.set(y.get()+%2B+x.get())%3B%0A++++x.get()%0A%7D%0A%0A%23%5Binline(never)%5D%0Apub+fn+foo3(x:+%26Cell%3Ci32%3E,+y:+%26mut+Cell%3Ci32%3E)+-%3E+i32+%7B%0A++++y.set(y.get()+%2B+x.get())%3B%0A++++x.get()%0A%7D'),l:'5',n:'0',o:'Rust+source+%232',t:'0')),header:(),k:50,l:'4',m:50,n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:r1940,filters:(b:'0',binary:'1',binaryObject:'1',commentOnly:'0',debugCalls:'1',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1',verboseDemangling:'0'),flagsViewOpen:'1',fontScale:30,fontUsePx:'0',j:2,lang:rust,libs:!(),options:'-C+opt-level%3D3',overrides:!((name:edition,value:'2024')),selection:(endColumn:12,endLineNumber:15,positionColumn:12,positionLineNumber:15,selectionStartColumn:12,selectionStartLineNumber:15,startColumn:12,startLineNumber:15),source:2),l:'5',n:'0',o:'+rustc+1.94.0+(Editor+%232)',t:'0')),header:(),k:50,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',m:100,n:'0',o:'',t:'0')),version:4>
