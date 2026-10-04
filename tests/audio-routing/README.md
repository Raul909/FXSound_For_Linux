# Audio-routing tests

End-to-end checks of how FXSound routes system audio, run against a real
PipeWire or PulseAudio server. They exist because both bugs behind 1.1.4's
broken release were invisible to unit tests:

- **#510**: the window froze at launch, blocked on the audio server.
- **#511**: applications kept playing to the speakers while FXSound played a
  processed copy into the same speakers and re-recorded it, so audio doubled
  and fed back on itself.

## What is checked

`verify.sh` starts a headless audio session with two fake output devices
("Speakers" and "Headphones", null sinks whose monitors record exactly what
each would play), launches FXSound under Xvfb and checks:

1. It creates the FXSound device, makes it the default, records its monitor
   and plays to the real device, and the UI thread is never blocked.
2. A probe sound played like any application arrives at the speakers
   **exactly once**, and nothing is still sounding after it ends (no
   feedback). `analyze.py` cross-correlates the recording with the probe, so
   every copy of the sound appears as its own peak.
3. Picking another device in the system sound settings is followed.
4. Unplugging the device falls back to another, and plugging it back in
   returns to it.
5. A second launch exits and leaves the first running.
6. A clean exit (SIGTERM) removes the FXSound device, restores the default
   output and saves the settings.
7. After a crash (SIGKILL) a relaunch cleans up the leftover device.
8. The audio server restarting underneath FXSound is recovered from.
9. A frozen audio server never blocks the UI, and FXSound recovers when it
   responds again.

## Running it

These scripts start their own audio server and wipe their runtime directory,
so they refuse to run outside a container or CI. With Docker:

```bash
npm ci && npm run build                 # the app embeds the built frontend
tests/audio-routing/run-in-docker.sh pw   # PipeWire (Ubuntu 24.04)
tests/audio-routing/run-in-docker.sh pa   # native PulseAudio
```

CI runs both on every pull request (`.github/workflows/ci.yml`).
