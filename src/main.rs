fn main(){
    let result = add(20,54); 
    println!("result : {}", result)
}

fn add(x:i64 , y:i64) -> i64{
    let res = x + y;
    res
}