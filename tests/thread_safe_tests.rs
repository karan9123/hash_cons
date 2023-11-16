#[cfg(test)]
mod thread_safe_tests {
    use hash_cons::Ahc;

    #[derive(Hash, PartialEq, Eq, Clone)]
    enum BoolExpr {
        Const(bool),
        And(Ahc<BoolExpr>, Ahc<BoolExpr>),
        Or(Ahc<BoolExpr>, Ahc<BoolExpr>),
        Not(Ahc<BoolExpr>),
    }
    mod single_tests {
        use crate::thread_safe_tests::BoolExpr;
        use hash_cons::{Ahc, AhcTable};
        use rand::Rng;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::panic;
        use std::panic::AssertUnwindSafe;

        // Test case for basic hashconsing of a simple constant value.
        #[test]
        fn test_basic_const_hashconsing() {
            let table: AhcTable<BoolExpr> = AhcTable::new();
            let expr_true = BoolExpr::Const(true);
            let expr_true_v2 = BoolExpr::Const(true);
            let ahc_true: Ahc<BoolExpr> = table.hashcons(expr_true);
            let ahc_true_v2: Ahc<BoolExpr> = table.hashcons(expr_true_v2);
            let ahc_not_true = table.hashcons(BoolExpr::Not(ahc_true.clone()));

            assert!(
                ahc_true == ahc_true_v2,
                "Identical constants should share the same hashconsed reference."
            );
            assert!(
                ahc_true_v2 != ahc_not_true,
                "Not True and False are not same values"
            );
        }

        // Test case for hashconsing complex boolean expressions.
        #[test]
        fn test_complex_expression_hashconsing() {
            let table = AhcTable::new();
            let expr_true = BoolExpr::Const(true);
            let expr_false = BoolExpr::Const(false);
            let ahc_true = table.hashcons(expr_true);
            let ahc_false = table.hashcons(expr_false);
            let expr_and_v1 = BoolExpr::And(ahc_true.clone(), ahc_false.clone());
            let expr_and_v2 = BoolExpr::And(ahc_true, table.hashcons(BoolExpr::Const(false)));
            let ahc_and_v1 = table.hashcons(expr_and_v1);
            let ahc_and_v2 = table.hashcons(expr_and_v2);

            assert!(
                ahc_and_v1 == ahc_and_v2,
                "Identical complex expressions should share the same hashconsed reference."
            );
        }

        #[test]
        fn test_cleanup_effectiveness() {
            let table = AhcTable::<BoolExpr>::new();

            // hash cons several values
            let ahc_true = table.hashcons(BoolExpr::Const(true));
            let ahc_false = table.hashcons(BoolExpr::Const(false));
            let ahc_and = table.hashcons(BoolExpr::And(ahc_true.clone(), ahc_false.clone()));

            // Get the size of the table after interning
            let size_before_cleanup = table.len();
            assert_eq!(
                size_before_cleanup, 3,
                "Table should have 3 items after interning"
            );

            drop(ahc_true);
            drop(ahc_false);
            drop(ahc_and);

            // Call cleanup method on the table
            table.cleanup();

            // Get the size of the table after cleanup
            let size_after_cleanup = table.len();

            // Assert that the size of the table has decreased appropriately
            assert_eq!(
                size_after_cleanup, 0,
                "The table size should be zero after cleanup, because values are dropped"
            );
        }

        #[test]
        fn test_drop_behavior() {
            let table = AhcTable::<BoolExpr>::new();

            // hash cons a value and keep a reference to it
            let expr_true = table.hashcons(BoolExpr::Const(true));

            // Get the size of the table after hash consing
            let size_after_consing = table.len();
            assert_eq!(
                size_after_consing, 1,
                "Table should have 1 item after interning"
            );

            // Drop the reference to the hash consed value
            drop(expr_true);

            // Get the size of the table after dropping the reference
            let size_after_dropping = table.len();

            // Assert that the table is empty after the value is dropped
            assert_eq!(
                size_after_dropping, 0,
                "Table should be empty after dropping the only reference"
            );
        }

        #[test]
        fn test_memory_usage_hash_consing() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();
            let mut data = Vec::new();

            for _ in 0..100 {
                // Add repetitive BoolExpr values
                let expr_true = BoolExpr::Const(true);
                let expr_false = BoolExpr::Const(false);
                data.push(expr_true.clone());
                data.push(expr_false.clone());
                ahc_data.push(table.hashcons(expr_true));
                ahc_data.push(table.hashcons(expr_false));
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

        #[test]
        fn test_hash_collision() {
            #[derive(PartialEq, Eq)]
            enum BoolExprHc {
                Const(bool),
                Not(Ahc<BoolExprHc>),
            }

            impl Hash for BoolExprHc {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    match self {
                        BoolExprHc::Const(flag) => {
                            // Hash 1 for true, 0 for false
                            let value = if *flag { 1 } else { 0 };
                            value.hash(state);
                        }
                        BoolExprHc::Not(val) => {
                            let mut temp_hasher = DefaultHasher::new();
                            val.hash(&mut temp_hasher);
                            1.hash(state);
                        }
                    }
                }
            }

            let table = AhcTable::<BoolExprHc>::new();
            let expr_true = BoolExprHc::Const(true);
            let expr_not_false = BoolExprHc::Not(table.hashcons(BoolExprHc::Const(false)));
            let ahc_true = table.hashcons(expr_true);
            let ahc_not_false = table.hashcons(expr_not_false);

            let mut hasher = DefaultHasher::new();
            ahc_true.hash(&mut hasher);
            let hash_value_ahc_true = hasher.finish();

            // Reinitialize the hasher for a new calculation
            hasher = DefaultHasher::new();
            ahc_not_false.hash(&mut hasher);
            let hash_value_ahc_not_false = hasher.finish();

            assert_eq!(
                hash_value_ahc_true, hash_value_ahc_not_false,
                "Hash values should be equal"
            );
            assert_eq!(table.len(), 3, "Table should have 3 items");
        }

        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        struct PanicOnDrop;

        impl Drop for PanicOnDrop {
            fn drop(&mut self) {
                panic!("Panic on drop");
            }
        }

        #[test]
        fn test_reentrant_drop() {
            let table = AhcTable::<PanicOnDrop>::new();

            // Intern the PanicOnDrop
            let ahc = table.hashcons(PanicOnDrop);

            // Use catch_unwind to handle the panic
            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                drop(ahc);
            }));

            // Assert that a panic occurred
            assert!(result.is_err(), "A panic should occur on drop");
        }

        #[test]
        fn test_large_unique() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();

            let expr_true = BoolExpr::Const(true);
            let expr_false = BoolExpr::Const(false);

            ahc_data.push(table.hashcons(expr_true));
            ahc_data.push(table.hashcons(expr_false));

            for i in 2..10_000 {
                ahc_data.push(table.hashcons(BoolExpr::Or(
                    ahc_data[i - 1].clone(),
                    ahc_data[i - 2].clone(),
                )));
                ahc_data.push(table.hashcons(BoolExpr::Or(
                    ahc_data[i - 1].clone(),
                    ahc_data[i - 2].clone(),
                )));
            }
            assert_eq!(
                table.len(),
                10_000,
                "table length should be equal to the  data length"
            );
        }

        // Tests hashconsing with a large set of non-unique boolean expressions.
        #[test]
        fn test_large_non_unique() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();

            let expr_true = BoolExpr::Const(true);
            let expr_false = BoolExpr::Const(false);

            ahc_data.push(table.hashcons(expr_true.clone()));
            ahc_data.push(table.hashcons(expr_false.clone()));
            ahc_data.push(table.hashcons(expr_true));
            ahc_data.push(table.hashcons(expr_false));

            for i in 4..10_000 {
                let mut rng = rand::thread_rng();
                let first = rng.gen_range(0..i);
                let second = rng.gen_range(0..i);

                ahc_data.push(table.hashcons(BoolExpr::And(
                    ahc_data[first].clone(),
                    ahc_data[second].clone(),
                )));
            }

            assert!(
                table.len() < 10_000,
                "Data length should always be greater than length of the table"
            );
        }

        #[test]
        fn stress_test_ahc_table() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();
            let expr_true = BoolExpr::Const(true);
            let expr_false = BoolExpr::Const(false);
            ahc_data.push(table.hashcons(expr_true.clone()));
            ahc_data.push(table.hashcons(expr_false.clone()));
            ahc_data.push(table.hashcons(expr_true.clone()));
            ahc_data.push(table.hashcons(expr_false.clone()));

            for i in 3..1_000 {
                let mut rng = rand::thread_rng();
                let first = rng.gen_range(0..i);
                let second = rng.gen_range(0..i);

                if i % 5 == 0 {
                    ahc_data.push(table.hashcons(BoolExpr::And(
                        ahc_data[first].clone(),
                        ahc_data[second].clone(),
                    )));
                } else if i % 3 == 0 {
                    ahc_data.push(table.hashcons(BoolExpr::Or(
                        ahc_data[first].clone(),
                        ahc_data[second].clone(),
                    )));
                } else {
                    ahc_data.push(table.hashcons(BoolExpr::Not(ahc_data[first].clone())));
                }
            }
            drop(ahc_data);
            table.cleanup();

            // Consistency checks
            assert_eq!(
                table.len(),
                0,
                "Table size should be zero because all values are dropped"
            );
        }
    }

    mod multi_threaded_tests {
        use crate::thread_safe_tests::BoolExpr;
        use hash_cons::{Ahc, AhcTable};
        use rand::Rng;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::thread;

        #[test]
        fn test_multi_threaded_basic_const_hashconsing() {
            let table = AhcTable::new();

            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let expr_true = BoolExpr::Const(true);
                let ahc_true = table_clone.hashcons(expr_true);
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let expr_true_v2 = BoolExpr::Const(true);

            let table_clone = table.clone();
            let thread_handle_ahc_true_v2 = thread::spawn(move || {
                let ahc_true_v2 = table_clone.hashcons(expr_true_v2);
                ahc_true_v2
            });
            let ahc_true_v2 = thread_handle_ahc_true_v2
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let table_clone = table.clone();
            let ahc_true_clone = ahc_true.clone();
            let thread_handle_ahc_not_true = thread::spawn(move || {
                let ahc_not_true = table_clone.hashcons(BoolExpr::Not(ahc_true_clone));
                ahc_not_true
            });
            let ahc_not_true = thread_handle_ahc_not_true.join().expect(
                "Thread should finish and return `Ahc<Not <Ahc <Const <true>>>>` without panicking",
            );

            assert!(
                ahc_true == ahc_true_v2,
                "Identical constants should share the same hashconsed reference."
            );

            assert!(
                ahc_true_v2 != ahc_not_true,
                "Not True and False are not same values"
            );
        }

        #[test]
        fn test_multi_threaded_complex_expression_hashconsing() {
            let table = AhcTable::<BoolExpr>::new();

            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let expr_true = BoolExpr::Const(true);
                let ahc_true = table_clone.hashcons(expr_true);
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_ahc_false = thread::spawn(move || {
                let expr_false = BoolExpr::Const(false);
                let ahc_false = table_clone.hashcons(expr_false);
                ahc_false
            });
            let ahc_false = thread_handle_ahc_false
                .join()
                .expect("Thread should finish and return `Ahc<Const <false>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_ahc_not_false = thread::spawn(move || {
                let expr_not_false = BoolExpr::Not(ahc_false.clone());
                let ahc_not_false = table_clone.hashcons(expr_not_false);
                ahc_not_false
            });
            let ahc_not_false = thread_handle_ahc_not_false
                .join()
                .expect("Thread should finish and return `Ahc<Not <Ahc <Const <false>>>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_and = thread::spawn(move || {
                let expr_and = BoolExpr::And(ahc_true.clone(), ahc_not_false);
                table_clone.hashcons(expr_and)
            });
            let _ahc_and = thread_handle_and
                .join()
                .expect("Thread should finish and return `Ahc<And <Ahc<Const <true>>>, Ahc<Not <Ahc <Const <false>>>>> `without panicking");

            assert_eq!(
                table.len(),
                4,
                "Table should have 4 item after threading operations"
            );
        }

        #[test]
        fn test_multi_threaded_cleanup_effectiveness() {
            let table = AhcTable::<BoolExpr>::new();

            // hash cons several values
            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let ahc_true = table_clone.hashcons(BoolExpr::Const(true));
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_ahc_false = thread::spawn(move || {
                let ahc_false = table_clone.hashcons(BoolExpr::Const(false));
                ahc_false
            });
            let ahc_false = thread_handle_ahc_false
                .join()
                .expect("Thread should finish and return `Ahc<Const <false>>` without panicking");

            let table_clone = table.clone();
            let ahc_true_clone = ahc_true.clone();
            let ahc_false_clone = ahc_false.clone();
            let thread_handle_ahc_and = thread::spawn(move || {
                let ahc_and = table_clone.hashcons(BoolExpr::And(ahc_true_clone, ahc_false_clone));
                ahc_and
            });
            let ahc_and = thread_handle_ahc_and.join().expect(
                "Thread should finish and return `Ahc<And <Ahc<Const <true>>>, \
                Ahc<Const <false>>>>` without panicking",
            );

            // Get the size of the table after interning
            let size_before_cleanup = table.len();
            assert_eq!(
                size_before_cleanup, 3,
                "Table should have 3 items after interning"
            );

            drop(ahc_true);
            drop(ahc_false);
            drop(ahc_and);

            // Call cleanup method on the table
            table.cleanup();

            // Get the size of the table after cleanup
            let size_after_cleanup = table.len();

            // Assert that the size of the table has decreased appropriately
            assert_eq!(
                size_after_cleanup, 0,
                "The table size should be zero after cleanup, because values are dropped"
            );
        }

        #[test]
        fn test_multi_threaded_drop_behavior() {
            let table = AhcTable::<BoolExpr>::new();

            // hash cons a value and keep a reference to it
            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let ahc_true = table_clone.hashcons(BoolExpr::Const(true));
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            // Get the size of the table after interning
            let size_after_interning = table.len();
            assert_eq!(
                size_after_interning, 1,
                "Table should have 1 item after interning"
            );

            // Drop the reference to the interned value
            drop(ahc_true);

            // Get the size of the table after dropping the reference
            let size_after_dropping = table.len();

            // Assert that the table is empty after the value is dropped
            assert_eq!(
                size_after_dropping, 0,
                "Table should be empty after dropping the only reference"
            );
        }

        #[test]
        fn test_multi_threaded_memory_usage_hash_consing() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();
            let mut data = Vec::new();

            for _ in 0..100 {
                // Add repetitive BoolExpr values
                let table_clone = table.clone();
                let expr_true = BoolExpr::Const(true);
                let thread_handle_ahc_true = thread::spawn(move || {
                    let ahc_true = table_clone.hashcons(BoolExpr::Const(true));
                    ahc_true
                });
                let ahc_true = thread_handle_ahc_true.join().expect(
                    "Thread should finish and return `Ahc<Const <true>>` without panicking",
                );

                let expr_false = BoolExpr::Const(false);
                let table_clone = table.clone();
                let thread_handle_ahc_false = thread::spawn(move || {
                    let ahc_false = table_clone.hashcons(BoolExpr::Const(false));
                    ahc_false
                });
                let ahc_false = thread_handle_ahc_false.join().expect(
                    "Thread should finish and return `Ahc<Const <false>>` without panicking",
                );

                data.push(expr_true.clone());
                data.push(expr_false.clone());
                ahc_data.push(ahc_true);
                ahc_data.push(ahc_false);
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

        #[test]
        fn test_multi_threaded_hash_collision() {
            #[derive(PartialEq, Eq)]
            enum BoolExprHc {
                Const(bool),
                Not(Ahc<BoolExprHc>),
            }

            impl Hash for BoolExprHc {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    match self {
                        BoolExprHc::Const(flag) => {
                            // Hash 1 for true, 0 for false
                            let value = if *flag { 1 } else { 0 };
                            value.hash(state);
                        }
                        BoolExprHc::Not(val) => {
                            let mut temp_hasher = DefaultHasher::new();
                            val.hash(&mut temp_hasher);
                            1.hash(state);
                        }
                    }
                }
            }

            let table = AhcTable::<BoolExprHc>::new();

            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let expr_true = BoolExprHc::Const(true);
                let ahc_true = table_clone.hashcons(expr_true);
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_ahc_not_false = thread::spawn(move || {
                let expr_not_false =
                    BoolExprHc::Not(table_clone.hashcons(BoolExprHc::Const(false)));
                let ahc_not_false = table_clone.hashcons(expr_not_false);
                ahc_not_false
            });
            let ahc_not_false = thread_handle_ahc_not_false
                .join()
                .expect("Thread should finish and return `Ahc<Not <Ahc <Const <false>>>>` without panicking");

            let mut hasher = DefaultHasher::new();
            ahc_true.hash(&mut hasher);
            let hash_value_ahc_true = hasher.finish();

            // Reinitialize the hasher for a new calculation
            hasher = DefaultHasher::new();
            ahc_not_false.hash(&mut hasher);
            let hash_value_ahc_not_false = hasher.finish();

            assert_eq!(
                hash_value_ahc_true, hash_value_ahc_not_false,
                "Hash values should be equal"
            );
            assert_eq!(table.len(), 3, "Table should have 3 items");
        }

        #[test]
        fn test_multi_threaded_large_unique() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();

            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let expr_true = BoolExpr::Const(true);
                let ahc_true = table_clone.hashcons(expr_true);
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_false = thread::spawn(move || {
                let expr_false = BoolExpr::Const(false);
                let ahc_false = table_clone.hashcons(expr_false);
                ahc_false
            });
            let ahc_false = thread_handle_false
                .join()
                .expect("Thread should finish and return `Ahc<Const <false>>` without panicking");

            ahc_data.push(ahc_true);
            ahc_data.push(ahc_false);

            for i in 2..10_000 {
                let table_clone = table.clone();
                let first = ahc_data[i - 1].clone();
                let second = ahc_data[i - 2].clone();
                let thread_handle_ahc_or = thread::spawn(move || {
                    let ahc_or = table_clone.hashcons(BoolExpr::Or(first, second));
                    ahc_or
                });
                let ahc_or = thread_handle_ahc_or
                    .join()
                    .expect("Thread should finish and return `Ahc<Or <Ahc<Const <true>>>, Ahc<Const <false>>>>` without panicking");
                ahc_data.push(ahc_or.clone());
                ahc_data.push(ahc_or);
            }
            assert_eq!(
                table.len(),
                10_000,
                "table length should be equal to the  data length"
            );
        }

        #[test]
        fn test_multi_threaded_large_non_unique() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();

            let table_clone = table.clone();
            let thread_handle_ahc_true = thread::spawn(move || {
                let expr_true = BoolExpr::Const(true);
                let ahc_true = table_clone.hashcons(expr_true);
                ahc_true
            });
            let ahc_true = thread_handle_ahc_true
                .join()
                .expect("Thread should finish and return `Ahc<Const <true>>` without panicking");

            let table_clone = table.clone();
            let thread_handle_false = thread::spawn(move || {
                let expr_false = BoolExpr::Const(false);
                let ahc_false = table_clone.hashcons(expr_false);
                ahc_false
            });
            let ahc_false = thread_handle_false
                .join()
                .expect("Thread should finish and return `Ahc<Const <false>>` without panicking");

            ahc_data.push(ahc_true);
            ahc_data.push(ahc_false);

            for i in 2..10_000 {
                let table_clone = table.clone();

                let mut rng = rand::thread_rng();
                let first_index = rng.gen_range(0..i);
                let second_index = rng.gen_range(0..i);
                let first = ahc_data[first_index].clone();
                let second = ahc_data[second_index].clone();
                let thread_handle_ahc_or = thread::spawn(move || {
                    let ahc_or = table_clone.hashcons(BoolExpr::Or(first, second));
                    ahc_or
                });
                let ahc_or = thread_handle_ahc_or
                    .join()
                    .expect("Thread should finish and return `Ahc<Or <Ahc<Const <true>>>, Ahc<Const <false>>>>` without panicking");
                ahc_data.push(ahc_or.clone());
                ahc_data.push(ahc_or);
            }
            assert!(
                table.len() < 10_000,
                "table length should be equal to the  data length"
            );
        }

        #[test]
        fn test_multi_threaded_stress_test_ahc_table() {
            let table = AhcTable::<BoolExpr>::new();
            let mut ahc_data = Vec::new();

            let expr_true = BoolExpr::Const(true);
            let expr_false = BoolExpr::Const(false);
            ahc_data.push(table.hashcons(expr_true.clone()));
            ahc_data.push(table.hashcons(expr_false.clone()));
            ahc_data.push(table.hashcons(expr_true.clone()));
            ahc_data.push(table.hashcons(expr_false.clone()));

            for i in 3..1_000 {
                let mut rng = rand::thread_rng();
                let first = rng.gen_range(0..i);
                let second = rng.gen_range(0..i);

                if i % 5 == 0 {
                    let table_clone = table.clone();
                    let first_ahc = ahc_data[first].clone();
                    let second_ahc = ahc_data[second].clone();

                    let thread_handle_ahc_and = thread::spawn(move || {
                        let ahc_and = table_clone.hashcons(BoolExpr::And(first_ahc, second_ahc));
                        ahc_and
                    });
                    let ahc_and = thread_handle_ahc_and
                        .join()
                        .expect("Thread should finish and return `Ahc<And <Ahc<Const <true>>>, Ahc<Const <false>>>>` without panicking");

                    ahc_data.push(ahc_and);
                } else if i % 3 == 0 {
                    let table_clone = table.clone();
                    let first_ahc = ahc_data[first].clone();
                    let second_ahc = ahc_data[second].clone();

                    let thread_handle_ahc_or = thread::spawn(move || {
                        let ahc_or = table_clone.hashcons(BoolExpr::Or(first_ahc, second_ahc));
                        ahc_or
                    });

                    let ahc_or = thread_handle_ahc_or
                        .join()
                        .expect("Thread should finish and return `Ahc<Or <Ahc<Const <true>>>, Ahc<Const <false>>>>` without panicking");

                    ahc_data.push(ahc_or);
                } else {
                    let table_clone = table.clone();
                    let first_ahc = ahc_data[first].clone();
                    let thread_handle_ahc_not = thread::spawn(move || {
                        let ahc_not = table_clone.hashcons(BoolExpr::Not(first_ahc));
                        ahc_not
                    });
                    let ahc_not = thread_handle_ahc_not
                        .join()
                        .expect("Thread should finish and return `Ahc<Not <Ahc<Const <true>>>>` without panicking");

                    ahc_data.push(ahc_not);
                }
            }
            drop(ahc_data);
            // table.cleanup();

            // Consistency checks
            assert_eq!(
                table.len(),
                0,
                "Table size should be zero because all values are dropped"
            );
        }
    }
}
