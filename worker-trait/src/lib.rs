#![feature(proc_macro)]

extern crate delegable_derive;

pub mod work
{
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

}
