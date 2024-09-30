#![feature(never_type, unboxed_closures, fn_traits)]

#![allow(incomplete_features)]
#![feature(lazy_type_alias)]

pub trait Variable: Copy + 'static {}
impl<T: Copy + 'static> Variable for T {}

#[derive(Clone, Copy)]
pub struct Imply<P: Variable, Q: Variable>(
    *const dyn Fn(P) -> Q
);
impl<P: Variable, Q: Variable> FnOnce<(P,)> for Imply<P, Q> {
    type Output = Q;
    extern "rust-call" fn call_once(self, (p,): (P,)) -> Self::Output {
        (unsafe {self.0.as_ref()}.unwrap())(p)
    }
}
#[doc(hidden)]
impl<P: Variable, Q: Variable> Imply<P, Q> {
    pub fn that(predicate: impl FnOnce(P) -> Q + Copy + 'static) -> Self {
        Self(&move |p: P| predicate(p))
    }
}
#[macro_export]
/// ```Imply! {(p: P) => p}```
macro_rules! Imply {
    ( ($p:ident: $P:ty) => $q:expr ) => {
        Imply::that(move |$p: $P| $q)
    };
}

#[derive(Clone, Copy)]
pub struct And<P: Variable, Q: Variable> {
    left:  P,
    right: Q
}
impl<P: Variable, Q: Variable> And<P, Q> {
    pub fn introduce(left: P, right: Q) -> Self {
        Self { left, right }
    }

    pub fn eliminate_left(&self) -> P {
        self.left
    }

    pub fn eliminate_right(&self) -> Q {
        self.right
    }
}

#[derive(Clone, Copy)]
pub enum Or<P: Variable, Q: Variable> {
    Left(P),
    Right(Q)
}
impl<P: Variable, Q: Variable> Or<P, Q> {
    pub fn eliminate<R: Variable>(self, left: Imply<P, R>, right: Imply<Q, R>) -> R {
        match self {
            Self::Left(p)  => left(p),
            Self::Right(q) => right(q)
        }
    }
}

pub type Bottom = !;

pub type Not<P: Variable> = Imply<P, Bottom>;

pub type Equivalent<P: Variable, Q: Variable> = And<Imply<P, Q>, Imply<Q, P>>;

mod examples {
    #![allow(non_snake_case, unused)]

    use super::*;

    /// (P → (P → Q)) → (P → Q)
    fn example_imply<P: Variable, Q: Variable>() -> Imply<Imply<P, Imply<P, Q>>, Imply<P, Q>> {
        Imply! {(impPimpPQ: Imply<P, Imply<P, Q>>) =>
            Imply! {(p: P) =>
                impPimpPQ(p)(p)
            }
        }
    }

    /// P ∧ Q → Q ∧ P
    fn example_and<P: Variable, Q: Variable>() -> Imply<And<P, Q>, And<Q, P>> {
        Imply! {(andPQ: And<P, Q>) =>
            And::introduce(
                andPQ.eliminate_right(),
                andPQ.eliminate_left()
            )
        }
    }

    /// P ∨ Q → Q ∨ P
    fn example_or<P: Variable, Q: Variable>() -> Imply<Or<P, Q>, Or<Q, P>> {
        Imply! {(orPQ: Or<P, Q>) =>
            orPQ.eliminate(
                Imply! {(p: P) => Or::Right(p)},
                Imply! {(q: Q) => Or::Left(q)}
            )
        }
    }

    /// P ∨ ⊥ → P
    fn example_bottom<P: Variable>() -> Imply<Or<P, Bottom>, P> {
        Imply! {(orPBottom: Or<P, Bottom>) =>
            orPBottom.eliminate(
                Imply! {(p: P) => p},
                Imply! {(b: Bottom) => b}
            )
        }
    }
}
