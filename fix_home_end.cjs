const fs = require('fs');

function fixFile(file, homeVal, endVal) {
    let content = fs.readFileSync(file, 'utf8');
    content = content.replace(
        /(case 'Home':\n\s*newValue = )(-?\d+)(;\n\s*break;\n\s*case 'End':\n\s*newValue = )(-?\d+)(;\n\s*break;)/,
        `$1${homeVal}$3${endVal}$5`
    );
    fs.writeFileSync(file, content);
}

fixFile('src/components/EQBand.jsx', '-12', '12');
fixFile('src/components/EffectSlider.jsx', '0', '100');
