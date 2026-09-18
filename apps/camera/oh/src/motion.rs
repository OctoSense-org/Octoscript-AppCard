//! The motion the Makepad host has, in the same numbers: a HarmonyOS-style
//! spring (`springMotion`, ~0.42 s response, 0.9 damping) stepped from the
//! 16 ms tick, and the finger's drag on the mode bar.

/// A critically-damped-ish spring in artboard units that settles at 0.
#[derive(Default, Clone, Copy)]
pub struct Spring { pub pos: f64, pub vel: f64, last_t: Option<f64> }

impl Spring {
    pub fn active(&self) -> bool { self.pos.abs() > 0.05 || self.vel.abs() > 1.0 }
    /// Start from `from` and settle at 0.
    pub fn kick(&mut self, from: f64) { self.pos = from; self.last_t = None; }
    pub fn hold(&mut self, at: f64) { self.pos = at; self.vel = 0.0; self.last_t = None; }
    pub fn step(&mut self, t: f64) {
        let dt = self.last_t.map(|l| (t - l).clamp(0.0, 0.05)).unwrap_or(0.0);
        self.last_t = Some(t);
        let w = std::f64::consts::TAU / 0.42;
        let z = 0.9;
        let (k, c) = (w * w, 2.0 * z * w);
        let h = dt / 4.0;
        for _ in 0..4 {
            let a = -k * self.pos - c * self.vel;
            self.vel += a * h;
            self.pos += self.vel * h;
        }
        if !self.active() { self.pos = 0.0; self.vel = 0.0; }
    }
}

/// A finger sliding the mode bar: where it started, the bar index at that time and the last velocity sample.
#[derive(Clone, Copy)]
pub struct BarDrag { pub start_x: f64, pub index0: usize, pub last_x: f64, pub last_t: f64, pub velocity: f64, pub moved: bool }

pub const MODE_PITCH: f64 = 52.3;
pub const CHIP_PITCH: f64 = 40.0;
