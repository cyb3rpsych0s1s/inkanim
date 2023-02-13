/// belongs to the same level or is nested below, in a tree hierarchy
pub trait SameOrNested {
    fn same_or_nested(&self, searched: &[usize]) -> bool;
}

impl SameOrNested for Vec<usize> {
    fn same_or_nested(&self, searched: &[usize]) -> bool {
        let count_own = self.len();
        let count_searched = searched.len();
        if count_searched == 0 {
            return true;
        }
        let last_searched = count_searched - 1;
        for (i, path_index) in self.iter().enumerate() {
            if *path_index != searched[i] {
                return false;
            }
            if i == last_searched && count_own >= count_searched {
                return true;
            }
        }
        false
    }
}
