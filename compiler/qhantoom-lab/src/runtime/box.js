export const Box = x => ({
  map: f => Box(f(x)),
  fold: f => f(x),
  text: _ => `Box(${ x })`,
});
