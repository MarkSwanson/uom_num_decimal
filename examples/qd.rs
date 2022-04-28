//! Example showing how to use the pre-built SI system.

use uom::fmt::DisplayStyle::Abbreviation;
//use uom::si::f32::*;
use uom::si::length::{centimeter, kilometer, meter, Length};
use uom::si::time::{second, Time};
use uom::si::velocity::{kilometer_per_second, meter_per_second};

use qd::qd;

fn main() {
    // Setup length and time quantities using different units.
    let l1 = Length::new::<meter>(qd!(15.0));
    let l2 = Length::new::<centimeter>(qd!(10.0));
    let t1 = Time::new::<second>(qd!(50.0));
    let v1 = l1 / t1;
    //let error = l1 + t1; // error[E0308]: mismatched types

    // Setup re-usable format arguments.
    let m = Length::format_args(meter, Abbreviation);
    let cm = Length::format_args(centimeter, Abbreviation);
    let s = Time::format_args(second, Abbreviation);

    // Print results of simple formulas using different output units.
    println!("{} + {} = {}", m.with(l1), cm.with(l2), m.with(l1 + l2));
    println!(
        "{} + {} = {}",
        m.with(l1),
        cm.with(l2),
        (l1 + l2).into_format_args(kilometer, Abbreviation)
    );
    println!(
        "{} / {} = {}",
        m.with(l1),
        s.with(t1),
        v1.into_format_args(meter_per_second, Abbreviation)
    );
    println!(
        "{} / {} = {}",
        m.with(l1),
        s.with(t1),
        v1.into_format_args(kilometer_per_second, Abbreviation)
    );
}
