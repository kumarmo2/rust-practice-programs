use async_trait::async_trait;
use std::str::FromStr;

trait Printable {
    fn print(&self) {}
}

/*
*
NOTE:
below declaration works
    ```fn print_str(x: &dyn AsRef<str>)```


but the below one does not
``` fn print_str(x: dyn AsRef<str>)```

Only difference is `dyn` and `&dyn`.
This is because `dyn Trait` is `!Sized`. whereas
`&dyn Trait` is a reference(to be exact, a fat/wide pointer) which is `Sized`


Another way this could work is to use a type which can work with `?Size` i.e (may not be sized)
like a `Box`.
Eg, if you re-write like below, it will work.

```fn print_box_dyn_str(x: Box<dyn AsRef<str>>)```


Now why and how `Box` can work with `?Size` is a topic for another day. Mainly because
I myself don't know the reason.

*/

fn print_str(x: &dyn AsRef<str>) {
    println!("x: {}", x.as_ref());
    print_str_lib(x)
}

fn print_box_dyn_str(x: Box<dyn AsRef<str>>) {
    println!("x: {}", x.as_ref().as_ref());
}

fn print_strs(_y: &[&dyn AsRef<str>]) {}

fn print_str_lib<T>(x: T)
where
    T: AsRef<str>,
{
    println!("x: {}", x.as_ref())
}

trait UseLess {
    fn useless(&self);
}

#[async_trait]
trait AsyncTrait {
    async fn method(&self);
}

impl<T> UseLess for &T
where
    T: UseLess + ?Sized,
{
    fn useless(&self) {
        (*self).useless()
    }
}

fn static_dispatch<T: UseLess>(t: T) {
    t.useless();
}

fn dynamic_dispatch(u: &dyn UseLess) {
    u.useless();
    static_dispatch(u);
}

struct A {}

impl UseLess for A {
    fn useless(&self) {
        println!("A is useless")
    }
}

struct B {}

impl UseLess for B {
    fn useless(&self) {
        println!("B is useless")
    }
}

fn main() {
    println!("Hello, world!");
    print_str(&"sfsdf");
    print_str(&String::from_str("sdfdsdf").unwrap());
    let name = String::from_str("sdfdsdf").unwrap();
    print_str_lib(&name);
    println!("name: {}", name);
    // Extend
    // some_method2(&A {});
    // some_method2(&B {});

    static_dispatch(A {});
    static_dispatch(B {});

    let a = A {};
    static_dispatch(&a);
    dynamic_dispatch(&a);

    let x = Box::new("i am mohit");

    print_box_dyn_str(x);

    let mut y: Vec<&dyn AsRef<str>> = Vec::new();
    y.push(&"strng");
    let z = String::from_str("sdfsfd").unwrap();
    y.push(&z);

    print_strs(&y);
}
