use std::iter;

pub mod exercises;

// Cut-Rod(p, n)
//
// 1  if n == 0
// 2      return 0
// 3  q = -∞
// 4  for i = 1 to n
// 5      q = max(q, p[i] + Cut-Rod(p, n - i))
// 6  return q

pub fn cut_rod(p: &[u32]) -> u32 {
    let n = p.len();

    p.iter()
        .enumerate()
        .map(|(i, p_i)| p_i + cut_rod(&p[..n - (i + 1)]))
        .max()
        .unwrap_or(0)
}

// Memoized-Cut-Rod-Aux(p, n, r)
//
// 1  if r[n] ≥ 0
// 2      return r[n]
// 3  if n == 0
// 4      q = 0
// 5  else q = -∞
// 6      for i = 1 to n
// 7          q = max(q, p[i] + Memoized-Cut-Rod-Aux(p, n - i, r))
// 8  r[n] = q
// 9  return q

fn memoized_cut_rod_aux(p: &[u32], r: &mut [Option<u32>]) -> u32 {
    let n = p.len();

    if let Some(r_n) = r[n] {
        r_n
    } else {
        let q = p
            .iter()
            .enumerate()
            .map(|(i, p_i)| p_i + memoized_cut_rod_aux(&p[..n - (i + 1)], r))
            .max()
            .unwrap_or(0);

        r[n] = Some(q);

        q
    }
}

// Memoized-Cut-Rod(p, n)
//
// 1  let r[0‥n] be a new array
// 2  for i = 0 to n
// 3      r[i] = -∞
// 4  return Memoized-Cut-Rod-Aux(p, n, r)

pub fn memoized_cut_rod(p: &[u32]) -> u32 {
    let n = p.len();
    let mut r = vec![None; n + 1];

    memoized_cut_rod_aux(p, &mut r)
}

// Bottom-Up-Cut-Rod(p, n)
//
// 1  let r[0‥n] be a new array
// 2  r[0] = 0
// 3  for j = 1 to n
// 4      q = -∞
// 5      for i = 1 to j
// 6          q = max(q, p[i] + r[j - i])
// 7      r[j] = q
// 8  return r[n]

pub fn bottom_up_cut_rod(p: &[u32]) -> u32 {
    let n = p.len();
    let mut r = vec![0; n + 1];

    for j in 1..=n {
        let q = p[..j]
            .iter()
            .enumerate()
            .map(|(i, p_i)| p_i + r[j - (i + 1)])
            .max()
            .unwrap();

        r[j] = q;
    }

    r[n]
}

// Extended-Bottom-Up-Cut-Rod(p, n)
//
//  1  let r[0‥n] and s[1‥n] be new arrays
//  2  r[0] = 0
//  3  for j = 1 to n
//  4      q = -∞
//  5      for i = 1 to j
//  6          if q < p[i] + r[j - i]
//  7              q = p[i] + r[j - i]
//  8              s[j] = i
//  9      r[j] = q
// 10  return r and s

pub fn extended_bottom_up_cut_rod(p: &[u32]) -> (Box<[u32]>, Box<[usize]>) {
    let n = p.len();
    let mut r = vec![0; n + 1];
    let mut s = vec![0; n];

    for j in 1..=n {
        let mut q = 0;
        let mut new_s_j = 0;

        for (i, p_i) in p[..j].iter().enumerate().map(|(i, p_i)| (i + 1, p_i)) {
            let new_q = p_i + r[j - i];

            if q < new_q {
                q = new_q;
                new_s_j = i
            }
        }

        s[j - 1] = new_s_j;
        r[j] = q;
    }

    (r.into(), s.into())
}

// Print-Cut-Rod-Solution(p, n)
//
// 1  (r, s) = Extended-Bottom-Up-Cut-Rod(p, n)
// 2  while n > 0
// 3      print s[n]
// 4      n = n - s[n]

pub fn print_cut_rod_solution(p: &[u32]) -> Box<[usize]> {
    let mut n = p.len();
    let (_, s) = extended_bottom_up_cut_rod(p);

    iter::from_fn(move || {
        if n == 0 {
            None
        } else {
            let result = s[n - 1];

            n -= result;

            Some(result)
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{bottom_up_cut_rod, cut_rod, extended_bottom_up_cut_rod, memoized_cut_rod, print_cut_rod_solution};

    fn run_cut_rod_tests<F: FnMut(&[u32]) -> u32>(mut f: F) {
        assert_eq!(f(&[]), 0);
        assert_eq!(f(&[1]), 1);
        assert_eq!(f(&[1, 5]), 5);
        assert_eq!(f(&[1, 5, 8,]), 8);
        assert_eq!(f(&[1, 5, 8, 9]), 10);
        assert_eq!(f(&[1, 5, 8, 9, 10]), 13);
        assert_eq!(f(&[1, 5, 8, 9, 10, 17]), 17);
        assert_eq!(f(&[1, 5, 8, 9, 10, 17, 17]), 18);
        assert_eq!(f(&[1, 5, 8, 9, 10, 17, 17, 20]), 22);
        assert_eq!(f(&[1, 5, 8, 9, 10, 17, 17, 20, 24]), 25);
        assert_eq!(f(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30]), 30);
    }

    #[test]
    fn test_cut_rod() {
        run_cut_rod_tests(cut_rod);
    }

    #[test]
    fn test_memoized_cut_rod() {
        run_cut_rod_tests(memoized_cut_rod);
    }

    #[test]
    fn test_bottom_up_cut_rod() {
        run_cut_rod_tests(bottom_up_cut_rod);
    }

    #[test]
    fn test_extended_bottom_up_cut_rod() {
        assert_eq!(extended_bottom_up_cut_rod(&[]), (vec![0].into(), vec![].into()));
        assert_eq!(extended_bottom_up_cut_rod(&[1]), (vec![0, 1].into(), vec![1].into()));

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5]),
            (vec![0, 1, 5].into(), vec![1, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8]),
            (vec![0, 1, 5, 8].into(), vec![1, 2, 3].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9]),
            (vec![0, 1, 5, 8, 10].into(), vec![1, 2, 3, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9, 10]),
            (vec![0, 1, 5, 8, 10, 13].into(), vec![1, 2, 3, 2, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9, 10, 17]),
            (vec![0, 1, 5, 8, 10, 13, 17].into(), vec![1, 2, 3, 2, 2, 6].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9, 10, 17, 17]),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18].into(),
                vec![1, 2, 3, 2, 2, 6, 1].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9, 10, 17, 17, 20]),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18, 22].into(),
                vec![1, 2, 3, 2, 2, 6, 1, 2].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9, 10, 17, 17, 20, 24]),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18, 22, 25].into(),
                vec![1, 2, 3, 2, 2, 6, 1, 2, 3].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30]),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18, 22, 25, 30].into(),
                vec![1, 2, 3, 2, 2, 6, 1, 2, 3, 10].into()
            )
        );
    }

    #[test]
    fn test_print_cut_rod_solution() {
        assert_eq!(print_cut_rod_solution(&[]), vec![].into());
        assert_eq!(print_cut_rod_solution(&[1]), vec![1].into());
        assert_eq!(print_cut_rod_solution(&[1, 5]), vec![2].into());
        assert_eq!(print_cut_rod_solution(&[1, 5, 8,]), vec![3].into());
        assert_eq!(print_cut_rod_solution(&[1, 5, 8, 9]), vec![2, 2].into());
        assert_eq!(print_cut_rod_solution(&[1, 5, 8, 9, 10]), vec![2, 3].into());
        assert_eq!(print_cut_rod_solution(&[1, 5, 8, 9, 10, 17]), vec![6].into());
        assert_eq!(print_cut_rod_solution(&[1, 5, 8, 9, 10, 17, 17]), vec![1, 6].into());
        assert_eq!(print_cut_rod_solution(&[1, 5, 8, 9, 10, 17, 17, 20]), vec![2, 6].into());

        assert_eq!(
            print_cut_rod_solution(&[1, 5, 8, 9, 10, 17, 17, 20, 24]),
            vec![3, 6].into()
        );

        assert_eq!(
            print_cut_rod_solution(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30]),
            vec![10].into()
        );
    }
}
