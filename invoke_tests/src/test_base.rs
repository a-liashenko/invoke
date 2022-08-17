#[cfg(test)]
mod tests {
    use invoke::{
        invoke, FnId, Invoke, InvokeError, InvokeExt, InvokeMeta, InvokeMetaExt, InvokeMut,
        InvokeMutExt,
    };
    use std::cell::RefCell;

    struct TestWarn;
    #[invoke]
    impl TestWarn {}

    #[derive(Default)]
    struct Test {
        test_static: RefCell<bool>,
        test_one_arg: RefCell<u32>,
        test_two_args: RefCell<(u32, String)>,
        test_mut: u32,
    }

    #[invoke]
    impl Test {
        #[invoke_fn]
        fn test_static(&self) {
            *self.test_static.borrow_mut() = true;
        }

        #[invoke_fn]
        fn test_one_arg(&self, arg: &u32) {
            *self.test_one_arg.borrow_mut() = *arg;
        }

        #[invoke_fn]
        fn test_two_args(&self, one: &u32, #[allow(clippy::ptr_arg)] two: &String) {
            *self.test_two_args.borrow_mut() = (*one, two.clone());
        }

        #[invoke_fn]
        fn test_mut(&mut self, arg: &u32) {
            self.test_mut = *arg;
        }
    }

    #[test]
    fn test_safe() {
        let mut test = Test::default();

        test.invoke(&Test::TEST_STATIC_ID, None)
            .expect("Failed to invoke static");
        assert!(*test.test_static.borrow());

        let one_arg = 10_u32;
        test.invoke(&Test::TEST_ONE_ARG_ID, Some(&one_arg))
            .expect("Failed to invoke one arg");
        assert_eq!(*test.test_one_arg.borrow(), one_arg);

        let two_args = (10, "Test me".to_owned());
        test.invoke(&Test::TEST_TWO_ARGS_ID, Some(&two_args))
            .expect("Failed to invoke two args");
        assert_eq!(test.test_two_args.borrow().0, two_args.0);
        assert_eq!(test.test_two_args.borrow().1, two_args.1);

        test.invoke_mut(&Test::TEST_MUT_ID, Some(&one_arg))
            .expect("Failed to invoke test_mut");
        assert_eq!(test.test_mut, one_arg);
    }

    #[test]
    fn test_unsafe() {
        unsafe {
            let mut test = Test::default();

            test.invoke_raw::<()>(&Test::TEST_STATIC_ID, None)
                .expect("Failed to invoke static");
            assert!(*test.test_static.borrow());

            let one_arg = 10_u32;
            test.invoke_raw(&Test::TEST_ONE_ARG_ID, Some(&one_arg))
                .expect("Failed to invoke one arg");
            assert_eq!(*test.test_one_arg.borrow(), one_arg);

            let two_args = (10, "Test me".to_owned());
            test.invoke_raw(&Test::TEST_TWO_ARGS_ID, Some(&two_args))
                .expect("Failed to invoke two args");
            assert_eq!(test.test_two_args.borrow().0, two_args.0);
            assert_eq!(test.test_two_args.borrow().1, two_args.1);

            test.invoke_mut_raw(&Test::TEST_MUT_ID, Some(&one_arg))
                .expect("Failed to invoke test_mut");
            assert_eq!(test.test_mut, one_arg);
        }
    }

    #[test]
    fn test_invalid_arg() {
        let test = Test::default();

        let val = 10_u16;
        let err = test.invoke(&Test::TEST_ONE_ARG_ID, Some(&val));
        assert!(err.is_err());
        assert!(matches!(err.unwrap_err(), InvokeError::BadArgs));
    }

    #[test]
    fn test_bad_method() {
        let invalid: invoke::FnId = FnId([0; 16]);
        let test = Test::default();
        let val = 10_u16;

        let err = test.invoke(&invalid, Some(&val));
        assert!(err.is_err());
        assert!(matches!(err.unwrap_err(), InvokeError::UnknownMethod));

        unsafe {
            let err = test.invoke_raw(&invalid, Some(&val));
            assert!(err.is_err());
            assert!(matches!(err.unwrap_err(), InvokeError::UnknownMethod));
        }
    }

    #[test]
    fn test_none_args() {
        let test = Test::default();

        let err = test.invoke(&Test::TEST_ONE_ARG_ID, None);
        assert!(err.is_err());
        assert!(matches!(err.unwrap_err(), InvokeError::NoneArgs));
    }

    #[test]
    fn test_meta() {
        let method = Test::get_method_id("Test::test_static");
        assert_eq!(method, Some(&Test::TEST_STATIC_ID));

        let method = Test::get_method_id_raw(&Test::test_static);
        assert_eq!(method, Some(&Test::TEST_STATIC_ID));

        let name = Test::get_method_name(&Test::TEST_STATIC_ID);
        assert_eq!(name, Some("Test::test_static"));
    }
}
