fn main() {
    let mut current = faultcrate::ATOMIC_PTR.load(std::sync::atomic::Ordering::Acquire);
}
