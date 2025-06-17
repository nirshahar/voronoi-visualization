use nannou::{prelude::Vec2, rand::Rng};

pub struct MultiOscillator<const N: usize> {
    oscillators: [DirecionalOscillator; N],
}

impl<const N: usize> MultiOscillator<N> {
    pub fn new(oscillators: [DirecionalOscillator; N]) -> Self {
        Self { oscillators }
    }

    pub fn rand_new<R: Rng>(rng: &mut R) -> Self {
        let oscillators = [(); N];
        let oscillators = oscillators.map(|_| DirecionalOscillator::rand_new(rng));

        Self::new(oscillators)
    }

    pub fn generate(&self, time: f32) -> Vec2 {
        self.oscillators
            .map(|oscillator| oscillator.generate(time))
            .iter()
            .sum::<Vec2>()
            / (N as f32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DirecionalOscillator {
    oscillator: Oscillator,
    direction: Vec2,
}

impl DirecionalOscillator {
    pub fn new(oscillator: Oscillator, direction: Vec2) -> Self {
        Self {
            oscillator,
            direction: direction.normalize(),
        }
    }

    pub fn rand_new<R: Rng>(rng: &mut R) -> Self {
        let oscillator = Oscillator::rand_new(rng);
        let direction = Vec2::new(rng.gen(), rng.gen());

        Self::new(oscillator, direction)
    }

    pub fn generate(&self, time: f32) -> Vec2 {
        self.oscillator.generate(time) * self.direction
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Oscillator {
    amplitude: f32,
    period: f32,
    offset: f32,
}

impl Oscillator {
    pub fn new(amplitude: f32, period: f32, offset: f32) -> Self {
        Self {
            amplitude,
            period,
            offset,
        }
    }

    pub fn rand_new<R: Rng>(rng: &mut R) -> Self {
        let amplitude = rng.gen_range(1.0..1.5) * 150.0;
        let period = rng.gen_range(0.8..1.2);
        let offset = rng.gen_range(0.0..100.0);

        Self::new(amplitude, period, offset)
    }

    pub fn generate(&self, time: f32) -> f32 {
        self.amplitude * ((time + self.offset) / self.period).sin()
    }
}
