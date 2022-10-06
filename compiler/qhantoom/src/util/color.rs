pub struct Color;

impl Color {
  /// FRENCH_SKY_BLUE: blue tint
  pub const BLUE_100: ariadne::Color = ariadne::Color::RGB(112, 161, 255);

  // SPIRO_DISCO_BALL: blue tint
  pub const BLUE_200: ariadne::Color = ariadne::Color::RGB(15, 188, 249);

  /// WATERFALL: green tint
  pub const GREEN_100: ariadne::Color = ariadne::Color::RGB(56, 173, 169);

  // ANSI Color 115: green tint
  pub const GREEN_200: ariadne::Color = ariadne::Color::Fixed(115);

  /// CARMINE_PINK: red tint
  pub const RED_100: ariadne::Color = ariadne::Color::RGB(232, 65, 24);

  /// BEEKEEPER: yellow tint
  pub const YELLOW_100: ariadne::Color = ariadne::Color::RGB(246, 229, 141);
}
