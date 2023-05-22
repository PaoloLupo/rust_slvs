use slvs::{constraint::PtPtDistance, entity::Point, solver::FailReason, target::In3d, System};

#[test]
fn inconsistent_constraints() {
    let mut sys = System::new();
    let g = sys.add_group();

    let p1 = sys
        .sketch(Point::<In3d>::new(g, 10.0, 10.0, 10.0))
        .expect("p1 created");
    let p2 = sys
        .sketch(Point::<In3d>::new(g, 20.0, 20.0, 20.0))
        .expect("p2 created");

    // distance between p1 and p2 is 10
    let c1 = sys
        .constrain(PtPtDistance::new(g, p1.into(), p2.into(), 10.0, None))
        .expect("distance constraint added");
    // distance between p1 and p2 is 20
    let c2 = sys
        .constrain(PtPtDistance::new(g, p1.into(), p2.into(), 20.0, None))
        .expect("distance constraint added");

    let solve_result = sys.solve(&g);

    if let Err(fail_result) = solve_result {
        assert_eq!(fail_result.reason, FailReason::Inconsistent);
        println!("{:?}", fail_result.failed_constraints);
        assert!(fail_result.constraint_did_fail(&c1));
        assert!(fail_result.constraint_did_fail(&c2));
    }
}
