/*
    Illustration of Structs and Enums:
    Implementing the game of Connect 4
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
// Copy: the trait implemented by very simple, easily copyable
// data like usize, u64, f64, &str
// Essentially: Copy means "implicitly Clone me wherever needed"
pub enum Player {
    X,
    O,
}

const BOARD_LEN: usize = 10;
const BOARD_HGT: usize = 5;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    board: [Vec<Player>; BOARD_LEN],
    // Alternative: Vec<Vec<Player>>, but
    // the above is technically a bit more accurate as we don't need dynamic
    // modification.
    to_play: Player,
}

// Instead of using the #[derive(...)] we can also implement traits manually
// Default trait: types that have a default value
// For example: default for usize is 0, default for Vec is Vec::new() i.e. vec![]
impl Default for Game {
    fn default() -> Self {
        Self { board: Default::default(), to_play: Player::X }
    }
}

impl Game {
    // pub fn: public API
    // fn: internal API

    pub fn new() -> Self {
        Default::default()
    }

    // Note: we shouldn't require ourselves to take &self
    // as a parameter if it isn't needed! E.g.:
    fn in_range(col: usize, row: usize) -> bool {
        col <= BOARD_LEN && row <= BOARD_HGT
    }

    pub fn get(&self, col: usize, row: usize) -> Option<Player> {
        // Remember:
        // assert!: called in debug and --release mode
        // debug_assert!: only called in debug mode
        debug_assert!(Self::in_range(col, row));
        self.board[col].get(row).cloned()
        // If you have an Option<&Player> and want an Option<Player>
        // then .cloned() is useful!
        // NB: not the same as .clone() !
    }
    pub fn playable(&self, col: usize) -> bool {
        debug_assert!(col <= BOARD_LEN);
        self.board[col].len() < BOARD_HGT
    }
    pub fn play(&mut self, col: usize, player: Player) {
        debug_assert!(col <= BOARD_LEN);
        debug_assert!(self.playable(col));
        self.board[col].push(player);
    }

    /*
        Useful to know: implementing lightweight iterators over your
        data structures
    */

    // pub fn valid_plays_1(&self) -> Vec<usize> {
    //     let mut results = Vec::new();
    //     for i in 0..BOARD_LEN {
    //         if self.playable(i) {
    //             results.push(i);
    //         }
    //     }
    //     results
    // }

    // In Rust, every for loop is implicitly what's called an Iterator
    // Iterator is a fancy trait, just like Copy, Clone, Debug, etc.
    // The way to use iterators: return 'impl Iterator<...> + '_'
    // Don't worry about the '_ for now, but it tells Rust to try
    // to figure out a valid lifetime for the result.
    pub fn valid_plays(&self) -> impl Iterator<Item = usize> + '_ {
        (0..BOARD_LEN).filter(move |&i| self.playable(i))
        // Doesn't actually call self.playable(i) on any i when the
        // function is called; it only calls self.playable when the
        // function is used.
    }

    pub fn example_use_valid_plays(&self) {
        let valid = self.valid_plays(); // No work is done here
        for play in valid {
            // All the work is done here
            println!("{:?}", play);
        }
    }

    fn cells() -> impl Iterator<Item = (usize, usize)> {
        // No need to take &self as a parameter if it's not needed.
        // This would be called like Self::cells()
        // rather than self.cells()
        (0..BOARD_LEN).flat_map(|i| (0..BOARD_HGT).map(move |j| (i, j)))
        // syntax: move |j| (i, j)
        // This is called a closure. It's like a lambda or anonymous
        // function in other languages.
        // The "move" indicates that its a closure that takes ownership
        // over things. ====> practically speaking, if you try to define
        // a closure and you run into a compiler error, try adding 'move'
        // before the closure as it often solves issues.
    }

    fn blocks_of_four() -> impl Iterator<Item = [(usize, usize); 4]> {
        let horiz = Self::cells()
            .map(|(i, j)| [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)]);
        let vert = Self::cells()
            .map(|(i, j)| [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)]);
        let diag1 = Self::cells().map(|(i, j)| {
            [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)]
        });
        let diag2 = Self::cells().map(|(i, j)| {
            [(i, j + 3), (i + 1, j + 2), (i + 2, j + 1), (i + 3, j)]
        });
        horiz
            .chain(vert)
            .chain(diag1)
            .chain(diag2)
            .filter(|&blck| Self::in_range(blck[0].0, blck[0].1))
            .filter(|&blck| Self::in_range(blck[3].0, blck[3].1))
    }

    pub fn winner(&self) -> Option<Player> {
        for blck in Self::blocks_of_four() {
            for &player in &[Player::X, Player::O] {
                if blck.iter().all(|&(i, j)| self.get(i, j) == Some(player)) {
                    return Some(player);
                }
            }
        }
        None
    }
}
