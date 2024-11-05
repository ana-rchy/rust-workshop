fn tldr() {
    // hello world
    println!("livvy dunne baby gronk rizzler sigma skibidi toilet");

    // variables immutable by default
    let a = 5;
    // can be made mutable with `mut`
    let mut b = 3;
    b = 1;

    // types are implicit, but still hard-typed
    // can be made explicit like this
    let c: u32 = 100;

    // can initialize without assigning
    let d: i32;

    // constants are also here
    // the value that goes into them has to be computable at compile time
    // you also need to specify their type explicitly
    const NOT_A_NUMBER: i64 = -50;

    // previous variables can be shadowed
    // i.e. the variable name is reused for a new variable
    // differs from using `mut`, you can shadow a non-mut variable
    let a = 10;

    // number types:
    // signed: i8, i16, i32, i64, i128, isize
    // unsigned: u8, u16, u32, u64, u128, usize
    // floating: f32, f64
    // "size" refers to the size of the architecture (32-bit, 64-bit)
    let e: i128 = -99999999999;
    let f: usize = 127721;
    let g: f32 = 3.1415926;

    // operations: +, -, *, /, % (remainder, NOT modulo)
    // assignment shorthands: +=, -=, *=, /=, %=
    let mut h = b * c;
    h %= 5;

    // boolean type. shrimple
    let i = true;
    let i: bool = false;

    // char type, supports unicode
    let j = 'q';
    let j = 'þ';
    let j = '🫃';

    // array type, its fixed-length, all elements must be the same type
    let k = [421, 422, 423, 424, 425];
    // can explicitly declare the type and length
    let l: [u8; 3] = [255, 127, 0];
    let l: [i8; 7];
    // access array elements with index
    let m = k[0];

    // tuple type, fixed-length, each element has its own type
    let n = (420, 'ඞ', "femboys 🤤");
    // explicit type declaration
    let o: (isize, char) = (-727, '🦞');

    // function usage
    some_function('p');
    let p = square(12);

    // scopes can be used on their own
    {
        let q = 0;
        println!("waow new scope");
    }
    // q is invalid after the scope ends

    // can assign to variables using scopes
    let q = {
        let x = 10;
        let y = 20;

        square(x) * square(y)
    };
}

// parameters need an explicit type
fn some_function(c: char) {
    // can format variables into println! with {}
    println!("waow {c}");
    println!("{}{}{}", c, c, c);
}

// return type given after the ->
fn square(x: i64) -> i64 {
    // ending a line with no semicolon returns the value of that line
    // can also use a statement such as `return x * x;` if needed for early return
    x * x
}
