const DIRECTIONS: &[((isize, isize), u8)] = &[((0, 1), b'>'), ((0, -1), b'<'), ((1, 0), b'v'), ((-1, 0), b'^')];

const MOVES: [u8; b"<^>vA ".len()] = *b"<^>vA ";
const NUMBERS: [u8; b"0123456789A ".len()] = *b"0123456789A ";

const PAD_MOVES: [[u8; 3]; 2] = [
    *b" ^A",
    *b"<v>",
];

const PAD_NUMBERS: [[u8; 3]; 4] = [
    *b"789",
    *b"456",
    *b"123",
    *b" 0A",
];

const fn find_position<const WIDTH: usize, const HEIGHT: usize>(pad: &[[u8; WIDTH]; HEIGHT], symbol: u8) -> (usize, usize) {
    let mut r = 0;
    loop {
        let row = &pad[r];
        
        let mut c = 0;
        loop {
            if row[c] == symbol {
                return (r, c);
            }
            
            c += 1;
            if c == WIDTH {
                break;
            }
        }
        
        r += 1;
        if r == HEIGHT {
            panic!("not found");
        }
    }
}

const fn pair_equals(&(a_r, a_c): &(usize, usize), &(b_r, b_c): &(usize, usize)) -> bool {
    a_r == b_r && a_c == b_c
}

const fn is_empty<const WIDTH: usize, const HEIGHT: usize>(v: &[[bool; WIDTH]; HEIGHT]) -> bool {
    let mut r = 0;
    loop {
        let row = &v[r];
        
        let mut c = 0;
        loop {
            if row[c] {
                return false;
            }

            c += 1;
            if c == WIDTH {
                break;
            }
        }
        
        r += 1;
        if r == HEIGHT {
            break true;
        }
    }
}

const fn symbol_to_index<const SYMBOLS: usize>(table: &[u8; SYMBOLS], symbol: u8) -> usize {
    let mut i = 0;
    loop {
        if index_to_symbol(table, i) == symbol {
            break i;
        }
        
        i += 1;
        if i == SYMBOLS {
            panic!("cannot find symbol");
        }
    }
}

const fn index_to_symbol<const SYMBOLS: usize>(table: &[u8; SYMBOLS], i: usize) -> u8 {
    table[i]
}

const fn find_min<const WIDTH: usize, const HEIGHT: usize>(grid: &[[u8; WIDTH]; HEIGHT], targets: &[[bool; WIDTH]; HEIGHT], costs: &[[usize; WIDTH]; HEIGHT]) -> Option<(u8, (usize, usize))> {
    let mut target = None;

    let mut min = usize::MAX;
    
    let mut r = 0;
    loop {
        let mut c = 0;
        {
            if targets[r][c] {
                let mut i = 0;
                loop {
                    let ((dr, dc), _) = DIRECTIONS[i];
                    match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                        (Some(nr), Some(nc)) if nr < HEIGHT && nc < WIDTH => {
                            if targets[nr][nc] {
                                let cost = costs[nr][nc];
                                if cost < min {
                                    let j = grid[nr][nc];
                                    target = Some((j, (nr, nc)));
                                    min = cost;
                                }
                            }
                        }
                        _ => {}                        
                    }
                    
                    i += 1;
                    if i == DIRECTIONS.len() {
                        break;
                    }
                }
            }

            c += 1;
            if c == WIDTH {
                break;
            }
                    
        }
        
        r += 1;
        if r == HEIGHT {
            break;
        }
    }

    target
}

const fn adjust_costs<const WIDTH: usize, const HEIGHT: usize, const SYMBOLS: usize>(grid: &[[u8; WIDTH]; HEIGHT], symbols: &[u8; SYMBOLS], targets: &[[bool; WIDTH]; HEIGHT], j: u8, (j_r, j_c): (usize, usize), edges: &[[usize; SYMBOLS]; SYMBOLS], costs: &mut [[usize; WIDTH]; HEIGHT]) {
    let cost = costs[j_r][j_c];

    let j_index = symbol_to_index(symbols, j);
    
    let mut i = 0;
    loop {
        let ((dr, dc), _) = DIRECTIONS[i];
        match (j_r.checked_add_signed(dr), j_c.checked_add_signed(dc)) {
            (Some(nr), Some(nc)) if nr < HEIGHT && nc < WIDTH => {
                if targets[nr][nc] {
                    let target = grid[nr][nc];
                    let target_index = symbol_to_index(symbols, target);
                    
                    let cost = cost.saturating_add(edges[j_index][target_index]);
                    if cost < costs[nr][nc] {
                        costs[nr][nc] = cost;
                    }
                }
            }
            _ => {}                        
        }
        
        i += 1;
        if i == DIRECTIONS.len() {
            break;
        }
    }
}

const fn dijkstra<const WIDTH: usize, const HEIGHT: usize, const SYMBOLS: usize>(grid: &[[u8; WIDTH]; HEIGHT], symbols: &[u8; SYMBOLS], from: u8, edges: &[[usize; SYMBOLS]; SYMBOLS]) -> [[usize; WIDTH]; HEIGHT] {
    let mut costs = [[usize::MAX; WIDTH]; HEIGHT];
    let mut targets = [[true; WIDTH]; HEIGHT];
    {
        let (start_r, start_c) = find_position(grid, from);
        let start_index = symbol_to_index(symbols, from);

        costs[start_r][start_c] = 0;

        let mut i = 0;
        loop {
            let ((dr, dc), _) = DIRECTIONS[i];
            
            match (start_r.checked_add_signed(dr), start_c.checked_add_signed(dc)) {
                (Some(nr), Some(nc)) if nr < HEIGHT && nc < WIDTH => {
                    let to = grid[nr][nc];
                    let to_index = symbol_to_index(symbols, to);
                    
                    costs[nr][nc] = edges[start_index][to_index];
                }
                _ => {}
            }
            
            i += 1;
            if i == DIRECTIONS.len() {
                break;
            }            
        }

        targets[start_r][start_c] = false;
    }

    loop {
        if is_empty(&targets) {
            break;
        }

        let Some((j, (j_r, j_c))) = find_min(grid, &targets, &costs) else { break; };

        targets[j_r][j_c] = false;

        if is_empty(&targets) {
            break;
        }

        adjust_costs(&grid, &symbols, &targets, j, (j_r, j_c), edges, &mut costs);
    }

    costs
}

#[cfg(test)]
mod tests {
    use std::array;
    
    use super::*;

    #[test]
    fn test_is_empty() {
        let v = [[true; 3]; 4];

        assert_eq!(is_empty(&v), false);
    }    

    #[test]
    fn test_dijkstra_pad_moves() {
        let edges = array::from_fn(|r| {
            array::from_fn(|c| {
                if r == c {
                    0
                } else if index_to_symbol(&MOVES, r) == b' ' || index_to_symbol(&MOVES, c) == b' ' {
                    usize::MAX
                } else {
                    1
                }
            })
        });

        let target = dijkstra(&PAD_MOVES, &MOVES, b'A', &edges);

        {
            let (r, c) = find_position(&PAD_MOVES, b'<');        
            assert_eq!(target[r][c], 3);
        }

        {
            let (r, c) = find_position(&PAD_MOVES, b'v');        
            assert_eq!(target[r][c], 2);
        }
    }    

    #[test]
    fn test_dijkstra_pad_numbers() {
        let edges = array::from_fn(|r| {
            array::from_fn(|c| {
                if r == c {
                    0
                } else if index_to_symbol(&NUMBERS, r) == b' ' || index_to_symbol(&NUMBERS, c) == b' ' {
                    usize::MAX
                } else {
                    1
                }
            })
        });

        let target = dijkstra(&PAD_NUMBERS, &NUMBERS, b'A', &edges);

        {
            let (r, c) = find_position(&PAD_NUMBERS, b'0');        
            assert_eq!(target[r][c], 1);
        }

        {
            let (r, c) = find_position(&PAD_NUMBERS, b'7');        
            assert_eq!(target[r][c], 5);
        }
    }    
}
