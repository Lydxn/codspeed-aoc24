const N: usize = 1000;

unsafe fn radix_sort_u17(arr: &mut [u32; N]) {
    let mut cnt_lo: [u16; 256] = [0; 256];
    let mut cnt_hi: [u16; 512] = [0; 512];

    for x in arr.iter() {
        *cnt_lo.get_unchecked_mut((*x & 255) as usize) += 1;
        *cnt_hi.get_unchecked_mut((*x >> 8) as usize) += 1;
    }

    for i in 0..255 { cnt_lo[i + 1] += cnt_lo[i]; }
    for i in 0..511 { cnt_hi[i + 1] += cnt_hi[i]; }

    let mut buf = [0; N];

    for x in arr.iter().rev() {
        let ptr = cnt_lo.get_unchecked_mut((*x & 255) as usize);
        *ptr -= 1;
        *buf.get_unchecked_mut(*ptr as usize) = *x;
    }

    for x in buf.iter().rev() {
        let ptr = cnt_hi.get_unchecked_mut((*x >> 8) as usize);
        *ptr -= 1;
        *arr.get_unchecked_mut(*ptr as usize) = *x;
    }
}

fn parse_5b(s: &[u8]) -> u32 {
    unsafe {
        (*s.get_unchecked(0) as u32) * 10000 +
        (*s.get_unchecked(1) as u32) * 1000 +
        (*s.get_unchecked(2) as u32) * 100 +
        (*s.get_unchecked(3) as u32) * 10 +
        (*s.get_unchecked(4) as u32) - 533328
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left = [0; N];
    let mut right = [0; N];

    let s = input.as_bytes();
    for i in 0..N {
        left[i] = parse_5b(&s[i*14..]);
        right[i] = parse_5b(&s[i*14+8..]);
    }

    unsafe {
        radix_sort_u17(&mut left);
        radix_sort_u17(&mut right);
    }

    left.iter().zip(&right)
        .map(|(a, &b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut assoc = [0; 2048];

    let s = input.as_bytes();
    for i in 0..N {
        let right = parse_5b(&s[i*14+8..]);
        let mut h = right & 2047;
        loop {
            let entry = &mut assoc[h as usize];
            if *entry == 0 {
                *entry = right | 1 << 20;
                break;
            }
            if (*entry & 0xfffff) == right {
                *entry += 1 << 20;
                break;
            }
            h = (h + 1) & 2047;
        }
    }

    let mut answer = 0;

    for i in 0..N {
        let left = parse_5b(&s[i*14..]);
        let mut h = left & 2047;
        loop {
            let entry = assoc[h as usize];
            if entry == 0 {
                break;
            }
            if (entry & 0xfffff) == left {
                answer += left * (entry >> 20);
            }
            h = (h + 1) & 2047;
        }
    }
    answer
}