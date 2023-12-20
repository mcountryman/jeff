//! Chess coordinate utilities

/// The horizontal component of a chess board coordinate.
///
/// Ranks are ordered from bottom-most to top-most.  The first rank would be the lowest row on the
/// chess board and the eighth rank will be the highest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
  First = 0,
  Second = 1,
  Third = 2,
  Fourth = 3,
  Fifth = 4,
  Sixth = 5,
  Seventh = 6,
  Eighth = 7,
}

impl Rank {
  /// An array containing all [Rank] variants.
  pub const ALL: [Self; 8] = [
    Self::First,
    Self::Second,
    Self::Third,
    Self::Fourth,
    Self::Fifth,
    Self::Sixth,
    Self::Seventh,
    Self::Eighth,
  ];

  /// Gets the numerical representation of the [Rank].
  pub const fn num(self) -> u8 {
    match self {
      Self::First => 0,
      Self::Second => 1,
      Self::Third => 2,
      Self::Fourth => 3,
      Self::Fifth => 4,
      Self::Sixth => 5,
      Self::Seventh => 6,
      Self::Eighth => 7,
    }
  }

  pub const fn wrapping_down_by(self, by: u8) -> Self {
    match Self::from_num(self.num().wrapping_sub(by) % 8) {
      Some(rank) => rank,
      None => Self::First,
    }
  }

  /// Gets a [Rank] from the given numerical represention.
  pub const fn from_num(val: u8) -> Option<Self> {
    match val {
      0 => Some(Self::First),
      1 => Some(Self::Second),
      2 => Some(Self::Third),
      3 => Some(Self::Fourth),
      4 => Some(Self::Fifth),
      5 => Some(Self::Sixth),
      6 => Some(Self::Seventh),
      7 => Some(Self::Eighth),
      _ => None,
    }
  }
}

/// The vertical component of a chess board coordinate.
///
/// Files are ordered from left-most to right-most.  `A` would be the left-most and `H` would be
/// the right-most column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum File {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
  F = 5,
  G = 6,
  H = 7,
}

impl File {
  /// An array containing all [File] variants.
  pub const ALL: [Self; 8] = [
    Self::A,
    Self::B,
    Self::C,
    Self::D,
    Self::E,
    Self::F,
    Self::G,
    Self::H,
  ];

  /// Gets the numerical representation of the [File].
  pub const fn num(self) -> u8 {
    match self {
      Self::A => 0,
      Self::B => 1,
      Self::C => 2,
      Self::D => 3,
      Self::E => 4,
      Self::F => 5,
      Self::G => 6,
      Self::H => 7,
    }
  }

  /// Converts the [File] to a lower case char.
  pub const fn into_char(self) -> char {
    match self {
      Self::A => 'a',
      Self::B => 'b',
      Self::C => 'c',
      Self::D => 'd',
      Self::E => 'e',
      Self::F => 'f',
      Self::G => 'g',
      Self::H => 'h',
    }
  }

  pub const fn right_by(self, by: u8) -> Option<Self> {
    Self::from_num(self.num().saturating_add(by))
  }

  pub const fn wrapping_right_by(self, by: u8) -> Self {
    match Self::from_num(self.num().wrapping_add(by) % 8) {
      Some(rank) => rank,
      None => Self::A,
    }
  }

  /// Gets a [File] from the given numerical representation.
  pub const fn from_num(num: u8) -> Option<Self> {
    match num {
      0 => Some(Self::A),
      1 => Some(Self::B),
      2 => Some(Self::C),
      3 => Some(Self::D),
      4 => Some(Self::E),
      5 => Some(Self::F),
      6 => Some(Self::G),
      7 => Some(Self::H),
      _ => None,
    }
  }

  /// Gets a [File] from the given char.
  pub const fn from_char(ch: char) -> Option<Self> {
    match ch {
      'a' | 'A' => Some(Self::A),
      'b' | 'B' => Some(Self::B),
      'c' | 'C' => Some(Self::C),
      'd' | 'D' => Some(Self::D),
      'e' | 'E' => Some(Self::E),
      'f' | 'F' => Some(Self::F),
      'g' | 'G' => Some(Self::G),
      'h' | 'H' => Some(Self::H),
      _ => None,
    }
  }
}

macro_rules! square {
  ($($name:ident = $index:literal),+) => {
    /// A square on a chess board.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Square {
      $($name = $index),+
    }

    impl Square {
      /// Gets a [Square] from the given `0`-based little-endian rank-file index.
      #[inline]
      pub const fn from_le_index(index: u8) -> Option<Self> {
        match index {
          $($index => Some(Self::$name),)+
          _ => None,
        }
      }

      /// Converts the [Square] into a `0`-based little-endian rank-file index.
      #[inline]
      pub const fn into_le_index(self) -> u8 {
        match self {
          $(Self::$name => $index,)+
        }
      }
    }
  };
}

square! {
  A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
  A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
  A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
  A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
  A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
  A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
  A2 = 8,  B2 = 9,  C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
  A1 = 0,  B1 = 1,  C1 = 2,  D1 = 3,  E1 = 4,  F1 = 5,  G1 = 6,  H1 = 7
}

impl Square {
  /// Gets a [Square] from the given [Rank] and [File].
  pub const fn from_rank_file(rank: Rank, file: File) -> Self {
    let rank = rank.num();
    let file = file.num();

    match Self::from_le_index(rank * 8 + file) {
      Some(square) => square,
      None => Self::A1,
    }
  }

  /// Gets the [Rank] of the [Square].
  pub const fn rank(self) -> Rank {
    let index = self.into_le_index();
    let rank = index / 8;

    match Rank::from_num(rank) {
      Some(rank) => rank,
      None => Rank::First,
    }
  }

  /// Gets the [File] of the [Square].
  pub const fn file(self) -> File {
    let index = self.into_le_index();
    let file = index % 8;

    match File::from_num(file) {
      Some(file) => file,
      None => unreachable!(),
    }
  }

  /// Gets a copy of the [Square] with the given [Rank].
  pub const fn with_rank(self, rank: Rank) -> Self {
    Self::from_rank_file(rank, self.file())
  }

  /// Gets a copy of the [Square] with the given [File].
  pub const fn with_file(self, file: File) -> Self {
    Self::from_rank_file(self.rank(), file)
  }

  pub const fn wrapping_down_by(self, by: u8) -> Self {
    Self::from_rank_file(self.rank().wrapping_down_by(by), self.file())
  }

  pub const fn right_by(self, by: u8) -> Option<Self> {
    match self.file().right_by(by) {
      Some(file) => Some(Self::from_rank_file(self.rank(), file)),
      None => None,
    }
  }

  pub const fn wrapping_right_by(self, by: u8) -> Self {
    Self::from_rank_file(self.rank(), self.file().wrapping_right_by(by))
  }
}

#[cfg(test)]
mod tests {
  use super::{File, Rank, Square};

  #[test]
  fn from_rank_file() {
    for rank in Rank::ALL {
      for file in File::ALL {
        let square = Square::from_rank_file(rank, file);

        assert_eq!(square.rank(), rank);
        assert_eq!(square.file(), file);
      }
    }
  }

  #[test]
  fn wrapping_dir() {
    assert_eq!(Square::A1.wrapping_right_by(1), Square::B1);
    assert_eq!(Square::H1.wrapping_right_by(1), Square::A1);

    assert_eq!(Square::A2.wrapping_down_by(1), Square::A1);
    assert_eq!(Square::A1.wrapping_down_by(1), Square::A8);
  }
}
