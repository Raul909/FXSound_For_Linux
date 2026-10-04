#!/bin/bash
# Who plays into what, who records from what.
echo "  default sink: $(pactl get-default-sink)"
pactl list sinks | awk '/^Sink #/{id=$2} /Name: /{n=$2} /Description: /{sub(/^[ \t]*Description: /,""); print "  sink " id " " n " (" $0 ")"}'
pactl list sink-inputs | awk '/^Sink Input #/{id=$2} /^\tSink: /{s=$2} /application.name = /{sub(/.*application.name = /,""); print "  playback #" id " " $0 " -> sink " s}'
pactl list source-outputs | awk '/^Source Output #/{id=$2} /^\tSource: /{s=$2} /application.name = /{sub(/.*application.name = /,""); print "  capture  #" id " " $0 " <- source " s}'
