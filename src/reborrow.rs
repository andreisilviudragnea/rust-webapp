// pub struct IterMut<'a, T> {
//     inner: &'a mut Vec<T>,
//     index: usize,
// }
//
// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//
//     fn next<'b>(&'b mut self) -> Option<&'a mut T> {
//         if self.inner.is_empty() {
//             return None;
//         }
//
//         let item = &mut self.inner[self.index];
//         self.index += 1;
//         Some(item)
//     }
// }

struct ByteIter<'remainder> {
    remainder: &'remainder [u8], // TODO understand why using mut here results in weird compile-time error
    index: usize,
}

impl<'remainder> ByteIter<'remainder> {
    fn next(&mut self) -> Option<&'remainder u8> {
        let item = self.remainder.get(self.index);
        self.index += 1;
        item
    }
}

#[test]
fn test() {
    let mut bytes = ByteIter {
        remainder: &mut b"1123".to_vec(),
        index: 0,
    };
    let byte_1 = bytes.next();
    let byte_2 = bytes.next();
    #[allow(clippy::drop_non_drop)]
    drop(bytes); // we can even drop the iterator now!
    assert_eq!(byte_1, byte_2);
}
