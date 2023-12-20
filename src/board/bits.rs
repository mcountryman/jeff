//! Bit boards.

use super::coord::Square;
use super::{Board, Piece, Player};

/// A set of bit boards for every piece and every color.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitBoards {
  pieces: [BitBoard; Board::PIECES * Board::PLAYERS],
}

impl BitBoards {
  pub const fn empty() -> Self {
    Self {
      pieces: [BitBoard::empty(); Board::PIECES * Board::PLAYERS],
    }
  }

  pub const fn with(mut self, player: Player, piece: Piece, at: Square) -> Self {
    let player = player as usize;
    let piece = piece as usize;
    let index = player * 6 + piece;

    self.pieces[index] = self.pieces[index].with(at);
    self
  }

  pub fn piece_at(self, at: Square) -> Option<Piece> {
    let mut piece = None;

    for (i, bits) in self.pieces.iter().enumerate() {
      if bits.at(at) {
        piece = Some(index_to_piece(i));
      }
    }

    piece
  }

  pub fn player_at(self, at: Square) -> Option<Player> {
    let mut player = None;

    for (i, bits) in self.pieces.iter().enumerate() {
      if bits.at(at) {
        player = Some(index_to_player(i));
      }
    }

    player
  }
}

fn index_to_piece(idx: usize) -> Piece {
  match idx % 6 {
    0 => Piece::Pawn,
    1 => Piece::Horse,
    2 => Piece::Bishop,
    3 => Piece::Rook,
    4 => Piece::Queen,
    5 => Piece::King,
    _ => unreachable!(),
  }
}

fn index_to_player(idx: usize) -> Player {
  if idx < 6 {
    Player::White
  } else {
    Player::Black
  }
}

/// A chess bit board.
///
/// Bit board uses little-endian rank-file mapping meaning bit `0` coorresponds to A1 and `63`
/// corresponds to H8.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
  /// Gets an empty [BitBoard].
  pub const fn empty() -> Self {
    Self(0)
  }

  /// Gets the bit for the given [Square].
  pub const fn at(self, square: Square) -> bool {
    self.0 & (1 << square.into_le_index()) != 0
  }

  /// Gets a copy of the [Bits] with the given square set.
  pub const fn with(mut self, square: Square) -> Self {
    self.0 |= 1 << square.into_le_index();
    self
  }
}

#[cfg(test)]
mod tests {
  use super::{BitBoard, BitBoards};
  use crate::board::coord::Square;
  use crate::board::{Piece, Player};

  #[test]
  fn with2() {
    let bits = BitBoards::empty().with(Player::White, Piece::Horse, Square::A1);

    assert_eq!(bits.piece_at(Square::A1), Some(Piece::Horse));
    assert_eq!(bits.player_at(Square::A1), Some(Player::White));

    let bits = BitBoards::empty().with(Player::White, Piece::Horse, Square::H8);

    assert_eq!(bits.piece_at(Square::H8), Some(Piece::Horse));
    assert_eq!(bits.player_at(Square::H8), Some(Player::White));
  }

  #[test]
  fn with() {
    let bits = BitBoard::default().with(Square::A1).with(Square::H8);

    assert!(bits.at(Square::A1));
    assert!(bits.at(Square::H8));
    assert!(!bits.at(Square::A2));
  }
}
