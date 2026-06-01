use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64 { // -> 뒤에가 return type 정의
    // TODO: implement the conversion formula.
    celsius * 9.0 / 5.0 + 32.0                  // return이 따로 없이 바로 return
                                                // 세미 콜론이 없음.
                                                // --> return으로 바로 넘겨줌
}

fn main() { // function은 fn
    let mut input = String::new();              // string 변수 새로 하나 정의
                                                // mut : 변경이 가능하다 표시
                                                // --> input 변수는 heap에서 변경이 가능한 변수다.
                                                // let은 특정 변수를 만들겠다. --> type은 rust가 알아서
    println!("Enter Celsius:"); 
    // !는 rust macro --> 기능을 좀 더 증강시켜놓은 느낌
    // 마치 python에서 print 사용할 수 있는 느낌으로

    io::stdin()
        .read_line(&mut input)                  // 변경이 가능한 input
                                                // read_line : 한 줄을 읽어 string에 저장 메서드
        .expect("failed to read line");         // expect : rust에서 사용하는 기본적인 errorhandler
                                                // --> error가 났을 때 뒤에 print 수행

    let celsius: f64 = input
        .trim()                                 // .trim : 공백 제거
        .parse()                                // .parse : 문자열을 다른 타입으로 변환하는 메서드
        .expect("please enter a number");

    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius:.1}C = {fahrenheit:.1}F");

    let value_a = "36.5".parse::<f64>().expect("number"); // ::<> :turbofish 문법
                                                          // --> 어떤 타입으로 변환할 지 명시하는 방법 
    let value_b = "36.5"
        .parse()
        .expect("please enter a number ");
}

#[cfg(test)]                                    // '얘는 cargo test 돌릴 때 만들어 주세요' 란 의미
                                                // test 전용
mod tests {
    use super::*;

    #[test]
    fn converts_freezing_point() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn converts_boiling_point() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }
}
