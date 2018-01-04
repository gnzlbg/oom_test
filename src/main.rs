// One GigaByte in Bytes:
const Gb: usize = 1_073_741_824;

fn main() {
    let total = 128. * (Gb as f64);
    println!("trying to allocate {} GigaBytes", total / Gb as f64);
    let mut vec: Vec<u8> = Vec::with_capacity(total as usize);
    println!("reserve succeeded");
    for _ in 0..total as usize {
        vec.push(0);
    }
    println!("push succeeded");
}
