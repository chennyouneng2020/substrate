//module:   Traficlight
//date:     2021/03/29
//author:   chenyouneng
//email:    bbeyondllove@gmail.com

enum Traficlight{
    Red,
    Green,
    Yellow,
}

trait TraficTime{
    fn get_time(&self)->u32;
}

impl TraficTime for Traficlight{
    fn get_time(&self)->u32{
        match self{
            Traficlight::Red=>60,
            Traficlight::Green=>30,
            Traficlight::Yellow=>10,
        }
    }
}

 
fn main(){
    let r1=Traficlight::Red;
    let r2=Traficlight::Green;
    let r3=Traficlight::Yellow;

    println!("{},{},{}",r1.get_time(),r2.get_time(),r3.get_time())
}