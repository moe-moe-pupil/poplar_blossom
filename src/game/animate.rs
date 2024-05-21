use std::{ops::Range, time::Duration};

use bevy::{prelude::Timer, time::TimerMode};

#[derive(Clone)]
pub struct AnimateRange {
    timer: Timer,
    ease: Ease,
    range: Range<f32>,
    direction: Option<f32>,
}

impl AnimateRange {
    pub fn new(
        duration: Duration,
        ease: Ease,
        range: Range<f32>,
        repeat: bool,
        direction: Option<f32>,
    ) -> Self {
        Self {
            timer: Timer::new(
                duration,
                if repeat {
                    TimerMode::Repeating
                } else {
                    TimerMode::Once
                },
            ),
            ease,
            range,
            direction,
        }
    }

    pub fn set_percent(&mut self, percent: f32) -> &mut Self {
        self.timer.set_elapsed(Duration::from_secs_f32(
            self.timer.duration().as_secs_f32() * percent,
        ));
        self
    }

    pub fn percent(&mut self) -> f32 {
        self.timer.fraction()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn just_finished(&mut self) -> bool {
        self.timer.just_finished()
    }

    pub fn finished(&mut self) -> bool {
        self.timer.finished()
    }

    pub fn set_range(&mut self, range: Range<f32>) {
        self.range = range
    }

    pub fn elapsed_time(&mut self) -> f32 {
        self.timer.elapsed_secs()
    }

    pub fn tick(&mut self, delta: Duration) -> f32 {
        self.timer.tick(delta);
        let amount = self.ease.ease(self.timer.fraction());
        self.direction.unwrap_or(1.0)
            * (self.range.start + ((self.range.end - self.range.start) * amount))
    }

    pub fn tick_f32(&mut self, delta: f32) -> f32 {
        let delta_abs = delta.abs();
        if let Some(direction) = self.direction {
            if delta > 0.0 {
                if direction > 0.0 {
                    self.tick(Duration::from_secs_f32(delta))
                } else {
                    if delta > self.elapsed_time() {
                        self.direction = Some(-1.0 * direction);
                        self.set_percent(0.0);
                        let delta_time = delta - self.elapsed_time();
                        self.tick(Duration::from_secs_f32(delta_time))
                    } else {
                        self.reverse_tick(Duration::from_secs_f32(delta))
                    }
                }
            } else {
                if direction > 0.0 {
                    if delta_abs > self.elapsed_time() {
                        self.direction = Some(-1.0 * direction);
                        self.set_percent(0.0);
                        let delta_time = delta_abs - self.elapsed_time();
                        self.tick(Duration::from_secs_f32(delta_time))
                    } else {
                        self.reverse_tick(Duration::from_secs_f32(delta_abs))
                    }
                } else {
                    self.tick(Duration::from_secs_f32(delta_abs))
                }
            }
        } else {
            if delta > 0.0 {
                self.tick(Duration::from_secs_f32(delta))
            } else {
                self.reverse_tick(Duration::from_secs_f32(delta_abs))
            }
        }
    }

    pub fn reverse_tick(&mut self, delta: Duration) -> f32 {
        let elapsed_time = self.timer.elapsed();
        if self.timer.finished() {
            self.timer.reset();
        }
        self.timer.set_elapsed(elapsed_time.saturating_sub(delta));
        let amount = self.ease.ease(self.timer.fraction());
        self.direction.unwrap_or(1.0)
            * (self.range.start + ((self.range.end - self.range.start) * amount))
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Ease {
    Linear,
    // Sin,
    InOutCirc,
    OutBack,
    // Custom(fn(f32) -> f32),
}

impl Ease {
    pub fn ease(&self, x: f32) -> f32 {
        match self {
            Ease::Linear => x,
            // Ease::Sin => x.sin(),
            Ease::InOutCirc => {
                if x < 0.5 {
                    (1. - (1. - (2. * x).powf(2.)).sqrt()) / 2.
                } else {
                    ((1. - (-2. * x + 2.).powf(2.)).sqrt() + 1.) / 2.
                }
            }
            Ease::OutBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;

                1. + C3 * (x - 1.).powf(3.) + C1 * (x - 1.).powf(2.)
            }
        }
    }
}
