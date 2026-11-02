//! Snake on a torus: the game of prototype 10 (`main.S`, RV32I for the
//! ESP32-C3-LCDkit's round panel and knob), step for step, with no hardware
//! in it. `render` draws it on a rectangular screen; the binary's `main.rs`
//! reads the remote and the framebuffer.
//!
//! The world is SECTORS slices round a circle by RINGS bands from the inner
//! edge out. Both directions wrap, so it is a torus. Cell number is
//! `ring * SECTORS + sector`. The disc inside the inner edge is UI: grey while
//! playing, red after a crash. A new apple appears every APPLE_STEPS steps,
//! at the next cell of a fixed pattern (skipped if that cell is taken).
//!
//! What `main.S` did with registers and RAM, this does with a struct:
//!
//! | main.S                 | here                       |
//! |------------------------|----------------------------|
//! | GRID (CELLS + 2 bytes) | `grid`                     |
//! | SNAKE ring buffer      | `snake`, `tail`, `head`    |
//! | s8, s9, s10            | `head_cell`, `sector`, `ring` |
//! | s11 heading            | `heading`                  |
//! | a6 turn from the knob  | `turn`                     |
//! | a4, a5 apples          | `apple_countdown`, `apple_cell` |
//! | `frame`                | [`Game::step`]             |
//! | `new_game`             | [`Game::new`]              |
//! | `game_over`            | `over == true`             |

pub mod render;

pub const SECTORS: usize = 60;
pub const RINGS: usize = 15;
pub const CELLS: usize = SECTORS * RINGS;
/// GRID entry drawn inside the inner edge.
pub const UI_CELL: usize = CELLS;
/// GRID entry drawn beyond the outer edge.
pub const OUTSIDE_CELL: usize = CELLS + 1;

/// What a GRID entry holds; also the index into the palette.
pub const EMPTY: u8 = 0;
pub const BODY: u8 = 1;
pub const HEAD: u8 = 2;
pub const APPLE: u8 = 3;
pub const BLACK: u8 = 4;
pub const UI_PLAY: u8 = 5;
pub const UI_OVER: u8 = 6;

pub const START_RING: usize = RINGS / 2;
/// The tail at the start; the snake is 3 cells clockwise.
pub const START_CELL: usize = START_RING * SECTORS;
/// SNAKE ring buffer: a power of two >= CELLS.
pub const SLOTS: usize = 1024;
/// A new apple every 3 s.
pub const APPLE_STEPS: u32 = 24;
/// Coprime with CELLS: the pattern visits every cell.
pub const APPLE_STRIDE: usize = 337;
/// Frame period: 2,000,000 SYSTIMER ticks at 16 MHz.
pub const PERIOD_MS: u64 = 125;

/// Headings, in the original order: turning left adds 1, turning right adds 3.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Heading {
    Clockwise = 0,
    Out = 1,
    Anticlockwise = 2,
    In = 3,
}

impl Heading {
    fn from_u8(v: u8) -> Heading {
        match v & 3 {
            0 => Heading::Clockwise,
            1 => Heading::Out,
            2 => Heading::Anticlockwise,
            _ => Heading::In,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Heading::Clockwise => "clockwise",
            Heading::Out => "out",
            Heading::Anticlockwise => "anticlockwise",
            Heading::In => "in",
        }
    }
}

/// One detent of the knob; here, one press of Left or Right.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    /// LEFT = 1, RIGHT = 3 in `main.S`.
    fn delta(self) -> u8 {
        match self {
            Turn::Left => 1,
            Turn::Right => 3,
        }
    }
}

/// What one step did, for the log and the tests. The display does not need
/// it: it repaints the whole grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Moved one cell; the tail followed.
    Moved,
    /// Ate the apple in the next cell; the tail stayed.
    Ate,
    /// Ran into the body: game over.
    Crashed,
}

pub struct Game {
    /// What is in each cell, plus the two UI entries.
    pub grid: [u8; CELLS + 2],
    /// The body, tail to head, wrapping.
    snake: [u16; SLOTS],
    tail: usize,
    head: usize,
    pub head_cell: usize,
    pub sector: usize,
    pub ring: usize,
    pub heading: Heading,
    /// The turn to apply at the next step; the latest one wins.
    pub turn: Option<Turn>,
    apple_countdown: u32,
    apple_cell: usize,
    pub over: bool,
    /// Steps taken this game.
    pub steps: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// `new_game`: every cell empty, a 3-cell snake halfway out heading clockwise.
    pub fn new() -> Game {
        Game::from_cells(&[START_CELL, START_CELL + 1, START_CELL + 2], Heading::Clockwise)
    }

    /// A snake along `cells` (tail first, consecutive cells adjacent), the
    /// head heading `heading`, nothing else in the world. `new` is the
    /// 3-cell case; longer snakes are for tests and previews.
    pub fn from_cells(cells: &[usize], heading: Heading) -> Game {
        assert!(!cells.is_empty() && cells.len() <= SLOTS);
        let mut g = Game {
            grid: [EMPTY; CELLS + 2],
            snake: [0; SLOTS],
            tail: 0,
            head: cells.len() - 1,
            head_cell: 0,
            sector: 0,
            ring: 0,
            heading,
            turn: None,
            apple_countdown: APPLE_STEPS,
            apple_cell: 0,
            over: false,
            steps: 0,
        };
        g.grid[UI_CELL] = UI_PLAY;
        g.grid[OUTSIDE_CELL] = BLACK;
        for (i, &c) in cells.iter().enumerate() {
            assert!(c < CELLS, "cell {c} is off the torus");
            if i > 0 {
                debug_assert!(adjacent(cells[i - 1], c), "cells {} and {c} are not adjacent", cells[i - 1]);
            }
            g.snake[i] = c as u16;
            g.grid[c] = BODY;
        }
        g.head_cell = cells[cells.len() - 1];
        g.grid[g.head_cell] = HEAD;
        g.sector = g.head_cell % SECTORS;
        g.ring = g.head_cell / SECTORS;
        g
    }

    /// The knob turned: remember it for the next step. A second turn before
    /// then replaces the first, as `a6` did.
    pub fn turn(&mut self, t: Turn) {
        self.turn = Some(t);
    }

    /// Cells of the snake in the world.
    pub fn length(&self) -> usize {
        (self.head + SLOTS - self.tail) % SLOTS + 1
    }

    /// The body, tail first.
    pub fn cells(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.length()).map(move |i| self.snake[(self.tail + i) % SLOTS] as usize)
    }

    /// The cell the apple pattern visited last (`a5`), where the next apple
    /// may have been placed.
    pub fn apple_cell(&self) -> usize {
        self.apple_cell
    }

    /// `frame`: turn, move one cell round the torus, eat or let the tail
    /// follow, maybe crash; then maybe a new apple. Nothing happens after a
    /// crash (the original left the frame schedule).
    pub fn step(&mut self) -> Option<Outcome> {
        if self.over {
            return None;
        }
        self.steps += 1;
        if let Some(t) = self.turn.take() {
            self.heading = Heading::from_u8(self.heading as u8 + t.delta());
        }
        // The next cell.
        let (mut sector, mut ring) = (self.sector, self.ring);
        match self.heading {
            Heading::Clockwise => sector = (sector + 1) % SECTORS,
            Heading::Out => ring = (ring + 1) % RINGS,
            Heading::Anticlockwise => sector = (sector + SECTORS - 1) % SECTORS,
            Heading::In => ring = (ring + RINGS - 1) % RINGS,
        }
        let next = ring * SECTORS + sector;

        let tail_cell = self.snake[self.tail] as usize;
        let outcome = if self.grid[next] == APPLE {
            Outcome::Ate // the tail stays: one cell longer
        } else {
            // Not eating: the tail moves out of the way first, so following
            // it round is allowed.
            self.grid[tail_cell] = EMPTY;
            self.tail = (self.tail + 1) % SLOTS;
            if self.grid[next] != EMPTY {
                // Body: crashed. The world stays as it is, tail gone, head
                // unmoved, and the UI disc turns red.
                self.over = true;
                self.grid[UI_CELL] = UI_OVER;
                return Some(Outcome::Crashed);
            }
            Outcome::Moved
        };
        self.grid[self.head_cell] = BODY;
        self.head = (self.head + 1) % SLOTS;
        self.snake[self.head] = next as u16;
        self.grid[next] = HEAD;
        self.head_cell = next;
        self.sector = sector;
        self.ring = ring;

        // Apples: every APPLE_STEPS steps, the next cell of the pattern, if
        // it is empty.
        self.apple_countdown -= 1;
        if self.apple_countdown == 0 {
            self.apple_countdown = APPLE_STEPS;
            self.apple_cell = (self.apple_cell + APPLE_STRIDE) % CELLS;
            if self.grid[self.apple_cell] == EMPTY {
                self.grid[self.apple_cell] = APPLE;
            }
        }
        Some(outcome)
    }
}

/// Whether two cells share an edge on the torus.
pub fn adjacent(a: usize, b: usize) -> bool {
    let (sa, ra) = (a % SECTORS, a / SECTORS);
    let (sb, rb) = (b % SECTORS, b / SECTORS);
    let ds = (sa + SECTORS - sb) % SECTORS;
    let dr = (ra + RINGS - rb) % RINGS;
    (dr == 0 && (ds == 1 || ds == SECTORS - 1)) || (ds == 0 && (dr == 1 || dr == RINGS - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(g: &mut Game, n: usize) -> Vec<Outcome> {
        (0..n).filter_map(|_| g.step()).collect()
    }

    #[test]
    fn new_game_is_the_original_start() {
        let g = Game::new();
        assert_eq!(g.length(), 3);
        assert_eq!(g.cells().collect::<Vec<_>>(), vec![420, 421, 422]);
        assert_eq!((g.head_cell, g.sector, g.ring), (422, 2, 7));
        assert_eq!(g.heading, Heading::Clockwise);
        assert_eq!(g.grid[420], BODY);
        assert_eq!(g.grid[422], HEAD);
        assert_eq!(g.grid[UI_CELL], UI_PLAY);
        assert_eq!(g.grid[OUTSIDE_CELL], BLACK);
        assert_eq!(g.grid[..CELLS].iter().filter(|&&v| v != EMPTY).count(), 3);
    }

    #[test]
    fn clockwise_moves_one_sector_and_the_tail_follows() {
        let mut g = Game::new();
        assert_eq!(g.step(), Some(Outcome::Moved));
        assert_eq!(g.cells().collect::<Vec<_>>(), vec![421, 422, 423]);
        assert_eq!(g.grid[420], EMPTY);
        assert_eq!(g.grid[422], BODY);
        assert_eq!(g.grid[423], HEAD);
        assert_eq!(g.length(), 3);
    }

    #[test]
    fn sectors_wrap_round_the_circle() {
        let mut g = Game::new();
        run(&mut g, 57); // head at sector 59
        assert_eq!((g.head_cell, g.sector), (479, 59));
        g.step();
        assert_eq!((g.head_cell, g.sector, g.ring), (420, 0, 7));
    }

    #[test]
    fn turning_left_adds_one_and_right_adds_three() {
        let mut g = Game::new();
        g.turn(Turn::Left);
        g.step();
        assert_eq!(g.heading, Heading::Out);
        assert_eq!(g.head_cell, 422 + SECTORS);
        g.turn(Turn::Left);
        g.step();
        assert_eq!(g.heading, Heading::Anticlockwise);
        assert_eq!(g.head_cell, 422 + SECTORS - 1);
        g.turn(Turn::Right);
        g.step();
        assert_eq!(g.heading, Heading::Out);
        g.turn(Turn::Right);
        g.turn(Turn::Right); // the latest turn wins, as in a6
        g.step();
        assert_eq!(g.heading, Heading::Clockwise);
        assert_eq!(g.turn, None);
    }

    #[test]
    fn rings_wrap_both_ways_so_it_is_a_torus() {
        let mut g = Game::new();
        g.turn(Turn::Left); // out
        run(&mut g, RINGS - 1 - START_RING); // to ring 14
        assert_eq!(g.ring, RINGS - 1);
        g.step();
        assert_eq!(g.ring, 0);
        assert_eq!(g.head_cell, g.sector);
        g.turn(Turn::Left);
        g.turn(Turn::Left);
        g.step(); // anticlockwise, then turn in
        g.turn(Turn::Left);
        g.step();
        assert_eq!(g.heading, Heading::In);
        assert_eq!(g.ring, RINGS - 1);
    }

    #[test]
    fn an_apple_appears_every_24_steps_along_the_pattern() {
        let mut g = Game::new();
        run(&mut g, 23);
        assert!(!g.grid[..CELLS].contains(&APPLE));
        g.step();
        assert_eq!(g.apple_cell(), 337);
        assert_eq!(g.grid[337], APPLE);
        run(&mut g, 24);
        assert_eq!(g.apple_cell(), (2 * 337) % CELLS);
        assert_eq!(g.grid[674], APPLE);
        assert_eq!(g.grid[337], APPLE, "the first apple is still there");
    }

    #[test]
    fn the_pattern_visits_every_cell() {
        let mut seen = [false; CELLS];
        let mut c = 0;
        for _ in 0..CELLS {
            c = (c + APPLE_STRIDE) % CELLS;
            seen[c] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn eating_grows_by_one_and_keeps_the_tail() {
        // An apple at 337 = ring 5, sector 37, and the snake right behind it.
        let mut g = Game::from_cells(&[5 * SECTORS + 34, 5 * SECTORS + 35, 5 * SECTORS + 36], Heading::Clockwise);
        g.grid[337] = APPLE;
        let tail = g.cells().next().unwrap();
        assert_eq!(g.step(), Some(Outcome::Ate));
        assert_eq!(g.length(), 4);
        assert_eq!(g.cells().next().unwrap(), tail, "the tail stayed");
        assert_eq!(g.grid[337], HEAD, "the apple is gone, the head is there");
        assert_eq!(g.grid[..CELLS].iter().filter(|&&v| v == APPLE).count(), 0);
    }

    #[test]
    fn the_apple_is_skipped_when_its_cell_is_taken() {
        let mut g = Game::new();
        g.grid[337] = BODY; // pretend the body is there
        run(&mut g, 24);
        assert_eq!(g.grid[337], BODY);
        assert_eq!(g.apple_cell(), 337, "the pattern moved on anyway");
    }

    /// A 7-cell snake turned into a U; going in closes the loop onto its body.
    fn u_shape() -> Game {
        let r = START_RING * SECTORS;
        let cells = [r + 10, r + 11, r + 12, r + 13, r + 13 + SECTORS, r + 13 + 2 * SECTORS, r + 12 + 2 * SECTORS];
        Game::from_cells(&cells, Heading::Anticlockwise)
    }

    #[test]
    fn running_into_the_body_is_game_over() {
        let mut g = u_shape();
        g.turn(Turn::Left); // anticlockwise + 1 = in: towards r + 12 + SECTORS... empty
        g.step();
        assert_eq!(g.heading, Heading::In);
        assert!(!g.over);
        // Now heading in from r+12+SECTORS lands on r+12, which is body.
        let before = g.length();
        assert_eq!(g.step(), Some(Outcome::Crashed));
        assert!(g.over);
        assert_eq!(g.grid[UI_CELL], UI_OVER);
        assert_eq!(g.grid[r_plus(12)], BODY, "the head did not move onto the body");
        assert_eq!(g.length(), before - 1, "the tail had already left, as in the original");
        assert_eq!(g.step(), None, "nothing happens after a crash");
    }

    fn r_plus(n: usize) -> usize {
        START_RING * SECTORS + n
    }

    #[test]
    fn following_the_tail_round_is_allowed() {
        // Four cells in a square: tail at A, head next to A. Moving onto A is
        // fine because the tail leaves first.
        let r = START_RING * SECTORS;
        let a = r + 20;
        let mut g = Game::from_cells(&[a, a + 1, a + 1 + SECTORS, a + SECTORS], Heading::Anticlockwise);
        g.turn(Turn::Left); // anticlockwise + 1 = in: onto a
        assert_eq!(g.step(), Some(Outcome::Moved));
        assert_eq!(g.head_cell, a);
        assert!(!g.over);
    }

    #[test]
    fn adjacency_wraps() {
        assert!(adjacent(0, 1));
        assert!(adjacent(0, SECTORS - 1));
        assert!(adjacent(0, SECTORS));
        assert!(adjacent(0, (RINGS - 1) * SECTORS));
        assert!(!adjacent(0, 2));
        assert!(!adjacent(0, SECTORS + 1));
    }

    #[test]
    fn a_long_game_stays_inside_the_world() {
        // Turn every few steps and make sure no index ever leaves its region,
        // which is what the masks in main.S guaranteed.
        let mut g = Game::new();
        let mut k = 0u32;
        for i in 0..5000 {
            if i % 7 == 0 {
                g.turn(if k % 3 == 0 { Turn::Left } else { Turn::Right });
                k += 1;
            }
            if g.step().is_none() {
                g = Game::new();
            }
            assert!(g.head_cell < CELLS);
            assert!(g.length() <= CELLS);
            for c in g.cells() {
                assert!(c < CELLS);
            }
        }
    }
}
