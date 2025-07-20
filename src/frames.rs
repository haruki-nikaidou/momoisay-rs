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
    pub interval_ms: Box<[u64]>,
}

lazy_static! {
    pub static ref STATIC_FRAME: Frame = Frame {
        lines: STATIC_FRAME_STR
            .lines()
            .map(|line| Cow::Borrowed(&line[0..line.len() - 1]))
            .collect(),
    };
    pub static ref ANIMATE1_FRAMES: AnimatedFrames = {
        let frames = ANIMATE1_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| Cow::Borrowed(&line[0..line.len() - 1]))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Box::new([
                frames[3].clone(),
                frames[4].clone(),
                frames[5].clone(),
                frames[1].clone(),
                frames[2].clone(),
            ]),
            interval_ms: Box::new([150, 75, 150, 150, 75]),
        }
    };
    pub static ref ANIMATE2_FRAMES: AnimatedFrames = {
        let frames = ANIMATE2_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| Cow::Borrowed(&line[0..line.len() - 1]))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Box::new([
                frames[2].clone(),
                frames[1].clone(),
                frames[5].clone(),
                frames[3].clone(),
                frames[4].clone(),
                frames[5].clone(),
                frames[1].clone(),
            ]),
            interval_ms: Box::new([70, 70, 70, 1500, 70, 70, 70]),
        }
    };
}
