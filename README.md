# Threadsafe BST

Threadsafe Binary Search Tree written in Rust. Nodes are wrapped by
`Arc<Mutex<>>`.

Implementation based on Chapter 15 of [The Complete Rust Programming Reference Guide][TCRPRG] red-black tree and binary search tree.

Other helpful resources:

- [Of Boxes and Trees - Smart Pointers in Rust][of-boxes]
- [Understanding Rust Through AVL Trees][avl-trees]

## License

[MIT License](https://cnord.mit-license.org/)

[TCRPRG]: https://github.com/PacktPublishing/The-Complete-Rust-Programming-Reference-Guide/
[of-boxes]: https://endler.dev/2017/boxes-and-trees/
[avl-trees]: https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/
