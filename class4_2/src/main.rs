 
fn Sum(list:&[u32])->u32{
    let mut ret=0_u32;
    for i in list.iter() {
        ret+=i;
    }
    ret
}
 
fn main(){
     let a=[12,33,20,333,84];
    println!("{}",Sum(&a));
}