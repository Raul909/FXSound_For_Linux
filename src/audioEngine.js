// Web Audio API engine for real-time audio processing
export class AudioEngine {
  constructor() {
    this.audioContext = null;
    this.source = null;
    this.analyser = null;
    this.eqFilters = [];
    this.gainNode = null;
    this.isActive = false;
  }

  async init() {
    try {
      // Request microphone access
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      
      this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
      this.source = this.audioContext.createMediaStreamSource(stream);
      
      // Create 10-band EQ (biquad filters)
      const frequencies = [32, 64, 125, 250, 500, 1000, 2000, 4000, 8000, 16000];
      this.eqFilters = frequencies.map(freq => {
        const filter = this.audioContext.createBiquadFilter();
        filter.type = 'peaking';
        filter.frequency.value = freq;
        filter.Q.value = 1.0;
        filter.gain.value = 0;
        return filter;
      });

      // Analyser for visualizer
      this.analyser = this.audioContext.createAnalyser();
      this.analyser.fftSize = 64;

      // Gain node for effects
      this.gainNode = this.audioContext.createGain();

      // Connect chain: source -> EQ filters -> gain -> analyser -> destination
      let current = this.source;
      this.eqFilters.forEach(filter => {
        current.connect(filter);
        current = filter;
      });
      current.connect(this.gainNode);
      this.gainNode.connect(this.analyser);
      this.analyser.connect(this.audioContext.destination);

      this.isActive = true;
      return true;
    } catch (err) {
      console.error('Audio init failed:', err);
      return false;
    }
  }

  updateEQ(bandIndex, dbValue) {
    if (this.eqFilters[bandIndex]) {
      this.eqFilters[bandIndex].gain.value = dbValue;
    }
  }

  updateEffects(fx) {
    if (!this.gainNode) return;
    // Simplified: map fidelity to overall gain
    const gainValue = 1 + (fx.fidelity / 100) * 0.5;
    this.gainNode.gain.value = gainValue;
  }

  getVisualizerData() {
    if (!this.analyser) return new Uint8Array(32);
    const data = new Uint8Array(this.analyser.frequencyBinCount);
    this.analyser.getByteFrequencyData(data);
    return data;
  }

  setPower(enabled) {
    if (!this.audioContext) return;
    if (enabled) {
      this.audioContext.resume();
    } else {
      this.audioContext.suspend();
    }
  }

  destroy() {
    if (this.audioContext) {
      this.audioContext.close();
    }
  }
}
