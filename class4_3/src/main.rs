 
trait CalculateArea{
    fn calculat(&self) ;
}

impl CalculateArea for triangle{
    fn calculat(&self) {
       let areas= self.bottom*self.high/2;
       println!("area {}",areas);
    }
}


impl CalculateArea for square{
    fn calculat(&self) {
       let areas= self.length*self.length;
       println!("area {}",areas);
    }
}

struct triangle{
    bottom: u32,
    high: u32
}

struct square{
    length: u32
}

fn getArea<T:CalculateArea>(graphics: &T){
    graphics.calculat();
}

fn main() {
   
    //三角形面积
    let triangle=triangle{bottom: 10,high: 6 };
    getArea(&triangle);

     //正方形面积
     let square=square{length: 5 };
     getArea(&square);
}


 
