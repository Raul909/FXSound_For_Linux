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
  
  return (
    <div style={{ display:"flex", flexDirection:"column", alignItems:"center", gap:6, userSelect:"none", flex:1 }}>
      <span style={{ fontSize:10, color:"#b1b1b1", fontWeight:600, minHeight:14, fontFamily:"'Inter','Segoe UI',sans-serif" }}>
        {value>0?`+${value}`:value}
      </span>
      <div ref={trackRef} onMouseDown={onMouseDown} style={{ width:20, height:140, background:"#181818", borderRadius:10, position:"relative", cursor:disabled?"default":"pointer", border:"1px solid #2b2b2b" }}>
        {/* Center line */}
        <div style={{ position:"absolute", top:"50%", left:4, right:4, height:1, background:"#2b2b2b" }}/>
        
        {/* Dashed track line */}
        <svg style={{ position:"absolute", left:"50%", top:0, height:"100%", transform:"translateX(-50%)" }} width="2" height="140">
          <line x1="1" y1="0" x2="1" y2="140" stroke="#e33250" strokeWidth="1" strokeDasharray="5,2" opacity="0.4"/>
        </svg>
        
        {/* Thumb */}
        <div style={{ 
          position:"absolute", 
          left:"50%", 
          top:`${100-pct}%`, 
          transform:"translate(-50%,-50%)", 
          width:16, 
          height:16, 
          borderRadius:"50%", 
          background:"radial-gradient(circle at 35% 30%, #f7546f, #e63462)", 
          border:"2px solid #0c0c0c", 
          boxShadow:"0 2px 8px rgba(230,52,98,0.5), 0 0 0 0 rgba(230,52,98,0.3)", 
          zIndex:2,
          opacity: disabled ? 0.3 : 1
        }}/>
      </div>
      <span style={{ fontSize:9, color:"#666", fontWeight:500, fontFamily:"'Inter','Segoe UI',sans-serif" }}>{freq}</span>
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
    <div style={{ display:"flex", alignItems:"center", gap:12, marginBottom:18 }}>
      <span style={{ width:120, fontSize:13, color:"#b1b1b1", textAlign:"right", flexShrink:0, fontWeight:500, fontFamily:"'Inter','Segoe UI',sans-serif" }}>{label}</span>
      <div ref={trackRef} onMouseDown={onMouseDown} style={{ flex:1, height:6, background:"#181818", borderRadius:3, position:"relative", cursor:disabled?"default":"pointer", border:"1px solid #2b2b2b" }}>
        <div style={{ width:`${value}%`, height:"100%", background:"linear-gradient(to right, #e33250, #f7546f)", borderRadius:3, opacity: disabled ? 0.3 : 0.2 }}/>
        <div style={{ 
          position:"absolute", 
          left:`${value}%`, 
          top:"50%", 
          transform:"translate(-50%,-50%)", 
          width:16, 
          height:16, 
          borderRadius:"50%", 
          background:"radial-gradient(circle at 35% 30%, #f7546f, #e63462)", 
          border:"2px solid #0c0c0c", 
          boxShadow:"0 2px 8px rgba(230,52,98,0.5)",
          opacity: disabled ? 0.3 : 1
        }}/>
      </div>
      <span style={{ width:32, fontSize:12, color:"#e63462", flexShrink:0, fontFamily:"'Inter','Segoe UI',sans-serif", fontWeight:600, textAlign:"center" }}>{value}</span>
    </div>
  );
}

function Visualizer({ powered }) {
  const [fftData, setFftData] = useState(new Array(32).fill(3));

  useEffect(() => {
    if (!powered) {
      setFftData(new Array(32).fill(3));
      return;
    }

    const interval = setInterval(async () => {
      try {
        const data = await invoke('get_visualizer_data');
        setFftData(data);
      } catch (err) {
        console.error('Failed to get visualizer data:', err);
      }
    }, 100);

    return () => clearInterval(interval);
  }, [powered]);

  return (
    <div style={{ background:"#0c0c0c", height:64, borderBottom:"1px solid #2b2b2b", display:"flex", alignItems:"flex-end", padding:"0 16px 8px", gap:3, overflow:"hidden" }}>
      {fftData.map((magnitude, i) => {
        const h = powered ? Math.max(4, magnitude * 0.6) : 4;
        return (
          <div 
            key={i} 
            style={{ 
              flex:1, 
              borderRadius:"2px 2px 0 0", 
              height:`${h}px`, 
              background: powered 
                ? `linear-gradient(to top, #e63462, #f7546f)` 
                : "#2b2b2b",
              transition:"height 0.15s ease-out", 
              opacity: powered ? 0.95 : 0.4,
              minWidth:3,
              boxShadow: powered && h > 10 ? "0 0 8px rgba(230,52,98,0.4)" : "none"
            }}
          />
        );
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

  const applyPreset = (name) => {
    if (!PRESET_EQ[name]) return;
    setPreset(name); setEq([...PRESET_EQ[name]]); setFx({...PRESET_FX[name]});
    PRESET_EQ[name].forEach((gain, i) => {
      invoke('set_eq_band', { band: i, gain }).catch(console.error);
    });
    Object.entries(PRESET_FX[name]).forEach(([key, value]) => {
      invoke('set_effect', { effect: key, value }).catch(console.error);
    });
  };

  useEffect(() => {
    invoke('set_power', { enabled: powered }).catch(console.error);
  }, [powered]);

  const updateEQBand = (index, value) => {
    const newEq = [...eq];
    newEq[index] = value;
    setEq(newEq);
    setPreset("Custom");
    invoke('set_eq_band', { band: index, gain: value }).catch(console.error);
  };

  const updateEffect = (key, value) => {
    setFx(f => ({ ...f, [key]: value }));
    setPreset("Custom");
    invoke('set_effect', { effect: key, value }).catch(console.error);
  };

  return (
    <div style={{ minHeight:"100vh", background:"#181818", display:"flex", alignItems:"center", justifyContent:"center", fontFamily:"'Inter','Segoe UI',sans-serif", padding:20 }}>
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
        * { box-sizing:border-box; margin:0; padding:0; }
        select { appearance:none; }
        select option { background:#000; color:#b1b1b1; }
      `}</style>

      <div style={{ width:420, background:"#0c0c0c", borderRadius:21, border:"1px solid #2b2b2b", boxShadow:"0 20px 60px rgba(0,0,0,0.8)", overflow:"hidden" }}>

        {/* Title bar */}
        <div style={{ background:"#0c0c0c", padding:"12px 20px", display:"flex", alignItems:"center", justifyContent:"space-between", borderBottom:"1px solid #2b2b2b", height:57 }}>
          <div style={{ display:"flex", gap:8 }}>
            {["#ff5f57","#ffbd2e","#28ca42"].map((c,i) => <div key={i} style={{ width:12, height:12, borderRadius:"50%", background:c }}/>)}
          </div>
          <span style={{ fontSize:14, fontWeight:700, color:"#e63462", letterSpacing:2.5, fontFamily:"'Inter','Segoe UI',sans-serif" }}>FXSOUND</span>
          <div style={{ display:"flex", gap:12, alignItems:"center" }}>
            <button style={{ background:"transparent", border:"none", color:"#666", fontSize:16, cursor:"pointer", padding:0, width:20, height:20, display:"flex", alignItems:"center", justifyContent:"center" }}>⚙</button>
            <button style={{ background:"transparent", border:"none", color:"#666", fontSize:16, cursor:"pointer", padding:0, width:20, height:20, display:"flex", alignItems:"center", justifyContent:"center" }}>−</button>
          </div>
        </div>

        {/* Header with power and dropdowns */}
        <div style={{ background:"#0c0c0c", padding:"20px 20px 16px", borderBottom:"1px solid #2b2b2b", display:"flex", alignItems:"center", gap:16 }}>
          <button 
            onClick={() => setPowered(p=>!p)} 
            style={{ 
              width:52, 
              height:52, 
              borderRadius:"50%", 
              flexShrink:0, 
              cursor:"pointer", 
              background: powered 
                ? "radial-gradient(circle at 35% 30%, #f7546f, #d51535)" 
                : "#181818", 
              border: powered ? "3px solid #e63462" : "3px solid #2b2b2b", 
              boxShadow: powered ? "0 0 20px rgba(230,52,98,0.6), 0 4px 12px rgba(213,21,53,0.4)" : "none", 
              display:"flex", 
              alignItems:"center", 
              justifyContent:"center", 
              transition:"all 0.3s cubic-bezier(0.4, 0, 0.2, 1)" 
            }}
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill={powered?"#fff":"#666"}>
              <path d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z"/>
            </svg>
          </button>
          
          <div style={{ flex:1, display:"flex", flexDirection:"column", gap:10 }}>
            {[["PRESET", preset, PRESETS, (v) => applyPreset(v)], ["OUTPUT DEVICE", device, DEVICES, setDevice]].map(([lbl, val, opts, setter]) => (
              <div key={lbl}>
                <div style={{ fontSize:9, color:"#666", letterSpacing:1.2, marginBottom:4, fontWeight:600 }}>{lbl}</div>
                <div style={{ position:"relative" }}>
                  <select 
                    value={val} 
                    onChange={e=>setter(e.target.value)} 
                    disabled={!powered} 
                    style={{ 
                      width:"100%", 
                      background:"#000", 
                      color: powered ? "#b1b1b1" : "#666", 
                      border:"1px solid #2b2b2b", 
                      borderRadius:8, 
                      padding:"8px 32px 8px 16px", 
                      fontSize:13, 
                      cursor: powered ? "pointer" : "default", 
                      outline:"none", 
                      fontFamily:"'Inter','Segoe UI',sans-serif",
                      fontWeight:500
                    }}
                  >
                    {opts.map(o=><option key={o}>{o}</option>)}
                    {lbl==="PRESET" && <option value="Custom">Custom</option>}
                  </select>
                  <svg style={{ position:"absolute", right:12, top:"50%", transform:"translateY(-50%)", pointerEvents:"none" }} width="10" height="6" viewBox="0 0 10 6">
                    <path d="M0 0l5 6 5-6z" fill={powered?"#e63462":"#666"}/>
                  </svg>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Visualizer */}
        <Visualizer powered={powered}/>

        {/* Tabs */}
        <div style={{ display:"flex", background:"#0c0c0c", borderBottom:"1px solid #2b2b2b" }}>
          {[["eq","EQUALIZER"],["fx","EFFECTS"]].map(([id,label]) => (
            <button 
              key={id} 
              onClick={()=>setTab(id)} 
              style={{ 
                flex:1, 
                padding:"14px 0", 
                background:"transparent", 
                color: tab===id ? "#e63462" : "#666", 
                borderBottom: tab===id ? "3px solid #e63462" : "3px solid transparent", 
                fontSize:11, 
                letterSpacing:1.5, 
                fontFamily:"'Inter','Segoe UI',sans-serif", 
                cursor:"pointer", 
                fontWeight:600, 
                transition:"all 0.2s",
                border:"none"
              }}
            >
              {label}
            </button>
          ))}
        </div>

        {/* Content */}
        <div style={{ padding:"24px 20px", opacity: powered ? 1 : 0.4, pointerEvents: powered ? "auto" : "none", transition:"opacity 0.3s", minHeight:280 }}>
          {tab==="eq" && (
            <>
              <div style={{ display:"flex", justifyContent:"space-between", alignItems:"flex-end", gap:6, marginBottom:12 }}>
                {EQ_BANDS.map((freq,i) => <EQBand key={freq} freq={freq} value={eq[i]} onChange={v => updateEQBand(i, v)} disabled={!powered}/>)}
              </div>
              <div style={{ display:"flex", justifyContent:"space-between", paddingTop:12, borderTop:"1px solid #181818" }}>
                <span style={{ fontSize:9, color:"#666", fontWeight:500 }}>-12 dB</span>
                <span style={{ fontSize:9, color:"#666", fontWeight:600 }}>10-Band Parametric EQ</span>
                <span style={{ fontSize:9, color:"#666", fontWeight:500 }}>+12 dB</span>
              </div>
            </>
          )}
          {tab==="fx" && (
            <div style={{ paddingTop:8 }}>
              {[["Fidelity","fidelity"],["Ambiance","ambiance"],["Dynamic Boost","dynamic"],["3D Surround","surround"],["HyperBass","bass"]].map(([lbl,key]) => (
                <EffectSlider key={key} label={lbl} value={fx[key]} onChange={v => updateEffect(key, v)} disabled={!powered}/>
              ))}
            </div>
          )}
        </div>

        {/* Status bar */}
        <div style={{ background:"#000", borderTop:"1px solid #2b2b2b", padding:"10px 20px", display:"flex", alignItems:"center", justifyContent:"space-between" }}>
          <div style={{ display:"flex", alignItems:"center", gap:6 }}>
            <div style={{ width:8, height:8, borderRadius:"50%", background: powered ? "#28ca42" : "#666", boxShadow: powered ? "0 0 8px rgba(40,202,66,0.6)" : "none", transition:"all 0.3s" }}/>
            <span style={{ fontSize:10, color: powered ? "#28ca42" : "#666", letterSpacing:1, fontWeight:600 }}>{powered?"ACTIVE":"BYPASSED"}</span>
          </div>
          <span style={{ fontSize:10, color:"#666", fontWeight:500 }}>PulseAudio · 48kHz</span>
          <span style={{ fontSize:10, color:"#b1b1b1", fontWeight:600 }}>{preset.toUpperCase()}</span>
        </div>

      </div>
    </div>
  );
}
