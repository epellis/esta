/// This managed runtime is a wrapper around running ESTA code and will automatically
/// halt the runtime of the VM if the number of steps has exceeded the limit.
fn managed_runtime(program: &str, max_steps: usize) -> bool {
    use crate::vm::StepCode;
    use crate::*;

    let stmts = frontend::run(&program).unwrap();
    let (stmts, md) = middleend::run(stmts).unwrap();
    let inst = backend::generate(stmts, md).unwrap();
    for (j, i) in inst.iter().enumerate() {
        debug!("{: >3} {}", j, i);
    }
    let mut vm = vm::VirtualMachine::new(inst);
    let mut count = 0;
    let max_count = 2000;
    while let Ok(StepCode::CONTINUE) = vm.step() {
        count += 1;
        if count > max_count {
            return false;
            break;
        }
    }
    true
}

#[test]
fn test_function_calls() {
    use std::fs;
    let limit = 2000;

    let paths = vec![
        "demos/simple_function.est",
        "demos/simple_function2.est",
        "demos/simple_function3.est",
        "demos/simple_function4.est",
        //        "demos/simple_function5.est",
    ];
    let res = paths
        .iter()
        .map(|p| fs::read_to_string(p).expect("Couldn't read file!"))
        .map(|b| managed_runtime(&b, limit))
        .zip(paths.iter())
        .inspect(|(r, p)| println!("Test {}: {}", p, r))
        .map(|(r, p)| r)
        .fold(true, |acc, res| acc && res);
    assert_eq!(res, true);
}
