#![cfg(test)]

use crate::masking::MaskingArrayRingBuffer;

#[test]
fn enqueue_and_dequeue_once() {
    let mut buf = MaskingArrayRingBuffer::<_, 4>::default();
    buf.enqueue(1);
    assert_eq!(buf.dequeue(), Some(1));
    assert_eq!(buf.dequeue(), None);
}

#[test]
fn fill_buffer_up_before_dequeue() {
    let mut buf = MaskingArrayRingBuffer::<_, 4>::default();

    assert_eq!(None, buf.enqueue(1));
    assert_eq!(None, buf.enqueue(2));
    assert_eq!(None, buf.enqueue(3));
    assert_eq!(None, buf.enqueue(4));

    assert!(buf.is_full());
    assert_eq!(Some(1), buf.enqueue(5));
    assert!(buf.is_full());

    assert_eq!(Some(2), buf.dequeue());
    assert_eq!(Some(3), buf.dequeue());
    assert_eq!(Some(4), buf.dequeue());
    assert_eq!(Some(5), buf.dequeue());
}

#[test]
fn wrap_many_times() {
    let mut buf = MaskingArrayRingBuffer::<_, 4>::default();

    let mut total = 0;
    for i in 1..=40 {
        if let Some(n) = buf.enqueue(i) {
            total += n;
        }
    }

    while let Some(n) = buf.dequeue() {
        total += n;
    }

    assert_eq!(820, total);
}
