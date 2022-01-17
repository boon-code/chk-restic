use super::*;
use super::action::Action;
use std::path::Path;

#[test]
fn test_checked() {
    let a = Action::new_checked();
    assert!(a.is_checked());
}

#[test]
fn test_failed() {
    let a = Action::new_failed(Path::new("/test/path"));
    assert!(!a.is_checked());
    assert_eq!(Path::new("/test"), a.failed().unwrap().parent().unwrap());
}

#[test]
fn test_numbers() {
    let a: i32 = 5;
    let x: Vec<String> = (0..a)
        .into_iter()
        .map(|i| format!("bla-{}", i))
        .collect();
    assert_eq!(x, vec!["bla-0", "bla-1", "bla-2", "bla-3", "bla-4"]);
}

#[test]
fn test_option() {
    let mut opt: Option<i32> = Some(5);
    if let Some(a) = opt.take() {
        assert_eq!(5_i32, a);
    } else {
        panic!("Not good");
    }
}