use hardly_working::task::TodoList;

#[test]
fn test_add_task() {
    let mut todo = TodoList::new();
    todo.add_task("Task".to_string(), None);
    assert_eq!(todo.tasks.len(), 1);
    assert_eq!(todo.tasks[0].description, "Task");
    assert!(!todo.tasks[0].completed);
}

#[test]
fn test_toggle_task() {
    let mut todo = TodoList::new();
    todo.add_task("Task".to_string(), None);
    assert!(todo.toggle_task(1).is_ok());
    assert!(todo.tasks[0].completed);
}

#[test]
fn test_remove_task() {
    let mut todo = TodoList::new();
    todo.add_task("Task 1".to_string(), None);
    assert!(todo.remove_task(1).is_ok());
    assert_eq!(todo.tasks.len(), 0);
}

#[test]
fn test_clear_completed_tasks() {
    let mut todo = TodoList::new();
    todo.add_task("Task 1".to_string(), None);
    todo.add_task("Task 2".to_string(), None);
    todo.toggle_task(1).unwrap();
    todo.clear();
    assert_eq!(todo.tasks.len(), 1);
    assert_eq!(todo.tasks[0].description, "Task 2");
}

#[test]
fn test_edit_task() {
    let mut todo = TodoList::new();
    todo.add_task("Old description".to_string(), None);
    todo.edit_task(1, "New description".to_string()).unwrap();
    assert_eq!(todo.tasks[0].description, "New description");
}

#[test]
fn test_search_tasks() {
    let mut todo = TodoList::new();
    todo.add_task("Buy apples".to_string(), None);
    todo.add_task("Clean kitchen".to_string(), None);
    let results = todo.search_tasks("buy");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.description, "Buy apples");
}

#[test]
fn test_toggle_invalid_task_id() {
    let mut todo = TodoList::new();
    let result = todo.toggle_task(1); // No tasks yet
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Task 1 does not exist");
}

#[test]
fn test_remove_invalid_task_id() {
    let mut todo = TodoList::new();
    todo.add_task("A task".to_string(), None);
    let result = todo.remove_task(2);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Task 2 does not exist");
}

#[test]
fn test_edit_invalid_task_id() {
    let mut todo = TodoList::new();
    let result = todo.edit_task(1, "Doesn't exist".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Task 1 does not exist");
}

#[test]
fn test_search_empty_string_returns_all() {
    let mut todo = TodoList::new();
    todo.add_task("Task A".to_string(), None);
    todo.add_task("Task B".to_string(), None);
    let results = todo.search_tasks("");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_search_case_insensitive() {
    let mut todo = TodoList::new();
    todo.add_task("Wash Dishes".to_string(), None);
    let results = todo.search_tasks("wash");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1.description, "Wash Dishes");

    let results_upper = todo.search_tasks("WASH");
    assert_eq!(results_upper.len(), 1);
}

#[test]
fn test_group_sort_order() {
    let mut todo = TodoList::new();
    todo.add_task("Task A".to_string(), Some("Gamma".to_string()));
    todo.add_task("Task B".to_string(), Some("Alpha".to_string()));
    todo.add_task("Task C".to_string(), Some("Beta".to_string()));

    let md = todo.to_markdown();
    let expected_order = vec!["## Alpha", "## Beta", "## Gamma"];
    for (i, group) in expected_order.iter().enumerate() {
        assert!(
            md.contains(group),
            "Expected markdown to contain group heading: {}",
            group
        );
        let pos = md.find(group).unwrap();
        if i > 0 {
            let prev_pos = md.find(expected_order[i - 1]).unwrap();
            assert!(
                prev_pos < pos,
                "Expected group {} before {}",
                expected_order[i - 1],
                group
            );
        }
    }
}

#[test]
fn test_ungrouped_tasks_precede_grouped() {
    let mut todo = TodoList::new();
    todo.add_task("Ungrouped Task".to_string(), None);
    todo.add_task("Grouped Task".to_string(), Some("Group".to_string()));

    let md = todo.to_markdown();
    let ungrouped_pos = md.find("Ungrouped Task").unwrap();
    let grouped_pos = md.find("Grouped Task").unwrap();
    assert!(ungrouped_pos < grouped_pos);
}

#[test]
fn test_clear_on_empty_does_nothing() {
    let mut todo = TodoList::new();
    todo.clear();
    assert_eq!(todo.tasks.len(), 0);
}
