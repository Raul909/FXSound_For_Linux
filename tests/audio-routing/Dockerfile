# Test image for the audio-routing suite: Ubuntu 24.04 with either PipeWire
# (WirePlumber + pipewire-pulse) or native PulseAudio, plus the Tauri build
# dependencies and the tools the suite uses.
#   docker build --build-arg SERVER=pipewire -t fxsound-routing-pw .
#   docker build --build-arg SERVER=pulseaudio -t fxsound-routing-pa .
FROM ubuntu:24.04
ARG SERVER=pipewire
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential pkg-config curl ca-certificates git file \
      libpulse-dev libssl-dev \
      libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
      pulseaudio-utils dbus dbus-x11 sox python3 python3-numpy \
      xvfb x11-utils xdotool imagemagick procps psmisc \
  && if [ "$SERVER" = "pipewire" ]; then \
       apt-get install -y --no-install-recommends pipewire pipewire-pulse wireplumber pipewire-bin; \
     else \
       apt-get install -y --no-install-recommends pulseaudio; \
     fi \
  && rm -rf /var/lib/apt/lists/*
RUN useradd -m -s /bin/bash tester
USER tester
ENV HOME=/home/tester PATH=/home/tester/.cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
WORKDIR /src
