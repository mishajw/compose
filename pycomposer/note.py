from enum import Enum
from typing import NamedTuple


class Note(NamedTuple):
    """
    A note within an octave, e.g. A5.
    """

    abstract_note: "AbstractNote"
    octave: int

    def frequency(self) -> float:
        """
        The frequency, measured in hertz, of the note.
        """
        return self.abstract_note.frequency() * (2 ** (self.octave - 5))

    def next(self) -> "Note":
        """
        The next note (higher frequency).
        """
        next_abstract_note = self.abstract_note.next()
        next_octave = self.octave if next_abstract_note != AbstractNote.FIRST else self.octave + 1
        return Note(next_abstract_note, next_octave)

    def prev(self) -> "Note":
        """
        The previous note (higher frequency).
        """
        prev_abstract_note = self.abstract_note.prev()
        prev_octave = self.octave if prev_abstract_note != AbstractNote.LAST else self.octave - 1
        return Note(prev_abstract_note, prev_octave)

    def __add__(self, increment: int) -> "Note":
        new_note = self
        if increment >= 0:
            for _ in range(increment):
                new_note = new_note.next()
        else:
            for _ in range(-increment):
                new_note = new_note.prev()
        return new_note


class AbstractNote(Enum):
    """
    A note with no specific octave.
    """

    C = 0
    Cs = 1
    D = 2
    Ds = 3
    E = 4
    F = 5
    Fs = 6
    G = 7
    Gs = 8
    A = 9
    As = 10
    B = 11

    FIRST = C
    LAST = B

    @classmethod
    def from_str(cls, s: str) -> "AbstractNote":
        if s == "c":
            return AbstractNote.C
        elif s == "c#":
            return AbstractNote.Cs
        elif s == "d":
            return AbstractNote.D
        elif s == "d#":
            return AbstractNote.Ds
        elif s == "e":
            return AbstractNote.E
        elif s == "f":
            return AbstractNote.F
        elif s == "f#":
            return AbstractNote.Fs
        elif s == "g":
            return AbstractNote.G
        elif s == "g#":
            return AbstractNote.Gs
        elif s == "a":
            return AbstractNote.A
        elif s == "a#":
            return AbstractNote.As
        elif s == "b":
            return AbstractNote.B
        raise AssertionError(f"Unrecognized abstract note: {s}")

    def frequency(self) -> float:
        """
        The frequency in hertz at octave 5.
        """
        if self == AbstractNote.C:
            return 523.25
        elif self == AbstractNote.Cs:
            return 554.37
        elif self == AbstractNote.D:
            return 587.33
        elif self == AbstractNote.Ds:
            return 622.25
        elif self == AbstractNote.E:
            return 659.25
        elif self == AbstractNote.F:
            return 698.46
        elif self == AbstractNote.Fs:
            return 739.99
        elif self == AbstractNote.G:
            return 783.99
        elif self == AbstractNote.Gs:
            return 830.81
        elif self == AbstractNote.A:
            return 880.0
        elif self == AbstractNote.As:
            return 932.33
        elif self == AbstractNote.B:
            return 987.77
        raise AssertionError()

    def next(self) -> "AbstractNote":
        if self == AbstractNote.LAST:
            return AbstractNote.FIRST
        return next(n for n in _ABSTRACT_NOTES if n.value > self.value)

    def prev(self) -> "AbstractNote":
        if self == AbstractNote.FIRST:
            return AbstractNote.LAST
        return next(n for n in reversed(_ABSTRACT_NOTES) if n.value < self.value)


_ABSTRACT_NOTES = list(sorted([note for note in AbstractNote], key=lambda note: note.value))
