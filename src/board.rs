pub mod bits;
pub mod castling;
pub mod coord;
pub mod display;
pub mod fen;

use self::bits::BitBoards;
use self::castling::Castling;
use self::coord::Square;

/// A representation of a chess board.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board {
  /// Bits boards for each piece and each player.
  bits: BitBoards,
  // Who's turn it is.
  player: Player,
  // Castling rights for each player.
  castling: Castling,
  /// The [Square] where an en-passant capture can be made.
  en_passant: Option<Square>,
  /// The number of half moves since a piece capture or pawn move.
  half_moves: u8,
}

impl Board {
  /// The number of piece types.
  const PIECES: usize = 6;
  /// The number of players.
  const PLAYERS: usize = 2;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Player {
  White = 0,
  Black = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
  Pawn = 0,
  Horse = 1,
  Bishop = 2,
  Rook = 3,
  Queen = 4,
  King = 5,
}
