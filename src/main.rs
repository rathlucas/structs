// Include to print structs and arrays
#[derive(Debug)]

// Declaring a struct
struct User {
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        is_active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

fn main() {
    // Instantiating a struct
    let user1 = User {
        is_active: true,
        email: String::from("lucin189@gmail.com"),
        username: String::from("rathlucas"),
        sign_in_count: 1,
    };

    // Acessing properties from the struct
    println!("{}", user1.email);

    // Mutable structs
    let mut user2 = User {
        is_active: true,
        email: String::from("lucin290@gmail.com"),
        username: String::from("frankydxd"),
        sign_in_count: 1,
    };

    user2.email = String::from("lucascsilva7@hotmail.com");
    println!("{}", user2.email);

    let user3 = build_user(String::from("testmail@mail.com"), String::from("test user"));
    println!("{:?}", user3);

    // Struct update syntax
    let user4 = User {
        email: String::from("jureminha@mail.com"),
        username: String::from("jureminha furacão"),
        ..user1
    };

    // Remember String implements the move trait, removing the variable from memory
    // If you want to copy the data, create new String in the struct instance
    println!("{:?}", user4);
    println!("{:?}", user1)
}
