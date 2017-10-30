#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

///! Test that mockers can mock several traits using one mock.
///! In particular, it should work for mocking inherited traits.

extern crate mockers;

use mockers::Scenario;
use mockers::matchers::ANY;

/// Test mocking of inherited trait.
mod inherited_trait {
    use super::*;

    pub trait A {
        fn foo(&self, a: u32);
    }

    pub trait B: A {
        fn bar(&self, b: u32);
    }

    mock!{
        BMock,

        self,
        trait A {
            fn foo(&self, a: u32);
        },

        self,
        trait B {
            fn bar(&self, b: u32);
        }
    }

    #[test]
    fn test() {
        let scenario = Scenario::new();
        let mock = scenario.create_mock::<BMock>();

        scenario.expect(mock.foo_call(ANY).and_return_default().times(1));
        scenario.expect(mock.bar_call(ANY).and_return_default().times(1));

        mock.foo(3);
        mock.bar(4);
    }
}

/// Test creating mock for several independent traits at once.
mod multi_trait {
    use super::*;

    pub trait A {
        fn foo(&self, a: u32);
    }

    pub trait B {
        fn bar(&self, b: u32);
    }

    mock!{
        ABMock,

        self,
        trait A {
            fn foo(&self, a: u32);
        },

        self,
        trait B {
            fn bar(&self, b: u32);
        }
    }

    fn accept_cd<T: A+B>(t: T) {
        t.foo(1);
        t.bar(2);
    }

    #[test]
    fn test() {
        let scenario = Scenario::new();
        let mock = scenario.create_mock::<ABMock>();

        scenario.expect(mock.foo_call(ANY).and_return_default().times(1));
        scenario.expect(mock.bar_call(ANY).and_return_default().times(1));

        accept_cd(mock);
    }
}

/// Test that it is possible to specify parent trait when using `mock!`.
/// It is currently not used, but may be used in the future, so syntax
/// should be allowed.
mod inherited_trait_with_specified_parent {
    pub trait A {
        fn foo(&self, a: u32);
    }

    pub trait B: A {
        fn bar(&self, b: u32);
    }

    mock!{
        BMock,

        self,
        trait A {
            fn foo(&self, a: u32);
        },

        self,
        trait B: A {
            fn bar(&self, b: u32);
        }
    }
}
