use std::{error::Error, fmt::{Display, write}, ops::{Index, IndexMut}, cmp::max};

const USING_TEST_INPUT: bool = true;
const DELTAS: [Direction; 4] = [ Direction::Left, Direction::Right, Direction::Up, Direction::Down];

use get_aoc_input as gai;
fn main() -> Result<(), Box<dyn Error>>{
	let input = {
		if USING_TEST_INPUT {
".....
.S-7.
.|.|.
.L-J.
.....".into()
		} else { gai::load_input(1, 10)? }
		};

	let input_b: String = 
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...".into();
	let tiles = input.chars().filter(|c| c != &'\n').map(|c| c.into()).collect();

	let grid = Grid {tiles, width: input.lines().next().unwrap().len() };

	let start_pos: Position = pos_from_idx(
		&grid.tiles.iter().enumerate().filter(|(_, &t)| t == TileKind::Starting).next().unwrap().0,
		&grid.width
	);
	println!("S is at {start_pos}");

	let mut m_grid: MarkedGrid = grid.into();
	m_grid[start_pos].distance = Some(0);
	m_grid[start_pos].seen = true;
	let max_len = find_longest_path(&mut m_grid, start_pos);
	println!("Final grid: {m_grid}");

	println!("Answer is: {max_len}");

	Ok(())
}

fn find_longest_path(grid: &mut MarkedGrid, pos: Position) -> usize {
	println!("grid is {grid}");

	grid[pos].seen = true;
	let val = grid[pos].distance.unwrap();
	for delta in DELTAS {
		let Some(new_pos) = pos_plus_dir(pos, delta, grid.width) else {continue;};
		if !grid[new_pos].seen &&
			pipe_dir_is_valid(delta, grid[pos].tile, grid[new_pos].tile) // Check that it's a pipe that we can get to
		{
			grid.mark_pos(new_pos, val + 1);
		}
	}

	for delta in DELTAS {
		let Some(new_pos) = pos_plus_dir(pos, delta, grid.width) else {continue;};
		println!("Analyzing {new_pos}");
		if !grid[new_pos].seen &&
			pipe_dir_is_valid(delta, grid[pos].tile, grid[new_pos].tile) // Check that it's a pipe that we can get to
		{
			find_longest_path(grid, new_pos);
		}
	}
	grid.tiles.iter().filter_map(|v| v.distance).sum()
}

impl Display for MarkedGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut out = String::new();
		for (i, t) in self.tiles.iter().enumerate() {
			if i % (self.width ) == 0 { out.push('\n');}
			out.push(
				match t.distance {
					Some(t) => t.to_string().chars().next().unwrap(),
					None => '·',
				}
			)
		}

		write!(f,"{}", out)
    }
}

fn pipe_dir_is_valid(dir: Direction, curr: TileKind, next: TileKind) -> bool {
	use TileKind as TK;
	use Pipe as P;
	let can_leave = match dir { // Does the current space have an available exit
		Direction::Left => { // Must have a W
			curr == TK::Starting ||
				curr == TK::Pipe(P::NW) ||
				curr == TK::Pipe(P::SW) ||
				curr == TK::Pipe(P::EW) 
		}
		Direction::Right => { // Must have an E
			curr == TK::Starting ||
				curr == TK::Pipe(P::NE) ||
				curr == TK::Pipe(P::SE) ||
				curr == TK::Pipe(P::EW) 
		}
		Direction::Up => { // Must have a N
			curr == TK::Starting ||
				curr == TK::Pipe(P::NE) ||
				curr == TK::Pipe(P::NW) ||
				curr == TK::Pipe(P::NS) 
		}
		Direction::Down => { // Must have a S
			curr == TK::Starting ||
				curr == TK::Pipe(P::SE) ||
				curr == TK::Pipe(P::SW) ||
				curr == TK::Pipe(P::NS) 
		}
	};

	let can_enter = match dir {
		Direction::Left => { // Must have an E
			next == TK::Pipe(P::NE) || next == TK::Pipe(P::SE) || next == TK::Pipe(P::EW) 
		}
		Direction::Right => { // Must have a W
			next == TK::Pipe(P::NW) || next == TK::Pipe(P::SW) || next == TK::Pipe(P::EW) 
		}
		Direction::Up => { // Must have a S
			next == TK::Pipe(P::SE) || next == TK::Pipe(P::SW) || next == TK::Pipe(P::NS) 
		}
		Direction::Down => { // Must have a N
			next == TK::Pipe(P::NE) || next == TK::Pipe(P::NW) || next == TK::Pipe(P::NS) 
		}
	};
	//dbg!(curr, next, can_leave, can_enter);

	can_leave && can_enter
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

fn pos_plus_dir(pos: Position, dir: Direction, w: usize) -> Option<Position> {
	match dir {
		Direction::Left => {
			if pos.x == 0 {return None;}
			return Some(Position { x: pos.x - 1, ..pos })
		}
		Direction::Right => {
			if pos.x == w - 1 {return None;}
			return Some(Position { x: pos.x + 1, ..pos })
		}
		Direction::Up => {
			if pos.y == 0 {return None;}
			return Some(Position { y: pos.y - 1, ..pos  })
		}
		Direction::Down => {
			if pos.y == w - 1 {return None;}
			return Some(Position { y: pos.y + 1, ..pos })
		}
	}
}

impl From<Grid> for MarkedGrid {
	fn from(value: Grid) -> Self {
		let mut new_tiles = Vec::with_capacity(value.tiles.len());
		for t in value.tiles { new_tiles.push(MarkedTile { tile: t, distance: None, seen: false }); }
		let out = Self { tiles: new_tiles, width: value.width };
		out
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Position {
	x: usize,
	y: usize,
} 

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkedGrid {
	tiles: Vec<MarkedTile>,
	width: usize,
}

impl MarkedGrid {
	fn mark_pos(&mut self, pos: Position, val: usize) {
		self[pos].distance = Some(val);
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkedTile {
	tile: TileKind,
	distance: Option<usize>,
	seen: bool
}

impl Index<Position> for MarkedGrid {
    type Output = MarkedTile;

    fn index(&self, index: Position) -> &Self::Output {
		&self.tiles[index.x + index.y * self.width]
    }

}
impl IndexMut<Position> for MarkedGrid {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
		&mut self.tiles[index.x + index.y * self.width]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid{
	tiles: Vec<TileKind>,
	width: usize,
}

impl Index<Position> for Grid {
    type Output = TileKind;

    fn index(&self, index: Position) -> &Self::Output {
		&self.tiles[index.x + index.y * self.width]
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileKind {
	Ground,
	Pipe(Pipe),
	Starting,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pipe {
	NS, NE, NW,
	EW, SW, SE,
}

impl From<char> for TileKind {
    fn from(value: char) -> Self {
		match value {
			'|' => Self::Pipe(Pipe::NS), '-' => Self::Pipe(Pipe::EW), 'L' => Self::Pipe(Pipe::NE),
			'J' => Self::Pipe(Pipe::NW), '7' => Self::Pipe(Pipe::SW), 'F' => Self::Pipe(Pipe::SE),
			'.' => Self::Ground, 'S' => Self::Starting, c => panic!("Unrecognized symbol: {c}"),
		}
    }
}

impl From<Pipe> for char {
	fn from(value: Pipe) -> Self {
		match value {
			Pipe::NS => '|', Pipe::NE => 'L', Pipe::NW => 'J',
			Pipe::EW => '-', Pipe::SW => '7', Pipe::SE => 'F',
		} 
	}
}

impl Display for TileKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let c: char = match self {
			TileKind::Ground => '.',
			TileKind::Pipe(pipe) => (*pipe).into(),
			TileKind::Starting => 'S',
		};
		write!(f, "{}", c)
	}
}

fn pos_from_idx(i: &usize, w: &usize) -> Position {
	let x = i % w;
	let y = i / w;
	Position {x, y}
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
    }
}
