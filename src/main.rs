use std::sync::Arc;
use arc_swap::ArcSwap;

fn main() {
    let arc = Arc::new(42);
    let arc_swap = ArcSwap::from(arc);
    arc_swap.swap(Arc::new(50));
}
