use crate::prelude::*;

    use super::uni_source_variant::{ UniSourceVariant };
    use super::uni_source_variant::{ UniSourceVariant::* };


//  //  //  //  //  //  //  //
//      config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {

    pub(super) fn invoke_exec_core(&mut self, script_name: &str ) -> ResultOf< () > {
        let path = format!( "workflows.{script_name}" );
        let exec_list = call_list::from_toml_table( &self.core_config, &path )?;
        for exec_item in exec_list {
            self.process_core_exec_item( &exec_item )?;
        }
        Ok(())
    }

    fn process_core_exec_item(&mut self, exec_item: &call_list::CallItem ) -> ResultOf< () > {
        match exec_item {
            call_list::CallItem::Simple( s ) => {
                return self.exec_core_simple( s );
            },
            call_list::CallItem::WithNested( key, nested_item ) => {
                return self.exec_core_withparam( key, nested_item );
            },
        }
    }
}

impl AudioServer {

    fn exec_core_simple(&mut self, cmd: &str ) -> ResultOf<()> {
        match cmd {
            "stop" => {
                self.audio_core.stop();
                return Ok(());
            },
            "start" => {
                return self.audio_core.start();
            },
            _ => {
                let msg = format!( "<AudioServer.exec_core_simple>: unknown command <{cmd}>" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }

    fn exec_core_withparam(&mut self, key: &str, nested_item: &call_list::CallItem ) -> ResultOf<()> {
        match key {
            "AudioSource" => {
                self.uni_source = UniSourceVariant::new( 
                                        nested_item, 
                                        &self.audio_core.get_sample_rate(), 
                                        &self.audio_core.get_time_increment(), 
                                        &self.sf_array
                                        )?;
                self.install_source_to_audio();
                Ok(())
            },
            _ => {
                let msg = format!( "<AudioServer.exec_core_withparam>: unknown key <{key}>" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
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


