A (growing) list of things need to be figured out or solved if we'd like to adopt
Rust with LKMM.

(A large part of the list comes from Paul Mckenney)

* Add more items into this list from Paul's blog and other discussions in LKML.

* How does Rust do control dependencies.

* How does Rust do address/data dependencies.

* How can Rust best interface to sequence locking so as to cover the full range
  of use cases?
  - Valuable discussion at [Paul's blog][1] and [LKML][2]

* How can Rust best interface to RCU so as to cover the full range of use
  cases?

* Convert litmus tests in tools/memory-model/litmus-tests into Rust
  - Fully Rust versions.
  - Half Rust and Half C versions.

[1]: https://paulmck.livejournal.com/63957.html
[2]: https://lore.kernel.org/rust-for-linux/CANpmjNMqfVN=CfbxpMb9o=045thHLewB_eTOPFwT67gkO-vOuw@mail.gmail.com/T/#m3de979695d6cc3663f5028da89f9dc925ccadc4a
