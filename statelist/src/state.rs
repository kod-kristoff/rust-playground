use std::convert::From;
use ds_13::list;
use ds_13::unsync::list::{List, reverse};

type Plan<State, A> = Box<dyn Fn(&State) -> (A, State)>;
// type Plan<'a, State, A> = Box<dyn Fn(&State) -> (A, State) + 'a>;
//pub struct Plan<State, A>
//{
//    func: Fn(&State) -> (A, State),
//}
//
//impl<State, A> Plan<State, A> {
//    pub fn run(st)
//}

pub fn make_plan<State, A>(f: &dyn Fn(&State) -> (A, State)) -> Plan<State, A> {
    f
}
pub fn run_plan<State, A>(pl: Plan<State, A>, s: &State) -> (A, State) {
    pl(s)
}

pub fn mreturn<'a, State: Clone, A: Copy + 'a>(a: A) -> Plan<State, A> {
    |s: &State| {
        (a, s.clone())
    }
}

pub fn mbind<'a, State: 'a, A: 'a, B>(pl: &'a Plan<State, A>, k: impl Fn(A) -> Plan<State, B> + 'a) -> Plan<State, B> {
    Box::new(move |s: &State| {
        let (a, s1) = run_plan(Box::new(pl), s);
        let pl_b = k(a);
        run_plan(pl_b, &s1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn select(lst: &List<i32>) -> (i32, List<i32>) {
        match lst.front() {
            None => (-1, List::empty()),
            Some(x) => (*x, lst.popped_front())
        }
    }

    #[test]
    fn run_plan_creates_plan() {
        let lst = list!(1, 2, 3);

        let sel = make_plan(&select);
        let result = run_plan(sel, &lst);

        assert_eq!(result, (1, list!(2, 3)));
    }

    #[test]
    fn mreturn_creates_plan_that_return_pair() {
        let pl = mreturn::<List<i32>, i32>(5);
        let state = list!(1);
        let result = run_plan(pl, &state);

        assert_eq!(result, (5, list!(1)));
    }

    #[test]
    fn mbind_combines_plan() {
        let state = list!(1, 2, 3);

        let sel = make_plan(&select);

        let pl: Plan<List<i32>, (i32, i32)> =
            mbind(&sel, |i| {
                mbind(&sel,  move |j| {
                    mreturn((i, j))
                })
            });

        let result = run_plan(pl, &state);

        assert_eq!(result, ((1, 2), list!(3)));
    }
}
