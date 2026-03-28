use leptos::prelude::*;

/// Embed preview page — renders the full preview (audio/video/WASM/gallery)
/// without nav, footer, or purchase UI. Used by the launcher iframe and for sharing.
/// Rendered via EmbedShell (no main app wrapper).
#[component]
pub fn EmbedPreviewPage() -> impl IntoView {
    view! {
        <div id="embed-root" class="w-full h-screen bg-[#060608] overflow-hidden flex flex-col">
            <div id="embed-loading" class="flex items-center justify-center h-screen">
                <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
            </div>
            <div id="embed-content" class="hidden w-full h-full flex flex-col"></div>
        </div>
        <script>
            r##"
            let galleryItems = [];
            let activeGalleryIndex = 0;

            (async function() {
                const root = document.getElementById('embed-root');
                const slug = root?.dataset?.slug || window.location.pathname.split('/').pop();
                const res = await fetch('/api/marketplace/detail/' + slug);
                if (!res.ok) {
                    document.getElementById('embed-loading').innerHTML = '<p class="text-zinc-500 text-sm">Preview not available</p>';
                    return;
                }
                const a = await res.json();

                const mediaRes = await fetch('/api/marketplace/' + a.id + '/media');
                const mediaData = mediaRes.ok ? await mediaRes.json() : [];

                // Build gallery
                galleryItems = [];
                const audioItems = mediaData.filter(m => m.media_type === 'audio');
                const otherItems = mediaData.filter(m => m.media_type !== 'audio');
                const coverImg = a.thumbnail_url || otherItems.find(m => m.media_type === 'image')?.url || '';

                if (audioItems.length > 0 && coverImg) {
                    audioItems.forEach(m => galleryItems.push({ type: 'audio', url: m.url, cover: coverImg }));
                    otherItems.forEach(m => galleryItems.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
                } else {
                    if (a.thumbnail_url) galleryItems.push({ type: 'image', url: a.thumbnail_url });
                    mediaData.forEach(m => galleryItems.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
                }
                if (!galleryItems.length) galleryItems.push({ type: 'placeholder' });

                const mainPreviewHtml = renderMainPreview(galleryItems[0]);
                const thumbsHtml = galleryItems.length > 1 ? `
                    <div class="flex gap-2 mt-2 overflow-x-auto pb-1 px-2" id="gallery-thumbs">
                        ${galleryItems.map((item, i) => {
                            const isVideo = item.type === 'video';
                            const isAudio = item.type === 'audio';
                            const thumbSrc = item.thumb || item.url;
                            return `<button onclick="setGalleryItem(${i})" class="gallery-thumb shrink-0 w-20 h-14 rounded-lg border-2 overflow-hidden relative transition-all ${i === 0 ? 'border-accent' : 'border-zinc-800/50 hover:border-zinc-600'}" data-index="${i}">
                                ${item.type === 'placeholder' ? '<div class="w-full h-full bg-zinc-800 flex items-center justify-center"><i class="ph ph-image text-zinc-600"></i></div>' :
                                  isAudio ? '<div class="w-full h-full bg-zinc-900 flex items-center justify-center"><i class="ph ph-music-note text-xl text-accent"></i></div>' :
                                  isVideo ? '<div class="w-full h-full bg-zinc-900 flex items-center justify-center"><i class="ph ph-play-circle text-xl text-zinc-400"></i></div>' :
                                  '<img src="' + thumbSrc + '" class="w-full h-full object-cover" />'}
                                ${isVideo ? '<div class="absolute bottom-0.5 right-0.5 bg-black/70 rounded px-1 text-[8px] text-white">VIDEO</div>' : ''}
                                ${isAudio ? '<div class="absolute bottom-0.5 right-0.5 bg-accent/80 rounded px-1 text-[8px] text-white">AUDIO</div>' : ''}
                            </button>`;
                        }).join('')}
                    </div>` : '';

                document.getElementById('embed-loading').classList.add('hidden');
                const el = document.getElementById('embed-content');
                el.classList.remove('hidden');

                // Check for live preview (WASM)
                const cat = (a.category || '').toLowerCase();
                const previewable = ['3d models', 'animations', 'materials & shaders', 'textures & hdris', 'particle effects'];
                const hasLivePreview = previewable.some(c => cat.includes(c.split(' ')[0]));

                if (hasLivePreview) {
                    let previewMode = 'shader';
                    if (cat.includes('3d') || cat.includes('model')) previewMode = 'model';
                    else if (cat.includes('anim')) previewMode = 'animation';
                    else if (cat.includes('material') || cat.includes('shader')) previewMode = 'shader';
                    else if (cat.includes('texture') || cat.includes('hdri')) previewMode = 'texture';
                    else if (cat.includes('particle')) previewMode = 'particle';

                    window.__previewConfig = { mode: previewMode, fileUrl: '/api/marketplace/' + a.id + '/preview-file', category: cat };

                    const meshBtns = ['sphere','cube','plane','torus'].map(s =>
                        '<button onclick="previewSetMesh(this.dataset.mesh)" class="px-2 py-0.5 rounded text-[11px] ' +
                        (s === 'cube' ? 'bg-accent/20 text-accent' : 'text-zinc-500 hover:text-zinc-300') +
                        ' transition-colors" data-mesh="' + s + '">' + s[0].toUpperCase() + s.slice(1) + '</button>'
                    ).join('');

                    el.innerHTML =
                        '<div id="live-preview-section" class="flex-1 min-h-0">' +
                            '<div class="flex items-center justify-between px-3 py-2">' +
                                '<div class="flex items-center gap-2">' +
                                    '<i class="ph ph-play-circle text-accent"></i>' +
                                    '<span class="text-sm font-medium text-zinc-300">Live Preview</span>' +
                                    '<span class="px-1.5 py-0.5 rounded bg-accent/10 border border-accent/20 text-[10px] text-accent font-medium">BETA</span>' +
                                '</div>' +
                                '<div class="flex items-center gap-1.5" id="preview-mesh-controls" style="display:' +
                                    (previewMode === 'shader' || previewMode === 'material' ? '' : 'none') + '">' +
                                    '<span class="text-[11px] text-zinc-600 mr-1">Mesh:</span>' +
                                    meshBtns +
                                '</div>' +
                            '</div>' +
                            '<div class="flex-1 min-h-0 overflow-hidden relative bg-[#0f0f13]">' +
                                '<canvas id="preview-canvas" class="w-full h-full"></canvas>' +
                                '<div id="preview-loading" class="absolute inset-0 flex items-center justify-center bg-[#0f0f13]">' +
                                    '<div class="text-center">' +
                                        '<div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full mb-2"></div>' +
                                        '<p class="text-xs text-zinc-600">Loading preview engine...</p>' +
                                    '</div>' +
                                '</div>' +
                            '</div>' +
                            '<div id="preview-params"></div>' +
                        '</div>';
                    initLivePreview(window.__previewConfig);
                } else {
                    el.innerHTML =
                        '<div class="flex-1 min-h-0 overflow-hidden rounded-xl m-2 bg-zinc-900 relative" id="main-preview">' +
                            mainPreviewHtml +
                        '</div>' + thumbsHtml;
                    setTimeout(() => {
                        if (galleryItems[0]?.type === 'audio') initAudioPlayer();
                        if (galleryItems[0]?.type === 'video') initVideoPlayer();
                    }, 300);
                }
            })();

            // ── renderMainPreview — exact copy from asset_detail ──
            function renderMainPreview(item) {
                if (!item || item.type === 'placeholder') {
                    return '<div class="aspect-video flex items-center justify-center"><i class="ph ph-package text-6xl text-zinc-700"></i></div>';
                }
                if (item.type === 'audio') {
                    const hasCover = item.cover;
                    const coverBg = hasCover
                        ? `<div class="absolute inset-0 bg-cover bg-center" style="background-image:url('${item.cover}')"></div><div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>`
                        : `<div class="absolute inset-0 bg-gradient-to-b from-zinc-900 to-[#0a0a0b]"></div>`;
                    return `
                        <div class="aspect-video flex flex-col items-end justify-end relative overflow-hidden">
                            ${coverBg}
                            <audio id="audio-player" src="${item.url}" preload="metadata" crossorigin="anonymous" class="hidden"></audio>
                            <div class="absolute inset-0 z-[5] flex items-center justify-center pointer-events-none">
                                <canvas id="waveform-canvas" class="w-[85%] h-24 opacity-80 pointer-events-auto cursor-pointer" onclick="seekWaveform(event)"></canvas>
                            </div>
                            <div class="relative z-10 w-full px-5 pb-4">
                                <div class="flex items-center gap-3 w-full bg-black/40 backdrop-blur-md rounded-xl px-4 py-2.5">
                                    <button onclick="toggleAudioPlay()" id="audio-play-btn" class="w-10 h-10 rounded-full bg-accent hover:bg-accent-hover text-white flex items-center justify-center transition-colors shrink-0 shadow-lg shadow-accent/20">
                                        <svg id="audio-icon-play" class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"></polygon></svg>
                                        <svg id="audio-icon-pause" class="w-5 h-5 hidden" viewBox="0 0 24 24" fill="currentColor"><rect x="5" y="3" width="4" height="18"></rect><rect x="15" y="3" width="4" height="18"></rect></svg>
                                    </button>
                                    <div class="flex-1 min-w-0">
                                        <div class="relative w-full h-1 bg-white/10 rounded-full cursor-pointer" onclick="seekAudio(event)" id="audio-seek-bar">
                                            <div id="audio-progress" class="absolute left-0 top-0 h-full bg-accent rounded-full transition-all" style="width:0%"></div>
                                        </div>
                                        <div class="flex justify-between mt-1.5">
                                            <span id="audio-current" class="text-[11px] text-white/60 tabular-nums">0:00</span>
                                            <span id="audio-duration" class="text-[11px] text-white/60 tabular-nums">0:00</span>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-1.5 shrink-0">
                                        <button onclick="toggleAudioVolume()" class="text-white/40 hover:text-white/80 transition-colors">
                                            <svg id="audio-vol-on" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11,5 6,9 2,9 2,15 6,15 11,19" fill="currentColor"></polygon><path d="M19.07 4.93a10 10 0 010 14.14M15.54 8.46a5 5 0 010 7.07"></path></svg>
                                            <svg id="audio-vol-off" class="w-4 h-4 hidden" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11,5 6,9 2,9 2,15 6,15 11,19" fill="currentColor"></polygon><line x1="23" y1="9" x2="17" y2="15"></line><line x1="17" y1="9" x2="23" y2="15"></line></svg>
                                        </button>
                                        <input type="range" min="0" max="100" value="100" id="audio-vol-slider" oninput="setAudioVolume(this.value)" class="w-16 h-1 accent-accent bg-white/10 rounded-full appearance-none cursor-pointer" />
                                    </div>
                                </div>
                            </div>
                        </div>`;
                }
                if (item.type === 'video') {
                    if (item.url.includes('youtube.com') || item.url.includes('youtu.be')) {
                        const vid = item.url.match(/(?:v=|youtu\.be\/)([a-zA-Z0-9_-]+)/)?.[1];
                        return vid ? `<div class="aspect-video"><iframe src="https://www.youtube.com/embed/${vid}?autoplay=1" class="w-full h-full" frameborder="0" allowfullscreen></iframe></div>` :
                            `<div class="aspect-video flex items-center justify-center text-zinc-600">Invalid video</div>`;
                    }
                    return `
                        <div class="aspect-video relative bg-black group/vp" id="video-container">
                            <video id="video-player" src="${item.url}" ${item.thumb ? `poster="${item.thumb}"` : ''} preload="metadata" class="w-full h-full object-contain" onclick="toggleVideoPlay()" ondblclick="toggleVideoFullscreen()"></video>
                            <div id="video-big-play" class="absolute inset-0 flex items-center justify-center cursor-pointer" onclick="toggleVideoPlay()">
                                <div class="w-16 h-16 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center hover:bg-accent/80 transition-colors">
                                    <svg class="w-7 h-7 text-white ml-1" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"></polygon></svg>
                                </div>
                            </div>
                            <div id="video-controls" class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/80 to-transparent pt-10 pb-3 px-4 opacity-0 group-hover/vp:opacity-100 transition-opacity">
                                <div class="relative w-full h-1 bg-white/10 rounded-full cursor-pointer mb-3" onclick="seekVideo(event)">
                                    <div id="video-buffered" class="absolute left-0 top-0 h-full bg-white/10 rounded-full" style="width:0%"></div>
                                    <div id="video-progress" class="absolute left-0 top-0 h-full bg-accent rounded-full" style="width:0%"></div>
                                </div>
                                <div class="flex items-center gap-3">
                                    <button onclick="toggleVideoPlay()" class="text-white/80 hover:text-white">
                                        <svg id="video-icon-play" class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"></polygon></svg>
                                        <svg id="video-icon-pause" class="w-5 h-5 hidden" viewBox="0 0 24 24" fill="currentColor"><rect x="5" y="3" width="4" height="18"></rect><rect x="15" y="3" width="4" height="18"></rect></svg>
                                    </button>
                                    <span id="video-time" class="text-[11px] text-white/60 tabular-nums">0:00 / 0:00</span>
                                    <div class="flex-1"></div>
                                    <button onclick="toggleVideoFullscreen()" class="text-white/60 hover:text-white">
                                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3"></path></svg>
                                    </button>
                                </div>
                            </div>
                        </div>`;
                }
                return `<img src="${item.url}" class="w-full aspect-video object-cover" />`;
            }

            function setGalleryItem(index) {
                const oldAudio = document.getElementById('audio-player');
                if (oldAudio) oldAudio.pause();
                activeGalleryIndex = index;
                document.getElementById('main-preview').innerHTML = renderMainPreview(galleryItems[index]);
                if (galleryItems[index].type === 'audio') initAudioPlayer();
                if (galleryItems[index].type === 'video') initVideoPlayer();
                document.querySelectorAll('.gallery-thumb').forEach(el => {
                    const i = parseInt(el.dataset.index);
                    el.className = el.className.replace(/border-accent|border-zinc-800\/50/g, '');
                    el.classList.add(i === index ? 'border-accent' : 'border-zinc-800/50');
                });
            }

            // ── Audio Player with Frequency Analyser ──
            let audioCtx = null, analyser = null, animFrameId = null, analyserReady = false;

            function initAudioPlayer() {
                const audio = document.getElementById('audio-player');
                if (!audio) return;
                audio.addEventListener('loadedmetadata', () => { const d = document.getElementById('audio-duration'); if (d && audio.duration && isFinite(audio.duration)) d.textContent = fmtTime(audio.duration); });
                audio.addEventListener('timeupdate', () => { const c = document.getElementById('audio-current'); const p = document.getElementById('audio-progress'); if (c) c.textContent = fmtTime(audio.currentTime); if (p && audio.duration) p.style.width = ((audio.currentTime / audio.duration) * 100) + '%'; });
                audio.addEventListener('ended', () => { document.getElementById('audio-icon-play')?.classList.remove('hidden'); document.getElementById('audio-icon-pause')?.classList.add('hidden'); cancelAnimationFrame(animFrameId); drawIdleWaveform(); });
                drawIdleWaveform();
            }

            function drawCanvas() { const c = document.getElementById('waveform-canvas'); if (!c) return null; const ctx = c.getContext('2d'); const dpr = window.devicePixelRatio || 1; const w = c.offsetWidth; const h = c.offsetHeight; c.width = w * dpr; c.height = h * dpr; ctx.scale(dpr, dpr); ctx.clearRect(0, 0, w, h); return { ctx, w, h }; }

            function drawIdleWaveform() { const c = drawCanvas(); if (!c) return; const { ctx, w, h } = c; const barW = 2, gap = 1.5, step = barW + gap; const bars = Math.floor(w / step); const mid = h / 2; for (let i = 0; i < bars; i++) { const barH = 3 + Math.sin(i * 0.12) * 2; ctx.fillStyle = 'rgba(255,255,255,0.06)'; ctx.beginPath(); ctx.roundRect(i * step, mid - barH / 2, barW, barH, 1); ctx.fill(); } }

            function connectAnalyser() { if (analyserReady) return true; const audio = document.getElementById('audio-player'); if (!audio) return false; try { audioCtx = new (window.AudioContext || window.webkitAudioContext)(); analyser = audioCtx.createAnalyser(); analyser.fftSize = 1024; analyser.smoothingTimeConstant = 0.6; analyser.minDecibels = -70; analyser.maxDecibels = -5; const src = audioCtx.createMediaElementSource(audio); src.connect(analyser); analyser.connect(audioCtx.destination); analyserReady = true; return true; } catch(e) { return false; } }

            function drawWaveformFrame() {
                const audio = document.getElementById('audio-player');
                if (!audio || audio.paused || audio.ended) return;
                const c = drawCanvas(); if (!c) return;
                const { ctx, w, h } = c;
                const barW = 2, gap = 1.5, step = barW + gap;
                const bars = Math.floor(w / step);
                const mid = h / 2;
                if (analyserReady && analyser) {
                    const bufLen = analyser.frequencyBinCount;
                    const freqData = new Uint8Array(bufLen);
                    analyser.getByteFrequencyData(freqData);
                    for (let i = 0; i < bars; i++) {
                        const freqIdx = Math.min(Math.floor(Math.pow(i / bars, 1.4) * bufLen), bufLen - 1);
                        const val = freqData[freqIdx] / 255;
                        const barH = Math.max(2, val * h * 0.9);
                        let r, g, b;
                        if (val < 0.3) { const t = val / 0.3; r = Math.round(59 + 40 * t); g = Math.round(130 - 28 * t); b = Math.round(246 - 5 * t); }
                        else if (val < 0.6) { const t = (val - 0.3) / 0.3; r = Math.round(99 + 121 * t); g = Math.round(102 - 42 * t); b = Math.round(241 - 41 * t); }
                        else { const t = (val - 0.6) / 0.4; r = Math.round(220 + 35 * t); g = Math.round(60 - 10 * t); b = Math.round(200 - 120 * t); }
                        ctx.fillStyle = `rgba(${r},${g},${b},${0.15 + val * 0.85})`;
                        ctx.beginPath(); ctx.roundRect(i * step, mid - barH / 2, barW, barH, 1); ctx.fill();
                    }
                } else {
                    const t = audio.currentTime; const vol = audio.volume;
                    for (let i = 0; i < bars; i++) {
                        const wave = Math.sin(i * 0.08 + t * 3.5) * 0.3 + Math.sin(i * 0.05 + t * 5.5) * 0.25 + Math.sin(i * 0.15 + t * 2) * 0.25 + Math.sin(i * 0.22 + t * 7) * 0.2;
                        const amp = Math.max(0, Math.min(1, (wave + 1) / 2)) * vol;
                        const barH = Math.max(2, amp * h * 0.8);
                        ctx.fillStyle = `rgba(99,102,241,${0.15 + amp * 0.85})`;
                        ctx.beginPath(); ctx.roundRect(i * step, mid - barH / 2, barW, barH, 1); ctx.fill();
                    }
                }
                if (audio.duration) { const px = (audio.currentTime / audio.duration) * w; ctx.fillStyle = 'rgba(255,255,255,0.5)'; ctx.fillRect(px - 0.5, 0, 1, h); }
                animFrameId = requestAnimationFrame(drawWaveformFrame);
            }

            function seekWaveform(e) { const audio = document.getElementById('audio-player'); if (!audio || !audio.duration) return; const rect = e.currentTarget.getBoundingClientRect(); audio.currentTime = ((e.clientX - rect.left) / rect.width) * audio.duration; }
            function fmtTime(sec) { if (!sec || isNaN(sec)) return '0:00'; const m = Math.floor(sec / 60); const s = Math.floor(sec % 60); return m + ':' + (s < 10 ? '0' : '') + s; }

            function toggleAudioPlay() {
                const audio = document.getElementById('audio-player'); if (!audio) return;
                if (!analyserReady) connectAnalyser();
                if (audioCtx && audioCtx.state === 'suspended') audioCtx.resume();
                if (audio.paused) { audio.play().then(() => { document.getElementById('audio-icon-play')?.classList.add('hidden'); document.getElementById('audio-icon-pause')?.classList.remove('hidden'); drawWaveformFrame(); }).catch(() => {}); }
                else { audio.pause(); document.getElementById('audio-icon-play')?.classList.remove('hidden'); document.getElementById('audio-icon-pause')?.classList.add('hidden'); cancelAnimationFrame(animFrameId); }
            }
            function seekAudio(e) { const audio = document.getElementById('audio-player'); if (!audio || !audio.duration) return; const rect = e.currentTarget.getBoundingClientRect(); audio.currentTime = ((e.clientX - rect.left) / rect.width) * audio.duration; }
            function toggleAudioVolume() { const audio = document.getElementById('audio-player'); if (!audio) return; audio.muted = !audio.muted; document.getElementById('audio-vol-on')?.classList.toggle('hidden', audio.muted); document.getElementById('audio-vol-off')?.classList.toggle('hidden', !audio.muted); }
            function setAudioVolume(val) { const audio = document.getElementById('audio-player'); if (!audio) return; audio.volume = val / 100; audio.muted = val == 0; document.getElementById('audio-vol-on')?.classList.toggle('hidden', audio.muted); document.getElementById('audio-vol-off')?.classList.toggle('hidden', !audio.muted); }

            // ── Video Player ──
            function initVideoPlayer() { const v = document.getElementById('video-player'); if (!v) return; v.addEventListener('loadedmetadata', () => updateVideoTime()); v.addEventListener('timeupdate', () => { if (!v.duration) return; document.getElementById('video-progress').style.width = ((v.currentTime / v.duration) * 100) + '%'; updateVideoTime(); }); v.addEventListener('progress', () => { if (v.buffered.length > 0) document.getElementById('video-buffered').style.width = ((v.buffered.end(v.buffered.length - 1) / v.duration) * 100) + '%'; }); v.addEventListener('play', () => { document.getElementById('video-big-play')?.classList.add('hidden'); document.getElementById('video-icon-play')?.classList.add('hidden'); document.getElementById('video-icon-pause')?.classList.remove('hidden'); }); v.addEventListener('pause', () => { document.getElementById('video-icon-play')?.classList.remove('hidden'); document.getElementById('video-icon-pause')?.classList.add('hidden'); }); v.addEventListener('ended', () => { document.getElementById('video-big-play')?.classList.remove('hidden'); document.getElementById('video-icon-play')?.classList.remove('hidden'); document.getElementById('video-icon-pause')?.classList.add('hidden'); }); }
            function updateVideoTime() { const v = document.getElementById('video-player'); if (!v) return; const el = document.getElementById('video-time'); if (el) el.textContent = fmtTime(v.currentTime) + ' / ' + fmtTime(v.duration); }
            function toggleVideoPlay() { const v = document.getElementById('video-player'); if (!v) return; if (v.paused) v.play(); else v.pause(); }
            function seekVideo(e) { const v = document.getElementById('video-player'); if (!v || !v.duration) return; const rect = e.currentTarget.getBoundingClientRect(); v.currentTime = ((e.clientX - rect.left) / rect.width) * v.duration; }
            function toggleVideoFullscreen() { const c = document.getElementById('video-container'); if (!c) return; if (document.fullscreenElement) document.exitFullscreen(); else c.requestFullscreen().catch(() => {}); }

            // ── WASM Live Preview ──
            window.previewSetMesh = function(shape) { if (window.__previewWasm) window.__previewWasm.preview_set_mesh(shape); document.querySelectorAll('[data-mesh]').forEach(el => { if (el.dataset.mesh === shape) { el.className = el.className.replace('text-zinc-500 hover:text-zinc-300','').replace('text-zinc-500',''); el.classList.add('bg-accent/20','text-accent'); } else { el.className = el.className.replace('bg-accent/20','').replace('text-accent',''); el.classList.add('text-zinc-500'); } }); };
            window.previewSetParam = function(name, jsonValue) { if (window.__previewWasm) window.__previewWasm.preview_set_param(name, jsonValue); };
            window.previewSetColor = function(name, hex) { var r = parseInt(hex.slice(1,3),16)/255; var g = parseInt(hex.slice(3,5),16)/255; var b = parseInt(hex.slice(5,7),16)/255; if (window.__previewWasm) window.__previewWasm.preview_set_param(name, JSON.stringify({type:'Color',value:[r,g,b,1.0]})); };

            function buildPreviewParamUI(params, wasm) {
                var container = document.getElementById('preview-params');
                if (!container || !Object.keys(params).length) return;
                var html = '<div class="grid grid-cols-2 gap-2 p-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl mx-2 mt-2">';
                for (var name in params) {
                    var p = params[name];
                    if (p.param_type === 'Float') { var def = p.default_value?.Float ?? 0; var mn = p.min ?? 0; var mx = p.max ?? 10; html += '<div class="flex items-center gap-2"><label class="text-[11px] text-zinc-500 w-20 shrink-0 truncate">'+name+'</label><input type="range" min="'+mn+'" max="'+mx+'" step="0.01" value="'+def+'" oninput="previewSetParam(\''+name+'\', JSON.stringify({type:\'Float\',value:parseFloat(this.value)}))" class="flex-1 h-1 accent-accent" /></div>'; }
                    else if (p.param_type === 'Color') { var c = p.default_value?.Color ?? [1,1,1,1]; var hex = '#' + [c[0],c[1],c[2]].map(v => Math.round(v*255).toString(16).padStart(2,'0')).join(''); html += '<div class="flex items-center gap-2"><label class="text-[11px] text-zinc-500 w-20 shrink-0">'+name+'</label><input type="color" value="'+hex+'" oninput="previewSetColor(\''+name+'\', this.value)" class="w-6 h-6 border-0 bg-transparent cursor-pointer" /></div>'; }
                }
                html += '</div>';
                container.innerHTML = html;
            }

            async function initLivePreview(config) { doInitPreview(config); }
            async function doInitPreview(config) {
                try {
                    var wasm = await import('/assets/wasm/renzora_preview.js');
                    await wasm.default(); wasm.preview_init();
                    await new Promise(r => setTimeout(r, 500));
                    if (config.mode === 'shader' && config.fileUrl) { var res = await fetch(config.fileUrl); if (res.ok) { var source = await res.text(); wasm.preview_load_shader(source, 'Fragment'); var params = JSON.parse(wasm.preview_extract_params(source) || '{}'); buildPreviewParamUI(params, wasm); } }
                    else if (config.mode === 'model') wasm.preview_load_model(config.fileUrl);
                    else if (config.mode === 'animation') wasm.preview_load_animation(config.fileUrl);
                    else if (config.mode === 'particle') { var r2 = await fetch(config.fileUrl); if (r2.ok) wasm.preview_load_particle(await r2.text()); }
                    else if (config.mode === 'texture') wasm.preview_load_texture(config.fileUrl, config.category.includes('hdri') ? 'hdri' : 'texture');
                    var loading = document.getElementById('preview-loading'); if (loading) loading.remove();
                    window.__previewWasm = wasm;
                } catch (err) { console.warn('[preview]', err); var sec = document.getElementById('live-preview-section'); if (sec) sec.innerHTML = '<div class="flex items-center justify-center h-full text-zinc-600 text-sm">Preview not available</div>'; }
            }

            // Report height to parent for iframe auto-resize
            function reportHeight() {
                const h = document.documentElement.scrollHeight;
                window.parent.postMessage({ type: 'embed-resize', height: h }, '*');
            }
            new MutationObserver(reportHeight).observe(document.body, { childList: true, subtree: true, attributes: true });
            setInterval(reportHeight, 1000);
            "##
        </script>
        <style>
            ".gallery-thumb { transition: border-color 0.2s, transform 0.2s; }
            .gallery-thumb:hover { transform: scale(1.05); }"
        </style>
    }
}
