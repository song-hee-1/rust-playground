// fn main() {
//     let number = 3;

    // 조건식은 반드시 bool이어야 함. 해당 코드는 컴파일 에러 발생
    // if number {
    //     println!("number was three");
    // }

    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {number}");

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false")
    // }

//     println!("Hello, world!");
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result is {result}");
//
// }

// 루플 라벨이 없으면 break, continue는 해당 지점의 바깥쪽 루프에 적용
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{number}!");
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

fn main() {
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //
    //     index += 1;
    // }

    // for element in a {
    //     println!("the value is: {element}");
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}