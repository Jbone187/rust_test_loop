fn main() {
    looping();
}

fn looping() {
    let food = ["d", "e", "H", "1", "Hamburger"];

    for x in food {
        match x {
            "1" => println!("Have some bread"),
            "Apple" => println!("Have an Apple"),
            "Hamburger" => println!("Have a Burger"),
            _ => println!("No Results"),
        }

        println!("{}", x)
    }
}
