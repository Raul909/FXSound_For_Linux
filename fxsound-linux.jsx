import { useState, useRef, useCallback } from "react";

const PRESETS = [
  "Flat", "Music", "Movies", "Gaming", "Podcast",
  "Bass Boost", "Vocal Boost", "Deep Bass", "Treble Boost", "Night Mode"
];

const PRESET_EQ = {
  "Flat":        [0,0,0,0,0,0,0,0,0,0],
  "Music":       [3,2,1,0,-1,0,2,3,3,2],
  "Movies":      [4,3,2,0,-1,0,1,2,3,4],
  "Gaming":      [5,4,2,1,0,0,1,2,4,5],
  "Podcast":     [-1,0,2,4,4,3,2,1,0,-1],
  "Bass Boost":  [8,7,5,3,0,-1,-1,-1,-2,-2],
  "Vocal Boost": [-2,-1,0,3,5,5,3,1,0,-1],
  "Deep Bass":   [10,8,6,2,0,-1,-2,-2,-3,-3],
  "Treble Boost":[-3,-2,-1,0,1,2,4,6,7,8],
  "Night Mode":  [2,2,1,0,-2,-2,-1,0,1,2],
};

const PRESET_EFFECTS = {
  "Flat":        { fidelity:0, ambiance:0, dynamic:0, surround:0, bass:0 },
  "Music":       { fidelity:65, ambiance:40, dynamic:50, surround:30, bass:45 },
  "Movies":      { fidelity:70, ambiance:75, dynamic:60, surround:80, bass:55 },
  "Gaming":      { fidelity:60, ambiance:50, dynamic:70, surround:90, bass:60 },
  "Podcast":     { fidelity:80, ambiance:20, dynamic:55, surround:10, bass:20 },
  "Bass Boost":  { fidelity:50, ambiance:30, dynamic:65, surround:20, bass:90 },
  "Vocal Boost": { fidelity:85, ambiance:35, dynamic:50, surround:25, bass:15 },
  "Deep Bass":   { fidelity:45, ambiance:25, dynamic:70, surround:15, bass:95 },
  "Treble Boost":{ fidelity:75, ambiance:45, dynamic:45, surround:35, bass:10 },
  "Night Mode":  { fidelity:55, ambiance:60, dynamic:35, surround:40, bass:30 },
};

const EQ_BANDS = ["32","64","125","250","500","1K","2K","4K","8K","16K"];

const DEVICES = [
  "Built-in Speakers",
  "Headphones (3.5mm)",
  "USB Audio Device",
  "HDMI Audio Output",
  "Bluetooth Headset",
];

function EQBar({ freq, value, onChange }) {
  const trackRef = useRef(null);
  const dragging = useRef(false);

  const getVal = (clientY) => {
    const rect = trackRef.current.getBoundingClientRect();
    const pct = 1 - (clientY - rect.top) / rect.height;
    return Math.round(Math.max(-12, Math.min(12, pct * 24 - 12)));
  };

  const onMouseDown = (e) => {
    dragging.current = true;
    onChange(getVal(e.clientY));
    const move = (e2) => { if (dragging.current) onChange(getVal(e2.clientY)); };
    const up = () => { dragging.current = false; window.removeEventListener("mousemove", move); window.removeEventListener("mouseup", up); };
    window.addEventListener("mousemove", move);
    window.addEventListener("mouseup", up);
  };

  const pct = ((value + 12) / 24) * 100;
  const isPos = value >= 0;

  return (
    <div style={{ display:"flex", flexDirection:"column", alignItems:"center", gap:6, userSelect:"none" }}>
      <span style={{ fontSize:9, color:"#4fc3f7", fontFamily:"'JetBrains Mono',monospace", letterSpacing:1 }}>
        {value > 0 ? `+${value}` : value}
      </span>
      <div
        ref={trackRef}
        onMouseDown={onMouseDown}
        style={{
          width:18, height:120, background:"#0d1117",
          borderRadius:9, position:"relative", cursor:"ns-resize",
          border:"1px solid #1e2a3a", overflow:"visible",
        }}
      >
        {/* center line */}
        <div style={{ position:"absolute", top:"50%", left:0, right:0, height:1, background:"#1e3a50", transform:"translateY(-50%)" }}/>
        {/* fill bar */}
        <div style={{
          position:"absolute",
          left:3, right:3,
          bottom: isPos ? "50%" : `${pct}%`,
          height: isPos ? `${pct - 50}%` : `${50 - pct}%`,
          background: isPos
            ? "linear-gradient(to top, #00bcd4, #4fc3f7)"
            : "linear-gradient(to bottom, #ef5350, #e53935)",
          borderRadius:6,
          transition:"height 0.1s, bottom 0.1s",
          minHeight: value !== 0 ? 3 : 0,
        }}/>
        {/* thumb */}
        <div style={{
          position:"absolute",
          left:"50%", top:`${100 - pct}%`,
          transform:"translate(-50%, -50%)",
          width:16, height:16,
          borderRadius:"50%",
          background:"linear-gradient(135deg, #4fc3f7, #0288d1)",
          border:"2px solid #0d1117",
          boxShadow:"0 0 8px #4fc3f760",
          cursor:"ns-resize",
          zIndex:2,
        }}/>
      </div>
      <span style={{ fontSize:8.5, color:"#546e7a", fontFamily:"'JetBrains Mono',monospace", letterSpacing:0.5 }}>
        {freq}
      </span>
    </div>
  );
}

function EffectSlider({ label, value, onChange, color }) {
  const trackRef = useRef(null);
  const dragging = useRef(false);

  const getVal = (clientX) => {
    const rect = trackRef.current.getBoundingClientRect();
    const pct = Math.max(0, Math.min(1, (clientX - rect.left) / rect.width));
    return Math.round(pct * 100);
  };

  const onMouseDown = (e) => {
    dragging.current = true;
    onChange(getVal(e.clientX));
    const move = (e2) => { if (dragging.current) onChange(getVal(e2.clientX)); };
    const up = () => { dragging.current = false; window.removeEventListener("mousemove", move); window.removeEventListener("mouseup", up); };
    window.addEventListener("mousemove", move);
    window.addEventListener("mouseup", up);
  };

  return (
    <div style={{ display:"flex", alignItems:"center", gap:12, marginBottom:10 }}>
      <span style={{ width:100, fontSize:11, color:"#78909c", fontFamily:"'JetBrains Mono',monospace", letterSpacing:0.5, textAlign:"right", flexShrink:0 }}>
        {label}
      </span>
      <div
        ref={trackRef}
        onMouseDown={onMouseDown}
        style={{
          flex:1, height:6, background:"#0d1117",
          borderRadius:3, position:"relative", cursor:"pointer",
          border:"1px solid #1e2a3a",
        }}
      >
        <div style={{
          position:"absolute", left:0, top:0, bottom:0,
          width:`${value}%`,
          background:`linear-gradient(to right, ${color}88, ${color})`,
          borderRadius:3,
          transition:"width 0.05s",
        }}/>
        <div style={{
          position:"absolute",
          left:`${value}%`, top:"50%",
          transform:"translate(-50%, -50%)",
          width:14, height:14,
          borderRadius:"50%",
          background:color,
          border:"2px solid #0d1117",
          boxShadow:`0 0 8px ${color}80`,
          cursor:"pointer",
        }}/>
      </div>
      <span style={{ width:30, fontSize:11, color, fontFamily:"'JetBrains Mono',monospace", textAlign:"left", flexShrink:0 }}>
        {value}
      </span>
    </div>
  );
}

function Visualizer({ powered }) {
  const bars = 32;
  return (
    <div style={{ display:"flex", alignItems:"flex-end", gap:2, height:40, padding:"0 4px" }}>
      {Array.from({length:bars}).map((_, i) => {
        const h = powered ? Math.random() * 35 + 5 : 3;
        return (
          <div
            key={i}
            style={{
              flex:1,
              height: powered ? `${15 + Math.sin(i * 0.4 + Date.now()/500) * 12 + 10}px` : "3px",
              background: i < bars*0.4
                ? "linear-gradient(to top, #0288d1, #4fc3f7)"
                : i < bars*0.75
                ? "linear-gradient(to top, #00838f, #26c6da)"
                : "linear-gradient(to top, #00695c, #26a69a)",
              borderRadius:"2px 2px 0 0",
              transition:"height 0.15s ease",
              opacity: powered ? 0.85 : 0.25,
            }}
          />
        );
      })}
    </div>
  );
}

export default function FXSoundLinux() {
  const [powered, setPowered] = useState(true);
  const [preset, setPreset] = useState("Music");
  const [eq, setEq] = useState(PRESET_EQ["Music"]);
  const [effects, setEffects] = useState(PRESET_EFFECTS["Music"]);
  const [volume, setVolume] = useState(75);
  const [device, setDevice] = useState(DEVICES[0]);
  const [tab, setTab] = useState("eq");
  const [tick, setTick] = useState(0);

  // Animate visualizer
  useCallback(() => {
    const id = setInterval(() => setTick(t => t + 1), 120);
    return () => clearInterval(id);
  }, [])();

  const applyPreset = (name) => {
    setPreset(name);
    setEq([...PRESET_EQ[name]]);
    setEffects({...PRESET_EFFECTS[name]});
  };

  const setEqBand = (i, v) => {
    const next = [...eq];
    next[i] = v;
    setEq(next);
    setPreset("Custom");
  };

  const setEffect = (key, v) => {
    setEffects(e => ({...e, [key]: v}));
    setPreset("Custom");
  };

  const bg = "#080d13";
  const panel = "#0e1520";
  const border = "#162031";

  return (
    <div style={{
      minHeight:"100vh",
      background: `radial-gradient(ellipse at 20% 20%, #0a1929 0%, ${bg} 60%)`,
      display:"flex", alignItems:"center", justifyContent:"center",
      fontFamily:"'JetBrains Mono', 'Fira Code', monospace",
      padding:20,
    }}>
      {/* Load font */}
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;700&family=Rajdhani:wght@400;600;700&display=swap');
        ::-webkit-scrollbar { width:4px; }
        ::-webkit-scrollbar-track { background:#0d1117; }
        ::-webkit-scrollbar-thumb { background:#1e3a50; border-radius:2px; }
        * { box-sizing:border-box; margin:0; padding:0; }
        select option { background:#0e1520; }
      `}</style>

      <div style={{
        width:520, background:panel,
        borderRadius:12,
        border:`1px solid ${border}`,
        boxShadow:"0 24px 80px #00000090, 0 0 0 1px #162031, inset 0 1px 0 #1e3a5040",
        overflow:"hidden",
      }}>

        {/* Title bar */}
        <div style={{
          background:"#060b10",
          padding:"10px 16px",
          display:"flex", alignItems:"center", justifyContent:"space-between",
          borderBottom:`1px solid ${border}`,
        }}>
          <div style={{ display:"flex", alignItems:"center", gap:8 }}>
            <div style={{ width:8,height:8,borderRadius:"50%",background:"#ef5350" }}/>
            <div style={{ width:8,height:8,borderRadius:"50%",background:"#ffd54f" }}/>
            <div style={{ width:8,height:8,borderRadius:"50%",background:"#66bb6a" }}/>
          </div>
          <div style={{ display:"flex", alignItems:"center", gap:8 }}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="#4fc3f7">
              <path d="M12 3v9.28a4 4 0 00-1.5-.28C8 12 6 14 6 16.5S8 21 10.5 21c2.31 0 4.2-1.75 4.45-4H15V6h3V3h-6z"/>
            </svg>
            <span style={{ fontSize:12, color:"#4fc3f7", letterSpacing:2, fontFamily:"'Rajdhani',sans-serif", fontWeight:700 }}>
              FXSOUND — LINUX
            </span>
          </div>
          <span style={{ fontSize:9, color:"#37474f", letterSpacing:1 }}>v2.0.0</span>
        </div>

        {/* Main content */}
        <div style={{ padding:20 }}>

          {/* Top row: power + device + volume */}
          <div style={{ display:"flex", alignItems:"center", gap:12, marginBottom:18 }}>
            {/* Power */}
            <button
              onClick={() => setPowered(p => !p)}
              style={{
                width:44, height:44, borderRadius:"50%",
                background: powered
                  ? "radial-gradient(circle, #00bcd4 0%, #0288d1 100%)"
                  : "#111a24",
                border: powered ? "2px solid #4fc3f7" : "2px solid #1e2a3a",
                cursor:"pointer",
                display:"flex", alignItems:"center", justifyContent:"center",
                boxShadow: powered ? "0 0 16px #00bcd480, 0 0 4px #4fc3f7" : "none",
                transition:"all 0.3s",
                flexShrink:0,
              }}
            >
              <svg width="18" height="18" viewBox="0 0 24 24" fill={powered ? "#fff" : "#37474f"}>
                <path d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z"/>
              </svg>
            </button>

            {/* Device selector */}
            <div style={{ flex:1 }}>
              <div style={{ fontSize:9, color:"#37474f", letterSpacing:1, marginBottom:4 }}>OUTPUT DEVICE</div>
              <select
                value={device}
                onChange={e => setDevice(e.target.value)}
                disabled={!powered}
                style={{
                  width:"100%", background:"#060b10", color: powered ? "#78909c" : "#37474f",
                  border:`1px solid ${border}`, borderRadius:6,
                  padding:"6px 10px", fontSize:11, cursor:"pointer",
                  outline:"none", fontFamily:"inherit",
                }}
              >
                {DEVICES.map(d => <option key={d}>{d}</option>)}
              </select>
            </div>

            {/* Volume */}
            <div style={{ width:80, flexShrink:0 }}>
              <div style={{ fontSize:9, color:"#37474f", letterSpacing:1, marginBottom:4, textAlign:"center" }}>VOL {volume}%</div>
              <div style={{
                height:6, background:"#0d1117", borderRadius:3,
                border:`1px solid ${border}`, position:"relative", cursor:"pointer",
              }}
                onMouseDown={e => {
                  if (!powered) return;
                  const el = e.currentTarget;
                  const get = (cx) => Math.round(Math.max(0, Math.min(100, (cx - el.getBoundingClientRect().left) / el.offsetWidth * 100)));
                  setVolume(get(e.clientX));
                  const mv = (e2) => setVolume(get(e2.clientX));
                  const up = () => { window.removeEventListener("mousemove", mv); window.removeEventListener("mouseup", up); };
                  window.addEventListener("mousemove", mv);
                  window.addEventListener("mouseup", up);
                }}
              >
                <div style={{ width:`${volume}%`, height:"100%", background:"linear-gradient(to right, #0288d1, #4fc3f7)", borderRadius:3 }}/>
                <div style={{
                  position:"absolute", left:`${volume}%`, top:"50%",
                  transform:"translate(-50%,-50%)",
                  width:12, height:12, borderRadius:"50%",
                  background:"#4fc3f7", border:"2px solid #0d1117",
                  boxShadow:"0 0 6px #4fc3f760",
                }}/>
              </div>
            </div>
          </div>

          {/* Visualizer */}
          <div style={{
            background:"#060b10", borderRadius:8, padding:"10px 12px",
            border:`1px solid ${border}`, marginBottom:16,
            opacity: powered ? 1 : 0.4, transition:"opacity 0.3s",
          }}>
            <Visualizer powered={powered} tick={tick} />
          </div>

          {/* Preset scroller */}
          <div style={{ marginBottom:16 }}>
            <div style={{ fontSize:9, color:"#37474f", letterSpacing:1, marginBottom:8 }}>PRESETS</div>
            <div style={{ display:"flex", gap:6, flexWrap:"wrap" }}>
              {PRESETS.map(p => (
                <button
                  key={p}
                  onClick={() => powered && applyPreset(p)}
                  style={{
                    padding:"4px 10px", borderRadius:20,
                    fontSize:9.5, letterSpacing:0.5,
                    fontFamily:"inherit",
                    background: preset === p ? "linear-gradient(135deg, #0288d1, #00bcd4)" : "#060b10",
                    color: preset === p ? "#fff" : "#546e7a",
                    border: preset === p ? "1px solid #4fc3f7" : `1px solid ${border}`,
                    cursor: powered ? "pointer" : "default",
                    boxShadow: preset === p ? "0 0 8px #00bcd440" : "none",
                    transition:"all 0.2s",
                  }}
                >
                  {p}
                </button>
              ))}
            </div>
          </div>

          {/* Tab bar */}
          <div style={{ display:"flex", gap:2, marginBottom:16, background:"#060b10", borderRadius:8, padding:3, border:`1px solid ${border}` }}>
            {[["eq","EQUALIZER"],["fx","EFFECTS"]].map(([id, label]) => (
              <button
                key={id}
                onClick={() => setTab(id)}
                style={{
                  flex:1, padding:"7px 0", borderRadius:6,
                  background: tab===id ? "linear-gradient(135deg, #0d2137, #0a1929)" : "transparent",
                  color: tab===id ? "#4fc3f7" : "#37474f",
                  border: tab===id ? `1px solid ${border}` : "1px solid transparent",
                  fontSize:10, letterSpacing:1.5, fontFamily:"inherit",
                  cursor:"pointer", fontWeight: tab===id ? 700 : 400,
                  transition:"all 0.2s",
                }}
              >
                {label}
              </button>
            ))}
          </div>

          {/* EQ Tab */}
          {tab === "eq" && (
            <div style={{
              background:"#060b10", borderRadius:8, padding:"16px 12px",
              border:`1px solid ${border}`,
              opacity: powered ? 1 : 0.35, pointerEvents: powered ? "auto" : "none",
              transition:"opacity 0.3s",
            }}>
              <div style={{ display:"flex", justifyContent:"space-around", alignItems:"flex-end" }}>
                {EQ_BANDS.map((freq, i) => (
                  <EQBar key={freq} freq={freq} value={eq[i]} onChange={v => setEqBand(i, v)} />
                ))}
              </div>
              <div style={{ display:"flex", justifyContent:"space-between", marginTop:12, paddingTop:10, borderTop:`1px solid ${border}` }}>
                <span style={{ fontSize:9, color:"#37474f" }}>-12 dB</span>
                <span style={{ fontSize:9, color:"#37474f" }}>10-Band Parametric EQ</span>
                <span style={{ fontSize:9, color:"#37474f" }}>+12 dB</span>
              </div>
            </div>
          )}

          {/* Effects Tab */}
          {tab === "fx" && (
            <div style={{
              background:"#060b10", borderRadius:8, padding:"16px 16px",
              border:`1px solid ${border}`,
              opacity: powered ? 1 : 0.35, pointerEvents: powered ? "auto" : "none",
              transition:"opacity 0.3s",
            }}>
              <EffectSlider label="Fidelity" value={effects.fidelity} onChange={v => setEffect("fidelity",v)} color="#4fc3f7" />
              <EffectSlider label="Ambiance" value={effects.ambiance} onChange={v => setEffect("ambiance",v)} color="#26c6da" />
              <EffectSlider label="Dynamic Boost" value={effects.dynamic} onChange={v => setEffect("dynamic",v)} color="#ab47bc" />
              <EffectSlider label="3D Surround" value={effects.surround} onChange={v => setEffect("surround",v)} color="#42a5f5" />
              <EffectSlider label="Bass Boost" value={effects.bass} onChange={v => setEffect("bass",v)} color="#ef5350" />
            </div>
          )}

          {/* Status bar */}
          <div style={{
            display:"flex", justifyContent:"space-between", alignItems:"center",
            marginTop:14, padding:"8px 12px",
            background:"#060b10", borderRadius:6,
            border:`1px solid ${border}`,
          }}>
            <div style={{ display:"flex", alignItems:"center", gap:6 }}>
              <div style={{
                width:6, height:6, borderRadius:"50%",
                background: powered ? "#66bb6a" : "#ef5350",
                boxShadow: powered ? "0 0 6px #66bb6a80" : "none",
              }}/>
              <span style={{ fontSize:9, color: powered ? "#66bb6a" : "#ef5350", letterSpacing:1 }}>
                {powered ? "ACTIVE" : "BYPASSED"}
              </span>
            </div>
            <span style={{ fontSize:9, color:"#37474f", letterSpacing:0.5 }}>
              PulseAudio · 48kHz · 32-bit
            </span>
            <span style={{ fontSize:9, color:"#546e7a", letterSpacing:0.5 }}>
              {preset.toUpperCase()}
            </span>
          </div>

        </div>
      </div>
    </div>
  );
}
