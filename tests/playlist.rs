#[cfg(test)]
mod playlist {
    use projectm::core::ProjectM;
    use projectm::playlist::Playlist;

    #[test]
    fn playlist() {
        let mut projectm = ProjectM::new();
        let playlist = Playlist::create(&mut projectm);
        assert!(playlist.is_empty());

        // add ../presets to playlist
        // get absolute path to ../presets
        let path = std::env::current_dir().unwrap();
        let presets_dir = path.join("presets");
        playlist.add_path(presets_dir.to_str().unwrap(), true);
        assert_eq!(playlist.len(), 20);
    }
}
