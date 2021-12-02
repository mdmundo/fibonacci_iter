struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

pub fn get(position: usize) -> u32 {
    fibonacci().skip(position).take(1).collect::<Vec<u32>>()[0]
}

pub fn first(size: usize) -> Vec<u32> {
    fibonacci().take(size).collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_four() {
        assert_eq!(fibonacci().take(4).collect::<Vec<u32>>(), vec![1, 1, 2, 3]);
    }

    #[test]
    fn fifth() {
        assert_eq!(fibonacci().skip(4).take(1).collect::<Vec<u32>>(), vec![5]);
    }

    #[test]
    fn fifth_from_pub() {
        assert_eq!(get(4), 5);
    }

    #[test]
    fn first_five_from_pub() {
        assert_eq!(first(5), vec![1, 1, 2, 3, 5]);
    }

}
