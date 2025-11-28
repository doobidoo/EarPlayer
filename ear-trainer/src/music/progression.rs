use super::bricks::BrickLibrary;
use super::chord::{Chord, ChordQuality, Note};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChordChange {
    pub chord: Chord,
    pub duration: f32, // in beats
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub name: String,
    pub genre: String,
    pub key: Note,
    pub changes: Vec<ChordChange>,
    pub tempo: f32,
}

impl Progression {
    pub fn new(name: String, genre: String, key: Note, tempo: f32) -> Self {
        Self {
            name,
            genre,
            key,
            changes: Vec::new(),
            tempo,
        }
    }

    pub fn add_chord(&mut self, chord: Chord, duration: f32) {
        self.changes.push(ChordChange { chord, duration });
    }

    pub fn total_duration(&self) -> f32 {
        self.changes.iter().map(|c| c.duration).sum()
    }
}

pub struct ProgressionLibrary {
    progressions: HashMap<String, Vec<Progression>>,
}

impl ProgressionLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            progressions: HashMap::new(),
        };
        library.populate_jazz();
        library.populate_soul();
        library.populate_funk();
        library.populate_smooth_jazz();
        library.populate_pop();
        library.populate_lego_bricks();
        library
    }

    fn populate_jazz(&mut self) {
        let mut jazz = Vec::new();

        // Classic ii-V-I in C
        let mut prog = Progression::new(
            "ii-V-I (C)".to_string(),
            "Jazz".to_string(),
            Note::C,
            120.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 8.0);
        jazz.push(prog);

        // Major ii-V-I with extensions
        let mut prog = Progression::new(
            "ii-V-I Extended (C)".to_string(),
            "Jazz".to_string(),
            Note::C,
            140.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7b9), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major9), 8.0);
        jazz.push(prog);

        // Minor ii-V-i
        let mut prog = Progression::new(
            "ii-V-i Minor (C)".to_string(),
            "Jazz".to_string(),
            Note::C,
            120.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::HalfDiminished), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7b9), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor7), 8.0);
        jazz.push(prog);

        // iii-VI-ii-V
        let mut prog = Progression::new(
            "iii-VI-ii-V (C)".to_string(),
            "Jazz".to_string(),
            Note::C,
            140.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7), 4.0);
        jazz.push(prog);

        // I-VI-ii-V (Rhythm Changes A section)
        let mut prog = Progression::new(
            "I-VI-ii-V (Rhythm Changes)".to_string(),
            "Jazz".to_string(),
            Note::Bb,
            200.0,
        );
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Major6), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Dominant7), 4.0);
        jazz.push(prog);

        // Coltrane Changes (simplified)
        let mut prog = Progression::new(
            "Coltrane Changes".to_string(),
            "Jazz".to_string(),
            Note::C,
            140.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::Gb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        jazz.push(prog);

        // Autumn Leaves style (minor)
        let mut prog = Progression::new(
            "Autumn Leaves Style".to_string(),
            "Jazz".to_string(),
            Note::G,
            130.0,
        );
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        jazz.push(prog);

        // Blue Bossa style
        let mut prog = Progression::new(
            "Blue Bossa Style".to_string(),
            "Jazz".to_string(),
            Note::C,
            140.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::HalfDiminished), 2.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7b9), 2.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor7), 4.0);
        jazz.push(prog);

        // Jazz Blues in Bb
        let mut prog = Progression::new(
            "Jazz Blues (Bb)".to_string(),
            "Jazz".to_string(),
            Note::Bb,
            120.0,
        );
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7), 2.0);
        jazz.push(prog);

        // Turnaround
        let mut prog = Progression::new(
            "Jazz Turnaround".to_string(),
            "Jazz".to_string(),
            Note::C,
            140.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 2.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7), 2.0);
        jazz.push(prog);

        // Tritone Substitution
        let mut prog = Progression::new(
            "Tritone Sub ii-V-I".to_string(),
            "Jazz".to_string(),
            Note::C,
            130.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::Db, ChordQuality::Dominant7), 4.0); // Tritone sub for G7
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 8.0);
        jazz.push(prog);

        self.progressions.insert("Jazz".to_string(), jazz);
    }

    fn populate_soul(&mut self) {
        let mut soul = Vec::new();

        // Classic Soul Vamp
        let mut prog = Progression::new(
            "Soul Vamp I-IV".to_string(),
            "Soul".to_string(),
            Note::E,
            85.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::E, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        soul.push(prog);

        // Stevie Wonder Style
        let mut prog = Progression::new(
            "Stevie Wonder Style".to_string(),
            "Soul".to_string(),
            Note::E,
            95.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        soul.push(prog);

        // Soul Ballad
        let mut prog = Progression::new(
            "Soul Ballad".to_string(),
            "Soul".to_string(),
            Note::F,
            70.0,
        );
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Dominant7), 4.0);
        soul.push(prog);

        // Motown Style
        let mut prog = Progression::new(
            "Motown Style".to_string(),
            "Soul".to_string(),
            Note::G,
            115.0,
        );
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 2.0);
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 2.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Dominant7), 2.0);
        soul.push(prog);

        // R&B Progression
        let mut prog = Progression::new(
            "R&B Classic".to_string(),
            "Soul".to_string(),
            Note::D,
            90.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major7), 4.0);
        soul.push(prog);

        // Neo-Soul
        let mut prog = Progression::new(
            "Neo-Soul".to_string(),
            "Soul".to_string(),
            Note::Ab,
            85.0,
        );
        prog.add_chord(Chord::new(Note::Ab, ChordQuality::Major9), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Dominant9), 4.0);
        prog.add_chord(Chord::new(Note::Ab, ChordQuality::Major9), 4.0);
        soul.push(prog);

        // Gospel Influenced
        let mut prog = Progression::new(
            "Gospel Influenced".to_string(),
            "Soul".to_string(),
            Note::C,
            80.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 2.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major7), 2.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7), 2.0);
        soul.push(prog);

        // Slow Jam
        let mut prog = Progression::new(
            "Slow Jam".to_string(),
            "Soul".to_string(),
            Note::Eb,
            65.0,
        );
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Major9), 8.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Ab, ChordQuality::Major9), 4.0);
        soul.push(prog);

        // Uptempo Soul
        let mut prog = Progression::new(
            "Uptempo Soul".to_string(),
            "Soul".to_string(),
            Note::F,
            125.0,
        );
        prog.add_chord(Chord::new(Note::F, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Dominant7), 2.0);
        soul.push(prog);

        // Funk Soul Hybrid
        let mut prog = Progression::new(
            "Funk Soul Hybrid".to_string(),
            "Soul".to_string(),
            Note::E,
            100.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 2.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 2.0);
        soul.push(prog);

        self.progressions.insert("Soul".to_string(), soul);
    }

    fn populate_funk(&mut self) {
        let mut funk = Vec::new();

        // Classic Funk Vamp
        let mut prog = Progression::new(
            "Funk Vamp".to_string(),
            "Funk".to_string(),
            Note::E,
            105.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 8.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 8.0);
        funk.push(prog);

        // Dorian Funk
        let mut prog = Progression::new(
            "Dorian Funk".to_string(),
            "Funk".to_string(),
            Note::D,
            100.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 8.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Minor7), 8.0);
        funk.push(prog);

        // Funk Rock
        let mut prog = Progression::new(
            "Funk Rock".to_string(),
            "Funk".to_string(),
            Note::A,
            110.0,
        );
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 4.0);
        funk.push(prog);

        // Modal Funk
        let mut prog = Progression::new(
            "Modal Funk".to_string(),
            "Funk".to_string(),
            Note::G,
            95.0,
        );
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7), 8.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Dominant7), 8.0);
        funk.push(prog);

        // James Brown Style
        let mut prog = Progression::new(
            "James Brown Style".to_string(),
            "Funk".to_string(),
            Note::E,
            115.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Dominant9), 16.0);
        funk.push(prog);

        // P-Funk
        let mut prog = Progression::new(
            "P-Funk".to_string(),
            "Funk".to_string(),
            Note::F,
            100.0,
        );
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7), 4.0);
        funk.push(prog);

        // Chromatic Funk
        let mut prog = Progression::new(
            "Chromatic Funk".to_string(),
            "Funk".to_string(),
            Note::C,
            108.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::Db, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Dominant7), 2.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Dominant7), 2.0);
        funk.push(prog);

        // Herbie Hancock Style
        let mut prog = Progression::new(
            "Herbie Hancock Style".to_string(),
            "Funk".to_string(),
            Note::Bb,
            95.0,
        );
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::Ab, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::Db, ChordQuality::Major7), 4.0);
        funk.push(prog);

        // Funk Blues
        let mut prog = Progression::new(
            "Funk Blues".to_string(),
            "Funk".to_string(),
            Note::G,
            100.0,
        );
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant9), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Dominant9), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant9), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Dominant9), 2.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Dominant9), 2.0);
        funk.push(prog);

        // Synth Funk
        let mut prog = Progression::new(
            "Synth Funk".to_string(),
            "Funk".to_string(),
            Note::C,
            110.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Major7), 4.0);
        funk.push(prog);

        self.progressions.insert("Funk".to_string(), funk);
    }

    fn populate_smooth_jazz(&mut self) {
        let mut smooth = Vec::new();

        // Smooth Jazz Ballad
        let mut prog = Progression::new(
            "Smooth Jazz Ballad".to_string(),
            "Smooth Jazz".to_string(),
            Note::Db,
            75.0,
        );
        prog.add_chord(Chord::new(Note::Db, ChordQuality::Major9), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Ab, ChordQuality::Dominant9), 4.0);
        smooth.push(prog);

        // Extended ii-V-I
        let mut prog = Progression::new(
            "Extended ii-V-I".to_string(),
            "Smooth Jazz".to_string(),
            Note::Eb,
            85.0,
        );
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Dominant7b13), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Major9), 8.0);
        smooth.push(prog);

        // Modal Smooth Jazz
        let mut prog = Progression::new(
            "Modal Smooth Jazz".to_string(),
            "Smooth Jazz".to_string(),
            Note::F,
            90.0,
        );
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor9), 8.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Minor9), 8.0);
        smooth.push(prog);

        // Contemporary Jazz
        let mut prog = Progression::new(
            "Contemporary Jazz".to_string(),
            "Smooth Jazz".to_string(),
            Note::G,
            95.0,
        );
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7sharp11), 4.0);
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Dominant9), 4.0);
        smooth.push(prog);

        // Lydian Sound
        let mut prog = Progression::new(
            "Lydian Sound".to_string(),
            "Smooth Jazz".to_string(),
            Note::C,
            80.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7sharp11), 8.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7sharp11), 8.0);
        smooth.push(prog);

        // Bossa Nova Influenced
        let mut prog = Progression::new(
            "Bossa Nova Influenced".to_string(),
            "Smooth Jazz".to_string(),
            Note::D,
            130.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7), 4.0);
        smooth.push(prog);

        // Smooth Turnaround
        let mut prog = Progression::new(
            "Smooth Turnaround".to_string(),
            "Smooth Jazz".to_string(),
            Note::F,
            85.0,
        );
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major9), 2.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor9), 2.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Minor9), 2.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Dominant9), 2.0);
        smooth.push(prog);

        // Contemporary Ballad
        let mut prog = Progression::new(
            "Contemporary Ballad".to_string(),
            "Smooth Jazz".to_string(),
            Note::Ab,
            70.0,
        );
        prog.add_chord(Chord::new(Note::Ab, ChordQuality::Major9), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::Db, ChordQuality::Major9), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Dominant9), 4.0);
        smooth.push(prog);

        // Fusion Style
        let mut prog = Progression::new(
            "Fusion Style".to_string(),
            "Smooth Jazz".to_string(),
            Note::E,
            100.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Dominant7sharp9), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7sharp11), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7sharp11), 4.0);
        smooth.push(prog);

        // Romantic Jazz
        let mut prog = Progression::new(
            "Romantic Jazz".to_string(),
            "Smooth Jazz".to_string(),
            Note::Bb,
            72.0,
        );
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Major9), 4.0);
        prog.add_chord(Chord::new(Note::Eb, ChordQuality::Major9), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor9), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Dominant9), 4.0);
        smooth.push(prog);

        self.progressions.insert("Smooth Jazz".to_string(), smooth);
    }

    fn populate_pop(&mut self) {
        let mut pop = Vec::new();

        // Classic I-V-vi-IV
        let mut prog = Progression::new(
            "I-V-vi-IV".to_string(),
            "Pop".to_string(),
            Note::C,
            120.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // vi-IV-I-V
        let mut prog = Progression::new(
            "vi-IV-I-V".to_string(),
            "Pop".to_string(),
            Note::G,
            115.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // I-IV-V
        let mut prog = Progression::new(
            "I-IV-V".to_string(),
            "Pop".to_string(),
            Note::D,
            125.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // I-vi-IV-V (50s progression)
        let mut prog = Progression::new(
            "I-vi-IV-V (50s)".to_string(),
            "Pop".to_string(),
            Note::C,
            130.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // I-V-IV
        let mut prog = Progression::new(
            "I-V-IV".to_string(),
            "Pop".to_string(),
            Note::E,
            110.0,
        );
        prog.add_chord(Chord::new(Note::E, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::B, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::E, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // IV-V-iii-vi
        let mut prog = Progression::new(
            "IV-V-iii-vi".to_string(),
            "Pop".to_string(),
            Note::F,
            100.0,
        );
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        pop.push(prog);

        // I-iii-IV-iv (Beatles style)
        let mut prog = Progression::new(
            "Beatles Style".to_string(),
            "Pop".to_string(),
            Note::G,
            105.0,
        );
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::B, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Minor7), 4.0);
        pop.push(prog);

        // I-ii-iii-IV
        let mut prog = Progression::new(
            "I-ii-iii-IV".to_string(),
            "Pop".to_string(),
            Note::A,
            115.0,
        );
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::B, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::Db, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::D, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // vi-V-IV-V
        let mut prog = Progression::new(
            "vi-V-IV-V".to_string(),
            "Pop".to_string(),
            Note::D,
            108.0,
        );
        prog.add_chord(Chord::new(Note::B, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::A, ChordQuality::Major7), 4.0);
        pop.push(prog);

        // I-bVII-IV (Modal pop)
        let mut prog = Progression::new(
            "Modal Pop".to_string(),
            "Pop".to_string(),
            Note::C,
            120.0,
        );
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::Bb, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::F, ChordQuality::Major7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        pop.push(prog);

        self.progressions.insert("Pop".to_string(), pop);
    }

    fn populate_lego_bricks(&mut self) {
        let brick_library = BrickLibrary::new();
        let mut lego = Vec::new();

        // Common jazz keys to generate bricks in
        let common_keys = [
            Note::C,
            Note::F,
            Note::Bb,
            Note::Eb,
            Note::Ab,
            Note::G,
            Note::D,
        ];

        // Generate progressions from each brick in common keys
        for brick in brick_library.all() {
            // Generate in 2-3 common keys to avoid overwhelming the library
            for &key in common_keys.iter().take(3) {
                lego.push(brick.to_progression(key, 140.0));
            }
        }

        self.progressions.insert("LEGO Bricks".to_string(), lego);
    }

    pub fn get_by_genre(&self, genre: &str) -> Option<&Vec<Progression>> {
        self.progressions.get(genre)
    }

    pub fn all_genres(&self) -> Vec<String> {
        self.progressions.keys().cloned().collect()
    }

    pub fn all_progressions(&self) -> Vec<&Progression> {
        self.progressions.values().flatten().collect()
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Progression> {
        self.all_progressions()
            .into_iter()
            .find(|p| p.name == name)
    }
}

impl Default for ProgressionLibrary {
    fn default() -> Self {
        Self::new()
    }
}
