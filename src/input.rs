pub trait Input: Clone {
    type Item: Copy;

    fn next(&mut self) -> Option<Self::Item>;

    fn take_while<F: FnMut(&Self::Item) -> bool>(&mut self, pred: F) -> Self;


}

impl<'a> Input for &'a str {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars().next()?;
        *self = &self[ch.len_utf8()..];
        Some(ch)
    }

    fn take_while<F: FnMut(&Self::Item) -> bool>(&mut self, mut pred: F) -> Self {
        let mut s = *self;
        let mut len = 0;

        loop {
            match s.next() {
                None => break,
                Some(t) if pred(&t) => {
                    len += t.len_utf8();
                    continue
                },
                Some(_) => break
            }
        }

        let r = &self[..len];

        *self = &self[len..];

        r
    }

}

impl<'a, I> Input for &'a [I] where I: Copy{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None
        }

        let b = self[0];
        *self = &self[1..];
        Some(b)
    }

    fn take_while<F: FnMut(&Self::Item) -> bool>(&mut self, mut pred: F) -> Self {
        let mut b = *self;
        let mut len = 0;

        loop {
            match b.next() {
                None => break,
                Some(t) if pred(&t) => {
                    len += 1;
                    continue
                },
                Some(_) => break
            }
        }

        let r = &self[..len];

        *self = &self[len..];

        r
    }
}



mod test {
    use crate::Input;


    #[test]
    fn test() {
        let mut bs = &b"   123"[..];

        bs.take_while(|x| *x == b' ');

        println!("{:?}", bs.next());
        println!("{:?}", bs.next());
        println!("{:?}", bs.next());
        println!("{:?}", bs.next());
    }
}