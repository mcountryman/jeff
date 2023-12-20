use super::bits::{BitBoard, BitBoards};
use super::coord::{File, Rank, Square};
use super::{Piece, Player};
use core::fmt;

impl fmt::Display for BitBoards {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for rank in Rank::ALL.iter().rev().copied() {
      write!(f, "{} | ", rank.num() + 1)?;

      for file in File::ALL {
        let square = Square::from_rank_file(rank, file);

        let Some(piece) = self.piece_at(square) else {
          write!(f, "  ")?;
          continue;
        };

        let player = self.player_at(square).unwrap();
        let ch = match (piece, player) {
          (Piece::Pawn, Player::White) => "♙",
          (Piece::Horse, Player::White) => "♘",
          (Piece::Bishop, Player::White) => "♗",
          (Piece::Rook, Player::White) => "♖",
          (Piece::Queen, Player::White) => "♕",
          (Piece::King, Player::White) => "♔",

          (Piece::Pawn, Player::Black) => "♟",
          (Piece::Horse, Player::Black) => "♞",
          (Piece::Bishop, Player::Black) => "♝",
          (Piece::Rook, Player::Black) => "♜",
          (Piece::Queen, Player::Black) => "♛",
          (Piece::King, Player::Black) => "♚",
        };

        write!(f, "{ch} ")?;
      }

      writeln!(f)?;
    }

    writeln!(f, "   ----------------")?;
    write!(f, "    ")?;

    for file in File::ALL {
      write!(f, "{} ", file.into_char())?;
    }

    Ok(())
  }
}

impl fmt::Display for BitBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for rank in Rank::ALL.iter().rev().copied() {
      for file in File::ALL {
        let at = self.at(Square::from_rank_file(rank, file));
        let ch = if at { "X " } else { "_ " };

        write!(f, "{ch}")?;
      }

      writeln!(f)?;
    }

    Ok(())
  }
}
