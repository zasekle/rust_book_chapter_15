use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    //In Rust, they have smart pointers that offer additional functionality compared to the standard
    // references. The pointers often own the data they point to. String and Vec<T> are both
    // examples of smart pointers.
    using_box_to_point_to_data_on_the_heap();
    treating_smart_pointers_like_regular_references_with_deref_trait();
    running_code_on_cleanup_with_the_drop_trait();
    rc_the_reference_counted_smart_pointer();
    refcell_and_the_interior_mutability_pattern();
    reference_cycles_can_leak_memory();
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

fn rc_the_reference_counted_smart_pointer() {
    //Rc<T> is a reference counted object, so it can have multiple owners. Similar to shared_ptr in
    // c++.

    //Because of the borrow checker in Rust, the Rc<T> object is very useful (although lifetimes
    // are better if possible). As an example the below code will not compile because the String `a`
    // has already been moves. However, if an Rc<T> is used, then it can be set up to work.
    // let a = String::from("String value");
    // let b = a;
    // let c = a;

    let mut a = Rc::new(String::from("String value"));
    //Note that Rc::clone is used instead of a.clone(). This is a convention in Rust to be explicit
    // that it is an Rc<T> object that is being cloned. This is important because clone on Rc<T>
    // does not work the same was it does for most objects. For most object it makes a deep copy of
    // the object, for Rc<T> it increases the reference count instead.
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("a: {} b: {} c: {}", a, b, c);

    //The reference count can be checked with Rc::strong_count. Rc::weak_count has something to do
    // with creating a weak ptr, but that isn't covered yet.
    println!("ref count: {}", Rc::strong_count(&a));

    //A problem with Rc<T> is that all references are immutable. The next section will help when
    // dealing with this issue.
}

fn refcell_and_the_interior_mutability_pattern() {

    //The RefCell<T> allows for mutable elements to be used from an Rc<T> object, even when other
    // immutable references are available. This breaks the rules of the Rust borrow checker and so
    // the RefCell<T> must rely on unsafe code. Note that this does not mean that it is allowed to
    // have mutable references outstanding with other references. It simply means the check is
    // moved to a runtime check instead of a compile time check. So there will not be a compile time
    // error, instead it will panic and exit if the rules are broken.
    //An interesting note here is that just like Rc<T>, RefCell<T> is not for use in multithreaded
    // contexts. Apparently there is another value that is used for that and will be discussed
    // later. I think this is interesting because my first impression was that multithreaded
    // contexts would be the primary use case.

    //The example they give online is the ability to create Mock objects as a possible use case.
    // Instead I will do something that is actually a poor practice to get an example of its use.

    struct Hello {
        string: RefCell<String>,
    }

    trait World {
        fn print(&self);
    }

    impl World for Hello {
        fn print(&self) {
            //Note that on this line, the value is changed even though it is passed as an immutable
            // reference.
            self.string.borrow_mut().push('b');
            println!("string: {}", self.string.borrow());
        }
    }

    let hello = Hello {
        string: RefCell::new(String::from("a"))
    };

    hello.print();

    //RefCell<T> can also work well with Rc<T>. This is because Rc<T> only stores immutable values
    // and RefCell<T> allows for mutability. So an item can be stored in multiple places with Rc<T>
    // and modified with RefCell<T>.
    let my_item = Rc::new(RefCell::new(String::from("Hello world")));

    let ref_two = Rc::clone(&my_item);
    let ref_three = Rc::clone(&my_item);

    println!("No changes: {}", my_item.borrow());

    ref_two.borrow_mut().pop();
    println!("popped: {}", my_item.borrow());

    ref_three.borrow_mut().pop();
    println!("popped: {}", my_item.borrow());

    //Note that the below code will crash during runtime, but not during compile time. This is
    // because as stated above, only a single mutable reference can be outstanding at a time.
    // let first_one = ref_two.borrow_mut();
    // let crash = ref_three.borrow_mut();
}

fn reference_cycles_can_leak_memory() {
    //Rust does not actually guarantee no memory leaks. For example Rc<T> can have items that
    // reference each other and so are never cleaned up. This is actually interesting because from
    // a simplistic view I assumed the borrow checker would take care of memory leaks completely.
    // But it seems to be more of a soft guarantee where leaks are hard but not impossible.

    #[derive(Debug)]
    struct MemLeak {
        hello: Option<Rc<RefCell<MemLeak>>>,
    }

    let first = Rc::new(
        RefCell::new(
            MemLeak {
                hello: None
            }
        )
    );

    let second = Rc::new(
        RefCell::new(
            MemLeak {
                hello: Some(Rc::clone(&first))
            }
        )
    );

    //First now stores second and second stores first. This is a memory leak.
    first.borrow_mut().hello.replace(Rc::clone(&second));

    //This statement will cause a stack overflow and crash because it is an endless loop of printing
    // internals.
    // println!("first: {:?}", first);

    //If Rc<T> is a shared_ptr in c++, then Weak<T> is a weak_ptr. A weak pointer can be extracted
    // from Rc<T> by calling Rc::downgrade. This pointer will not have any effect on whether the
    // value is cleaned up because it does not have an effect on strong_count. Instead it has an
    // effect on weak_count. Note that a weak pointer is a good way to avoid the reference cycles
    // previously discussed. Also note that when upgrading a Weak<T> object, an Rc<T> object is
    // returned and a strong reference now exists.

    let mut weak_fails: Weak<i32>;
    {
        let pointer = Rc::new(5);
        let weak_exists = Rc::downgrade(&pointer);
        weak_fails = Rc::downgrade(&pointer);

        //The weak pointer exists because the Rc variable still exists.
        println!("weak_exists: {:?}", weak_exists.upgrade());
    }
    //The weak pointer no longer exists because the reference counter was cleaned up.
    println!("weak_fails: {:?}", weak_fails.upgrade());

}
