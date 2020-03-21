from typing import List
from . import Player, Timeline, Volume, Combiner, Time, SmoothBool


def keyboard(players: List[Player], timeline: str, event_duration: Time) -> Player:
    timeline = [t.strip() for t in timeline.split("\n") if t.strip() != ""]
    return Combiner(
        [
            Volume(
                p,
                SmoothBool(
                    Timeline(t.strip(), event_duration),
                    smooth_in="0.1 seconds",
                    smooth_out="0.1 seconds",
                ),
            )
            for (p, t) in zip(players, timeline)
        ]
    )
