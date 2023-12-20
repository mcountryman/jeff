//! Forsyth-Edwards notation.

use super::castling::Castling;
use super::coord::{File, Square};
use super::{Piece, Player};
use crate::board::bits::BitBoards;
use crate::board::coord::Rank;
use core::fmt;

/// An error that can occur during FEN parsing.
#[derive(Debug, Clone, Copy)]
pub enum FromFenError {
  /// Missing the piece info
  NoPieces,
  /// Missing side-to-move info
  NoSideToMove,
  /// Missing castling info
  NoCastling,
  /// Missing en-passant info
  NoEnPassant,
  /// En-passant info too short
  EnPassantTooShort,
  /// Missing half-moves info
  NoHalfmoves,
  /// Unexepected character
  Unexpected(char),
}

impl fmt::Display for FromFenError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NoPieces => write!(f, "Missing the piece info"),
      Self::NoSideToMove => write!(f, "Missing side-to-move info"),
      Self::NoCastling => write!(f, "Missing castling info"),
      Self::NoEnPassant => write!(f, "Missing en-passant info"),
      Self::EnPassantTooShort => write!(f, "En-passant info too short"),
      Self::NoHalfmoves => write!(f, "Missing half-moves info"),
      Self::Unexpected(ch) => write!(f, "Unexepected character `{ch}`"),
    }
  }
}

/// A representation of a chess board that has not been checked for validity.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoardUnchecked {
  /// Bits boards for each piece and each player.
  pub bits: BitBoards,
  /// Who's turn it is.
  pub player: Player,
  /// Castling rights for each player.
  pub castling: Castling,
  /// The [Square] where an en-passant capture can be made.
  pub en_passant: Option<Square>,
  /// The number of half moves since a piece capture or pawn move.
  pub half_moves: u8,
}

/// Parse a [BoardUnchecked] from the given FEN string.
///
/// Validity checks on FEN notation inputs are limited.  A known issue with this parser is that
/// you could overwrite pieces by placing more than eight in a row.
pub const fn parse_fen(fen: &str) -> Result<BoardUnchecked, FromFenError> {
  enum State {
    Pieces,
    SideToMove,
    Castling,
    EnPassant,
    HalfMoves,
    FullMoves,
  }

  let fen = fen.as_bytes();

  let mut i = 0;
  let mut at = Square::A8;
  let mut state = State::Pieces;
  let mut bits = BitBoards::empty();
  let mut player = Player::White;
  let mut castling = Castling::empty();
  let mut en_passant = None;
  let mut half_moves = 0u8;
  let mut full_moves = 0u8;

  while i < fen.len() {
    let ch = fen[i];

    i += 1;

    match state {
      State::Pieces => match ch {
        b' ' => state = State::SideToMove,

        b'/' => {
          at = at.wrapping_down_by(1);
          at = at.with_file(File::A);

          continue;
        }

        b'1'..=b'8' => {
          at = at.wrapping_right_by(ch.wrapping_sub(b'0'));

          continue;
        }

        b'P' => bits = bits.with(Player::White, Piece::Pawn, at),
        b'N' => bits = bits.with(Player::White, Piece::Horse, at),
        b'B' => bits = bits.with(Player::White, Piece::Bishop, at),
        b'R' => bits = bits.with(Player::White, Piece::Rook, at),
        b'Q' => bits = bits.with(Player::White, Piece::Queen, at),
        b'K' => bits = bits.with(Player::White, Piece::King, at),

        b'p' => bits = bits.with(Player::Black, Piece::Pawn, at),
        b'n' => bits = bits.with(Player::Black, Piece::Horse, at),
        b'b' => bits = bits.with(Player::Black, Piece::Bishop, at),
        b'r' => bits = bits.with(Player::Black, Piece::Rook, at),
        b'q' => bits = bits.with(Player::Black, Piece::Queen, at),
        b'k' => bits = bits.with(Player::Black, Piece::King, at),

        ch => return Err(FromFenError::Unexpected(ch as _)),
      },
      State::SideToMove => match ch {
        b' ' => state = State::Castling,

        b'w' => player = Player::White,
        b'b' => player = Player::Black,

        ch => return Err(FromFenError::Unexpected(ch as _)),
      },
      State::Castling => match ch {
        b' ' => state = State::EnPassant,

        b'K' => castling = castling.union(Castling::WHITE_KING_SIDE),
        b'Q' => castling = castling.union(Castling::WHITE_QUEEN_SIDE),
        b'k' => castling = castling.union(Castling::BLACK_KING_SIDE),
        b'q' => castling = castling.union(Castling::BLACK_QUEEN_SIDE),

        ch => return Err(FromFenError::Unexpected(ch as _)),
      },
      State::EnPassant => match ch {
        b'-' => continue,
        b' ' => state = State::HalfMoves,

        b'a'..=b'h' => {
          let file = match File::from_char(ch as _) {
            Some(file) => file,
            // This should be unreachable but, rust decides to emit a panic.  `continue` is an
            // hax.
            None => continue,
          };

          if i >= fen.len() {
            return Err(FromFenError::EnPassantTooShort);
          }

          let rank = match fen[i] {
            b'3' => Rank::Third,
            b'6' => Rank::Sixth,
            ch => return Err(FromFenError::Unexpected(ch as _)),
          };

          i += 1;
          en_passant = Some(Square::from_rank_file(rank, file));
        }
        ch => return Err(FromFenError::Unexpected(ch as _)),
      },
      State::HalfMoves => match ch {
        b' ' => state = State::FullMoves,

        b'0'..=b'9' => {
          half_moves *= 10;
          half_moves += ch.wrapping_sub(b'0');
        }

        ch => return Err(FromFenError::Unexpected(ch as _)),
      },
      State::FullMoves => match ch {
        b' ' => break,

        b'0'..=b'9' => {
          full_moves *= 10;
          full_moves += ch.wrapping_sub(b'0');
        }

        ch => return Err(FromFenError::Unexpected(ch as _)),
      },
    }

    at = at.wrapping_right_by(1);
  }

  Ok(BoardUnchecked {
    bits,
    player,
    castling,
    en_passant,
    half_moves,
  })
}

#[cfg(test)]
mod tests {
  use crate::board::coord::Square;
  use crate::board::{Piece, Player};

  #[test]
  fn starting() {
    let fen = super::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    assert_eq!(fen.bits.piece_at(Square::A1), Some(Piece::Rook));
    assert_eq!(fen.player, Player::White);
    assert_eq!(fen.half_moves, 0);
  }
}
