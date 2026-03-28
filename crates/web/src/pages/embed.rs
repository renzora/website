use leptos::prelude::*;

/// Lightweight embed page — shows only the media preview for an asset.
/// No nav, no footer, no auth required. Used by the launcher and for sharing.
/// URL: /embed/preview/{slug}
#[component]
pub fn EmbedPreviewPage() -> impl IntoView {
    view! {
        <div id="embed-root" class="w-full h-full bg-[#060608] overflow-hidden">
            <div id="embed-loading" class="flex items-center justify-center h-screen">
                <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-indigo-500 rounded-full"></div>
            </div>
            <div id="embed-content" class="hidden w-full h-full"></div>
        </div>
        <style>
            "body { margin: 0; padding: 0; background: #060608; overflow: hidden; }
            nav, header, footer, .nav-bar { display: none !important; }
            main { padding: 0 !important; margin: 0 !important; min-height: 100vh !important; }
            * { box-sizing: border-box; }
            .gallery-thumb { cursor: pointer; }"
        </style>
        <script>
            r##"
            let galleryItems = [];
            let activeGalleryIndex = 0;

            (async function() {
                const slug = window.location.pathname.split('/').pop();
                const res = await fetch('/api/marketplace/detail/' + slug);
                if (!res.ok) {
                    document.getElementById('embed-loading').innerHTML = '<p class="text-zinc-500 text-sm">Preview not available</p>';
                    return;
                }
                const a = await res.json();

                // Fetch media
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

                // Render
                const mainPreviewHtml = renderMainPreview(galleryItems[0]);
                const thumbsHtml = galleryItems.length > 1 ? `
                    <div style="display:flex;gap:8px;margin-top:8px;overflow-x:auto;padding-bottom:4px" id="gallery-thumbs">
                        ${galleryItems.map((item, i) => {
                            const isVideo = item.type === 'video';
                            const isAudio = item.type === 'audio';
                            return `<button onclick="setGalleryItem(${i})" class="gallery-thumb" style="flex-shrink:0;width:64px;height:44px;border-radius:6px;border:2px solid ${i===0?'#6366f1':'#27272a'};overflow:hidden;position:relative;background:#18181b" data-index="${i}">
                                ${isAudio ? '<div style="display:flex;align-items:center;justify-content:center;width:100%;height:100%"><span style="color:#6366f1;font-size:16px">♫</span></div>' :
                                  isVideo ? '<div style="display:flex;align-items:center;justify-content:center;width:100%;height:100%"><span style="color:#a1a1aa;font-size:16px">▶</span></div>' :
                                  item.type === 'placeholder' ? '<div style="display:flex;align-items:center;justify-content:center;width:100%;height:100%;color:#52525b">⬜</div>' :
                                  `<img src="${item.thumb||item.url}" style="width:100%;height:100%;object-fit:cover" />`}
                            </button>`;
                        }).join('')}
                    </div>` : '';

                document.getElementById('embed-loading').classList.add('hidden');
                const el = document.getElementById('embed-content');
                el.classList.remove('hidden');
                el.innerHTML = `
                    <div style="width:100%;height:100%;display:flex;flex-direction:column">
                        <div style="flex:1;min-height:0;border-radius:8px;overflow:hidden;background:#18181b;position:relative" id="main-preview">
                            ${mainPreviewHtml}
                        </div>
                        ${thumbsHtml}
                    </div>
                `;

                // Start audio if first item is audio
                if (galleryItems[0].type === 'audio') initAudio(galleryItems[0].url);
            })();

            function renderMainPreview(item) {
                if (!item || item.type === 'placeholder') {
                    return '<div style="display:flex;align-items:center;justify-content:center;width:100%;height:100%;color:#52525b;font-size:48px">📦</div>';
                }
                if (item.type === 'audio') {
                    return `
                        <div style="width:100%;height:100%;position:relative;overflow:hidden">
                            ${item.cover ? `<img src="${item.cover}" style="position:absolute;inset:0;width:100%;height:100%;object-fit:cover;filter:blur(20px) brightness(0.3);transform:scale(1.2)" />` : ''}
                            ${item.cover ? `<img src="${item.cover}" style="position:absolute;top:50%;left:50%;transform:translate(-50%,-55%);max-width:60%;max-height:60%;object-fit:contain;border-radius:8px;box-shadow:0 8px 30px rgba(0,0,0,0.5)" />` : ''}
                            <div style="position:absolute;bottom:0;left:0;right:0;padding:12px 16px;background:linear-gradient(transparent,rgba(0,0,0,0.8))">
                                <canvas id="audio-wave" style="width:100%;height:40px;border-radius:4px"></canvas>
                                <div style="display:flex;align-items:center;gap:8px;margin-top:6px">
                                    <button onclick="toggleAudio()" id="audio-btn" style="background:none;border:none;color:white;font-size:18px;cursor:pointer">▶</button>
                                    <span id="audio-time" style="color:#a1a1aa;font-size:11px">0:00 / 0:00</span>
                                    <div style="flex:1;height:3px;background:#27272a;border-radius:2px;cursor:pointer;position:relative" onclick="seekAudio(event)" id="audio-progress">
                                        <div id="audio-bar" style="height:100%;background:#6366f1;border-radius:2px;width:0%;transition:width 0.1s"></div>
                                    </div>
                                </div>
                            </div>
                        </div>`;
                }
                if (item.type === 'video') {
                    const isYT = item.url.includes('youtube') || item.url.includes('youtu.be');
                    if (isYT) {
                        const vid = item.url.match(/(?:v=|youtu\.be\/)([^&]+)/)?.[1] || '';
                        return `<iframe src="https://www.youtube.com/embed/${vid}?autoplay=1&rel=0" style="width:100%;height:100%;border:none" allow="autoplay;encrypted-media" allowfullscreen></iframe>`;
                    }
                    return `<video src="${item.url}" style="width:100%;height:100%;object-fit:contain;background:#000" controls autoplay></video>`;
                }
                // Image
                return `<img src="${item.url}" style="width:100%;height:100%;object-fit:contain" />`;
            }

            function setGalleryItem(index) {
                activeGalleryIndex = index;
                const item = galleryItems[index];
                document.getElementById('main-preview').innerHTML = renderMainPreview(item);
                document.querySelectorAll('.gallery-thumb').forEach(t => {
                    t.style.borderColor = parseInt(t.dataset.index) === index ? '#6366f1' : '#27272a';
                });
                if (item.type === 'audio') initAudio(item.url);
            }

            // ── Audio Player ──
            let audioCtx, audioSource, audioAnalyser, audioBuffer, audioStartTime = 0, audioOffset = 0, audioPlaying = false;

            function initAudio(url) {
                audioPlaying = false;
                audioCtx = new (window.AudioContext || window.webkitAudioContext)();
                audioAnalyser = audioCtx.createAnalyser();
                audioAnalyser.fftSize = 256;
                audioAnalyser.connect(audioCtx.destination);
                fetch(url).then(r => r.arrayBuffer()).then(buf => audioCtx.decodeAudioData(buf)).then(decoded => {
                    audioBuffer = decoded;
                    const dur = decoded.duration;
                    document.getElementById('audio-time').textContent = `0:00 / ${Math.floor(dur/60)}:${String(Math.floor(dur%60)).padStart(2,'0')}`;
                    drawWaveform();
                }).catch(() => {});
            }

            function toggleAudio() {
                if (!audioBuffer) return;
                if (audioPlaying) {
                    audioSource?.stop();
                    audioOffset += audioCtx.currentTime - audioStartTime;
                    audioPlaying = false;
                    document.getElementById('audio-btn').textContent = '▶';
                } else {
                    audioSource = audioCtx.createBufferSource();
                    audioSource.buffer = audioBuffer;
                    audioSource.connect(audioAnalyser);
                    audioSource.start(0, audioOffset);
                    audioStartTime = audioCtx.currentTime;
                    audioPlaying = true;
                    document.getElementById('audio-btn').textContent = '⏸';
                    audioSource.onended = () => { if (audioPlaying) { audioPlaying = false; audioOffset = 0; document.getElementById('audio-btn').textContent = '▶'; } };
                    animateAudio();
                }
            }

            function seekAudio(e) {
                if (!audioBuffer) return;
                const rect = e.currentTarget.getBoundingClientRect();
                const pct = (e.clientX - rect.left) / rect.width;
                audioOffset = pct * audioBuffer.duration;
                if (audioPlaying) { audioSource?.stop(); audioPlaying = false; toggleAudio(); }
                updateAudioUI();
            }

            function animateAudio() {
                if (!audioPlaying) return;
                updateAudioUI();
                drawFrequency();
                requestAnimationFrame(animateAudio);
            }

            function updateAudioUI() {
                if (!audioBuffer) return;
                const elapsed = audioPlaying ? audioOffset + (audioCtx.currentTime - audioStartTime) : audioOffset;
                const dur = audioBuffer.duration;
                const pct = Math.min((elapsed / dur) * 100, 100);
                const bar = document.getElementById('audio-bar');
                if (bar) bar.style.width = pct + '%';
                const fmt = (s) => `${Math.floor(s/60)}:${String(Math.floor(s%60)).padStart(2,'0')}`;
                const timeEl = document.getElementById('audio-time');
                if (timeEl) timeEl.textContent = `${fmt(elapsed)} / ${fmt(dur)}`;
            }

            function drawWaveform() {
                const canvas = document.getElementById('audio-wave');
                if (!canvas || !audioBuffer) return;
                const ctx = canvas.getContext('2d');
                canvas.width = canvas.offsetWidth * 2;
                canvas.height = canvas.offsetHeight * 2;
                const data = audioBuffer.getChannelData(0);
                const step = Math.floor(data.length / canvas.width);
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                ctx.strokeStyle = 'rgba(99,102,241,0.3)';
                ctx.lineWidth = 1;
                ctx.beginPath();
                for (let i = 0; i < canvas.width; i++) {
                    const idx = i * step;
                    const v = Math.abs(data[idx] || 0);
                    const y = (1 - v) * canvas.height / 2;
                    i === 0 ? ctx.moveTo(i, y) : ctx.lineTo(i, y);
                }
                ctx.stroke();
            }

            function drawFrequency() {
                const canvas = document.getElementById('audio-wave');
                if (!canvas || !audioAnalyser) return;
                const ctx = canvas.getContext('2d');
                const bufLen = audioAnalyser.frequencyBinCount;
                const dataArr = new Uint8Array(bufLen);
                audioAnalyser.getByteFrequencyData(dataArr);
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                const barW = canvas.width / bufLen;
                for (let i = 0; i < bufLen; i++) {
                    const v = dataArr[i] / 255;
                    const h = v * canvas.height;
                    const hue = (i / bufLen) * 240;
                    ctx.fillStyle = `hsla(${hue}, 80%, 60%, 0.8)`;
                    ctx.fillRect(i * barW, canvas.height - h, barW - 1, h);
                }
            }
            "##
        </script>
    }
}
