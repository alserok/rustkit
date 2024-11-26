# Rustkit

---

* Load balancer

## Kit for backend development written in Rust

## Load balancer

### Round robin

```rust
mod load_balancer;
use load_balancer::{LoadBalancer, RoundRobin};

fn main() {
    let mut bal: RoundRobin<i32> = RoundRobin::new();

    // adding 5 and 6 to balancer
    bal.add(5);
    bal.add(6);

    bal.pick(); // returns 5
    bal.pick(); // returns 6
    bal.pick(); // returns 5

    // remove element with value equal to 5
    bal.remove(5);

    // returns all values
    bal.all()
}

```

### Sticky Round robin

By default `stick` is equal to 1 (the same as default Round Robin).

```rust
mod load_balancer;
use load_balancer::{LoadBalancer, StickyRoundRobin};

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

```