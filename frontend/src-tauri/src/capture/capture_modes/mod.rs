mod helpers;
mod sequence;
mod defect_map;

use enum_dispatch::enum_dispatch;

use self::sequence::SequenceCapture;

#[enum_dispatch(StatefulCapture)]
pub enum CaptureMode {
    SequenceCapture,
}