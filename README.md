# delegate_test
Test repository for trait delegation support in rust.

Demonstrates how to use the experimental proc macro defined in [delegable-derive](https://github.com/dureuill/delegable-derive) to delegate the implementation of a trait for a type to a field of that type.

# How it works
The general approach of the macro is the following:

* Mark a trait `Tr` that can be derived as `#[delegable]`. This is required to generate a "delegation trait" `TrDelegable` for `Tr` without compiler support.
* Implement the delegation trait `TrDelegable` for the type `T` that whishes to delegate the implementation of `Tr` to a field or method.
* Use `Tr` in code and calls `Tr` methods on `T`.

# Organization of the repository

```
.
├── boss-type # type that delegates to the "worker trait"
├── src # "main"
├── worker-trait # trait whose implementation we delegate
└── worker-type # type that implements the "worker trait"
```

# Main parts

## Work trait

```rust
    use delegable_derive::delegable;

    // Trait to be delegated
    #[delegable]
    pub trait Work
    {
        fn do_work(&self) -> u64;

        fn do_nothing(&self);

        fn change_worker(&mut self);

        fn into_work(self) -> u64;

        fn clone_worker(&self) -> Self;

        fn increment_worker(&mut self, x: u64);
    }
```
## Worker type

```rust
    use worker_trait::work::Work;

    pub struct Worker {
        x: u64,
        y: u64
    }

    impl Work for Worker {
        // ... actual implementation omitted ...
    }
```

## Boss struct

```rust
    pub struct Boss {
        worker : Worker
    }

    impl work::delegate::Work for Boss {
        type Inner = Worker;

        fn inner(&self) -> &Self::Inner { &self.worker }
        fn inner_mut(&mut self) -> &mut Self::Inner { &mut self.worker }
        fn into_inner(self) -> Self::Inner { self.worker }
        fn from_inner(delegate: Self::Inner) -> Self { Boss { worker : delegate }}
    }
```

## The main using Boss

```rust
use worker_trait::work::Work;

// accepts some types that implements Work
fn print_work<T :use worker_trait::work::Work;


fn print_work<T : Work>(worker: &T) {
    println!("Here is my work: {}", worker.do_work());
    worker.do_nothing();
}

fn main() {
    use boss_type::boss::Boss;

    let boss = Boss::new(20, 21);
    print_work(&boss);
    let mut boss = boss;
    boss.change_worker();
    boss.do_nothing();
    boss.increment_worker(12);
    let second_boss = boss.clone_worker();
    println!("Here is my final work: {}", second_boss.into_work());
} Work>(worker: &T) {
    println!("Here is my work: {}", worker.do_work());
    worker.do_nothing();
}

fn main() {
    use boss_type::boss::Boss;

    let boss = Boss::new(20, 21);
    print_work(&boss); // this works
    let mut boss = boss;
    boss.change_worker(); // can call direct method of the trait too
    boss.do_nothing();
    boss.increment_worker(12);
    let second_boss = boss.clone_worker();
    println!("Here is my final work: {}", second_boss.into_work());
}
```

# Current limitations

The current implementation is meant as a quick proof-of-concept. It particular, it exhibits the following limitations:

* Having to mark delegable trait
* No delegation of struct (could be a natural extension with [inherent traits](https://github.com/rust-lang/rfcs/pull/2375).
* Having to implement the delegable trait, which can be more boilerplate than implementing the original trait! This could be alleviated with macros for common cases (e.g., delegating `MyTrait` to a field with a `delegate!(self.x, MyTrait)`).
* The proc macro doesn't properly report errors yet.
* The proc macro doesn't use explicit call syntax with `<self as Trait>::method(...)` everywhere, which may cause problems in some cases.
* I'm unsure if the delegation trait should always expose `into_inner`, or only when required by the trait, or never and fail for traits needing this.
* Currently no support of associated traits or constants, as I didn't look into it.
