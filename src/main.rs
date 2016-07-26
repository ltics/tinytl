static mut N: u8 = 97;

fn main() {
    unsafe {
        println!("{}", (N as char).to_string());
        //let a: &'static str = &(N as char).to_string();
        N += 1;
        println!("{}", (N as char).to_string());
    }
}
