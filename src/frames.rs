use std::borrow::Cow;

use lazy_static::lazy_static;

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
    pub lines: Box<[Cow<'static, str>]>,
}

#[derive(Debug, Clone)]
pub struct AnimatedFrames {
    pub frames: Box<[Frame]>,
    pub interval_ms: u64,
}

lazy_static! {
    pub static ref STATIC_FRAME: Frame = Frame {
        lines: STATIC_FRAME_STR
            .lines()
            .map(|line| Cow::Borrowed(&line[0..line.len() - 1]))
            .collect(),
    };
    pub static ref ANIMATE1_FRAMES: AnimatedFrames = AnimatedFrames {
        frames: ANIMATE1_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| Cow::Borrowed(&line[0..line.len() - 1]))
                    .collect(),
            })
            .collect(),
        interval_ms: 100,
    };
    pub static ref ANIMATE2_FRAMES: AnimatedFrames = AnimatedFrames {
        frames: ANIMATE2_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| Cow::Borrowed(&line[0..line.len() - 1]))
                    .collect(),
            })
            .collect(),
        interval_ms: 100,
    };
}
