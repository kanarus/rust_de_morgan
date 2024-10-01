macro_rules! static_assert {
    ($condition:expr) => {
        {
            const _: bool = $condition;
            const _: [(); $condition as usize] = [(); true as usize];
        }
    };
}

macro_rules! for_all_cominations_of_two_bools {
    (($P:ident, $Q:ident) => $proc:expr) => {
        for_all_cominations_of_two_bools! {
            @($P, $Q) in checked[
                (true,  true ),
                (true,  false),
                (false, true ),
                (false, false)
            ] {
                $proc
            }
        }
    };
    (@($P:ident, $Q:ident) in checked[$( ($p:literal, $q:literal) ),*] $proc:expr) => {
        fn __assert_exausted__(p: bool, q: bool) {
            match (p, q) {$(
                ($p, $q) => {
                    const $P: bool = $p;
                    const $Q: bool = $q;
                    $proc
                },
            )*}
        }
    };
}

const fn main() {
    for_all_cominations_of_two_bools! {(P, Q) => {
        static_assert!((!(P || Q)) == (!P && !Q));
        static_assert!((!(P && Q)) == (!P || !Q));
    }}
}
