use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use image::RgbaImage;
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize)]
struct AsepriteData {
    pub frames: HashMap<String, FrameData>,
    meta: MetaData,
}

#[derive(Deserialize)]
struct FrameData {
    frame: Rect,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: Rect,
    #[serde(rename = "sourceSize")]
    source_size: Size,
    duration: u32,
}

#[derive(Deserialize)]
struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Deserialize)]
struct Size {
    w: u32,
    h: u32,
}

#[derive(Deserialize)]
struct MetaData {
    image: String,
    size: Size,
    #[serde(rename = "frameTags")]
    frame_tags: Vec<FrameTag>,
}

#[derive(Deserialize)]
struct FrameTag {
    name: String,
    from: usize,
    to: usize,
    direction: String,
    color: String,
}

pub struct SpriteFrame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    // pub pixels: Vec<Option<Rgba<u8>>>,
    pub duration: Option<u32>,
}

pub struct SpriteFrames {
    pub sprite_frames: Vec<SpriteFrame>,
    pub max: usize,
    pub current_frame: usize,
    pub first_frame: bool,
    pub last_frame_time: Instant,
}

#[derive(PartialEq, Eq)]
pub enum RunningState {
    Running,
    Done,
}

pub enum MaintainOrFill {
    Maintain,
    Fill,
}

pub enum Message {
    Quit,
    Fill,
    Maintain,
}

pub struct Model {
    pub running_state: RunningState,
    pub maintain_or_fill: MaintainOrFill,
    pub sprite_frame: Option<SpriteFrame>,
    pub sprite_frames: Option<SpriteFrames>,
}

pub struct CellPixel {
    pub top: Option<Color>,
    pub bottom: Option<Color>,
}

impl CellPixel {
    pub fn new() -> Self {
        CellPixel {
            top: None,
            bottom: None,
        }
    }
}

impl AsepriteData {
    pub fn new(json: &String) -> Self {
        serde_json::from_str(json).expect("Deserialization failed")
    }
}

impl SpriteFrame {
    pub fn new(image: &RgbaImage) -> Self {
        let width = image.width();
        let height = image.height();

        Self {
            x: 0,
            y: 0,
            width,
            height,
            duration: None,
        }
    }
}

impl SpriteFrames {
    pub fn new(json: &String) -> Self {
        let aseprite_data = AsepriteData::new(json);

        // 按每一帧的顺序排序
        let mut frames = aseprite_data.frames.into_iter().collect::<Vec<_>>();
        frames.sort_by_key(|(_, frame_data)| frame_data.frame.x);

        let sprite_frames = frames
            .iter()
            .map(|(_, frame_data)| SpriteFrame {
                x: frame_data.frame.x,
                y: frame_data.frame.y,
                width: frame_data.frame.w,
                height: frame_data.frame.h,
                duration: Some(frame_data.duration),
            })
            .collect::<Vec<SpriteFrame>>();

        let max = sprite_frames.len() - 1;

        Self {
            sprite_frames,
            max,
            current_frame: 0,
            first_frame: true,
            last_frame_time: Instant::now(),
        }
    }

    pub fn update(&mut self) -> &SpriteFrame {
        if self.first_frame {
            self.first_frame = false;
            &self.sprite_frames[0]
        } else {
            let current = &self.sprite_frames[self.current_frame];
            let duration = match current.duration {
                Some(duration) => duration,
                None => panic!("The animation's sprite frames are missing a duration."),
            };

            if self.last_frame_time.elapsed() >= Duration::from_millis(duration as u64) {
                self.last_frame_time = Instant::now();

                // 对于前后循环动画
                // 后面可以增加对不同动画模式的支持
                if self.current_frame >= self.max && self.max > 0 {
                    self.current_frame -= 1;
                    self.max -= 1;
                } else {
                    self.current_frame += 1;
                    self.max = self.sprite_frames.len() - 1;
                }
            }

            &self.sprite_frames[self.current_frame]
        }
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            running_state: RunningState::Running,
            maintain_or_fill: MaintainOrFill::Maintain,
            sprite_frame: None,
            sprite_frames: None,
        }
    }
}
