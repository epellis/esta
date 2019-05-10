///// This managed runtime is a wrapper around running ESTA code and will automatically
///// halt the runtime of the VM if the number of steps has exceeded the limit.
//fn managed_runtime(program: &str, max_steps: usize) -> bool {
//    use crate::vm::StepCode;
//    use crate::*;
//
//    let stmts = frontend::run(&program).unwrap();
//    let (stmts, md) = middleend::run(stmts).unwrap();
//    let (inst, data) = backend::generate(stmts, md).unwrap();
//    for (j, i) in inst.iter().enumerate() {
//        debug!("{: >3} {}", j, i);
//    }
//    debug!("Data Segment: {:?}", data);
//    let mut vm = vm::VirtualMachine::new(inst, data);
//    let mut count = 0;
//    while let Ok(StepCode::CONTINUE) = vm.step() {
//        count += 1;
//        if count > max_steps {
//            return false;
//            break;
//        }
//    }
//    true
//}
//
//fn do_tests(paths: &Vec<&str>, limit: usize) {
//    use std::fs;
//
//    let res = paths
//        .iter()
//        .map(|p| fs::read_to_string(p).expect("Couldn't read file!"))
//        .map(|b| managed_runtime(&b, limit))
//        .zip(paths.iter())
//        .inspect(|(r, p)| println!("Test {}: {}", p, r))
//        .map(|(r, p)| r)
//        .fold(true, |acc, res| acc && res);
//    assert_eq!(res, true);
//}
//
//#[test]
//fn test_function_calls() {
//    let paths = vec![
//        "testsuite/simple_function1.est",
//        "testsuite/simple_function2.est",
//        "testsuite/simple_function3.est",
//        "testsuite/simple_function4.est",
//        "testsuite/simple_function5.est",
//    ];
//    do_tests(&paths, 2000);
//}
//
//#[test]
//fn test_control_flow() {
//    let paths = vec![
//        "testsuite/while.est",
//        "testsuite/for.est",
//        "testsuite/if.est",
//    ];
//    do_tests(&paths, 20000);
//}
//
//#[test]
//fn test_assignments() {
//    let paths = vec!["testsuite/assignment.est"];
//    do_tests(&paths, 2000);
//}
//
//#[test]
//fn test_struct() {
//    let paths = vec!["testsuite/struct.est"];
//    do_tests(&paths, 2000);
//}
//
//#[test]
//fn test_real_world() {
//    let paths = vec!["testsuite/realworld.est"];
//    do_tests(&paths, 20000);
//}
