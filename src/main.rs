mod load_balancer;
use load_balancer::{LoadBalancer, RoundRobin, StickyRoundRobin};

fn main() {
    let mut bal: StickyRoundRobin<i32> = StickyRoundRobin::new(2);

    // adding 5 6 7 to balancer
    bal.add(5);
    bal.add(6);
    bal.add(7);

    bal.pick(); // returns 5
    bal.pick(); // returns 5
    bal.pick(); // returns 6
    bal.pick(); // returns 6

    // remove element with value equal to 6
    bal.remove(6);

    // returns all values
    bal.all()
}
