use salt::common::filename::Filename;
use std::convert::From;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

/// A single point in a file.
#[derive(Copy, Eq, Hash, Ord)]
pub struct Point {
    /// The line number, starting at 1
    pub line: u32,
    /// The column number, starting at 1
    pub col: u32
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point { line: self.line, col: self.col }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.line == other.line && self.col == other.col
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.line.cmp(&other.line).then(self.col.cmp(&other.col)))
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}", self.line, self.col)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}", self.line, self.col)
    }
}

/// A location in a source file.
#[derive(Debug, Eq, Hash, Ord)]
pub enum Location {
   /// A span in a source file, starting at `start` and ending at `end`.
    Span {
        /// The starting position.
        start: Point,
        /// The ending position.
        end: Point
    },
    /// A specific point in a source file.
    Point {
        /// The point in the file.
        point: Point
    }
}

impl<'a> PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        match (self, other) {
            (Location::Span { start: start1, end: end1 },
             Location::Span { start: start2, end: end2 }) =>
                start1 == end1 && start2 == end2,

            (Location::Point { point: point1 },
             Location::Point { point: point2 }) =>
                point1 == point2,

            _ => false
        }
    }
}

impl<'a> PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        match (self, other) {
            (Location::Span { start: start1, end: end1 },
             Location::Span { start: start2, end: end2 }) =>
                Some(start1.cmp(start2).then(end1.cmp(end2))),

            (Location::Span { .. }, Location::Point { .. }) =>
                Some(Ordering::Less),

            (Location::Point { .. }, Location::Span { .. }) =>
                Some(Ordering::Greater),

            (Location::Point { point: point1 },
             Location::Point { point: point2 }) =>
                Some(point1.cmp(point2))
        }
    }
}

impl<'a> Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Location::Span { start, end } if start.line == end.line =>
                write!(f, "{}.{}-{}", start.line, start.col, end.col),
            Location::Span { start, end } =>
                write!(f, "{}-{}", start, end),
            Location::Point { point } => write!(f, "{}", point)
        }
    }
}

/// A position referring to a point in the file `filename`, at
/// location `loc`.
#[derive(Debug, Eq, Hash, Ord)]
pub struct FilePosition<'a> {
    /// The file in which this occurs.
    pub filename: Filename<'a>,
    /// The location in the file.
    pub loc: Location
}

pub trait FilePositionCtx<'a> {
    fn point(&mut self, line: u32, col: u32) -> &FilePosition<'a>;

    fn span(&mut self, start: &FilePosition<'a>,
            end: &FilePosition<'a>) -> &FilePosition<'a>;
}

impl<'a> PartialEq for FilePosition<'a> {
    fn eq(&self, other: &FilePosition<'a>) -> bool {
        self.filename == other.filename && self.loc == other.loc
    }
}

impl<'a> PartialOrd for FilePosition<'a> {
    fn partial_cmp(&self, other: &FilePosition<'a>) -> Option<Ordering> {
        Some(self.filename.cmp(&other.filename).then(self.loc.cmp(&other.loc)))
    }
}


impl<'a> Display for FilePosition<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.filename, self.loc)
    }
}

/// A basic position type, useful for compiler frontends.
#[derive(Debug, Eq, Hash, Ord)]
pub enum BasicPosition<'a> {
    /// A position referring to a point in a file.
    Content {
        /// The file position.
        filepos: FilePosition<'a>
    },
    /// A position referring to an entire file.
    File {
        /// The name of the file.
        filename: Filename<'a>
    },
    /// A position referring to command-line arguments.
    CmdLine {
        /// Indexes of the command-line arguments to which this refers.
        idxs: Vec<usize>
    },
    /// A synthetic position.
    Synthetic {
        /// The description of the synthetic position.
        desc: &'static str
    }
}

impl<'a> PartialEq for BasicPosition<'a> {
    fn eq(&self, other: &BasicPosition<'a>) -> bool {
        match (self, other) {
            (BasicPosition::Content { filepos: filepos1 },
             BasicPosition::Content { filepos: filepos2 }) =>
                filepos1 == filepos2,

            (BasicPosition::File { filename: filename1 },
             BasicPosition::File { filename: filename2 }) =>
                filename1 == filename2,

            (BasicPosition::CmdLine { idxs: idxs1 },
             BasicPosition::CmdLine { idxs: idxs2 }) =>
                idxs1 == idxs2,

            (BasicPosition::Synthetic { desc: desc1 },
             BasicPosition::Synthetic { desc: desc2 }) =>
                desc1 == desc2,

            _ => false
        }
    }
}

impl<'a> PartialOrd for BasicPosition<'a> {
    fn partial_cmp(&self, other: &BasicPosition<'a>) -> Option<Ordering> {
        match (self, other) {
            (BasicPosition::Content { filepos: filepos1 },
             BasicPosition::Content { filepos: filepos2 }) =>
                Some(filepos1.cmp(filepos2)),

            (BasicPosition::Content { .. }, _) => Some(Ordering::Less),

            (_, BasicPosition::Content { .. }) => Some(Ordering::Greater),

            (BasicPosition::File { filename: filename1 },
             BasicPosition::File { filename: filename2 }) =>
                Some(filename1.cmp(filename2)),

            (BasicPosition::File { .. }, _) => Some(Ordering::Less),

            (_, BasicPosition::File { .. }) => Some(Ordering::Greater),

            (BasicPosition::CmdLine { idxs: idxs1 },
             BasicPosition::CmdLine { idxs: idxs2 }) =>
                Some(idxs1.cmp(idxs2)),

            (BasicPosition::CmdLine { .. }, _) => Some(Ordering::Less),

            (_, BasicPosition::CmdLine { .. }) => Some(Ordering::Greater),

            (BasicPosition::Synthetic { desc: desc1 },
             BasicPosition::Synthetic { desc: desc2 }) =>
                Some(desc1.cmp(desc2))
        }
    }
}

/// Expanded position with DWARF information.
#[derive(Debug, Eq, Hash, Ord)]
pub enum DWARFPosition<'a, T, D> {
    /// A position within a definition.
    Def {
        /// The definition's ID.
        id: D,
        /// The position of the definition.
        pos: FilePosition<'a>
    },
    /// A position within a type definition.
    TypeDef {
        /// The definition's ID.
        id: T,
        /// The position of the definition.
        pos: FilePosition<'a>
    },
    /// A position within a basic block.
    Block {
        /// The position of the basic block.
        ctx: Box<DWARFPosition<'a, T, D>>,
        /// The position within the basic block.
        pos: FilePosition<'a>
    },
    /// A basic position.
    Basic {
        /// The position information.
        pos: BasicPosition<'a>
    }
}

impl<'a, T: Eq, D: Eq> PartialEq for DWARFPosition<'a, T, D> {
    fn eq(&self, other: &DWARFPosition<'a, T, D>) -> bool {
        match (self, other) {
            (DWARFPosition::Def { id: id1, pos: pos1 },
             DWARFPosition::Def { id: id2, pos: pos2 }) =>
                id1 == id2 && pos1 == pos2,

            (DWARFPosition::TypeDef { id: id1, pos: pos1 },
             DWARFPosition::TypeDef { id: id2, pos: pos2 }) =>
                id1 == id2 && pos1 == pos2,

            (DWARFPosition::Block { ctx: ctx1, pos: pos1 },
             DWARFPosition::Block { ctx: ctx2, pos: pos2 }) =>
                ctx1 == ctx2 && pos1 == pos2,

            (DWARFPosition::Basic { pos: pos1 },
             DWARFPosition::Basic { pos: pos2 }) =>
                pos1 == pos2,

            _ => false
        }
    }
}

impl<'a, T: Ord, D: Ord> PartialOrd for DWARFPosition<'a, T, D> {
    fn partial_cmp(&self, other: &DWARFPosition<'a, T, D>) -> Option<Ordering> {
        match (self, other) {
            (DWARFPosition::Def { id: id1, pos: pos1 },
             DWARFPosition::Def { id: id2, pos: pos2 }) =>
                Some(id1.cmp(id2).then(pos1.cmp(pos2))),

            (DWARFPosition::Def { .. }, _) => Some(Ordering::Less),

            (_, DWARFPosition::Def { .. }) => Some(Ordering::Greater),

            (DWARFPosition::TypeDef { id: id1, pos: pos1 },
             DWARFPosition::TypeDef { id: id2, pos: pos2 }) =>
                Some(id1.cmp(id2).then(pos1.cmp(pos2))),

            (DWARFPosition::TypeDef { .. }, _) => Some(Ordering::Less),

            (_, DWARFPosition::TypeDef { .. }) => Some(Ordering::Greater),

            (DWARFPosition::Block { ctx: ctx1, pos: pos1 },
             DWARFPosition::Block { ctx: ctx2, pos: pos2 }) =>
                Some(ctx1.cmp(ctx2).then(pos1.cmp(pos2))),

            (DWARFPosition::Block { .. }, _) => Some(Ordering::Less),

            (_, DWARFPosition::Block { .. }) => Some(Ordering::Greater),

            (DWARFPosition::Basic { pos: pos1 },
             DWARFPosition::Basic { pos: pos2 }) =>
                Some(pos1.cmp(pos2))
        }
    }
}

/// Get information about position representations.
pub trait PositionInfo<'a> {
    /// Get the basic position
    fn location(&self) -> Option<(&'a Filename, Option<&'a Location>)>;

    /// Get the children of the current position.
    fn children(&self) -> &[&Self];

    /// Whether or not to show source context when printing this
    /// message.  Note that some rendering modes omit all source
    /// context.
    fn show_ctx(&self) -> bool;

    /// Get a textual description of the message.
    fn description(&self) -> Option<&'a str>;
}

impl<'a> From<FilePosition<'a>> for BasicPosition<'a> {
    /// Create a BasicPosition from a FilePosition
    fn from(filepos: FilePosition<'a>) -> BasicPosition<'a> {
        BasicPosition::Content { filepos: filepos }
    }
}

impl<'a, T, D> From<BasicPosition<'a>> for DWARFPosition<'a, T, D> {
    /// Create a DWARFPosition from a BasicPosition
    fn from(pos: BasicPosition<'a>) -> DWARFPosition<'a, T, D> {
        DWARFPosition::Basic { pos: pos }
    }
}

impl<'a> PositionInfo<'a> for FilePosition<'a> {
    fn location(&self) -> Option<(&'a Filename, Option<&'a Location>)> {
        Some((&self.filename, Some(&self.loc)))
    }

    fn children(&self) -> &[&Self] { &[] }
    fn show_ctx(&self) -> bool { true }
    fn description(&self) -> Option<&'a str> { None }
}

impl<'a> PositionInfo<'a> for BasicPosition<'a> {
    fn location(&self) -> Option<(&'a Filename, Option<&'a Location>)> {
        match self {
            BasicPosition::Content { filepos } => filepos.location(),
            BasicPosition::File { filename } => Some((filename, None)),
            BasicPosition::CmdLine { .. } => None,
            BasicPosition::Synthetic { .. } => None
        }
    }

    fn description(&self) -> Option<&'a str> {
        match self {
            BasicPosition::Synthetic { desc } => Some(desc),
            _ => None
        }
    }

    fn children(&self) -> &[&Self] { &[] }

    fn show_ctx(&self) -> bool {
        match self {
            BasicPosition::Content { .. } => true,
            _ => false
        }
    }
}
