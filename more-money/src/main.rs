use ds_13::{
    list,
    unsync::{
        list::{List, reverse}
    },
};
use persi_ds::{
    pair::{Pair, make_pair},
};
use statelist::{
    state_list::{
        eval_state_list,
        make_state_list,
        mbind,
        mreturn,
        mthen,
        guard,
        PairList,
        StateList
    },
};
use std::fmt;

fn main() {

    solve_constraints();
}

fn solve_constraints() {
    let lst = list!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

    let sel = make_state_list(&select::<i32>);

    let solve = mbind(&sel, |s|
    mbind(&sel, move |e|
    mbind(&sel, move |n|
    mbind(&sel, move |d|
    mbind(&sel, move |m|
    mbind(&sel, move |o|
    mbind(&sel, move |r|
    mbind(&sel, move |y|
        mthen(guard(s != 0 && m != 0), move |_| {
            let send = as_number(&vec!(s, e, n, d));
            let more = as_number(&vec!(m, o, r, e));
            let money = as_number(&vec!(m, o, n, e, y));
            mthen(guard(send + more == money), |_|
                mreturn(make_triple(send, more, money)))
          })
    ))))))));
    let solution = eval_state_list(&solve, &lst);
    println!("{}", solution);
}

// fn solve<'a>() -> persi_ds::state_list::StateList<'a, List<i32>, Triple<i32, i32, i32>> {
//     let sel = make_state_list(&select::<i32>);
//
//     mbind(&sel, |s|
//     mbind(&sel, |e|
//     mbind(&sel, move |n|
//     mbind(&sel, move |d|
//     mbind(&sel, move |m|
//     mbind(&sel, move |o|
//     mbind(&sel, move |r|
//     mbind(&sel, move |y|
//         mthen(guard(s != 0 && m != 0), move |_| {
//             let send = as_number(&vec!(s, e, n, d));
//             let more = as_number(&vec!(m, o, r, e));
//             let money = as_number(&vec!(m, o, n, e, y));
//             mthen(guard(send + more == money), |_|
//                 mreturn(make_triple(send, more, money)))
//           })
//     ))))))))
// }


fn select<A: Copy>(lst: &List<A>) -> PairList<List<A>, A> {
    match lst.front() {
        None => PairList::empty(),
        Some(x) => {
            let mut result = PairList::empty();
            for p in select(&lst.popped_front()).into_iter() {
                let y = p.first;
                // let ys = p.second;
                result = result.pushed_front(make_pair(y, p.second.pushed_front(*x)))
            }
            result.pushed_front(make_pair(*x, lst.popped_front().clone()))
        }
    }
}

fn as_number(v: &[i32]) -> i32 {
    let mut acc = 0;
    for i in v {
        acc = 10 * acc + i;
    }
    acc
}

struct Triple<T, U, V> {
    t: T,
    u: U,
    v: V,
}

impl<T: Copy, U: Copy, V: Copy> Copy for Triple<T, U, V> {}

impl<T: Copy, U: Copy, V: Copy> Clone for Triple<T, U, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: fmt::Display> fmt::Display for Triple<T, T, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.t, self.u, self.v)
    }
}

fn make_triple<T, U, V>(t: T, u: U, v: V) -> Triple<T, U, V> {
    Triple { t: t, u: u, v: v }
}

