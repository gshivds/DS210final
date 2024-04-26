fn main() {
    let mut F:[u128; 101] = [0;101];
    F[1]=1;
    for i in 2..F.len(){
        F[i]=F[i-2]+F[i-1];
    }


    

    println!("{:?}",F);
}
