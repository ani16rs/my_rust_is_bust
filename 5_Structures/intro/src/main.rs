struct User {
    username: String,
    email: String,
    sign_in_account: u32,
    active: bool,
}


fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_account: 1,
    }

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");     // entire struct must be mutable, not just one field

    user3 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );



    // CREATING INSTANCES FROM OTHER INSTANCES WITH STRUCT UPDATE SYNTAX
    let user4 {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    }

    let user5 {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    }                                                   // user 4 and 5 are same



    // USING TUPLE STRUCTS WITHOUT NAMED FIELDS TO CREATE DIFFERENT TYPES
    struct Color(i32, i32, i32);            // tuple structs don't have field names, only types
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);



    // UNIT LIKE STRUCTS WITHOUT ANY FIELDS
    // they don't have any fields
    // behave similiar to the unit type - ()
    // useful when implementing a trait on a type but have no data to store in the type itself
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,                 
        sign_in_count: 1,
    }
}
