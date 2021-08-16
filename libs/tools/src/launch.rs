use crate::ElasticSearchWrapper;
use std::process::Command;

pub fn launch_and_assert(
    cmd: &str,
    args: &[std::string::String],
    es_wrapper: &ElasticSearchWrapper,
) {
    let status = Command::new(cmd).args(args).status().unwrap();
    assert!(
        status.success(),
        "`{}` with args {:?} failed with status {}",
        cmd,
        args,
        &status
    );
    es_wrapper.refresh();
}
