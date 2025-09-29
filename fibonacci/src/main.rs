const N: u32 = 20;

fn main() {
    for i in {
        let mut arr = [0; N as usize];
        arr[1] = 1;
        for i in 2..N {
            arr[i as usize] = arr[(i - 1) as usize] + arr[(i - 2) as usize];
        }
        arr
    } {
        print!("{} ", i);
    }
}
