use super::*;

#[test]
fn test_walker_simple() {
    let pw = ParallelWalker::new(1, 5);
    pw.add(Path::new("/bla"));
    pw.add(Path::new("/ble"));
    pw.add(Path::new("/ha/ha"));
    let a = pw.recv().unwrap();
    assert!(a.is_checked());
    pw.stop();
}

#[test]
fn test_multiple_workers() {

}

#[test]
fn test_crossbeam_close() {
    let (s, r) = bounded(2);
    s.send(5_i32).ok().unwrap();
    s.send(7_i32).ok().unwrap();
    drop(s);

    assert_eq!(5_i32, r.recv().ok().unwrap());
    assert_eq!(7_i32, r.recv().ok().unwrap());
    assert!(r.recv().is_err());
}

#[test]
fn test_crossbeam_closed_rx() {
    let (s, r) = bounded(2);
    s.send(5_i32).ok().unwrap();
    s.send(7_i32).ok().unwrap();
    assert_eq!(5_i32, r.recv().ok().unwrap());
    drop(r);
    assert!(s.send(9_i32).is_err());
}