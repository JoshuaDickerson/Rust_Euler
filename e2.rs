fn main(){
    let mut ll: i64 = 1;
    let mut last: i64 = 2;
    let mut sum: i64 = 0;
    let mut done = false;

    while !done{
        if last > 4000000{
            done = true;
        }else{
            if last % 2 == 0{
                sum += last;
            }
            let new = last + ll;
            ll = last;
            last = new;
        }
    }

    println!("sum: {}", sum);
}