// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use name_variant::NamedVariant;

#[derive(NamedVariant)]
enum TestEnum {
    A,
    B(),
    C(i32, i32),
    D { _name: String, _age: i32 },
    VariantTest,
}

#[test]
fn smoke_test() {
    let x = TestEnum::C(1, 2);
    assert_eq!(x.variant_name(), "C");

    let x = TestEnum::A;
    assert_eq!(x.variant_name(), "A");

    let x = TestEnum::B();
    assert_eq!(x.variant_name(), "B");

    let x = TestEnum::D {
        _name: "Jane Doe".into(),
        _age: 30,
    };
    assert_eq!(x.variant_name(), "D");

    let x = TestEnum::VariantTest;
    assert_eq!(x.variant_name(), "VariantTest");
}
