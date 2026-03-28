// Renzora Embed Preview Player
// Standalone — no framework dependencies

let items = [];
let activeIdx = 0;
let audioCtx, audioSrc, analyser, audioBuf;
let startT = 0, offset = 0, playing = false;

(async function () {
    const slug = window.location.pathname.split('/').pop();
    if (!slug) return showError();

    try {
        const res = await fetch('/api/marketplace/detail/' + slug);
        if (!res.ok) return showError();
        const asset = await res.json();

        const mRes = await fetch('/api/marketplace/' + asset.id + '/media');
        const media = mRes.ok ? await mRes.json() : [];

        buildGallery(asset, media);
    } catch {
        showError();
    }
})();

function showError() {
    document.getElementById('root').innerHTML = '<div class="error">Preview not available</div>';
}

function buildGallery(asset, media) {
    items = [];
    const audioFiles = media.filter(m => m.media_type === 'audio');
    const otherFiles = media.filter(m => m.media_type !== 'audio');
    const cover = asset.thumbnail_url || otherFiles.find(m => m.media_type === 'image')?.url || '';

    if (audioFiles.length > 0 && cover) {
        audioFiles.forEach(m => items.push({ type: 'audio', url: m.url, cover }));
        otherFiles.forEach(m => items.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
    } else {
        if (asset.thumbnail_url) items.push({ type: 'image', url: asset.thumbnail_url });
        media.forEach(m => items.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
    }
    if (!items.length) items.push({ type: 'placeholder' });

    // Render
    const thumbsHtml = items.length > 1
        ? '<div id="thumbs">' + items.map((it, i) => {
            const cls = 'thumb' + (i === 0 ? ' active' : '');
            if (it.type === 'audio') return `<button class="${cls}" onclick="setItem(${i})"><span style="color:#6366f1;font-size:14px">\u266b</span></button>`;
            if (it.type === 'video') return `<button class="${cls}" onclick="setItem(${i})"><span style="color:#a1a1aa;font-size:14px">\u25b6</span></button>`;
            if (it.type === 'placeholder') return `<button class="${cls}" onclick="setItem(${i})">?</button>`;
            return `<button class="${cls}" onclick="setItem(${i})"><img src="${it.thumb || it.url}"/></button>`;
        }).join('') + '</div>'
        : '';

    document.getElementById('root').innerHTML =
        '<div id="preview">' + renderItem(items[0]) + '</div>' + thumbsHtml;

    if (items[0].type === 'audio') initAudio(items[0].url);
}

function renderItem(it) {
    if (!it || it.type === 'placeholder') {
        return '<div class="placeholder">\ud83d\udce6</div>';
    }
    if (it.type === 'audio') {
        let html = '<div style="width:100%;height:100%;position:relative;overflow:hidden">';
        if (it.cover) {
            html += `<img class="cover-blur" src="${it.cover}"/>`;
            html += `<img class="cover-art" src="${it.cover}"/>`;
        }
        html += `<div id="controls">
            <canvas id="wave"></canvas>
            <div class="ctrl-row">
                <button id="play-btn" onclick="toggle()">\u25b6</button>
                <span id="time">0:00 / 0:00</span>
                <div id="progress" onclick="seek(event)"><div id="bar"></div></div>
            </div>
        </div></div>`;
        return html;
    }
    if (it.type === 'video') {
        const yt = it.url.match(/(?:v=|youtu\.be\/)([^&]+)/);
        if (yt) return `<iframe src="https://www.youtube.com/embed/${yt[1]}?autoplay=1&rel=0" style="width:100%;height:100%;border:none" allow="autoplay;encrypted-media" allowfullscreen></iframe>`;
        return `<video src="${it.url}" style="width:100%;height:100%;object-fit:contain;background:#000" controls autoplay></video>`;
    }
    return `<img src="${it.url}" style="width:100%;height:100%;object-fit:contain"/>`;
}

function setItem(i) {
    if (playing) { audioSrc?.stop(); playing = false; }
    activeIdx = i;
    document.getElementById('preview').innerHTML = renderItem(items[i]);
    document.querySelectorAll('.thumb').forEach((t, j) => t.classList.toggle('active', j === i));
    if (items[i].type === 'audio') initAudio(items[i].url);
}

// ── Audio Player ──

function initAudio(url) {
    playing = false;
    offset = 0;
    audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    analyser = audioCtx.createAnalyser();
    analyser.fftSize = 256;
    analyser.connect(audioCtx.destination);

    fetch(url)
        .then(r => r.arrayBuffer())
        .then(b => audioCtx.decodeAudioData(b))
        .then(decoded => {
            audioBuf = decoded;
            document.getElementById('time').textContent = '0:00 / ' + fmt(decoded.duration);
            drawWave();
        })
        .catch(() => {});
}

function toggle() {
    if (!audioBuf) return;
    if (playing) {
        audioSrc?.stop();
        offset += audioCtx.currentTime - startT;
        playing = false;
        document.getElementById('play-btn').textContent = '\u25b6';
    } else {
        audioSrc = audioCtx.createBufferSource();
        audioSrc.buffer = audioBuf;
        audioSrc.connect(analyser);
        audioSrc.start(0, offset);
        startT = audioCtx.currentTime;
        playing = true;
        document.getElementById('play-btn').textContent = '\u23f8';
        audioSrc.onended = () => {
            if (playing) { playing = false; offset = 0; document.getElementById('play-btn').textContent = '\u25b6'; }
        };
        anim();
    }
}

function seek(e) {
    if (!audioBuf) return;
    const r = e.currentTarget.getBoundingClientRect();
    offset = (e.clientX - r.left) / r.width * audioBuf.duration;
    if (playing) { audioSrc?.stop(); playing = false; toggle(); }
    updateUI();
}

function anim() {
    if (!playing) return;
    updateUI();
    drawFreq();
    requestAnimationFrame(anim);
}

function updateUI() {
    if (!audioBuf) return;
    const elapsed = playing ? offset + (audioCtx.currentTime - startT) : offset;
    const dur = audioBuf.duration;
    const bar = document.getElementById('bar');
    if (bar) bar.style.width = Math.min(elapsed / dur * 100, 100) + '%';
    const t = document.getElementById('time');
    if (t) t.textContent = fmt(elapsed) + ' / ' + fmt(dur);
}

function fmt(s) {
    return Math.floor(s / 60) + ':' + String(Math.floor(s % 60)).padStart(2, '0');
}

function drawWave() {
    const c = document.getElementById('wave');
    if (!c || !audioBuf) return;
    const ctx = c.getContext('2d');
    c.width = c.offsetWidth * 2;
    c.height = c.offsetHeight * 2;
    const data = audioBuf.getChannelData(0);
    const step = Math.floor(data.length / c.width);
    ctx.clearRect(0, 0, c.width, c.height);
    ctx.strokeStyle = 'rgba(99,102,241,.3)';
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (let i = 0; i < c.width; i++) {
        const v = Math.abs(data[i * step] || 0);
        const y = (1 - v) * c.height / 2;
        i === 0 ? ctx.moveTo(i, y) : ctx.lineTo(i, y);
    }
    ctx.stroke();
}

function drawFreq() {
    const c = document.getElementById('wave');
    if (!c || !analyser) return;
    const ctx = c.getContext('2d');
    const n = analyser.frequencyBinCount;
    const arr = new Uint8Array(n);
    analyser.getByteFrequencyData(arr);
    ctx.clearRect(0, 0, c.width, c.height);
    const w = c.width / n;
    for (let i = 0; i < n; i++) {
        const v = arr[i] / 255;
        const h = v * c.height;
        ctx.fillStyle = `hsla(${(i / n) * 240}, 80%, 60%, .8)`;
        ctx.fillRect(i * w, c.height - h, w - 1, h);
    }
}
