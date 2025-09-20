// impl<'a, Digit: Value> Mul for &'a PadicInteger<'a, Digit> {
//     type Output = AdditionPadicInteger<'a, Digit>;
//
//     fn mul(self, rhs: Self) -> Self::Output {
//         AdditionPadicInteger::new(self, rhs)
//     }
// }
//
// impl<'a, Digit: Value> Div for Rc<dyn PadicAccessor<'a, Digit> + 'a> {
//     type Output = SubtractionPadicInteger<'a, Digit>;
//
//     fn div(self, rhs: Self) -> Self::Output {
//         // struct Check<'a, Digit: Invertible>(PhantomData<'a, Digit>);
//         // #[const_trait]
//         // trait Checker {
//         //     const INVERTIBLE_P: ();
//         // }
//         // impl<'a, Digit: ~const Invertible> const Checker for Check<'a, Digit> {
//         //     const INVERTIBLE_P: () = assert!(
//         //         Digit::is_base_invertible(),
//         //         "P must be prime in order to use division"
//         //     );
//         // }
//         // let _ = Check::<'a, Digit>::INVERTIBLE_P;
//         todo!()
//     }
// }
