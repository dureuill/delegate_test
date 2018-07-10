extern crate worker_trait;
extern crate worker_type;

pub mod boss {
    use worker_type::worker::Worker;
    use worker_trait::work;

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

    impl Boss {
        pub fn new(x : u64, y : u64) -> Self {
            Boss { worker : Worker::new(x, y) }
        }

        pub fn do_nothing(&self) {
            println!("Boss doing nothing...")
        }
    }
}
