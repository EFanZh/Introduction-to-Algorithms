use std::iter;

pub struct Employee {
    name: String,
    conviviality: f64,
    left_child: Option<Box<Self>>,
    right_sibling: Option<Box<Self>>,
}

impl Employee {
    pub fn new<I: IntoIterator<Item = Self>>(name: String, conviviality: f64, children: I) -> Self {
        let mut result = Self {
            name,
            conviviality,
            left_child: None,
            right_sibling: None,
        };

        let mut new_child_location = &mut result.left_child;

        for child in children {
            *new_child_location = Some(Box::new(child));

            new_child_location = &mut new_child_location.as_mut().unwrap().right_sibling;
        }

        result
    }

    #[must_use]
    pub fn new_leaf(name: String, conviviality: f64) -> Self {
        Self {
            name,
            conviviality,
            left_child: None,
            right_sibling: None,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn conviviality(&self) -> f64 {
        self.conviviality
    }

    pub fn subordinates(&self) -> impl Iterator<Item = &Employee> {
        iter::successors(self.left_child.as_deref(), |node| node.right_sibling.as_deref())
    }
}

struct CacheItem<'a> {
    employee: &'a Employee,
    attend: bool,
    total_conviviality: f64,
    children: Box<[Self]>,
}

fn build_conviviality_cache(employee: &Employee) -> CacheItem<'_> {
    let mut children = Vec::new();

    for subordinate in employee.subordinates() {
        children.push(build_conviviality_cache(subordinate));
    }

    // Conviviality if this employee attends.

    let conviviality_if_attends = employee.conviviality()
        + children
            .iter()
            .flat_map(|child| child.children.iter())
            .map(|child| child.total_conviviality)
            .sum::<f64>();

    let conviviality_if_not_attends = children.iter().map(|child| child.total_conviviality).sum();

    let (attend, total_conviviality) = if conviviality_if_attends > conviviality_if_not_attends {
        (true, conviviality_if_attends)
    } else {
        (false, conviviality_if_not_attends)
    };

    CacheItem {
        employee,
        attend,
        total_conviviality,
        children: children.into(),
    }
}

fn build_attendance_list<'a>(cache: &CacheItem<'a>, result: &mut Vec<&'a Employee>) {
    if cache.attend {
        result.push(cache.employee);

        for child in cache.children.iter().flat_map(|child| child.children.iter()) {
            build_attendance_list(child, result);
        }
    } else {
        for child in cache.children.iter() {
            build_attendance_list(child, result);
        }
    }
}

#[must_use]
pub fn plan_party(president: &Employee) -> (Box<[&Employee]>, f64) {
    let cache = build_conviviality_cache(president);
    let mut result = Vec::new();

    build_attendance_list(&cache, &mut result);

    (result.into(), cache.total_conviviality)
}

#[cfg(test)]
mod tests {
    use super::{plan_party, Employee};
    use approx::assert_relative_eq;
    use std::f64;

    #[test]
    fn test_plan_party() {
        fn run_test(president: &Employee, expected_attendances: &[&str], expected_conviviality: f64) {
            let (actual_attendances, actual_expected_conviviality) = plan_party(president);
            let actual_iter = actual_attendances.iter().map(|employee| employee.name());
            let expected_iter = expected_attendances.iter().copied();

            assert!(actual_iter.eq(expected_iter));
            assert_relative_eq!(actual_expected_conviviality, expected_conviviality);
        }

        run_test(&Employee::new_leaf("Jim".to_owned(), 1.0), &["Jim"], 1.0);

        run_test(
            &Employee::new(
                "Jim".to_owned(),
                1.0,
                vec![
                    Employee::new_leaf("Tom".to_owned(), 1.0),
                    Employee::new_leaf("Sam".to_owned(), 1.0),
                ],
            ),
            &["Tom", "Sam"],
            2.0,
        );

        run_test(
            &Employee::new(
                "Jim".to_owned(),
                3.0,
                vec![
                    Employee::new_leaf("Tom".to_owned(), 1.0),
                    Employee::new_leaf("Sam".to_owned(), 1.0),
                ],
            ),
            &["Jim"],
            3.0,
        );

        run_test(
            &Employee::new(
                "Jim".to_owned(),
                1.0,
                vec![
                    Employee::new(
                        "Tom".to_owned(),
                        1.0,
                        vec![
                            Employee::new_leaf("Sam".to_owned(), 1.0),
                            Employee::new_leaf("Alice".to_owned(), 1.0),
                        ],
                    ),
                    Employee::new(
                        "Bob".to_owned(),
                        1.0,
                        vec![
                            Employee::new_leaf("Jack".to_owned(), 1.0),
                            Employee::new_leaf("John".to_owned(), 1.0),
                        ],
                    ),
                ],
            ),
            &["Jim", "Sam", "Alice", "Jack", "John"],
            5.0,
        );

        run_test(
            &Employee::new(
                "Jim".to_owned(),
                1.0,
                vec![
                    Employee::new(
                        "Tom".to_owned(),
                        4.0,
                        vec![
                            Employee::new_leaf("Sam".to_owned(), 1.0),
                            Employee::new_leaf("Alice".to_owned(), 1.0),
                        ],
                    ),
                    Employee::new(
                        "Bob".to_owned(),
                        1.0,
                        vec![
                            Employee::new_leaf("Jack".to_owned(), 1.0),
                            Employee::new_leaf("John".to_owned(), 1.0),
                        ],
                    ),
                ],
            ),
            &["Tom", "Jack", "John"],
            6.0,
        );
    }
}
