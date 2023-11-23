#[cfg(not(feature = "thread-safe"))]
#[cfg(test)]
mod single_threaded_tests {
    use hash_cons::{Hc, HcTable};
    use rand::Rng;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[derive(Hash, PartialEq, Eq, Clone)]
    enum BoolExpr {
        Const(bool),
        And(Hc<BoolExpr>, Hc<BoolExpr>),
        Or(Hc<BoolExpr>, Hc<BoolExpr>),
        Not(Hc<BoolExpr>),
    }

    /// Test case for basic hashconsing of a simple constant value.
    #[test]
    fn test_basic_const_hashconsing() {
        let table: HcTable<BoolExpr> = HcTable::new();
        let expr_true = BoolExpr::Const(true);
        let expr_true_v2 = BoolExpr::Const(true);
        let hc_true: Hc<BoolExpr> = table.hashcons(expr_true);
        let hc_true_v2: Hc<BoolExpr> = table.hashcons(expr_true_v2);
        let hc_not_true = table.hashcons(BoolExpr::Not(hc_true.clone()));

        assert!(
            hc_true == hc_true_v2,
            "Identical constants should share the same hashconsed reference."
        );
        assert!(
            hc_true_v2 != hc_not_true,
            "Not True and False are not same values"
        );
    }

    /// Test case for hashconsing complex boolean expressions.
    #[test]
    fn test_complex_expression_hashconsing() {
        let table = HcTable::new();
        let expr_true = BoolExpr::Const(true);
        let expr_false = BoolExpr::Const(false);
        let hc_true = table.hashcons(expr_true);
        let hc_false = table.hashcons(expr_false);
        let expr_and_v1 = BoolExpr::And(hc_true.clone(), hc_false.clone());
        let expr_and_v2 = BoolExpr::And(hc_true, table.hashcons(BoolExpr::Const(false)));
        let hc_and_v1 = table.hashcons(expr_and_v1);
        let hc_and_v2 = table.hashcons(expr_and_v2);

        assert!(
            hc_and_v1 == hc_and_v2,
            "Identical complex expressions should share the same hashconsed reference."
        );
    }

    /// Tests the effectiveness of the cleanup method in HCTable.
    /// This test is only run when the auto-cleanup feature is disabled.
    #[cfg(not(feature = "auto-cleanup"))]
    #[test]
    fn test_manual_cleanup_effectiveness() {
        let table = HcTable::<BoolExpr>::new();

        // hash cons several values
        let hc_true = table.hashcons(BoolExpr::Const(true));
        let hc_false = table.hashcons(BoolExpr::Const(false));
        let hc_and = table.hashcons(BoolExpr::And(hc_true, hc_false));

        // Get the size of the table after interning
        let size_before_cleanup = table.len();

        assert_eq!(
            size_before_cleanup, 3,
            "Table should have 3 items before drop"
        );

        // Drop the references to the interned values
        drop(hc_and);

        // Get the size of the table after interning
        let size_before_cleanup = table.len();

        assert_eq!(
            size_before_cleanup, 3,
            "Table should have 3 items after drop"
        );

        // Call cleanup method on the table
        table.cleanup();

        // Get the size of the table after cleanup
        let size_after_cleanup = table.len();

        // Assert that the size of the table has decreased appropriately
        assert_eq!(
            size_after_cleanup, 0,
            "The table size should decrease after cleanup."
        );
    }

    /// Tests the auto cleanup behavior of HCTable
    /// This test is only run when the auto-cleanup feature is enabled.
    #[cfg(feature = "auto-cleanup")]
    #[test]
    fn test_auto_cleanup() {
        let table = HcTable::<BoolExpr>::new();

        // Intern a value and keep a reference to it
        let interned_expr = table.hashcons(BoolExpr::Const(true));

        // Get the size of the table after interning
        let size_after_interning = table.len();
        assert_eq!(
            size_after_interning, 1,
            "Table should have 1 item after interning"
        );

        // Drop the reference to the interned value
        drop(interned_expr);

        // Get the size of the table after dropping the reference
        let size_after_dropping = table.len();

        // Assert that the table is empty after the value is dropped
        assert_eq!(
            size_after_dropping, 0,
            "Table should be empty after dropping the only reference"
        );
    }

    /// Tests the hash collision scenario in hashconsing.
    #[test]
    fn test_hash_collision() {
        #[derive(PartialEq, Eq)]
        enum BoolExprHC {
            Const(bool),
            Not(Hc<BoolExprHC>),
        }

        impl Hash for BoolExprHC {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match self {
                    BoolExprHC::Const(flag) => {
                        // Hash 1 for true, 0 for false
                        let value = if *flag { 1 } else { 0 };
                        value.hash(state);
                    }
                    BoolExprHC::Not(val) => {
                        let mut temp_hasher = DefaultHasher::new();
                        val.hash(&mut temp_hasher);
                        1.hash(state);
                    }
                }
            }
        }

        let table = HcTable::<BoolExprHC>::new();
        let expr_true = BoolExprHC::Const(true);
        let expr_not_false = BoolExprHC::Not(table.hashcons(BoolExprHC::Const(false)));
        let hc_true = table.hashcons(expr_true);
        let hc_not_false = table.hashcons(expr_not_false);

        let mut hasher = DefaultHasher::new();
        hc_true.hash(&mut hasher);
        let hash_value_hc_true = hasher.finish();

        // Reinitialize the hasher for a new calculation
        hasher = DefaultHasher::new();
        hc_not_false.hash(&mut hasher);
        let hash_value_hc_not_false = hasher.finish();

        assert_eq!(
            hash_value_hc_true, hash_value_hc_not_false,
            "Hash values should be equal"
        );
        assert_eq!(table.len(), 3, "Table should have 3 items");
    }

    /// Tests memory usage efficiency in hash consing.
    #[test]
    fn test_memory_usage_hash_consing() {
        let table = HcTable::<BoolExpr>::new();
        let mut hc_data = Vec::new();
        let mut data = Vec::new();

        for _ in 0..100 {
            // Add repetitive BoolExpr values
            let expr_true = BoolExpr::Const(true);
            let expr_false = BoolExpr::Const(false);
            data.push(expr_true.clone());
            data.push(expr_false.clone());
            hc_data.push(table.hashcons(expr_true));
            hc_data.push(table.hashcons(expr_false));
        }

        assert!(
            table.len() < data.len(),
            "Table length should be way less than the data length"
        );

        assert_eq!(
            table.len(),
            2,
            "Table should only have two values(Const True and Const False)"
        );
    }

    /*
    #[cfg(feature = "auto-cleanup")]
    #[test]
    fn stress_test_hc_table() {
        let table = HcTable::<BoolExpr>::new();
        let mut hc_data = Vec::new();
        let true_bool_expr = BoolExpr::Const(true);
        let false_bool_expr = BoolExpr::Const(false);
        hc_data.push(table.hashcons(true_bool_expr.clone()));
        hc_data.push(table.hashcons(false_bool_expr.clone()));
        hc_data.push(table.hashcons(true_bool_expr.clone()));
        hc_data.push(table.hashcons(false_bool_expr.clone()));
        for i in 3..1_00 {
            let mut rng = rand::thread_rng();
            let first = rng.gen_range(0..i);
            let second = rng.gen_range(0..i);

            if i % 5 == 0 {
                hc_data.push(table.hashcons(BoolExpr::And(
                    hc_data[first].clone(),
                    hc_data[second].clone(),
                )));
            } else if i % 3 == 0 {
                hc_data.push(table.hashcons(BoolExpr::Or(
                    hc_data[first].clone(),
                    hc_data[second].clone(),
                )));
            } else {
                hc_data.push(table.hashcons(BoolExpr::Not(hc_data[first].clone())));
            }
        }
        drop(hc_data);

        // Consistency checks
        assert_eq!(
            table.len(),
            0,
            "Table size should be zero because all values are dropped"
        );
    }

    */
    /*
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        struct PanicOnDrop;

        impl Drop for PanicOnDrop {
            fn drop(&mut self) {
                panic!("Panic on drop");
            }
        }

        #[test]
        fn test_reentrant_drop() {
            let table = HcTable::<PanicOnDrop>::new();

            // Intern the PanicOnDrop
            let hc = table.hashcons(PanicOnDrop);

            // Use catch_unwind to handle the panic
            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                drop(hc);
            }));

            // Assert that a panic occurred
            assert!(result.is_err(), "A panic should occur on drop");
        }
    */

    /// Tests hashconsing with a large set of unique boolean expressions.
    #[test]
    fn test_large_unique() {
        let table = HcTable::<BoolExpr>::new();
        let mut hc_data = Vec::new();

        let true_bool_expr = BoolExpr::Const(true);
        let false_bool_expr = BoolExpr::Const(false);

        hc_data.push(table.hashcons(true_bool_expr.clone()));
        hc_data.push(table.hashcons(false_bool_expr.clone()));

        for i in 2..10_000 {
            hc_data
                .push(table.hashcons(BoolExpr::Or(hc_data[i - 1].clone(), hc_data[i - 2].clone())));
            hc_data
                .push(table.hashcons(BoolExpr::Or(hc_data[i - 1].clone(), hc_data[i - 2].clone())));
        }

        assert_eq!(
            table.len(),
            10_000,
            "table length should be equal to the data length"
        );
    }

    /// Tests hashconsing with a large set of non-unique boolean expressions.
    #[test]
    fn test_large_non_unique() {
        let table = HcTable::<BoolExpr>::new();
        let mut hc_data = Vec::new();

        let true_bool_expr = BoolExpr::Const(true);
        let false_bool_expr = BoolExpr::Const(false);

        hc_data.push(table.hashcons(true_bool_expr.clone()));
        hc_data.push(table.hashcons(false_bool_expr.clone()));
        hc_data.push(table.hashcons(true_bool_expr));
        hc_data.push(table.hashcons(false_bool_expr));

        for i in 4..10_000 {
            let mut rng = rand::thread_rng();
            let first = rng.gen_range(0..i);
            let second = rng.gen_range(0..i);

            hc_data.push(table.hashcons(BoolExpr::And(
                hc_data[first].clone(),
                hc_data[second].clone(),
            )));
        }

        assert!(
            table.len() < 10_000,
            "Data length should always be greater than length of the table"
        );
    }
}
