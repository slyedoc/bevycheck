#![allow(unused)]
use bevy::prelude::*;

struct Foo;

#[bevycheck::system]
fn system(
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
    _: Res<Foo>,
) {
}

fn main() {
    system.system();
}
