fn main() {
    let s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로
                                        // 이동시킵니다

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데,
                                        // 이 함수 또한 자신의 반환 값을 s3로
                                        // 이동시킵니다
} // 여기서 s3가 스코프 밖으로 벗어나면서 버려집니다. s2는 이동되어서 아무 일도
  // 일어나지 않습니다. s1은 스코프 밖으로 벗어나고 버려집니다.

fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
                                             // 자신의 호출자 함수로 이동시킬
                                             // 것입니다

    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다

    some_string                              // some_string이 반환되고
                                             // 호출자 함수 쪽으로
                                             // 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로
                                                      // 들어옵니다

    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}


// fn main() {
//     let s = String::from("hello");
//     takes_ownership(s); // s의 값이 함수 안으로 이동합니다
//     // 이 시점에서 s는 더 이상 유효하지 않습니다
    
//     let x = 5;
//     makes_copy(x); // x는 Copy 트레이트를 가지므로 계속 유효합니다
//     // 이 시점에서도 x를 사용할 수 있습니다

//     let y = String::from("world");


//     println!("{}", x);
//     println!("{}", y);

//     takes_ownership(y);

// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } // 함수가 끝나면 some_string이 드롭됩니다

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// } // 함수가 끝나도 some_integer는 스택에 있으므로 별다른 일이 없습니다