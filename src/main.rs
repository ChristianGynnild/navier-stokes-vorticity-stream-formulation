use faer::{Mat, Scale, mat};
mod rendering;


fn main() {







    let X_LENGTH = 128;
    let Y_LENGTH = 128;

    let vorticity = Mat::from_fn(X_LENGTH, Y_LENGTH, |i, j| (i + j) as f64);
    

    rendering::run();
    
    println!("Hello, world!");


}



