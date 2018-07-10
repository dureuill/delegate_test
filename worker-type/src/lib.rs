extern crate worker_trait;

pub mod worker {

    use worker_trait::work::Work;

    pub struct Worker {
        x: u64,
        y: u64
    }

    impl Work for Worker {
        fn do_work(&self) -> u64 {
            self.x + self.y
        }

        fn change_worker(&mut self) {
            if self.x <= self.y {
                self.x += 1;
            }
        }

        fn into_work(self) -> u64 {
            self.do_work()
        }

        fn clone_worker(&self) -> Self {
            Worker {x : self.x, y : self.y }
        }

        fn do_nothing(&self) {
            // do nothing
            println!("Worker doing nothing...")
        }

        fn increment_worker(&mut self, x : u64) {
            self.x += x;
        }
    }

    impl Worker {
        pub fn new(x: u64, y : u64) -> Self {
            Worker { x, y }
        }
    }
}
