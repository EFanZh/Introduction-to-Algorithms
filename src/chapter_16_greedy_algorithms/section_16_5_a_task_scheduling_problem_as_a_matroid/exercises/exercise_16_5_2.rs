pub fn check_schedule(deadline_tasks: &[usize], deadline: usize) -> bool {
    let (left, right) = deadline_tasks.split_at(deadline - 1);
    let mut sum = left.iter().sum::<usize>() + 1;

    for (i, num) in (deadline..).zip(right) {
        sum += num;

        if sum > i {
            return false;
        }
    }

    true
}
