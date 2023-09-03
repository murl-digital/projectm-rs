#[cfg(test)]
mod core {
    use std::process::Command;
    use std::str;
    use projectm::core::Projectm;

    fn get_git_hash_by_command() -> Option<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .ok()?;
        
        if output.status.success() {
            let git_hash = str::from_utf8(&output.stdout).ok()?.trim().to_string();
            println!("git_hash: {}", git_hash);
            Some(git_hash)
        } else {
            None
        }
    }

    #[test]
    fn test_get_versions() {
        let version_tuple = Projectm::get_version_components();
        assert_eq!(version_tuple, (4, 0, 0));

        let version_string = Projectm::get_version_string();
        assert_eq!(version_string, "4.0.0");

        let vcs_version_string = Projectm::get_vcs_version_string();
        assert_eq!(vcs_version_string, get_git_hash_by_command().unwrap());
    }

    // #[test]
    // fn test_sample() {
    //     let projectm = Projectm::create();

    //     Projectm::sample(projectm, 60);
    //     assert_eq!(projectm, sample);
    // }
}
