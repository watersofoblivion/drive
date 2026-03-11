mod example;
mod lat_lon;

use drive::cli::App;

// pub use lat_lon::LatLon;

// use drive_macros::unit_test_assertions;

// #[unit_test_assertions]
// struct Foo<'a> {
//     foo: &'a str,
//     bar: i32,
//     baz: lat_lon::LatLon,
//     quux: &'a lat_lon::LatLon,
// }

// #[unit_test_assertions]
// struct Bar(u32, u32, u32);

fn main() {
    App::run();
}

drive_macros::test_macro! {
    @foo(bar, baz = "quux");
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_it() {
//         let ll1 = lat_lon::LatLon::new(1.2, 3.4);
//         let ll2 = lat_lon::LatLon::new(2.1, 4.3);
//         let foo = Foo {
//             foo: "a string",
//             bar: 42,
//             baz: ll1.clone(),
//             quux: &ll2,
//         };

//         foo.assert_foo("a string");
//         foo.assert_bar(&42);
//         foo.assert_baz(&ll1);
//         foo.assert_quux(&ll2);
//     }
// }
