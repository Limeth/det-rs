#[macro_export]
macro_rules! det_copy {
    ($_00:expr) => {
        $_00
    };

    ($tuple:expr, 1) => {{
        let ((_00,),) = $tuple;

        det_copy!(_00)
    }};

    ($_00:expr, $_10:expr,
     $_01:expr, $_11:expr) => {
          $_00 * det_copy!($_11)
        - $_10 * det_copy!($_01)
    };

    ($tuple:expr, 2) => {{
        let ((_00, _10),
             (_01, _11)) = $tuple;

        det_copy!(_00, _10,
                  _01, _11)
    }};

    ($_00:expr, $_10:expr, $_20:expr,
     $_01:expr, $_11:expr, $_21:expr,
     $_02:expr, $_12:expr, $_22:expr) => {
          $_00 * det_copy!($_11, $_21,
                           $_12, $_22)
        - $_10 * det_copy!($_01, $_21,
                           $_02, $_22)
        + $_20 * det_copy!($_01, $_11,
                           $_02, $_12)
    };

    ($tuple:expr, 3) => {{
        let ((_00, _10, _20),
             (_01, _11, _21),
             (_02, _12, _22)) = $tuple;

        det_copy!(_00, _10, _20,
                  _01, _11, _21,
                  _02, _12, _22)
    }};

    ($_00:expr, $_10:expr, $_20:expr, $_30:expr,
     $_01:expr, $_11:expr, $_21:expr, $_31:expr,
     $_02:expr, $_12:expr, $_22:expr, $_32:expr,
     $_03:expr, $_13:expr, $_23:expr, $_33:expr) => {
          $_00 * det_copy!($_11, $_21, $_31,
                           $_12, $_22, $_32,
                           $_13, $_23, $_33)
        - $_10 * det_copy!($_01, $_21, $_31,
                           $_02, $_22, $_32,
                           $_03, $_23, $_33)
        + $_20 * det_copy!($_01, $_11, $_31,
                           $_02, $_12, $_32,
                           $_03, $_13, $_33)
        - $_30 * det_copy!($_01, $_11, $_21,
                           $_02, $_12, $_22,
                           $_03, $_13, $_23)
    };

    ($tuple:expr, 4) => {{
        let ((_00, _10, _20, _30),
             (_01, _11, _21, _31),
             (_02, _12, _22, _32),
             (_03, _13, _23, _33)) = $tuple;

        det_copy!(_00, _10, _20, _30,
                  _01, _11, _21, _31,
                  _02, _12, _22, _32,
                  _03, _13, _23, _33)
    }};
}

#[macro_export]
macro_rules! det_clone {
    ($_00:expr) => {
        det_copy!($_00.clone())
    };

    ($tuple:expr, 1) => {{
        let ((_00,),) = $tuple;

        det_copy!(_00.clone())
    }};

    ($_00:expr, $_10:expr,
     $_01:expr, $_11:expr) => {{
        det_copy!($_00.clone(), $_10.clone(),
                  $_01.clone(), $_11.clone())
    }};

    ($tuple:expr, 2) => {{
        let ((_00, _10),
             (_01, _11)) = $tuple;

        det_copy!(_00.clone(), _10.clone(),
                  _01.clone(), _11.clone())
    }};

    ($_00:expr, $_10:expr, $_20:expr,
     $_01:expr, $_11:expr, $_21:expr,
     $_02:expr, $_12:expr, $_22:expr) => {{
        det_copy!($_00.clone(), $_10.clone(), $_20.clone(),
                  $_01.clone(), $_11.clone(), $_21.clone(),
                  $_02.clone(), $_12.clone(), $_22.clone())
    }};

    ($tuple:expr, 3) => {{
        let ((_00, _10, _20),
             (_01, _11, _21),
             (_02, _12, _22)) = $tuple;

        det_copy!(_00.clone(), _10.clone(), _20.clone(),
                  _01.clone(), _11.clone(), _21.clone(),
                  _02.clone(), _12.clone(), _22.clone())
    }};

    ($_00:expr, $_10:expr, $_20:expr, $_30:expr,
     $_01:expr, $_11:expr, $_21:expr, $_31:expr,
     $_02:expr, $_12:expr, $_22:expr, $_32:expr,
     $_03:expr, $_13:expr, $_23:expr, $_33:expr) => {{
        det_copy!($_00.clone(), $_10.clone(), $_20.clone(), $_30.clone(),
                  $_01.clone(), $_11.clone(), $_21.clone(), $_31.clone(),
                  $_02.clone(), $_12.clone(), $_22.clone(), $_32.clone(),
                  $_03.clone(), $_13.clone(), $_23.clone(), $_33.clone())
    }};

    ($tuple:expr, 4) => {{
        let ((_00, _10, _20, _30),
             (_01, _11, _21, _31),
             (_02, _12, _22, _32),
             (_03, _13, _23, _33)) = $tuple;

        det_copy!(_00.clone(), _10.clone(), _20.clone(), _30.clone(),
                  _01.clone(), _11.clone(), _21.clone(), _31.clone(),
                  _02.clone(), _12.clone(), _22.clone(), _32.clone(),
                  _03.clone(), _13.clone(), _23.clone(), _33.clone())
    }};
}

#[macro_export]
macro_rules! det {
    // Cannot pass a reference, because no operation is used on it
    // and we want a value back, not a reference.
    ($_00:expr) => {
        det_copy!($_00)
    };

    // Cannot pass a reference, because no operation is used on it
    // and we want a value back, not a reference.
    ($tuple:expr, 1) => {
        det_copy!($tuple, 1)
    };

    ($_00:expr, $_10:expr,
     $_01:expr, $_11:expr) => {{
        let (_00, _10,
             _01, _11) = ($_00, $_10,
                          $_01, $_11);

        det_copy!(&_00, &_10,
                  &_01, &_11)
    }};

    ($tuple:expr, 2) => {{
        let ((_00, _10),
             (_01, _11)) = $tuple;

        det_copy!(((&_00, &_10),
                   (&_01, &_11)), 2)
    }};

    ($_00:expr, $_10:expr, $_20:expr,
     $_01:expr, $_11:expr, $_21:expr,
     $_02:expr, $_12:expr, $_22:expr) => {{
        let (_00, _10, _20,
             _01, _11, _21,
             _02, _12, _22) = ($_00, $_10, $_20,
                               $_01, $_11, $_21,
                               $_02, $_12, $_22);

        det_copy!(&_00, &_10, &_20,
                  &_01, &_11, &_21,
                  &_02, &_12, &_22)
    }};

    ($tuple:expr, 3) => {{
        let ((_00, _10, _20),
             (_01, _11, _21),
             (_02, _12, _22)) = $tuple;

        det_copy!(((&_00, &_10, &_20),
                   (&_01, &_11, &_21),
                   (&_02, &_12, &_22)), 3)
    }};

    ($_00:expr, $_10:expr, $_20:expr, $_30:expr,
     $_01:expr, $_11:expr, $_21:expr, $_31:expr,
     $_02:expr, $_12:expr, $_22:expr, $_32:expr,
     $_03:expr, $_13:expr, $_23:expr, $_33:expr) => {{
        let (_00, _10, _20, _30,
             _01, _11, _21, _31,
             _02, _12, _22, _32,
             _03, _13, _23, _33) = ($_00, $_10, $_20, $_30,
                                    $_01, $_11, $_21, $_31,
                                    $_02, $_12, $_22, $_32,
                                    $_03, $_13, $_23, $_33);

        det_copy!(&_00, &_10, &_20, &_30,
                  &_01, &_11, &_21, &_31,
                  &_02, &_12, &_22, &_32,
                  &_03, &_13, &_23, &_33)
    }};

    ($tuple:expr, 4) => {{
        let ((_00, _10, _20, _30),
             (_01, _11, _21, _31),
             (_02, _12, _22, _32),
             (_03, _13, _23, _33)) = $tuple;

        det_copy!(((&_00, &_10, &_20, &_30),
                   (&_01, &_11, &_21, &_31),
                   (&_02, &_12, &_22, &_32),
                   (&_03, &_13, &_23, &_33)), 4)
    }};
}

#[cfg(test)]
mod tests {
    use std::ops::{Mul, Sub, Add};

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[derive(Eq, PartialEq, Debug, Clone)]
    struct Cloneable(i32);

    macro_rules! impl_binop {
        ($imp:ident, $method:ident for $item_type:ident) => {
            impl $imp<$item_type> for $item_type {
                type Output = $item_type;

                fn $method(self, other: $item_type) -> $item_type {
                    $item_type($imp::$method(self.0, other.0))
                }
            }
        }
    }

    macro_rules! impl_binop_ref_val {
        ($imp:ident, $method:ident for $item_type:ident) => {
            impl<'a> $imp<$item_type> for &'a $item_type {
                type Output = $item_type;

                fn $method(self, other: $item_type) -> $item_type {
                    $item_type($imp::$method(self.0, other.0))
                }
            }
        }
    }

    macro_rules! impl_binop_val_ref {
        ($imp:ident, $method:ident for $item_type:ident) => {
            impl<'a> $imp<&'a $item_type> for $item_type {
                type Output = $item_type;

                fn $method(self, other: &'a $item_type) -> $item_type {
                    $item_type($imp::$method(self.0, other.0))
                }
            }
        }
    }

    macro_rules! impl_binop_ref_ref {
        ($imp:ident, $method:ident for $item_type:ident) => {
            impl<'a, 'b> $imp<&'a $item_type> for &'b $item_type {
                type Output = $item_type;

                fn $method(self, other: &'a $item_type) -> $item_type {
                    $item_type($imp::$method(self.0, other.0))
                }
            }
        }
    }

    impl_binop!(Mul, mul for NonCopy);
    impl_binop_ref_ref!(Mul, mul for NonCopy);
    impl_binop_ref_val!(Mul, mul for NonCopy);
    impl_binop!(Sub, sub for NonCopy);
    impl_binop!(Add, add for NonCopy);

    impl_binop!(Mul, mul for Cloneable);
    impl_binop!(Sub, sub for Cloneable);
    impl_binop!(Add, add for Cloneable);

    #[test]
    fn det_1_copy() {
        assert_eq! {
            det_copy!(42),
            42
        }
    }

    #[test]
    fn det_1_copy_tuple() {
        let tuple = ((42,),);

        assert_eq! {
            det_copy!(tuple, 1),
            42
        }
    }

    #[test]
    fn det_1_clone() {
        assert_eq! {
            det_clone!(Cloneable(42)),
            Cloneable(42)
        }
    }

    #[test]
    fn det_1_clone_tuple() {
        let tuple = ((Cloneable(42),),);

        assert_eq! {
            det_clone!(tuple, 1),
            Cloneable(42)
        }
    }

    #[test]
    fn det_1() {
        assert_eq! {
            det!(NonCopy(42)),
            NonCopy(42)
        }
    }

    #[test]
    fn det_1_tuple() {
        let tuple = ((NonCopy(42),),);

        assert_eq! {
            det!(tuple, 1),
            NonCopy(42)
        }
    }

    #[test]
    fn det_2_copy() {
        assert_eq! {
            det_copy!(1, 2,
                      3, 4),
            -2
        }
    }

    #[test]
    fn det_2_copy_tuple() {
        let tuple = ((1, 2),
                     (3, 4));

        assert_eq! {
            det_copy!(tuple, 2),
            -2
        }
    }

    #[test]
    fn det_2_clone() {
        assert_eq! {
            det_clone!(Cloneable(1), Cloneable(2),
                       Cloneable(3), Cloneable(4)),
            Cloneable(-2)
        }
    }

    #[test]
    fn det_2_clone_tuple() {
        let tuple = ((Cloneable(1), Cloneable(2)),
                     (Cloneable(3), Cloneable(4)));

        assert_eq! {
            det_clone!(tuple, 2),
            Cloneable(-2)
        }
    }

    #[test]
    fn det_2() {
        assert_eq! {
            det!(NonCopy(1), NonCopy(2),
                 NonCopy(3), NonCopy(4)),
            NonCopy(-2)
        }
    }

    #[test]
    fn det_2_tuple() {
        let tuple = ((NonCopy(1), NonCopy(2)),
                     (NonCopy(3), NonCopy(4)));

        assert_eq! {
            det!(tuple, 2),
            NonCopy(-2)
        }
    }

    #[test]
    fn det_3_copy() {
        assert_eq! {
            det_copy!(1, 2, 3,
                      4, 5, 6,
                      7, 8, 9),
            0
        }
    }

    #[test]
    fn det_3_copy_tuple() {
        let tuple = ((1, 2, 3),
                     (4, 5, 6),
                     (7, 8, 9));

        assert_eq! {
            det_copy!(tuple, 3),
            0
        }
    }

    #[test]
    fn det_3_clone() {
        assert_eq! {
            det_clone!(Cloneable(1), Cloneable(2), Cloneable(3),
                       Cloneable(4), Cloneable(5), Cloneable(6),
                       Cloneable(7), Cloneable(8), Cloneable(9)),
            Cloneable(0)
        }
    }

    #[test]
    fn det_3_clone_tuple() {
        let tuple = ((Cloneable(1), Cloneable(2), Cloneable(3)),
                     (Cloneable(4), Cloneable(5), Cloneable(6)),
                     (Cloneable(7), Cloneable(8), Cloneable(9)));

        assert_eq! {
            det_clone!(tuple, 3),
            Cloneable(0)
        }
    }

    #[test]
    fn det_3() {
        assert_eq! {
            det!(NonCopy(1), NonCopy(2), NonCopy(3),
                 NonCopy(4), NonCopy(5), NonCopy(6),
                 NonCopy(7), NonCopy(8), NonCopy(9)),
            NonCopy(0)
        }
    }

    #[test]
    fn det_3_tuple() {
        let tuple = ((NonCopy(1), NonCopy(2), NonCopy(3)),
                     (NonCopy(4), NonCopy(5), NonCopy(6)),
                     (NonCopy(7), NonCopy(8), NonCopy(9)));

        assert_eq! {
            det!(tuple, 3),
            NonCopy(0)
        }
    }

    #[test]
    fn det_4_copy() {
        assert_eq! {
            det_copy!(1,  2,  3,  4,
                      5,  6,  7,  8,
                      9,  10, 11, 12,
                      13, 14, 15, 16),
            0
        }
    }

    #[test]
    fn det_4_copy_tuple() {
        let tuple = ((1,  2,  3,  4 ),
                     (5,  6,  7,  8 ),
                     (9,  10, 11, 12),
                     (13, 14, 15, 16));

        assert_eq! {
            det_copy!(tuple, 4),
            0
        }
    }

    #[test]
    fn det_4_clone() {
        assert_eq! {
            det_clone!(Cloneable(1),  Cloneable(2),  Cloneable(3),  Cloneable(4),
                       Cloneable(5),  Cloneable(6),  Cloneable(7),  Cloneable(8),
                       Cloneable(9),  Cloneable(10), Cloneable(11), Cloneable(12),
                       Cloneable(13), Cloneable(14), Cloneable(15), Cloneable(16)),
            Cloneable(0)
        }
    }

    #[test]
    fn det_4_clone_tuple() {
        let tuple = ((Cloneable(1),  Cloneable(2),  Cloneable(3),  Cloneable(4) ),
                     (Cloneable(5),  Cloneable(6),  Cloneable(7),  Cloneable(8) ),
                     (Cloneable(9),  Cloneable(10), Cloneable(11), Cloneable(12)),
                     (Cloneable(13), Cloneable(14), Cloneable(15), Cloneable(16)));

        assert_eq! {
            det_clone!(tuple, 4),
            Cloneable(0)
        }
    }

    #[test]
    fn det_4() {
        assert_eq! {
            det!(NonCopy(1),  NonCopy(2),  NonCopy(3),  NonCopy(4),
                 NonCopy(5),  NonCopy(6),  NonCopy(7),  NonCopy(8),
                 NonCopy(9),  NonCopy(10), NonCopy(11), NonCopy(12),
                 NonCopy(13), NonCopy(14), NonCopy(15), NonCopy(16)),
            NonCopy(0)
        }
    }

    #[test]
    fn det_4_tuple() {
        let tuple = ((NonCopy(1),  NonCopy(2),  NonCopy(3),  NonCopy(4) ),
                     (NonCopy(5),  NonCopy(6),  NonCopy(7),  NonCopy(8) ),
                     (NonCopy(9),  NonCopy(10), NonCopy(11), NonCopy(12)),
                     (NonCopy(13), NonCopy(14), NonCopy(15), NonCopy(16)));

        assert_eq! {
            det!(tuple, 4),
            NonCopy(0)
        }
    }
}
