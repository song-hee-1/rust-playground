use std::io;



fn main() {
    // let y = {
    //     let x = 3;
    //     x + 1  // 세미 콜론을 사용하면 표현식이 구문이 된다. 이 경우에는 세미 콜론을 사용하지 않아야 바인딩 됨.
    // };

    // println!("The value of y is: {y}");

    // let x = five();  // let x = 5;와 동일

    let x = plus_one(5);

    println!("The value of x is {x}");


}

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn five() -> i32 {
    5 // 반환하는 값에 대한 표현식이기 때문에
}

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn another_function(x: i32) {
//     println!("Another function.")
// }

