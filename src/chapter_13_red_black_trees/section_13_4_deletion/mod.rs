// RB-Delete-Fixup(T, x)
//
//  1  while x ≠ T.root and x.color == black
//  2      if x == x.p. left
//  3          w = x.p.right
//  4          if w.color == red
//  5              w.color = black           // case 1
//  6              x.p.color = red           // case 1
//  7              Left-Rotate(T, x.p)       // case 1
//  8              w = x.p.right             // case 1
//  9          if w.left.color == black and w.right.color == black
// 10              w.color = red             // case 2
// 11              x = x.p                   // case 2
// 12          else if w.right.color == black
// 13                   w.left.color = black // case 3
// 14                   w.color = red        // case 3
// 15                   Right-Rotate(T, w)   // case 3
// 16                   w = x.p.right        // case 3
// 17               w.color = x.p.color      // case 4
// 18               x.p.color = black        // case 4
// 19               w.right.color = black    // case 4
// 20               Left-Rotate(T, x.p)      // case 4
// 21               x = T.root               // case 4
// 22      else (same as then clause with “right” and “left” exchanged)
// 23  x.color = black

// TODO.
