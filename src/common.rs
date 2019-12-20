pub trait Advent {
    fn advent_number() -> u8;
    fn main1(input: &Vec<String>) -> String;
    fn main2(input: &Vec<String>) -> String;
}

pub fn read_input(filename: &str) -> Vec<String> {
    let file = std::fs::File::open(filename).unwrap();
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf)
        .map(|line| line.unwrap())
        .collect()
}

pub mod dijkstra {
    pub fn search<State, GetReprState, ReprState, Expand, EIter>(
        initial_state: State,
        target_repr_state: &ReprState,
        repr_state: GetReprState,
        expand: Expand,
    ) -> Option<Vec<ReprState>>
    where
        GetReprState: FnMut(&State) -> ReprState,
        ReprState: Eq + std::hash::Hash + Clone,
        Expand: FnMut(State) -> EIter,
        EIter: IntoIterator<Item = (State, usize)>,
    {
        let result = dijkstra(initial_state, repr_state, expand, |r| {
            r == target_repr_state
        });
        let mut path = result
            .iterate(target_repr_state)?
            .map(|t| t.clone())
            .collect::<Vec<_>>();
        path.reverse();
        path.push(target_repr_state.clone());
        Some(path)
    }

    pub fn all<State, GetReprState, ReprState, Expand, EIter>(
        initial_state: State,
        repr_state: GetReprState,
        expand: Expand,
    ) -> PathfindingResult<ReprState>
    where
        GetReprState: FnMut(&State) -> ReprState,
        ReprState: Eq + std::hash::Hash + Clone,
        Expand: FnMut(State) -> EIter,
        EIter: IntoIterator<Item = (State, usize)>,
    {
        dijkstra(initial_state, repr_state, expand, |_| false)
    }

    pub fn dijkstra<State, GetReprState, ReprState, Expand, EIter, ShouldStop>(
        initial_state: State,
        mut repr_state: GetReprState,
        mut expand: Expand,
        mut should_stop: ShouldStop,
    ) -> PathfindingResult<ReprState>
    where
        GetReprState: FnMut(&State) -> ReprState,
        ReprState: Eq + std::hash::Hash + Clone,
        Expand: FnMut(State) -> EIter,
        EIter: IntoIterator<Item = (State, usize)>,
        ShouldStop: FnMut(&ReprState) -> bool,
    {
        let mut parent_and_cost = std::collections::HashMap::<ReprState, (ReprState, usize)>::new();
        let mut queue = super::VecBinaryMinHeap::new();

        let initial_repr = repr_state(&initial_state);
        parent_and_cost.insert(initial_repr.clone(), (initial_repr, 0));
        queue.insert(CmpCost(initial_state, 0));

        while let Some(CmpCost(current_state, current_cost)) = queue.pop() {
            let current_repr = repr_state(&current_state);
            if should_stop(&current_repr) {
                break;
            }
            if parent_and_cost[&current_repr].1 != current_cost {
                continue;
            }

            for (next_state, step_cost) in expand(current_state) {
                let next_repr = repr_state(&next_state);
                let next_cost = current_cost + step_cost;
                match parent_and_cost.entry(next_repr.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut x) => {
                        if x.get().1 > next_cost {
                            *x.get_mut() = (current_repr.clone(), next_cost);
                            queue.insert(CmpCost(next_state, next_cost));
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(x) => {
                        x.insert((current_repr.clone(), next_cost));
                        queue.insert(CmpCost(next_state, next_cost));
                    }
                }
            }
        }
        PathfindingResult {
            parent_and_cost: parent_and_cost,
        }
    }

    #[derive(Debug)]
    pub struct PathfindingResult<T>
    where
        T: std::cmp::Eq + std::hash::Hash,
    {
        parent_and_cost: std::collections::HashMap<T, (T, usize)>,
    }
    impl<T> PathfindingResult<T>
    where
        T: Eq + std::hash::Hash + std::cmp::Eq,
    {
        pub fn get(&self, t: &T) -> Option<&(T, usize)> {
            self.parent_and_cost.get(t)
        }
        pub fn get_cost(&self, t: &T) -> Option<usize> {
            self.get(t).map(|(_, c)| *c)
        }
        pub fn iterate(&self, t: &T) -> Option<impl Iterator<Item = &T>> {
            ParentIterator::new(self, t)
        }
    }

    struct ParentIterator<'a, T>
    where
        T: std::cmp::Eq + std::hash::Hash,
    {
        result: &'a PathfindingResult<T>,
        current: &'a T,
        done: bool,
    }
    impl<'a, T> ParentIterator<'a, T>
    where
        T: Eq + std::hash::Hash + std::cmp::Eq,
    {
        fn new(result: &'a PathfindingResult<T>, start: &T) -> Option<Self> {
            result.get(start).map(|r| Self {
                result: result,
                current: &r.0,
                done: false,
            })
        }
    }
    impl<'a, T> Iterator for ParentIterator<'a, T>
    where
        T: Eq + std::hash::Hash,
    {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.done {
                return None;
            }
            let ret = self.current;
            if let Some((next, _)) = self.result.get(self.current) {
                if self.current == next {
                    self.done = true;
                }
                self.current = next;
            }
            Some(ret)
        }
    }

    struct CmpCost<T>(T, usize);

    impl<T> PartialOrd for CmpCost<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.1.cmp(&other.1))
        }
    }
    impl<T> PartialEq for CmpCost<T> {
        fn eq(&self, other: &Self) -> bool {
            self.1 == other.1
        }
    }
}

pub struct VecBinaryMinHeap<T> {
    storage: Vec<T>,
}

impl<T> VecBinaryMinHeap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { storage: vec![] }
    }
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn insert(&mut self, t: T) {
        let index = self.len();
        self.storage.push(t);
        self.percolate_down(index);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.len();
        if len > 0 {
            self.storage.swap(0, len - 1);
            let t = self.storage.pop();
            self.percolate_up(0);
            t
        } else {
            None
        }
    }

    fn percolate_down(&mut self, mut index: usize) {
        while index > 0 && self.storage[index] > self.storage[Self::parent(index)] {
            self.storage.swap(index, Self::parent(index));
            index = Self::parent(index);
        }
    }
    fn percolate_up(&mut self, mut index: usize) {
        while let Some(next_index) = self.next_child(index) {
            if self.storage[next_index] < self.storage[index] {
                self.storage.swap(index, next_index);
                index = next_index;
            } else {
                break;
            }
        }
    }
    fn next_child(&self, index: usize) -> Option<usize> {
        let (a, b) = Self::children(index);
        if a < self.len() && b < self.len() {
            Some(if self.storage[a] < self.storage[b] {
                a
            } else {
                b
            })
        } else if a < self.len() {
            Some(a)
        } else {
            None
        }
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }
    fn children(index: usize) -> (usize, usize) {
        (2 * index + 1, 2 * index + 2)
    }
}
