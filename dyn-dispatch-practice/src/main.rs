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
}

fn print_box_dyn_str(x: Box<dyn AsRef<str>>) {
    println!("x: {}", x.as_ref().as_ref());
}

fn print_strs(_y: &[&dyn AsRef<str>]) {}

fn main() {
    println!("Hello, world!");
    print_str(&"sfsdf");
    print_str(&String::from_str("sdfdsdf").unwrap());

    let x = Box::new("i am mohit");

    print_box_dyn_str(x);

    let mut y: Vec<&dyn AsRef<str>> = Vec::new();
    y.push(&"strng");
    let z = String::from_str("sdfsfd").unwrap();
    y.push(&z);

    print_strs(&y);
}
