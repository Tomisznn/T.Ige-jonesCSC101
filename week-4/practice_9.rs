fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A: {}",a );
    println!("Value pf B: {}",b );

    let mut _res:i32;

    let res = a>b;
    println!("A greater than B: {}",res );

    let res = a<b ;
    println!("A lesser than B: {}",res );

    let res = a>=b ;
    println!("A greater than or equal to B: {}",res );

    let res = a<=b ;
    println!("A lesser than or equal to B: {}",res );

    let res = a==b;
    println!("A is equal to B: {}", res);

    let res = a!=b;
    println!("A is not equal to B: {}",res );
}
