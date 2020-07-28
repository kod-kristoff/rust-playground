use std::rc::Rc;
use ds_13::list::{concat_all, fmap, List};

pub type PairList<State, A> = List<(A, State)>;

pub type StateList<'a, State, A> = Rc<dyn Fn(&State) -> PairList<State, A> + 'a>;

pub fn make_state_list<'a, State, A>(f: &'a dyn Fn(&State) -> PairList<State, A>) -> StateList<'a, State, A> {
        Rc::new(f)
    }

pub fn run_state_list<State, A>(st: &StateList<State, A>, s: &State) -> PairList<State, A> {
    st(s)
}

pub fn eval_state_list<State, A: Copy>(st: &StateList<State, A>, s: &State) -> List<A> {
    fmap(|(a, _state)| *a, &st(s))
}

// StateList Monad
pub fn mreturn<'a, State: Clone, A: Copy + 'a>(a: A) -> StateList<'a, State, A> {
    Rc::new(
        move |s: &State| PairList::from_value((a, s.clone()))
    )
}

pub fn mbind<'a, State: Clone + 'a, A: Clone + 'a, B: Clone>(g: StateList<'a, State, A>, k: impl Fn(A) -> StateList<'a, State, B> + 'a) -> StateList<'a, State, B> {
    Rc::new(
        move |s: &State| {
            let plst = g(s);
            let lst2 = fmap(|(a, s1): &(A, State)| {
                    let ka = k(a.clone());
                    run_state_list(&ka, &s1)
                },
                &plst
            );
            concat_all(&lst2)
        }
    )
}

pub fn mthen<'a, State: Clone + 'a, A: Copy + 'a, B: Clone>(g: StateList<'a, State, A>, k: impl Fn(A) -> StateList<'a, State, B> + 'a) -> StateList<'a, State, B> {
    Rc::new(
        move |s: &State| {
            let plst = g(s);
            let lst2 = fmap(|(a, s1): &(A, State)| {
                    let ka = k(a.clone());
                    run_state_list(&ka, &s1)
                },
                &plst
            );
            concat_all(&lst2)
        }
    )
}

pub fn mzero<'a, State, A>() -> StateList<'a, State, A> {
    Rc::new(move |_s: &State| PairList::empty())
}

pub fn guard<'a, State: Clone>(b: bool) -> StateList<'a, State, ()> {
    if b {
        Rc::new(move |s: &State| List::from_value(((), s.clone())))
    } else {
        mzero::<State, ()>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_13::list;
    use ds_13::list::{reverse};

    fn select(lst: &List<i32>) -> PairList<List<i32>, i32> {
        match lst.front() {
            None => PairList::empty(),
            Some(x) => {

                let mut result = PairList::empty();
                for (y, ys) in select(&lst.pop_front()).into_iter() {
                    result = result.push_front((*y, ys.push_front(*x)))
                }
                result.push_front((*x, lst.pop_front().clone()))
            }

        }
    }

    #[test]
    fn select_w_empty_list_returns_empty_pairlist() {
        let lst = List::<i32>::empty();

        let plst = select(&lst);

        assert!(plst.is_empty());
    }
    
    #[test]
    fn select_w_singleton_list_returns_singleton_pairlist() {
        let lst = list!(2);

        let plst = select(&lst);

        assert_eq!(
            plst,
            list!((2, List::<i32>::empty()))
        );
    }

    #[test]
    fn select_w_list_returns_pairlist_of_same_size() {
        let lst = list!(1, 2);

        let plst = select(&lst);

        assert_eq!(
            plst,
            list!(
                (1, list!(2)),
                (2, list!(1))
            )
        );

        let lst2 = list!(1, 2, 3);
        let plst2 = select(&lst2);

        assert_eq!(
            plst2,
            list!(
                (1, list!(2, 3)),
                (3, list!(1, 2)),
                (2, list!(1, 3))
            )
        );
        
    }
    #[test]
    fn run_state_list_creates_something() {
        let state = list!(1, 2, 3);

        let sel = make_state_list(&select);

        let plst = run_state_list(&sel, &state);

        assert_eq!(
            plst,
            list!(
                (1, list!(2, 3)),
                (3, list!(1, 2)),
                (2, list!(1, 3))
            )
        );

        assert_eq!(
            eval_state_list(&sel, &state),
            list!(2, 3, 1)
        );
    } 

    #[test]
    fn mreturn_creates_statelist_that_can_run() {
        let sl = mreturn::<List<i32>, i32>(77);

        let state = list!(1, 2, 3);

        let plst = run_state_list(&sl, &state);

        assert_eq!(
            plst,
            list!((77, list!(1, 2, 3)))
        );

        assert_eq!(eval_state_list(&sl, &state), list!(77));
    }

    #[test]
    fn mbind_combines_statelists_that_can_run() {
        let sel = make_state_list(&select);

        let sl = 
            mbind(make_state_list(&select), |i| 
                mbind(make_state_list(&select), move |j|
                    mreturn((i, j))
            )
        );

        let state = list!(1, 2, 3);

        assert_eq!(
            run_state_list(&sl, &state),
            list!(
                (
                    (1, 3),
                    list!(2)
                ),
                (
                    (1, 2),
                    list!(3)
                ),
                (
                    (3, 2),
                    list!(1)
                ),
                (
                    (3, 1),
                    list!(2)
                ),
                (
                    (2, 3),
                    list!(1)
                ),
                (
                    (2, 1),
                    list!(3)
                )
            )
        );
    }

    #[test]
    fn mzero_returns_empty_list() {
        let sl = mzero::<List<i32>, i32>();
        let state = list!(1, 2, 3);

        assert_eq!(
            run_state_list(&sl, &state),
            PairList::<List<i32>, i32>::empty()
        );

        assert_eq!(
            eval_state_list(&sl, &state),
            List::<i32>::empty()
        );
    }

    #[test]
    fn guard_called_with_false_returns_mzero() {
        let g = guard(false);

        let state = list!(1, 2, 3);

        assert_eq!(
            run_state_list(&g, &state),
            PairList::<List<i32>, ()>::empty()
        );

        assert_eq!(
            eval_state_list(&g, &state),
            List::<()>::empty()
        );
    }

    #[test]
    fn guard_called_with_true_returns_unit_and_state() {
        let g = guard(true);

        let state = list!(1, 2, 3);

        assert_eq!(
            run_state_list(&g, &state),
            list!(((), list!(1, 2, 3)))
        );

        assert_eq!(
            eval_state_list(&g, &state),
            list!(())
        );
    }

    #[test]
    fn mbind_combined_with_guard_returns_pairlist() {
        let sel = make_state_list(&select);

        let sl = mbind(make_state_list(&select), |i|
            mbind(make_state_list(&select), move |j| {
                mthen(guard(i + j != 3), move |_| 
                    mreturn((i, j))
                )
            })
        );

        let state = list!(1, 2, 3);

        assert_eq!(
            run_state_list(&sl, &state),
            list!(
                (
                    (1, 3),
                    list!(2)
                ),
                (
                    (3, 2),
                    list!(1)
                ),
                (
                    (3, 1),
                    list!(2)
                ),
                (
                    (2, 3),
                    list!(1)
                )
            )
        );
    }

    #[test]
    fn mbind_combined_more_with_guard_returns_pairlist() {
        let sel = make_state_list(&select);

        let sl = mbind(make_state_list(&select), |i|
            mbind(make_state_list(&select), move |j| 
                mbind(make_state_list(&select), move |k| 
                mthen(guard(i + j == 3), move |_| 
                    mreturn((i + j, k))
                )
            )
        ));

        let state = list!(1, 2, 3);

        assert_eq!(
            run_state_list(&sl, &state),
            list!(
                (
                    (3, 3),
                    List::empty()
                ),
                (
                    (3, 3),
                    List::empty()
                )
            )
        );    
    }
}
