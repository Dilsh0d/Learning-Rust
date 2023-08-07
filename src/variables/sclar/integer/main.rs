fn main(){
    println!("-------- SCALAR NUMBER ------------");
    println!("1. Integers");
    println!("2. Floating-point numbers");
    println!("3. Booleans");
    println!("4. Characters");

    println!();
    println!("Integer Types");

    println!("Length	Signed	Unsigned");
    println!("8-bit	i8	u8");
    println!("16-bit	i16	u16");
    println!("32-bit	i32	u32");
    println!("64-bit	i64	u64");
    println!("128-bit	i128	u128");
    println!("arch	isize	usize");

    let signed = -23;
    let un_signed:u32 = 23;
    println!("Signed : {} ",signed);
    println!("UnSigned : {} ",un_signed);


    const TEXT:&str = "Hello Text!";

    println!("CONSTANT {}", TEXT);

    let _guess:u32 = "42".parse().expect("Not a number!");

    println!("String number parse : {}",_guess)
}