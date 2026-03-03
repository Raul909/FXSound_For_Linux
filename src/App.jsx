import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const PRESETS = [
  "Flat","Music","Movies","Gaming","Podcast",
  "Bass Boost","Vocal Boost","Deep Bass","Treble Boost","Night Mode"
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
const PRESET_FX = {
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
const DEVICES  = ["Built-in Speakers","Headphones (3.5mm)","USB Audio Device","HDMI Audio","Bluetooth Headset"];
const VIS_SEED = [8,15,22,30,38,42,35,48,44,38,30,24,18,28,36,42,38,30,22,18,24,32,28,20,14,22,30,26,18,12,8,10];

function EQBand({ freq, value, onChange, disabled }) {
  const trackRef = useRef(null);
  const getVal = (y) => {
    const r = trackRef.current.getBoundingClientRect();
    return Math.round(Math.max(-12, Math.min(12, (1-(y-r.top)/r.height)*24-12)));
  };
  const onMouseDown = (e) => {
    if (disabled) return;
    onChange(getVal(e.clientY));
    const mv = (e2) => onChange(getVal(e2.clientY));
    const up = () => { window.removeEventListener("mousemove",mv); window.removeEventListener("mouseup",up); };
    window.addEventListener("mousemove",mv); window.addEventListener("mouseup",up);
  };
  const pct  = ((value+12)/24)*100;
  const fillH = Math.abs(value)/24*100;
  return (
    <div style={{ display:"flex", flexDirection:"column", alignItems:"center", gap:5, userSelect:"none" }}>
      <span style={{ fontSize:8, color:"#e53935", fontFamily:"monospace", minHeight:12 }}>
        {value>0?`+${value}`:value}
      </span>
      <div ref={trackRef} onMouseDown={onMouseDown} style={{ width:16, height:110, background:"#0a0a0a", borderRadius:8, position:"relative", cursor:disabled?"default":"ns-resize", border:"1px solid #1e1e1e" }}>
        <div style={{ position:"absolute", top:"50%", left:0, right:0, height:1, background:"#222" }}/>
        {value>=0 ? (
          <div style={{ position:"absolute", left:3, right:3, bottom:"50%", height:`${fillH}%`, background:"linear-gradient(to top,#e53935,#ef5350)", borderRadius:"3px 3px 0 0", minHeight: value!==0?2:0 }}/>
        ) : (
          <div style={{ position:"absolute", left:3, right:3, top:"50%", height:`${fillH}%`, background:"linear-gradient(to bottom,#c62828,#b71c1c)", borderRadius:"0 0 3px 3px", minHeight: value!==0?2:0 }}/>
        )}
        <div style={{ position:"absolute", left:"50%", top:`${100-pct}%`, transform:"translate(-50%,-50%)", width:14, height:14, borderRadius:"50%", background:"radial-gradient(circle at 40% 35%,#ef5350,#c62828)", border:"2px solid #111", boxShadow:"0 0 6px rgba(229,57,53,0.6)", zIndex:2 }}/>
      </div>
      <span style={{ fontSize:8, color:"#444" }}>{freq}</span>
    </div>
  );
}

function EffectSlider({ label, value, onChange, disabled }) {
  const trackRef = useRef(null);
  const getVal = (x) => {
    const r = trackRef.current.getBoundingClientRect();
    return Math.round(Math.max(0, Math.min(100, (x-r.left)/r.width*100)));
  };
  const onMouseDown = (e) => {
    if (disabled) return;
    onChange(getVal(e.clientX));
    const mv = (e2) => onChange(getVal(e2.clientX));
    const up = () => { window.removeEventListener("mousemove",mv); window.removeEventListener("mouseup",up); };
    window.addEventListener("mousemove",mv); window.addEventListener("mouseup",up);
  };
  return (
    <div style={{ display:"flex", alignItems:"center", gap:10, marginBottom:16 }}>
      <span style={{ width:115, fontSize:11, color:"#888", textAlign:"right", flexShrink:0 }}>{label}</span>
      <div ref={trackRef} onMouseDown={onMouseDown} style={{ flex:1, height:4, background:"#1a1a1a", borderRadius:2, position:"relative", cursor:disabled?"default":"pointer", border:"1px solid #222" }}>
        <div style={{ width:`${value}%`, height:"100%", background:"linear-gradient(to right,#c62828,#e53935)", borderRadius:2 }}/>
        <div style={{ position:"absolute", left:`${value}%`, top:"50%", transform:"translate(-50%,-50%)", width:12, height:12, borderRadius:"50%", background:"radial-gradient(circle at 40% 35%,#ef5350,#c62828)", border:"2px solid #111", boxShadow:"0 0 6px rgba(229,57,53,0.5)" }}/>
      </div>
      <span style={{ width:28, fontSize:10, color:"#e53935", flexShrink:0, fontFamily:"monospace" }}>{value}</span>
    </div>
  );
}

function Visualizer({ powered, tick }) {
  return (
    <div style={{ background:"#080808", height:52, borderBottom:"1px solid #1e1e1e", display:"flex", alignItems:"flex-end", padding:"0 10px 4px", gap:2, overflow:"hidden" }}>
      {VIS_SEED.map((base, i) => {
        const h = powered ? Math.max(3, base + Math.sin((tick+i)*0.4)*8) : 3;
        const r = Math.floor(180+(h/48)*75);
        return <div key={i} style={{ flex:1, borderRadius:"1px 1px 0 0", height:`${h}px`, background:`rgb(${r},${Math.floor(h*1.2)},${Math.floor(h*0.5)})`, transition:"height 0.12s ease", opacity:powered?0.9:0.3, minWidth:2 }}/>;
      })}
    </div>
  );
}

export default function App() {
  const [powered, setPowered] = useState(true);
  const [preset,  setPreset]  = useState("Music");
  const [eq,      setEq]      = useState([...PRESET_EQ["Music"]]);
  const [fx,      setFx]      = useState({...PRESET_FX["Music"]});
  const [device,  setDevice]  = useState(DEVICES[0]);
  const [tab,     setTab]     = useState("eq");
  const [tick,    setTick]    = useState(0);

  useEffect(() => {
    const id = setInterval(() => setTick(t => t+1), 120);
    return () => clearInterval(id);
  }, []);

  const applyPreset = (name) => {
    if (!PRESET_EQ[name]) return;
    setPreset(name); setEq([...PRESET_EQ[name]]); setFx({...PRESET_FX[name]});
    // Apply to backend
    PRESET_EQ[name].forEach((gain, i) => {
      invoke('set_eq_band', { band: i, gain }).catch(console.error);
    });
  };

  // Sync power state to backend
  useEffect(() => {
    invoke('set_power', { enabled: powered }).catch(console.error);
  }, [powered]);

  // Update EQ band in backend
  const updateEQBand = (index, value) => {
    const newEq = [...eq];
    newEq[index] = value;
    setEq(newEq);
    setPreset("Custom");
    invoke('set_eq_band', { band: index, gain: value }).catch(console.error);
  };

  // Update effect in backend
  const updateEffect = (key, value) => {
    setFx(f => ({ ...f, [key]: value }));
    setPreset("Custom");
    invoke('set_effect', { effect: key, value }).catch(console.error);
  };

  return (
    <div style={{ minHeight:"100vh", background:"#1a1a1a", display:"flex", alignItems:"center", justifyContent:"center", fontFamily:"'Roboto','Segoe UI',sans-serif", padding:20 }}>
      <style>{`@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&family=Rajdhani:wght@500;600;700&display=swap');*{box-sizing:border-box;margin:0;padding:0;}select{appearance:none;}select option{background:#1a1a1a;}`}</style>

      <div style={{ width:380, background:"#111", borderRadius:8, border:"1px solid #2a2a2a", boxShadow:"0 24px 80px rgba(0,0,0,0.85)", overflow:"hidden" }}>

        {/* Title bar */}
        <div style={{ background:"#0d0d0d", padding:"8px 12px", display:"flex", alignItems:"center", justifyContent:"space-between", borderBottom:"1px solid #1e1e1e" }}>
          <div style={{ display:"flex", gap:6 }}>
            {["#e53935","#fdd835","#43a047"].map((c,i) => <div key={i} style={{ width:10, height:10, borderRadius:"50%", background:c }}/>)}
          </div>
          <span style={{ fontFamily:"'Rajdhani',sans-serif", fontSize:13, fontWeight:700, color:"#e53935", letterSpacing:2 }}>FXSOUND LINUX</span>
          <span style={{ fontSize:9, color:"#333" }}>v2.0.0</span>
        </div>

        {/* Header */}
        <div style={{ background:"#0d0d0d", padding:"14px 16px 12px", borderBottom:"1px solid #1e1e1e", display:"flex", alignItems:"center", gap:12 }}>
          <button onClick={() => setPowered(p=>!p)} style={{ width:46, height:46, borderRadius:"50%", flexShrink:0, cursor:"pointer", background:powered?"radial-gradient(circle at 40% 35%,#e53935,#b71c1c)":"#1a1a1a", border:powered?"2px solid #ef5350":"2px solid #2a2a2a", boxShadow:powered?"0 0 16px rgba(229,57,53,0.55),0 0 4px #ef5350":"none", display:"flex", alignItems:"center", justifyContent:"center", transition:"all 0.25s" }}>
            <svg width="20" height="20" viewBox="0 0 24 24" fill={powered?"#fff":"#444"}><path d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z"/></svg>
          </button>
          <div style={{ flex:1, display:"flex", flexDirection:"column", gap:7 }}>
            {[["Preset", preset, PRESETS, (v) => applyPreset(v)], ["Output Device", device, DEVICES, setDevice]].map(([lbl, val, opts, setter]) => (
              <div key={lbl}>
                <div style={{ fontSize:9, color:"#555", letterSpacing:1, marginBottom:3, textTransform:"uppercase" }}>{lbl}</div>
                <div style={{ position:"relative" }}>
                  <select value={val} onChange={e=>setter(e.target.value)} disabled={!powered} style={{ width:"100%", background:"#1a1a1a", color:powered?"#ccc":"#555", border:"1px solid #2a2a2a", borderRadius:4, padding:"5px 28px 5px 10px", fontSize:11, cursor:powered?"pointer":"default", outline:"none", fontFamily:"inherit" }}>
                    {opts.map(o=><option key={o}>{o}</option>)}
                    {lbl==="Preset" && <option value="Custom">Custom</option>}
                  </select>
                  <div style={{ position:"absolute", right:8, top:"50%", transform:"translateY(-50%)", pointerEvents:"none", borderLeft:"4px solid transparent", borderRight:"4px solid transparent", borderTop:"5px solid #555", width:0, height:0 }}/>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Visualizer */}
        <div style={{ opacity:powered?1:0.3, transition:"opacity 0.3s" }}>
          <Visualizer powered={powered} tick={tick}/>
        </div>

        {/* Tabs */}
        <div style={{ display:"flex", background:"#0d0d0d", borderBottom:"1px solid #1e1e1e" }}>
          {[["eq","Equalizer"],["fx","Effects"]].map(([id,label]) => (
            <button key={id} onClick={()=>setTab(id)} style={{ flex:1, padding:"8px 0", background:"transparent", color:tab===id?"#e53935":"#444", borderBottom:tab===id?"2px solid #e53935":"2px solid transparent", fontSize:10, letterSpacing:1.5, fontFamily:"inherit", cursor:"pointer", textTransform:"uppercase", fontWeight:500, transition:"all 0.2s" }}>{label}</button>
          ))}
        </div>

        {/* Content */}
        <div style={{ padding:16, opacity:powered?1:0.35, pointerEvents:powered?"auto":"none", transition:"opacity 0.3s" }}>
          {tab==="eq" && (
            <>
              <div style={{ display:"flex", justifyContent:"space-between", alignItems:"flex-end", gap:4 }}>
                {EQ_BANDS.map((freq,i) => <EQBand key={freq} freq={freq} value={eq[i]} onChange={v => updateEQBand(i, v)} disabled={!powered}/>)}
              </div>
              <div style={{ display:"flex", justifyContent:"space-between", marginTop:10, paddingTop:8, borderTop:"1px solid #1a1a1a" }}>
                <span style={{ fontSize:8, color:"#333" }}>-12 dB</span>
                <span style={{ fontSize:8, color:"#333" }}>10-Band Parametric EQ</span>
                <span style={{ fontSize:8, color:"#333" }}>+12 dB</span>
              </div>
            </>
          )}
          {tab==="fx" && (
            <div style={{ paddingTop:2 }}>
              {[["Fidelity","fidelity"],["Ambiance","ambiance"],["Dynamic Boost","dynamic"],["3D Surround","surround"],["HyperBass","bass"]].map(([lbl,key]) => (
                <EffectSlider key={key} label={lbl} value={fx[key]} onChange={v => updateEffect(key, v)} disabled={!powered}/>
              ))}
            </div>
          )}
        </div>

        {/* Status bar */}
        <div style={{ background:"#0a0a0a", borderTop:"1px solid #1a1a1a", padding:"6px 14px", display:"flex", alignItems:"center", justifyContent:"space-between" }}>
          <div style={{ display:"flex", alignItems:"center", gap:5 }}>
            <div style={{ width:6, height:6, borderRadius:"50%", background:powered?"#43a047":"#e53935", boxShadow:powered?"0 0 5px rgba(67,160,71,0.7)":"none" }}/>
            <span style={{ fontSize:9, color:powered?"#43a047":"#e53935", letterSpacing:0.8 }}>{powered?"ACTIVE":"BYPASSED"}</span>
          </div>
          <span style={{ fontSize:9, color:"#333" }}>PulseAudio · 48kHz</span>
          <span style={{ fontSize:9, color:"#555" }}>{preset.toUpperCase()}</span>
        </div>

        {/* UI Demo Notice */}
        <div style={{ background:"#1a1a1a", borderTop:"1px solid #2a2a2a", padding:"10px 14px", textAlign:"center" }}>
          <span style={{ fontSize:10, color:"#888", display:"block", marginBottom:4 }}>⚠️ UI Demo Only</span>
          <span style={{ fontSize:9, color:"#555", lineHeight:1.4 }}>
            This is a visual mockup. Audio processing requires native PulseAudio/PipeWire integration (coming soon).
          </span>
        </div>

      </div>
    </div>
  );
}
