use raalog::log;
use crate::prelude::*;

    use super::midi_lib::{ MidiMessage, MidiSequence };
    use super::uni_source_variant::UniSourceVariant::Sequencer;


//  //  //  //  //  //  //  //
//      exec impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(super) fn invoke_debug_midi_parsing(&mut self, commands: &str) -> ResultOf<()> {
        match commands {
            "play loop" => {
                if let Some(seq_orig) = &self.midi_sequence {
                    self.set_sequence( seq_orig.clone(), true);
                    return Ok(());
                }else{
                    let msg = format!( "<AudioServer.invoke_exec_parsing>({commands}): there is no loaded sequence for play" );
                    return Err( Box::from( msg.as_str() ) );
                }
            },
            "play once" => {
                if let Some(seq_orig) = &self.midi_sequence {
                    self.set_sequence( seq_orig.clone(), false);
                    return Ok(());
                }else{
                    let msg = format!( "<AudioServer.invoke_exec_parsing>({commands}): there is no loaded sequence for play" );
                    return Err( Box::from( msg.as_str() ) );
                }
            },
            _ => {
                let midi = MidiMessage::from_str( commands )?;
                self.uni_source.send_to_synth(&midi);
                return Ok(());
            },
        }
    }
}

//  //  //  //  //  //  //  //
//      internal
//  //  //  //  //  //  //  //
impl AudioServer {

    fn set_sequence(&mut self, seq: MidiSequence, is_auto_repeat: bool ) {
        match &self.uni_source {
            Sequencer(sequencer) => {
                let mut locked_sequencer = sequencer.lock()
                    .expect("FATAL of locking Sequencer");
                locked_sequencer.set_midi_sequence(seq, is_auto_repeat );
            },
            _ => {
                log::info("set_sequence: NOT a Sequencer.Ignoring")
            },
            
        }
    }

}

