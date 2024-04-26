use std::time::SystemTime;
fn main() {

    for k in 1..=50 {
        let before = SystemTime::now();
        println!("the fibbonacci sequence for {} is {}", k, fib(k));
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("Time it took: {:?}", difference);
    }

}
fn fib(k:u32) -> u128{
    if k == 0{  
        return 0;
    }else if k ==1{
        return 1;
    }else{
        return fib(k-2)+fib(k-1);
    }
}
