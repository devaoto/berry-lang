mod lexer;
use lexer::tokenize;

fn main() {
    let input =
        r#"fn add(a, b) {
        return a + b;
    };
    
    fn isEven(a) {
        whether (a % 2 == 0) {
            return true;
        } otherwise {
         return false;
        };
    }
    
    fn isOdd(a) {
        return !isEven(a);
    }

    print("Is 2 odd", isOdd(2));
    print("Is 2 even", isEven(2));
    "#;

    println!("{:?}", tokenize(input));
}
