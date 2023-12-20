bitflags::bitflags! {
  /// Castling rights for either player.
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct Castling : u8 {
    /// White can castle king side.
    const WHITE_KING_SIDE  = 0b0001;
    /// White can castle queen side.
    const WHITE_QUEEN_SIDE = 0b0010;
    /// Black can castle king side.
    const BLACK_KING_SIDE  = 0b0100;
    /// Black can castle queen side.
    const BLACK_QUEEN_SIDE = 0b1000;
  }
}
