use std::f32::consts::PI;
#[allow(unused_imports)]
use micromath::F32Ext;
use super::AudioBuffer;

trait Oscillator {
    // OSCILLATORS
    fn sin_wave(sample_size : usize, frequency : usize) -> Self;
    fn sawtooth_wave(sample_size : usize, frequency : usize) -> Self;
    fn triangle_wave(sample_size : usize, frequency : usize) -> Self;
    fn square_wave(sample_size : usize, frequency : usize) -> Self;
    fn square_wave_with_value(sample_size : usize, frequency : usize, value : f32) -> Self;
}

impl Oscillator for AudioBuffer {
    fn sin_wave(sample_size : usize, frequency : usize) -> Self {
        let mut buffer = Vec::with_capacity(sample_size);

        let f2pi = frequency as f32 * 2f32 * PI / 44_000f32;

        for i in 0..sample_size {
            buffer.push((i as f32 * f2pi).cos());
        }

        buffer
    }

    fn sawtooth_wave(sample_size : usize, frequency : usize) -> Self {
        let frequency = frequency as f32;
        let mut buffer = Vec::with_capacity(sample_size);

        let period = 1. / frequency; 

        for i in 0..sample_size {
            buffer.push(2.*frequency*((i as f32 / 44_000.) % period));
        }

        buffer
    }

    fn square_wave(sample_size : usize, frequency : usize) -> Self {
        let frequency = frequency as f32;
        let mut buffer = Vec::with_capacity(sample_size);

        let half_period = 0.5 / frequency;
        let period = 1. / frequency;

        for i in 0..sample_size {
            buffer.push(if ((i as f32 / 44_000.)%period) > half_period {1.} else {-1.});
        }

        buffer
    }

    fn square_wave_with_value(sample_size : usize, frequency : usize, value : f32) -> Self {
        let frequency = frequency as f32;
        let mut buffer = Vec::with_capacity(sample_size);
        let neg_value = -value;

        let half_period = 0.5 / frequency;
        let period = 1. / frequency;

        for i in 0..sample_size {
            buffer.push(if ((i as f32 / 44_000.)%period) > half_period {value} else {neg_value});
        }

        buffer
    }

    fn triangle_wave(sample_size : usize, frequency : usize) -> Self {
        let mut buffer = Vec::with_capacity(sample_size);
        let square_buffer = AudioBuffer::square_wave_with_value(sample_size, frequency, 1./44_000.);

        for i in 0..sample_size {
            buffer.push(buffer.last().unwrap_or(&0.) + square_buffer[i]);
        }

        buffer
    }

    
}

