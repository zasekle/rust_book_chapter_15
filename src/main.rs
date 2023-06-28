
fn main() {
    //In Rust, they have smart pointers that offer additional functionality compared to the standard
    // references. The pointers often own the data they point to. String and Vec<T> are both
    // examples of smart pointers.
    using_box_to_point_to_data_on_the_heap();
    treating_smart_pointers_like_regular_references_with_deref_trait();
}

fn using_box_to_point_to_data_on_the_heap() {
    //Box<T> is the most straightforward type of smart pointer. It seems to simply store the value
    // on the heap instead of the stack.
    let my_box = Box::new(123);

    //This will still print "my_box: 123", the pointer does not need to be dereferenced here.
    println!("my_box: {}", my_box);

    //One use for a box would be to fix the below situation. This struct is a recursive type.
    // struct Hello {
    //     hello: Hello,
    // }
    //This struct technically has the potential for termination. However, the compiler cannot
    // calculate its size at compile time.
    // struct Hello {
    //     hello: Option<Hello>
    // }
    //The below can be used to solve both problems.
    #[derive(Debug)]
    struct Hello {
        hello: Option<Box<Hello>>,
    }

    let my_hello = Hello {
        hello: Some(
            Box::new(
                Hello {
                    hello: None
                }
            )
        )
    };

    println!("my_hello.hello: {:?}", my_hello.hello);

    //It should be noted that the same thing as above can be accomplished with an enum instead of
    // nesting an Option<T> inside of the struct.
    #[derive(Debug)]
    enum HelloEnum {
        Hello(Box<HelloEnum>),
        Null
    }

    let my_hello = HelloEnum::Hello(
        Box::new(
            HelloEnum::Null
        )
    );

    println!("my_hello: {:?}", my_hello);
}

fn treating_smart_pointers_like_regular_references_with_deref_trait() {

}