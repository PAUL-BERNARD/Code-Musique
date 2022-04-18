use super::AudioBuffer;

trait Oscillator {
    // OSCILLATORS
    fn sin_wave(sample_size : usize, frequency : usize) -> Self;
    fn sawtooth_wave(sample_size : usize, frequency : usize) -> Self;
    fn triangle_wave(sample_size : usize, frequency : usize) -> Self;
    fn square_wave(sample_size : usize, frequency : usize) -> Self;
}

impl Oscillator for AudioBuffer {
    fn sin_wave(sample_size : usize, frequency : usize) -> Self {
        todo!()
    }

    fn sawtooth_wave(sample_size : usize, frequency : usize) -> Self {
        todo!()
    }

    fn triangle_wave(sample_size : usize, frequency : usize) -> Self {
        todo!()
    }

    fn square_wave(sample_size : usize, frequency : usize) -> Self {
        todo!()
    }
}

