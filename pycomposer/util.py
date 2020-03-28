import re
from typing import List, Iterable

from . import Player, Timeline, Volume, Combiner, Time, Note, AbstractNote, BoundedInput
from .chord import Chord
from .scale import Scale, ABSTRACT_SCALES
from .scale_index import SCALE_INDICES

_NOTE_STR = r"([a-g]#?)([0-9]+)"
_NOTE_NO_GROUP_STR = r"[a-g]#?[0-9]+"

# e.g. "a4", "c7".
_NOTE_REGEX = re.compile("^" + _NOTE_STR + "$")
# e.g. "a4 maj".
_SCALE_REGEX = re.compile(fr"^({_NOTE_NO_GROUP_STR}) ([0-9a-z-]+)$")
# e.g. "a4 dim" where "dim" is a chord name.
_CHORD_REGEX = re.compile(fr"^({_NOTE_NO_GROUP_STR}) ([0-9a-z-]+)$")
# e.g. "a4 min 4" where "min" is a scale name.
_CHORD_SCALE_INDEXED_REGEX = re.compile(fr"^({_NOTE_NO_GROUP_STR} [0-9a-z-]+) ([0-9]+)$")


def keyboard(players: Iterable[Player], inputs: Iterable[BoundedInput]) -> Player:
    return Combiner([Volume(p, i) for p, i in zip(players, inputs)])


def timelines(s: str, event_duration: Time) -> List[BoundedInput]:
    return [Timeline(t.strip(), event_duration) for t in s.split("\n") if t.strip() != ""]


def note(s: str) -> float:
    return __parse_note(s).frequency()


def __parse_note(s: str) -> Note:
    match = _NOTE_REGEX.match(s)
    if not match:
        raise AssertionError(f"Unrecognized note: {s}")
    abstract_note = match.group(1)
    octave = int(match.group(2))
    return Note(AbstractNote.from_str(abstract_note), octave)


def scale(s: str) -> List[float]:
    return __parse_scale(s).frequencies()


def __parse_scale(s: str) -> Scale:
    match = _SCALE_REGEX.match(s)
    if not match:
        raise AssertionError(f"Unrecognized scale: {s}")
    _note = __parse_note(match.group(1))
    abstract_scale = match.group(2)
    if abstract_scale not in ABSTRACT_SCALES:
        raise AssertionError(f"Unrecognized abstract scale: {abstract_scale}")
    return Scale(_note, ABSTRACT_SCALES[abstract_scale])


def chord(s: str) -> List[float]:
    chord_match = _CHORD_REGEX.match(s)
    if chord_match:
        _note = __parse_note(chord_match.group(1))
        scale_indices = chord_match.group(2)
        if scale_indices not in SCALE_INDICES:
            raise AssertionError(f"Unrecognized chord type: {scale_indices}")
        return Chord.major_scale_indexed(_note, SCALE_INDICES[scale_indices]).frequencies()

    chord_scale_index_match = _CHORD_SCALE_INDEXED_REGEX.match(s)
    if chord_scale_index_match:
        _scale = __parse_scale(chord_scale_index_match.group(1))
        index = int(chord_scale_index_match.group(2))
        return Chord.scale_indexed(_scale, index).frequencies()

    raise AssertionError(f"Unrecognized chord: {s}")
