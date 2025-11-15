use core::mem::{needs_drop, MaybeUninit};
use core::ptr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmptyField;

pub fn try_map_array_init<I, O, E, const N: usize>(
    input: [I; N],
    mut mapper: impl FnMut(I) -> Result<O, E>,
) -> Result<[O; N], E> {
    if const { needs_drop::<O>() } {
        // Makes sure to drop initialized elements on panic or error.
        struct ArrayDropGuard<T, const N: usize> {
            array: [MaybeUninit<T>; N],
            initialized: usize,
        }
        impl<T, const N: usize> Drop for ArrayDropGuard<T, N> {
            fn drop(&mut self) {
                self.array[..self.initialized].iter_mut().for_each(|x| {
                    // Safety: We are only dropping initialized elements.
                    unsafe { MaybeUninit::assume_init_drop(x) }
                });
            }
        }

        let mut out_array = ArrayDropGuard {
            array: [const { MaybeUninit::uninit() }; N],
            initialized: 0,
        };

        // Kept as an iterator so later items are dropped on panic or error.
        for (i, arg) in input.into_iter().enumerate() {
            match mapper(arg) {
                Ok(o) => {
                    // Safety: `i` is in bounds because `i < N`.
                    unsafe { out_array.array.get_unchecked_mut(i).write(o) };
                    out_array.initialized += 1;
                }
                Err(e) => return Err(e),
            }
        }

        // Safety: All elements of the array were initialized, and `MaybeUninit` is transparent to
        // `O`. `MaybeUninit` also will not drop so `out_array` will not drop the read elements.
        Ok(unsafe { ptr::from_mut(&mut out_array.array).cast::<[O; N]>().read() })
    } else {
        let mut out_array: [MaybeUninit<O>; N] = [const { MaybeUninit::uninit() }; N];

        // Kept as an iterator so later items are dropped on panic or error.
        for (i, arg) in input.into_iter().enumerate() {
            match mapper(arg) {
                // Safety: `i` is in bounds because `i < N`.
                Ok(o) => unsafe {
                    out_array.get_unchecked_mut(i).write(o);
                },
                Err(e) => return Err(e),
            };
        }
        // Safety: All elements of the array were initialized, and `MaybeUninit` is transparent to
        // `O`.
        Ok(unsafe { ptr::from_mut(&mut out_array).cast::<[O; N]>().read() })
    }
}
