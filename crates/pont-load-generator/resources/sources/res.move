module self::resource_test {
    struct Test has store {
        vec: vector<u64>,
    }

    struct TestTest has key {
        test: Test,
    }

    entry public fun init(acc: &signer) {
        let i = 0;
        let v: vector<u64> = std::vector::empty();

        while (i < 1000) {
            std::vector::push_back(&mut v, i);
            i = i + 1;
        };

        let t = Test { vec: v };
        move_to(acc, TestTest { test: t });
    }

    entry public fun get_resource() acquires TestTest {
        let _v = borrow_global<TestTest>(@self);
    }
}