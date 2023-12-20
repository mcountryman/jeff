use super::bits::BitBoards;
use super::castling::Castling;
use super::coord::Square;
use super::Player;

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
