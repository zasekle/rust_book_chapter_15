use std::ops::Deref;

fn main() {
    //In Rust, they have smart pointers that offer additional functionality compared to the standard
    // references. The pointers often own the data they point to. String and Vec<T> are both
    // examples of smart pointers.
    using_box_to_point_to_data_on_the_heap();
    treating_smart_pointers_like_regular_references_with_deref_trait();
    running_code_on_cleanup_with_the_drop_trait();
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
        Null,
    }

    let my_hello = HelloEnum::Hello(
        Box::new(
            HelloEnum::Null
        )
    );

    println!("my_hello: {:?}", my_hello);
}

fn treating_smart_pointers_like_regular_references_with_deref_trait() {
    //The Deref trait allows something similar to overloading the dereference operator `*`. This
    // allows a smart pointer to be treated like a regular reference. The Box<T> struct is an
    // example of something that implements Deref. This allows for the below code to compile and
    // display `y: 5`.
    let x = 5;
    let y = Box::new(x);

    println!("x: {} y: {}", x, *y);

    struct CustomBox<T>(T);

    impl<T> CustomBox<T> {
        fn new(x: T) -> CustomBox<T> {
            CustomBox(x)
        }
    }

    //This is how to implement the Deref trait and make a custom smart pointer.
    impl<T> Deref for CustomBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let custom_box = CustomBox::new(5);

    //The `*` operator seems to be syntactic sugar for calling *(custom_box.deref()). The reason
    // that it calls a .deref() first is because the Deref trait itself returns a reference. So
    // it calls .deref() which returns the reference, then calls the `*` operator which dereferences
    // that returned reference.
    println!("*custom_box: {} *(custom_box.deref()): {}", *custom_box, *(custom_box.deref()));

    fn hello_world(str: &str) {
        println!("str: {str}");
    }

    //Deref coercion is a thing in Rust. This means that it changes the type automatically depending
    // on the situation. The most straightforward situation is when de-referencing String into &str.
    // Technically &String should be the type returned. However, there is an implementation of Deref
    // on String that returns the string slice instead.
    hello_world(&String::from("My_string"));

    //It is also worth noting that Rust will run Deref::deref as many times as it has to to get
    // a reference to match the parameter's type. This is all resolved at compile time as well
    // meaning there is no performance penalty for using deref coercion.

    //In order to override `*` operator on mutable references, the DerefMut trait must be used.
}

fn running_code_on_cleanup_with_the_drop_trait() {
    //Drop lets me customize what happens when a value is about to go out of scope. This seems to be
    // similar to a deconstructor in c++. Just like in c++, this is often used to deallocate memory
    // for the object.

    #[derive(Debug)]
    struct Square {
        side: isize,
    }

    impl Drop for Square {
        fn drop(&mut self) {
            println!("Square {:?} dropped", self);
        }
    }

    println!("Before scope");
    {
        //Note that the objects are deallocated in the reverse order they are allocated in. I
        // assume that this is because they are popped from the stack as they are removed.
        let square_one = Square {
            side: 5,
        };

        let square_two = Square {
            side: 1,
        };

        println!("squares created {:?} {:?}", square_one, square_two);
    }
    println!("After scope");

    let square_three = Square {
        side: 2,
    };

    //The drop function cannot be manually called here. Instead Rust provides a function called
    // std::mem::drop that can be used to drop.
    // square_three.drop() //Invalid, will cause a compiler error.

    println!("Before square_three dropped");
    drop(square_three);
    println!("After square_three dropped");

}