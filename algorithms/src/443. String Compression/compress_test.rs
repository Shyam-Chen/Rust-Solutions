use super::*;

#[test]
fn compress() {
    {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let len = Solution::compress(&mut chars);
        assert_eq!(len, 6);
        assert_eq!(&chars[..len as usize], ['a', '2', 'b', '2', 'c', '3']);
    }

    {
        let mut chars = vec!['a'];
        let len = Solution::compress(&mut chars);
        assert_eq!(len, 1);
        assert_eq!(&chars[..len as usize], ['a']);
    }

    {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let len = Solution::compress(&mut chars);
        assert_eq!(len, 4);
        assert_eq!(&chars[..len as usize], ['a', 'b', '1', '2']);
    }
}
