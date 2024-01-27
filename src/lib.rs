    mod audio_server;
pub use audio_server::AudioServer as AudioServer;



//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;
    use raalog::log;

    #[test]
    fn creation() {
        log::info("simple create and drop AudioServer");
        let _ = AudioServer::new();
    }
    #[test]
    fn state() {
        let mut audio = AudioServer::new();
        assert!(audio.state() == "inactive", "AudioServer should be in <inactive> state after creation");
        let _ = audio.exec("start");
        assert!(audio.state() == "running", "AudioServer should be in <running>");
        let _ = audio.exec("stop");
        assert!(audio.state() == "inactive", "AudioServer should be in <inactive>");
        let _ = audio.exec("stop");
        assert!(audio.state() == "inactive", "AudioServer should be in <inactive>");
        let _ = audio.exec("start");
        assert!(audio.state() == "running", "AudioServer should be in <running>");
        let _ = audio.exec("start");
        assert!(audio.state() == "running", "AudioServer should be in <running>");
    }
}