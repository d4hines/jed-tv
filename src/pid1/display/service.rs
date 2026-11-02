//! Persistent display owner, usable directly from PID1 without a helper process.
use crate::{osd_scanout::Scanout, frames::Frames, overlay::Overlay, pixels};
use std::{io, time::Instant};

pub trait FrameSource {
    fn dimensions(&self) -> (usize, usize);
    fn copy_if_new(&self, previous: u32, output: &mut Vec<u32>) -> io::Result<Option<u32>>;
}
impl FrameSource for Frames {
    fn dimensions(&self) -> (usize, usize) { (self.width, self.height) }
    fn copy_if_new(&self, previous: u32, output: &mut Vec<u32>) -> io::Result<Option<u32>> {
        Frames::copy_if_new(self, previous, output)
    }
}
pub trait Output {
    fn present(&mut self, pixels: &[u32]) -> io::Result<()>;
    fn hide(&mut self) -> io::Result<()>;
}
impl Output for Scanout {
    fn present(&mut self, pixels: &[u32]) -> io::Result<()> { self.draw_rgba(pixels) }
    fn hide(&mut self) -> io::Result<()> { Scanout::hide(self) }
}

pub struct Service<F = Frames, O = Scanout> {
    source: F,
    output: O,
    input: Vec<u32>,
    composed: Vec<u32>,
    overlay: Overlay,
    sequence: u32,
    paused: bool,
}
impl Service {
    /// Requires exclusive, already-initialized display ownership. Allocates scanout.
    pub fn open(path: &str) -> io::Result<Self> {
        let frames = Frames::open(path)?;
        let output = Scanout::create()?;
        Ok(Self::from_parts(frames, output))
    }
}
impl<F: FrameSource, O: Output> Service<F, O> {
    pub fn from_parts(source: F, output: O) -> Self {
        Self { source, output, input: Vec::new(), composed: vec![0; 1280 * 720],
            overlay: Overlay::default(), sequence: 0, paused: false }
    }
    /// Switch sources without reallocating the physical scanout. Sequence
    /// numbers belong to each source, so never compare across a switch.
    pub fn set_source(&mut self, source: F) {
        self.source = source;
        self.sequence = 0;
        self.input.clear();
    }

    /// One non-blocking scheduling step apart from bounded shared-frame acquisition.
    /// Pausing retains the layer, buffers and latest frame. Returns a presented sequence.
    pub fn tick(&mut self, paused: bool) -> io::Result<Option<u32>> {
        if paused {
            if !self.paused { self.output.hide()?; self.paused = true; }
            return Ok(None);
        }
        let resumed = self.paused;
        let next = self.source.copy_if_new(self.sequence, &mut self.input)?;
        let changed = self.overlay.refresh();
        let visible = self.overlay.until.is_some_and(|end| end > Instant::now());
        if !resumed && next.is_none() && !changed && visible == self.overlay.was_visible {
            return Ok(None);
        }
        if let Some(sequence) = next { self.sequence = sequence; }
        if self.sequence == 0 { self.paused = false; return Ok(None); }
        let (width, height) = self.source.dimensions();
        pixels::compose_pixels(&mut self.composed, 1280, 720, 1280,
            &self.input, width, height, visible.then_some(self.overlay.pixels.as_slice()))?;
        self.output.present(&self.composed)?;
        self.paused = false;
        self.overlay.was_visible = visible;
        Ok(Some(self.sequence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::{Cell, RefCell}, rc::Rc};
    struct Source { reads: Rc<Cell<u32>> }
    impl FrameSource for Source {
        fn dimensions(&self) -> (usize, usize) { (1, 1) }
        fn copy_if_new(&self, previous: u32, output: &mut Vec<u32>) -> io::Result<Option<u32>> {
            self.reads.set(self.reads.get() + 1);
            if previous == 7 { return Ok(None); }
            *output = vec![0xff332211]; Ok(Some(7))
        }
    }
    struct Display { events: Rc<RefCell<Vec<&'static str>>> }
    impl Output for Display {
        fn present(&mut self, pixels: &[u32]) -> io::Result<()> {
            assert_eq!(pixels.len(), 1280 * 720);
            assert!(pixels.contains(&0xff332211));
            self.events.borrow_mut().push("present"); Ok(())
        }
        fn hide(&mut self) -> io::Result<()> { self.events.borrow_mut().push("hide"); Ok(()) }
    }
    #[test]
    fn pause_preserves_frame_and_resume_republishes_without_new_input() {
        let reads = Rc::new(Cell::new(0)); let events = Rc::new(RefCell::new(vec![]));
        let mut service = Service::from_parts(Source { reads: reads.clone() }, Display { events: events.clone() });
        assert_eq!(service.tick(false).unwrap(), Some(7));
        assert_eq!(service.tick(false).unwrap(), None);
        service.tick(true).unwrap(); service.tick(true).unwrap();
        assert_eq!(reads.get(), 2);
        assert_eq!(service.tick(false).unwrap(), Some(7));
        assert_eq!(*events.borrow(), vec!["present", "hide", "present"]);
    }
}

#[cfg(test)]
mod source_switch_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    struct Source(u32);
    impl FrameSource for Source {
        fn dimensions(&self) -> (usize, usize) { (1, 1) }
        fn copy_if_new(&self, old: u32, out: &mut Vec<u32>) -> io::Result<Option<u32>> {
            if old == 1 { return Ok(None); }
            *out = vec![self.0]; Ok(Some(1))
        }
    }
    struct Sink(Rc<RefCell<Vec<u32>>>);
    impl Output for Sink {
        fn present(&mut self, pixels: &[u32]) -> io::Result<()> { self.0.borrow_mut().push(pixels[360 * 1280 + 640]); Ok(()) }
        fn hide(&mut self) -> io::Result<()> { Ok(()) }
    }
    #[test]
    fn switching_to_home_does_not_reuse_app_pixels_or_sequence() {
        let sink = Rc::new(RefCell::new(vec![]));
        let mut service = Service::from_parts(Source(0xff0000ff), Sink(sink.clone()));
        service.tick(false).unwrap();
        service.set_source(Source(0xff00ff00));
        service.tick(false).unwrap();
        assert_eq!(*sink.borrow(), vec![0xff0000ff, 0xff00ff00]);
    }
}
