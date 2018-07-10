extern crate worker_type;
extern crate boss_type;
extern crate worker_trait;

use worker_trait::work::Work;


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
}
