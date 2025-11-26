pub mod chord;
pub mod scale;
pub mod progression;
pub mod theory;

pub use chord::{Chord, ChordQuality, ChordTone, Note};
pub use scale::{Scale, ScaleType, Mode};
pub use progression::{Progression, ProgressionLibrary, ChordChange};
pub use theory::{VoiceLeading, GuideTone, ChordScaleMatcher};
