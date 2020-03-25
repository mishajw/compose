from typing import NamedTuple, Dict, List


class ScaleIndex(NamedTuple):
    """
    An index in a scale, with an optional flat/sharp adjustment.
    """

    # 1-based, for consistency with music theory.
    index: int
    step_adjustment: int = 0


SCALE_INDICES: Dict[str, List[ScaleIndex]] = {
    "maj": [ScaleIndex(1), ScaleIndex(3), ScaleIndex(5)],
    "min": [ScaleIndex(1), ScaleIndex(3, -1), ScaleIndex(5)],
    "dim": [ScaleIndex(1), ScaleIndex(3, -1), ScaleIndex(5, -1)],
    "sus2": [ScaleIndex(1), ScaleIndex(2), ScaleIndex(5)],
    "sus4": [ScaleIndex(1), ScaleIndex(4), ScaleIndex(5)],
    "aug": [ScaleIndex(1), ScaleIndex(3), ScaleIndex(5, 1)],
    "6": [ScaleIndex(1), ScaleIndex(3), ScaleIndex(5), ScaleIndex(6)],
    "7": [ScaleIndex(1), ScaleIndex(3), ScaleIndex(5), ScaleIndex(7)],
    "dom": [ScaleIndex(1), ScaleIndex(3), ScaleIndex(5), ScaleIndex(7, -1)],
}
