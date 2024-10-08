/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
	let mut f = || {
        let p = 0usize;
        let mut left = 0;
        let mut right = array.len() - 1;
        while left < right {
            while array[right] >= array[p] && right > left {
                right -= 1;
            }
            while array[left] <= array[p] && right > left {
                left += 1;
            }
            array.swap(left, right);
        }
        array.swap(p, left);
        left
    };
    let l = f();
    let len = array.len();
    if l > 1 {
        let left = &mut array[0..l];
        sort(left);
    }
    if (l + 2) < len {
        let right = &mut array[(l+1)..len];
        sort(right);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}