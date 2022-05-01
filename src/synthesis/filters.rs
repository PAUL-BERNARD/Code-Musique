#![allow(arithmetic_overflow)]

use super::AudioBuffer;
use std::f32::consts::PI;

pub trait FilterTrait {
    fn low_pass(&self, cutoff_freq : f32) -> Self;
    fn adsr(&self, duration : f32, attack : f32, decay : f32, sustain : f32, release : f32) -> Result<Self,String> where Self: Sized;
    fn echo(&self, delta : f32, loudness : f32) -> Self;
}

impl FilterTrait for AudioBuffer {
    fn low_pass(&self, cutoff_freq : f32) -> Self {
        let mut buffer = Vec::with_capacity(self.len());

        let a1 = (-2.*PI*cutoff_freq / 44_000.).exp();
        let a2 = 1. - a1;

        buffer.push(0.);
        for i in 1..self.len() {
            buffer.push(a1*buffer[i-1] + a2*self[i]);
        }

        buffer
    }

    // Attack decay sustain release
    fn adsr(&self,duration : f32, attack : f32, decay : f32, sustain : f32, release : f32) -> Result<Self,String> {
        
        let attack_s = (attack*44_000.) as usize;
        let decay_s = (decay*44_000.) as usize;
        let release_s = (release*44_000.) as usize;
        let duration_s = (duration*44_000.) as usize;

        if duration_s < attack_s + decay_s {
            return Err(format!("Note duration ({}) should be longer than attack and decay time ({})", duration, attack+decay))
        }
        let sustain_s = duration_s - attack_s - decay_s;

        if duration_s+release_s > self.len() {
            return Err(format!("Note duration exceeds buffer size. Should be less than {}s, but is {}s",self.len() as f32 / 44_000.,duration+release))
        }
                
        let mut buffer = Vec::with_capacity(attack_s+decay_s+sustain_s+release_s);

        // Attack
        for i in 0..attack_s {
            buffer.push((i as f32)/(attack * 44_000.) * self[i]);
        }
        // Decay
        let k = (1. - sustain) / (decay * 44_000.);
        for i in 0..decay_s {
            buffer.push((1. - k * (i as f32)) * self[i]);
        }

        // Sustain
        for i in 0..sustain_s {
            buffer.push( sustain * self[i]);
        }

        // Release
        let k = sustain / release;
        for i in 0..release_s {
            buffer.push((sustain - k * (i as f32)) * self[i]);
        }

        Ok(buffer)
    }

    fn echo(&self, delta : f32, loudness : f32) -> AudioBuffer {
        let mut buffer = Vec::with_capacity(self.len()+(delta*44_000.) as usize);
        let delta : usize = (delta * 44_000.) as usize;
        for i in 0..self.len() {
            buffer[i] = self[i];
            buffer[i+delta] = self[i] * loudness;
        }

        buffer
    }

}
