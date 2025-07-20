use lazy_static::lazy_static;
use std::sync::Arc;

const STATIC_FRAME_STR: &str = include_str!("../frames/static-01.txt");
const ANIMATE1_FRAMES_STR: [&str; 5] = [
    include_str!("../frames/animated-01-01.txt"),
    include_str!("../frames/animated-01-02.txt"),
    include_str!("../frames/animated-01-03.txt"),
    include_str!("../frames/animated-01-04.txt"),
    include_str!("../frames/animated-01-05.txt"),
];
const ANIMATE2_FRAMES_STR: [&str; 5] = [
    include_str!("../frames/animated-02-01.txt"),
    include_str!("../frames/animated-02-02.txt"),
    include_str!("../frames/animated-02-03.txt"),
    include_str!("../frames/animated-02-04.txt"),
    include_str!("../frames/animated-02-05.txt"),
];

#[derive(Debug, Clone)]
pub struct Frame {
    pub lines: Arc<[&'static str]>,
}

#[derive(Debug, Clone)]
pub struct AnimatedFrames {
    pub frames: Arc<[Frame]>,
    pub interval_ms: Arc<[u64]>,
}

impl AnimatedFrames {
    pub fn iter(&self) -> AnimatedFramesIterator {
        AnimatedFramesIterator {
            frames: self.frames.clone(),
            interval_ms: self.interval_ms.clone(),
            current_frame: 0,
        }
    }
}

pub struct AnimatedFramesIterator {
    frames: Arc<[Frame]>,
    interval_ms: Arc<[u64]>,
    current_frame: usize,
}

impl Iterator for AnimatedFramesIterator {
    type Item = (Frame, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames.is_empty() || self.interval_ms.is_empty() {
            return None;
        }
        let max_index = self.frames.len().max(self.interval_ms.len()) - 1;
        if self.current_frame >= max_index {
            return None;
        }
        let frame = self.frames[self.current_frame].clone();
        let interval = self.interval_ms[self.current_frame];
        self.current_frame += 1;
        Some((frame, interval))
    }
}

lazy_static! {
    pub static ref STATIC_FRAME: Frame = Frame {
        lines: STATIC_FRAME_STR
            .lines()
            .map(|line| &line[0..line.len() - 1])
            .collect(),
    };
    pub static ref ANIMATE1_FRAMES: AnimatedFrames = {
        let frames = ANIMATE1_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| &line[0..line.len() - 1])
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[2].clone(),
                frames[3].clone(),
                frames[4].clone(),
                frames[0].clone(),
                frames[1].clone(),
            ]),
            interval_ms: Arc::new([150, 75, 150, 150, 75]),
        }
    };
    pub static ref ANIMATE2_FRAMES: AnimatedFrames = {
        let frames = ANIMATE2_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| &line[0..line.len() - 1])
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[1].clone(),
                frames[0].clone(),
                frames[4].clone(),
                frames[2].clone(),
                frames[3].clone(),
                frames[4].clone(),
                frames[0].clone(),
            ]),
            interval_ms: Arc::new([70, 70, 70, 1500, 70, 70, 70]),
        }
    };
}
