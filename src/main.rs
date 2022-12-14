fn main() {
    
    // "For" test
   for first_var in -10..10   {
        println!("{}", first_var);
        if first_var % 3 == 0 && first_var % 9 == 0 {
            println!("Stopped!");
            break;
        }
    }

    // "While" test  
    let mut second_var = 0;
    while second_var <= 70 {
        println!("{}", second_var);
        if second_var == 70 {
            println!("Stopped!");
            break;
        }
        second_var += 1;
    }

    // "Loop" test
    let mut third_var = 0;

    loop {
        println!("{}", third_var);
        if third_var == 15 {
            println!("Stopped!");
            break;
        }
        third_var+= 1;
    }

    
}