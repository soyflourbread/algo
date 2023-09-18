pub struct DisjointSet {
    parents: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let parents = (0..n)
            .collect::<Vec<_>>();
        Self { parents }
    }

    pub fn query(&mut self, token: usize) -> usize {
        assert!(token < self.parents.len());

        let token_ne = self.parents[token];
        if token_ne == token {
            return token;
        } // self-representative

        let token_ne = self.query(token_ne);
        self.parents[token] = token_ne; // path compression

        token_ne
    }

    pub fn link(&mut self, token_0: usize, token_1: usize) {
        let token_0 = self.query(token_0);
        let token_1 = self.query(token_1);
        if token_0 == token_1 {
            return; // same subset
        }

        // token_1 is no longer a representative
        self.parents[token_1] = token_0;
    }
}

fn main() {
    let mut set = DisjointSet::new(8);

    set.link(0, 1);
    set.link(1, 2);

    set.link(4, 3);
    set.link(3, 5);
    set.link(3, 6);

    println!(
        "subset of 0..8: {:?}",
        (0..8)
            .map(|e| set.query(e))
            .collect::<Vec<_>>()
    );
}
