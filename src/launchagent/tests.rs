use super::*;

#[test]
fn can_create_simple_launch_agent() {
    let agent = LaunchAgent::new("com.example.test", "/usr/bin/example");

    assert_eq!(agent.label, "com.example.test");
    assert_eq!(agent.program.unwrap(), "/usr/bin/example");
    assert_eq!(agent.program_arguments, None);
}

#[test]
fn can_create_simple_launch_agent_with_args() {
    let agent = LaunchAgent::new_with_args(
        "com.example.test",
        vec!["/usr/bin/example", "--option", "value"],
    );

    assert_eq!(agent.label, "com.example.test");
    assert_eq!(
        agent.program_arguments.unwrap(),
        vec!["/usr/bin/example", "--option", "value"]
    );
}
