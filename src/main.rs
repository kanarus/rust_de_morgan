#![allow(non_snake_case)]

use rust_de_morgan::*;

/// ¬(P ∨ Q) ↔ ¬P ∧ ¬Q
fn __de_morgan_s_law_1__<P: Variable, Q: Variable>() ->
    Equivalent<Not<Or<P, Q>>, And<Not<P>, Not<Q>>>
{
    /// ¬(P ∨ Q) → ¬P ∧ ¬Q
    fn left<P: Variable, Q: Variable>() ->
        Imply<Not<Or<P, Q>>, And<Not<P>, Not<Q>>>
    {
        Imply! {(notOrPQ: Not<Or<P, Q>>) =>
            And::introduce(
                Imply! {(p: P) => notOrPQ(Or::Left(p))},
                Imply! {(q: Q) => notOrPQ(Or::Right(q))}
            )
        }
    }

    /// ¬(P ∨ Q) → ¬P ∧ ¬Q
    fn right<P: Variable, Q: Variable>() ->
        Imply<And<Not<P>, Not<Q>>, Not<Or<P, Q>>>
    {
        Imply! {(andNotPNotQ: And<Not<P>, Not<Q>>) =>
            Imply! {(orPR: Or<P, Q>) =>
                orPR.eliminate(
                    Imply! {(p: P) => andNotPNotQ.eliminate_left()(p)},
                    Imply! {(q: Q) => andNotPNotQ.eliminate_right()(q)}
                )
            }
        }
    }

    And::introduce(left(), right())
}

/// ¬(P ∧ Q) ↔ ¬P ∨ ¬Q
fn __de_morgan_s_law_2__<P: Variable, Q: Variable>() ->
    Equivalent<Not<And<P, Q>>, Or<Not<P>, Not<Q>>>
{
    /// ¬(P ∧ Q) ← ¬P ∨ ¬Q
    fn right<P: Variable, Q: Variable>() ->
        Imply<Or<Not<P>, Not<Q>>, Not<And<P, Q>>>
    {
        Imply! {(orNotPNotQ: Or<Not<P>, Not<Q>>) =>
            orNotPNotQ.eliminate(
                Imply! {(notP: Not<P>) =>
                    Imply! {(andPQ: And<P, Q>) =>
                        notP(andPQ.eliminate_left())
                    }
                },
                Imply! {(notQ: Not<Q>) =>
                    Imply! {(andPQ: And<P, Q>) =>
                        notQ(andPQ.eliminate_right())
                    }
                }
            )
        }
    }

    /// ¬(P ∧ Q) → ¬P ∨ ¬Q
    fn left<P: Variable, Q: Variable>() ->
        Imply<Not<And<P, Q>>, Or<Not<P>, Not<Q>>>
    {
        /// P ∨ ¬P
        fn excluded_middle<P: Variable>() -> Or<P, Not<P>> {
            unreachable!("axios")
        }

        Imply! {(notAndPQ: Not<And<P, Q>>) =>
            excluded_middle::<P>().eliminate(
                Imply! {(p: P) =>
                    Or::Right(
                        Imply! {(q: Q) =>
                            notAndPQ(And::introduce(p, q))
                        }
                    )
                },
                Imply! {(notP: Not<P>) =>
                    Or::Left(notP)
                }
            )
        }
    }

    And::introduce(left(), right())
}

fn main() {
    println!("De Morgan's laws are proven.");
}
