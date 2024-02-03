use std::error::Error;
use toml::{ Table, Value };

    use super::uni_source_variant::{ UniSourceVariant };
    use super::uni_source_variant::{ UniSourceVariant::* };

//  //  //  //  //  //  //  //
//      config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {

    pub(crate) fn invoke_core_config_loading(&mut self, tbl: &Table, sf_array: &Vec<&'static [u8]> ) -> Result<(), Box<dyn Error>> {
        self.sf_array = sf_array.clone();
        self.core_config = tbl.clone();
        if let Some(exec_value) = self.core_config.get( "autoexec" ) {
            let list = call_list::from_toml_value(exec_value)?;
            for i in list {
                match i {
                    call_list::CallItem::Simple(cmd) => {
                        println!(" --> : {}", cmd );
                    },
                    call_list::CallItem::WithParam(cmd, param ) => {
                        println!(" --> : {}({})", cmd, param );
                    },
                }
            }
        }
        Ok(())
    }


    pub(crate) fn invoke_config_parsing(&mut self, tbl: &Table, data: Option<&[u8]> ) -> Result<(), Box<dyn Error>> {
        if let Some(au_val) = tbl.get("AudioSource") {
            if let Value::Table(au_tbl) = au_val {
                let sample_rate = self.audio_core.get_sample_rate();
                let time_increment = self.audio_core.get_time_increment();
                self.uni_source = UniSourceVariant::new( &au_tbl, &sample_rate, time_increment, data )?;
                self.install_source_to_audio();
            }else{
                return Err(Box::from("invalid section AudioSource"));
            }
        }
        if let Some(seq_val) = tbl.get("Sequence") {
            self.midi_sequence = Some( Self::create_midi_sequence( seq_val )? );
        }

        Ok(())
    }

    //  //  //  //  //  //  //
    fn install_source_to_audio(&mut self) {
        match &self.uni_source {
            Silence => {
                self.audio_core.install_render(None);
            },
            Audio(wrapped_audio_render) => {
                self.audio_core.install_render(Some( wrapped_audio_render.clone() ));
            },
            Simple(simsyn) => {
                self.audio_core.install_render(Some( simsyn.clone() ));
            },
            Rusty(ryssyn) => {
                self.audio_core.install_render(Some( ryssyn.clone() ));
            },
            Sequencer(sequencer) => {
                self.audio_core.install_render(Some( sequencer.clone() ));
            },
        }
    }
}



//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod basic {
    use super::*;

    #[ignore]
    #[test]
    fn no_error() {
        let mut aud = AudioServer::new();
        let flag;
        if let Ok(()) = aud.config(TEST_CONFIG, None) {
            flag = true;
        }else{
            flag = false;
        }
        assert!(flag, "TEST_CONFIG is not valid");
    }
    #[ignore]
    #[test]
    fn is_error() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(_) = aud.config("]][{", None) {
            flag = true;
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
}

#[cfg(test)]
static TEST_CONFIG: &str = r#"
    anystring = 'any'
    [AudioSource]
    Name = 'None'
    [Sequencer]
"#;

#[cfg(test)]
mod audio_source {
    use super::*;
    use raalog::log;

    #[ignore]
    #[test]
    fn empty_config() {
        let mut aud = AudioServer::new();
        let flag;
        if let Ok(()) = aud.config("", None) {
            flag = true;
        }else{
            flag = false;
        }
        assert!(flag, "empty config have not be an error");
    }
    #[ignore]
    #[test]
    fn test_config() {
        let mut aud = AudioServer::new();
        let flag;
        if let Ok(()) = aud.config(TEST_CONFIG, None) {
            flag = true;
        }else{
            flag = false;
        }
        assert!(flag, "empty config have not be an error");
    }
    #[ignore]
    #[test]
    fn audio_source_error() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(e) = aud.config("AudioSource = 'err'", None) {
            flag = true;
            log::error(&e.to_string());
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
    #[ignore]
    #[test]
    fn no_main() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(e) = aud.config("[AudioSource]", None) {
            flag = true;
            log::error(&e.to_string());
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
    #[ignore]
    #[test]
    fn invalid_main() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(e) = aud.config("[AudioSource]\nMain = ['d']", None) {
            flag = true;
            log::error(&e.to_string());
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
}


