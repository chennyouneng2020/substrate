fn Sum(vec: &Vec<u32>)-> Option<u32> {
    let mut sums: u32=0;
    for i in vec{
       match  sums.checked_add(*i) {
            Some(s) => {sums=sums+s;}
            None => {return None;}  
       };
    }
    Some(sums)
}

fn main(){
    let a=vec![12,33,20,333,84];
    println!("{:?}",Sum(&a));

}
