use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <script type="importmap">
            r#"{"imports":{"three":"https://unpkg.com/three@0.170.0/build/three.module.js","three/addons/":"https://unpkg.com/three@0.170.0/examples/jsm/"}}"#
        </script>

        <section class="h-screen -mt-14 pt-14 relative overflow-hidden">
            <canvas id="lobby-canvas" class="absolute inset-0 w-full h-full block z-0"></canvas>

            // Auth card
            <div class="absolute inset-0 z-10 flex items-center justify-center">
                <div id="lobby-auth" class="w-[380px] bg-[rgba(8,8,14,0.75)] backdrop-blur-3xl border border-white/[0.1] rounded-3xl shadow-2xl shadow-black/60 p-8">
                    <div class="text-center mb-6">
                        <h1 class="text-xl font-bold tracking-tight">"Renzora"</h1>
                        <p class="text-zinc-500 text-sm mt-1" id="lobby-subtitle">"Sign in to continue"</p>
                    </div>

                    <div id="lobby-error" class="hidden mb-4 p-3 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-xs"></div>

                    // Login form
                    <form id="lobby-login" class="flex flex-col gap-3" onsubmit="return window._doLogin(event)">
                        <input type="email" name="email" required placeholder="Email" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <input type="password" name="password" required placeholder="Password" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <button type="submit" id="lobby-login-btn" class="w-full mt-1 py-2.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all">"Sign In"</button>
                    </form>

                    // Register form
                    <form id="lobby-register" class="hidden flex flex-col gap-3" onsubmit="return window._doRegister(event)">
                        <input type="text" name="username" required minlength="3" maxlength="32" placeholder="Username" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <input type="email" name="email" required placeholder="Email" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <input type="password" name="password" required minlength="8" placeholder="Password" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <input type="password" name="confirm_password" required placeholder="Confirm password" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <button type="submit" id="lobby-register-btn" class="w-full mt-1 py-2.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all">"Create Account"</button>
                    </form>

                    <p class="text-center text-xs text-zinc-500 mt-5">
                        <span id="lobby-toggle-text">"Don't have an account? "</span>
                        <button onclick="window._toggleAuth()" class="text-accent hover:text-accent-hover" id="lobby-toggle-btn">"Register"</button>
                    </p>
                </div>
            </div>
        </section>

        <script type="module">
            r##"
            import * as THREE from 'three';
            import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

            // ── Check if already logged in ──
            const existingToken = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (existingToken) {
                const redirect = new URLSearchParams(window.location.search).get('redirect') || '/';
                window.location.href = redirect;
            }

            // ── 3D Lobby Scene ──
            const canvas = document.getElementById('lobby-canvas');
            const w = canvas.clientWidth || window.innerWidth, h = canvas.clientHeight || window.innerHeight;
            const SUN_DIR = new THREE.Vector3(0.6, 0.35, -0.7).normalize();
            const scene = new THREE.Scene();
            scene.fog = new THREE.FogExp2(0x9ac4e0, 0.018);

            const camera = new THREE.PerspectiveCamera(40, w/h, 0.1, 80);
            camera.position.set(0, 1.8, 5);

            const renderer = new THREE.WebGLRenderer({canvas, antialias:true});
            renderer.setSize(w,h);
            renderer.setPixelRatio(Math.min(window.devicePixelRatio,2));
            renderer.shadowMap.enabled = true;
            renderer.shadowMap.type = THREE.PCFSoftShadowMap;
            renderer.outputColorSpace = THREE.SRGBColorSpace;
            renderer.toneMapping = THREE.ACESFilmicToneMapping;
            renderer.toneMappingExposure = 1.15;

            // Lighting — warm summer afternoon
            scene.add(new THREE.HemisphereLight(0x88ccff, 0x446633, 0.65));
            scene.add(new THREE.AmbientLight(0xffeedd, 0.3));
            const sunPos = SUN_DIR.clone().multiplyScalar(30);
            const sun = new THREE.DirectionalLight(0xfff4dd, 1.8);
            sun.position.copy(sunPos); sun.castShadow=true;
            sun.shadow.mapSize.set(2048,2048); sun.shadow.bias=-0.0003;
            sun.shadow.camera.left=-15;sun.shadow.camera.right=15;sun.shadow.camera.top=15;sun.shadow.camera.bottom=-15;sun.shadow.camera.far=50;
            scene.add(sun);
            scene.add(new THREE.DirectionalLight(0xaaccff, 0.2).translateX(-5).translateY(3).translateZ(6));

            // ── Sky dome with clouds + sun ──
            const skyMat = new THREE.ShaderMaterial({
                side: THREE.BackSide,
                uniforms: { uTime:{value:0}, uSunDir:{value:SUN_DIR} },
                vertexShader: 'uniform float uTime; varying vec3 vDir; void main(){ vDir=position; gl_Position=projectionMatrix*modelViewMatrix*vec4(position,1.0); }',
                fragmentShader: `
                    uniform float uTime; uniform vec3 uSunDir; varying vec3 vDir;
                    float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}
                    float sn(vec2 p){vec2 i=floor(p),f=fract(p);f=f*f*(3.0-2.0*f);return mix(mix(hash(i),hash(i+vec2(1,0)),f.x),mix(hash(i+vec2(0,1)),hash(i+vec2(1,1)),f.x),f.y);}
                    float fbm(vec2 p){float v=0.0,a=0.5;for(int i=0;i<5;i++){v+=a*sn(p);p*=2.0;a*=0.5;}return v;}
                    void main(){
                        vec3 d=normalize(vDir); float h=d.y;
                        vec3 zen=vec3(0.22,0.42,0.82); vec3 hor=vec3(0.65,0.78,0.92); vec3 haze=vec3(0.75,0.82,0.88);
                        vec3 col=h>0.0?mix(hor,zen,pow(h,0.45)):mix(hor,haze,min(-h*5.0,1.0));
                        // Atmospheric scattering near horizon
                        col=mix(col,haze,exp(-abs(h)*4.0)*0.5);
                        // Clouds
                        if(h>-0.05){
                            vec2 uv=d.xz/(h+0.25)*3.0+uTime*0.01;
                            float c1=fbm(uv*1.2); float c2=fbm(uv*2.5+5.0);
                            float cloud=smoothstep(0.4,0.7,c1)*0.7+smoothstep(0.45,0.75,c2)*0.3;
                            cloud*=smoothstep(-0.05,0.1,h);
                            vec3 lit=vec3(1.0,0.98,0.95); vec3 shd=vec3(0.65,0.7,0.8);
                            float sunLight=dot(d,uSunDir)*0.5+0.5;
                            col=mix(col,mix(shd,lit,sunLight*0.6+0.4),cloud*0.85);
                        }
                        // Sun disc + glare
                        float sd=dot(d,uSunDir);
                        col+=vec3(1.0,0.95,0.8)*pow(max(sd,0.0),256.0)*5.0; // hard disc
                        col+=vec3(1.0,0.9,0.7)*pow(max(sd,0.0),32.0)*0.4; // inner glow
                        col+=vec3(1.0,0.8,0.5)*pow(max(sd,0.0),4.0)*0.12; // wide haze
                        // Lens flare streaks
                        col+=vec3(1.0,0.85,0.6)*pow(max(sd,0.0),64.0)*0.8;
                        gl_FragColor=vec4(col,1.0);
                    }
                `
            });
            scene.add(new THREE.Mesh(new THREE.SphereGeometry(40,32,20),skyMat));

            // Perlin noise
            const perm=new Uint8Array(512);
            const p0=[151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168,68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180];
            for(let i=0;i<256;i++){perm[i]=p0[i];perm[i+256]=p0[i];}
            function fade(t){return t*t*t*(t*(t*6-15)+10);}
            function lerp(a,b,t){return a+t*(b-a);}
            function grad(hash,x,y){const h=hash&3;return((h&1)?-x:x)+((h&2)?-y:y);}
            function perlin(x,y){
                const X=Math.floor(x)&255, Y=Math.floor(y)&255;
                const xf=x-Math.floor(x), yf=y-Math.floor(y);
                const u=fade(xf), v=fade(yf);
                const a=perm[X]+Y, b=perm[X+1]+Y;
                return lerp(lerp(grad(perm[a],xf,yf),grad(perm[b],xf-1,yf),u),lerp(grad(perm[a+1],xf,yf-1),grad(perm[b+1],xf-1,yf-1),u),v);
            }
            function fbmTerrain(x,z){
                let v=0, amp=1, freq=0.15, total=0;
                for(let i=0;i<5;i++){v+=perlin(x*freq,z*freq)*amp;total+=amp;freq*=2.1;amp*=0.45;}
                return v/total*0.8;
            }
            function th(x,z){return fbmTerrain(x,z);}
            const WATER_LEVEL = -0.25;

            // ── Terrain with hi-res procedural texture ──
            const geo = new THREE.PlaneGeometry(50,50,150,150);
            const gp = geo.attributes.position;
            for(let i=0;i<gp.count;i++){gp.setZ(i, th(gp.getX(i),gp.getY(i)));}
            geo.computeVertexNormals();

            // Generate terrain texture on canvas
            const TEX_SIZE = 1024;
            const texCanvas = document.createElement('canvas');
            texCanvas.width = TEX_SIZE; texCanvas.height = TEX_SIZE;
            const tc = texCanvas.getContext('2d');

            // Noise functions for texture
            function txHash(x,y){return((Math.sin(x*127.1+y*311.7)*43758.5453)%1+1)%1;}
            function txSn(x,y){
                const ix=Math.floor(x),iy=Math.floor(y),fx=x-ix,fy=y-iy;
                const u=fx*fx*(3-2*fx),v=fy*fy*(3-2*fy);
                return (txHash(ix,iy)*(1-u)+txHash(ix+1,iy)*u)*(1-v)+(txHash(ix,iy+1)*(1-u)+txHash(ix+1,iy+1)*u)*v;
            }
            function txFbm(x,y,oct){
                let v=0,a=0.5,tx=x,ty=y;
                for(let i=0;i<oct;i++){v+=txSn(tx,ty)*a;tx*=2;ty*=2;a*=0.5;}
                return v;
            }

            const imgData = tc.createImageData(TEX_SIZE, TEX_SIZE);
            const d = imgData.data;
            for(let py=0;py<TEX_SIZE;py++){
                for(let px=0;px<TEX_SIZE;px++){
                    const wx=(px/TEX_SIZE-0.5)*50;
                    const wz=(py/TEX_SIZE-0.5)*50;

                    // Grass with 2 noise layers + fine grain
                    const n1=txFbm(wx*0.8,wz*0.8,3);
                    const n2=txSn(wx*12,wz*12);

                    let r=0.24+n1*0.08, g=0.46+n1*0.1, b=0.13+n1*0.04;
                    const grain=0.92+n2*0.08;
                    r*=grain; g*=grain; b*=grain;


                    const idx=(py*TEX_SIZE+px)*4;
                    d[idx]=Math.min(255,r*255)|0;
                    d[idx+1]=Math.min(255,g*255)|0;
                    d[idx+2]=Math.min(255,b*255)|0;
                    d[idx+3]=255;
                }
            }
            tc.putImageData(imgData,0,0);

            const groundTex = new THREE.CanvasTexture(texCanvas);
            groundTex.wrapS=groundTex.wrapT=THREE.ClampToEdgeWrapping;
            groundTex.minFilter=THREE.LinearMipmapLinearFilter;
            groundTex.magFilter=THREE.LinearFilter;
            groundTex.anisotropy=renderer.capabilities.getMaxAnisotropy();

            const groundMat = new THREE.MeshStandardMaterial({
                map: groundTex,
                roughness: 0.92,
                metalness: 0.0,
            });
            const ground = new THREE.Mesh(geo, groundMat);
            ground.rotation.x=-Math.PI/2; ground.receiveShadow=true; scene.add(ground);

            const waterMat={uniforms:{uTime:{value:0}}};
            const treeGroups=[];
            const fireParticles=[];
            const smokeParticles=[];
            const fireLight={intensity:0};
            const fireGlow={intensity:0};
            const flameMat={uniforms:{uTime:{value:0}}};

            /*
            const barkMat=new THREE.MeshStandardMaterial({color:0x4a3520,roughness:0.95});
            const barkMat2=new THREE.MeshStandardMaterial({color:0x3a2a18,roughness:0.95});
            const leafMats=[
                new THREE.MeshStandardMaterial({color:0x2a6a1a,roughness:0.8}),
                new THREE.MeshStandardMaterial({color:0x1e5a12,roughness:0.8}),
                new THREE.MeshStandardMaterial({color:0x358a25,roughness:0.8}),
                new THREE.MeshStandardMaterial({color:0x2d7a20,roughness:0.8}),
            ];

            function addBranch(group, origin, dir, length, radius, depth, maxDepth) {
                if(depth > maxDepth || radius < 0.005) return;
                // Branch cylinder
                const cyl = new THREE.Mesh(
                    new THREE.CylinderGeometry(radius*0.65, radius, length, Math.max(3, 6-depth)),
                    depth===0 ? barkMat : barkMat2
                );
                // Position and orient along direction
                const end = origin.clone().add(dir.clone().multiplyScalar(length));
                const mid = origin.clone().add(end).multiplyScalar(0.5);
                cyl.position.copy(mid);
                // Align cylinder to direction
                const up = new THREE.Vector3(0,1,0);
                const quat = new THREE.Quaternion().setFromUnitVectors(up, dir.clone().normalize());
                cyl.quaternion.copy(quat);
                cyl.castShadow=true;
                group.add(cyl);

                // Add leaves at branch tips
                if(depth >= maxDepth-1 || (depth >= 1 && Math.random() > 0.3)) {
                    const leafCount = 2 + Math.floor(Math.random()*3);
                    for(let l=0; l<leafCount; l++) {
                        const leafSize = 0.08 + Math.random()*0.15;
                        const leaf = new THREE.Mesh(
                            new THREE.IcosahedronGeometry(leafSize, 0),
                            leafMats[Math.floor(Math.random()*4)]
                        );
                        leaf.position.copy(end);
                        leaf.position.x += (Math.random()-0.5)*0.15;
                        leaf.position.y += Math.random()*0.1;
                        leaf.position.z += (Math.random()-0.5)*0.15;
                        leaf.scale.y = 0.5 + Math.random()*0.3;
                        leaf.castShadow=true;
                        group.add(leaf);
                    }
                }

                // Sub-branches
                const childCount = depth === 0 ? 3+Math.floor(Math.random()*2) : 1+Math.floor(Math.random()*2);
                for(let i=0; i<childCount; i++) {
                    // Branch direction: mostly continue upward with spread
                    const spread = 0.4 + depth * 0.15;
                    const childDir = dir.clone();
                    childDir.x += (Math.random()-0.5) * spread;
                    childDir.z += (Math.random()-0.5) * spread;
                    childDir.y += 0.1 + Math.random()*0.3;
                    childDir.normalize();

                    const childLen = length * (0.55 + Math.random()*0.2);
                    const childRad = radius * (0.5 + Math.random()*0.2);
                    addBranch(group, end, childDir, childLen, childRad, depth+1, maxDepth);
                }
            }

            function makeTree(px, pz) {
                const g = new THREE.Group();
                const y = th(px, pz);
                const trunkLen = 0.5 + Math.random()*0.8;
                const trunkRad = 0.04 + Math.random()*0.03;
                const maxD = 2 + Math.floor(Math.random()*2); // 2-3 levels of branching
                const trunkDir = new THREE.Vector3(
                    (Math.random()-0.5)*0.1,
                    1,
                    (Math.random()-0.5)*0.1
                ).normalize();
                addBranch(g, new THREE.Vector3(0,0,0), trunkDir, trunkLen, trunkRad, 0, maxD);
                g.position.set(px, y, pz);
                g.scale.setScalar(0.8 + Math.random()*0.8);
                return g;
            }

            const treeGroups=[];
            for(let i=0;i<20;i++){
                const a=Math.random()*Math.PI*2, d=4+Math.random()*13;
                const x=Math.cos(a)*d, z=Math.sin(a)*d;
                if(th(x,z) < WATER_LEVEL+0.1) continue;
                const tree = makeTree(x,z);
                scene.add(tree);
                treeGroups.push(tree);
            }

            // ── Campfire ──
            const firePos=new THREE.Vector3(2.5,th(2.5,-1.5)+0.03,-1.5);
            const logMat=new THREE.MeshStandardMaterial({color:0x2a1808,roughness:0.95});
            for(let i=0;i<3;i++){
                const log=new THREE.Mesh(new THREE.CylinderGeometry(0.025,0.035,0.35,5),logMat);
                log.position.copy(firePos);log.position.y+=0.02;
                log.rotation.set(Math.PI/2,i*Math.PI/3,0.15);log.castShadow=true;scene.add(log);
            }
            for(let i=0;i<7;i++){
                const a=i/7*Math.PI*2;
                const s=new THREE.Mesh(new THREE.DodecahedronGeometry(0.04,0),rMat1);
                s.position.set(firePos.x+Math.cos(a)*0.2,firePos.y-0.01,firePos.z+Math.sin(a)*0.2);
                s.scale.y=0.5;scene.add(s);
            }
            // Dirt under campfire
            const cfDirt=new THREE.Mesh(new THREE.CircleGeometry(0.35,8),new THREE.MeshStandardMaterial({color:0x2a2015,roughness:1.0}));
            cfDirt.rotation.x=-Math.PI/2;cfDirt.position.set(firePos.x,firePos.y-0.01,firePos.z);scene.add(cfDirt);

            const fireLight=new THREE.PointLight(0xff6622,1.5,4);
            fireLight.position.copy(firePos);fireLight.position.y+=0.2;scene.add(fireLight);
            // Secondary warm glow
            const fireGlow=new THREE.PointLight(0xff4400,0.6,2.5);
            fireGlow.position.copy(firePos);fireGlow.position.y+=0.1;scene.add(fireGlow);

            // Flames — shader billboard quads
            const flameMat=new THREE.ShaderMaterial({
                transparent:true,depthWrite:false,blending:THREE.AdditiveBlending,
                uniforms:{uTime:{value:0}},
                vertexShader:`varying vec2 vUv;void main(){vUv=uv;
                    // Billboard
                    vec3 pos=position;
                    vec4 mvPos=modelViewMatrix*vec4(0.0,0.0,0.0,1.0);
                    mvPos.xy+=pos.xy;
                    gl_Position=projectionMatrix*mvPos;}`,
                fragmentShader:`uniform float uTime;varying vec2 vUv;
                    float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}
                    float sn(vec2 p){vec2 i=floor(p),f=fract(p);f=f*f*(3.0-2.0*f);return mix(mix(hash(i),hash(i+vec2(1,0)),f.x),mix(hash(i+vec2(0,1)),hash(i+vec2(1,1)),f.x),f.y);}
                    void main(){
                        vec2 uv=vUv*2.0-1.0;
                        float dist=length(uv);
                        // Flame shape — narrow at top, wide at bottom
                        float shape=1.0-smoothstep(0.0,0.5+uv.y*0.3,dist);
                        shape*=smoothstep(-1.0,-0.2,uv.y);
                        // Flickering noise
                        float n=sn(vec2(uv.x*3.0,uv.y*2.0-uTime*4.0))*0.5+0.5;
                        float n2=sn(vec2(uv.x*6.0+1.0,uv.y*4.0-uTime*6.0))*0.5+0.5;
                        shape*=0.5+n*0.3+n2*0.2;
                        // Color gradient — white core → yellow → orange → red
                        float t=1.0-uv.y*0.5-0.5;
                        vec3 core=vec3(1.0,0.95,0.8);
                        vec3 mid=vec3(1.0,0.6,0.1);
                        vec3 outer=vec3(0.8,0.2,0.0);
                        vec3 col=t<0.3?mix(core,mid,t/0.3):mix(mid,outer,(t-0.3)/0.7);
                        col*=1.5;
                        float alpha=shape*0.9;
                        if(alpha<0.01) discard;
                        gl_FragColor=vec4(col,alpha);
                    }`,side:THREE.DoubleSide
            });
            // Multiple flame planes at different angles
            const flames=[];
            for(let i=0;i<5;i++){
                const plane=new THREE.Mesh(new THREE.PlaneGeometry(0.2,0.35),flameMat);
                plane.position.copy(firePos);
                plane.position.y+=0.15;
                plane.position.x+=(Math.random()-0.5)*0.06;
                plane.position.z+=(Math.random()-0.5)*0.06;
                plane.rotation.y=i*Math.PI/5;
                scene.add(plane);
                flames.push(plane);
            }

            // Embers/sparks
            const fireParticles=[];
            const embGeo=new THREE.SphereGeometry(0.008,3,3);
            const embMat1=new THREE.MeshBasicMaterial({color:0xff8833,transparent:true});
            const embMat2=new THREE.MeshBasicMaterial({color:0xffaa22,transparent:true});
            for(let i=0;i<25;i++){
                const m=new THREE.Mesh(embGeo,Math.random()>0.5?embMat1:embMat2);
                m.position.copy(firePos);
                fireParticles.push({mesh:m,vx:(Math.random()-0.5)*0.15,vy:0.3+Math.random()*0.6,vz:(Math.random()-0.5)*0.15,life:Math.random()});
                scene.add(m);
            }
            // Smoke
            const smokeMat=new THREE.MeshBasicMaterial({color:0x444444,transparent:true,opacity:0.08});
            const smokeParticles=[];
            for(let i=0;i<8;i++){
                const m=new THREE.Mesh(new THREE.SphereGeometry(0.04+Math.random()*0.04,4,4),smokeMat.clone());
                m.position.copy(firePos);m.position.y+=0.3;
                smokeParticles.push({mesh:m,vy:0.15+Math.random()*0.1,life:Math.random(),vx:(Math.random()-0.5)*0.05});
                scene.add(m);
            }

            */
            // ── Pollen/dust ──
            const dustParticles=[];
            const dGeo=new THREE.SphereGeometry(0.006,3,3);
            const dMat=new THREE.MeshBasicMaterial({color:0xffffee,transparent:true,opacity:0.35});
            for(let i=0;i<80;i++){
                const m=new THREE.Mesh(dGeo,dMat);
                m.position.set((Math.random()-0.5)*16,0.3+Math.random()*2.5,(Math.random()-0.5)*16);
                dustParticles.push({mesh:m,ph:Math.random()*Math.PI*2,sp:0.15+Math.random()*0.25});
                scene.add(m);
            }

            // ── Birds ──
            const birds=[];
            const wingMat=new THREE.MeshBasicMaterial({color:0x222222,side:THREE.DoubleSide});
            for(let i=0;i<5;i++){
                const bird=new THREE.Group();
                // Simple V-shape wings
                const wingL=new THREE.Mesh(new THREE.PlaneGeometry(0.12,0.03),wingMat);
                wingL.position.x=-0.06;bird.add(wingL);
                const wingR=new THREE.Mesh(new THREE.PlaneGeometry(0.12,0.03),wingMat);
                wingR.position.x=0.06;bird.add(wingR);
                const body=new THREE.Mesh(new THREE.SphereGeometry(0.015,3,3),wingMat);
                bird.add(body);
                bird.position.set((Math.random()-0.5)*20,4+Math.random()*4,(Math.random()-0.5)*20);
                scene.add(bird);
                birds.push({group:bird,cx:bird.position.x,cz:bird.position.z,radius:3+Math.random()*5,phase:Math.random()*Math.PI*2,speed:0.3+Math.random()*0.4,flapSpeed:5+Math.random()*3,wingL,wingR});
            }

            // ── Shader grass — skip blades in water ──
            const GRID_SIZE = 28;
            const CELL = 0.048;
            const COLS = Math.floor(GRID_SIZE / CELL);
            const maxGrass = COLS * COLS;
            const grassPositions = new Float32Array(maxGrass * 3);
            const grassPhases = new Float32Array(maxGrass);
            const grassHeights = new Float32Array(maxGrass);
            let gi = 0;
            for (let row = 0; row < COLS; row++) {
                for (let col = 0; col < COLS; col++) {
                    const x = -GRID_SIZE/2 + col * CELL + (Math.random() - 0.5) * CELL * 0.9;
                    const z = -GRID_SIZE/2 + row * CELL + (Math.random() - 0.5) * CELL * 0.9;
                    const y = th(x, z);
                    grassPositions[gi * 3] = x;
                    grassPositions[gi * 3 + 1] = y + 0.06;
                    grassPositions[gi * 3 + 2] = z;
                    grassPhases[gi] = Math.random() * Math.PI * 2;
                    grassHeights[gi] = 0.1 + Math.random() * 0.18;
                    gi++;
                }
            }
            const GRASS_COUNT = gi;

            // Blade: 4-segment quad strip (wide at base, tapered at tip, slight curve)
            const SEGS = 4;
            const bVerts = [];
            const bUVs = [];
            const bIdx = [];
            for (let s = 0; s <= SEGS; s++) {
                const t = s / SEGS;
                const w = 0.03 * (1.0 - t * 0.85); // taper
                const curve = t * t * 0.02; // slight forward lean
                bVerts.push(-w, t, curve);
                bVerts.push( w, t, curve);
                bUVs.push(0, t);
                bUVs.push(1, t);
                if (s < SEGS) {
                    const i = s * 2;
                    bIdx.push(i, i+1, i+2, i+1, i+3, i+2);
                }
            }

            const bladeGeo = new THREE.InstancedBufferGeometry();
            bladeGeo.setAttribute('position', new THREE.BufferAttribute(new Float32Array(bVerts), 3));
            bladeGeo.setAttribute('uv', new THREE.BufferAttribute(new Float32Array(bUVs), 2));
            bladeGeo.setIndex(bIdx);
            bladeGeo.setAttribute('aOffset', new THREE.InstancedBufferAttribute(grassPositions.slice(0, GRASS_COUNT*3), 3));
            bladeGeo.setAttribute('aPhase', new THREE.InstancedBufferAttribute(grassPhases.slice(0, GRASS_COUNT), 1));
            bladeGeo.setAttribute('aHeight', new THREE.InstancedBufferAttribute(grassHeights.slice(0, GRASS_COUNT), 1));

            const grassShaderMat = new THREE.ShaderMaterial({
                uniforms: {
                    uTime: { value: 0 },
                },
                vertexShader: `
                    attribute vec3 aOffset;
                    attribute float aPhase;
                    attribute float aHeight;
                    uniform float uTime;
                    varying float vT;
                    varying float vColorVar;

                    void main() {
                        float t = uv.y; // 0 at base, 1 at tip
                        vT = t;
                        vColorVar = sin(aPhase * 3.7) * 0.15;

                        vec3 pos = position;
                        pos.y *= aHeight;
                        pos.x *= aHeight * 1.2;
                        pos.z *= aHeight;

                        // Wind — increases with height (cubic)
                        float windPow = t * t * t;
                        float w1 = sin(uTime * 1.8 + aOffset.x * 0.25 + aOffset.z * 0.15 + aPhase);
                        float w2 = sin(uTime * 1.2 + aOffset.z * 0.3 + aPhase * 0.5);
                        float w3 = sin(uTime * 0.7 + aOffset.x * 0.1 + aOffset.z * 0.1); // large gusts
                        float windX = (w1 * 0.12 + w3 * 0.08) * windPow;
                        float windZ = (w2 * 0.08) * windPow;
                        pos.x += windX;
                        pos.z += windZ;

                        // Per-blade rotation
                        float angle = aPhase * 2.5;
                        float c = cos(angle), s = sin(angle);
                        vec3 r = vec3(pos.x * c - pos.z * s, pos.y, pos.x * s + pos.z * c);

                        gl_Position = projectionMatrix * modelViewMatrix * vec4(r + aOffset, 1.0);
                    }
                `,
                fragmentShader: `
                    varying float vT;
                    varying float vColorVar;

                    void main() {
                        // Summer grass: rich base → bright mid → sunlit tip
                        vec3 base = vec3(0.15, 0.3, 0.05);
                        vec3 mid = vec3(0.28, 0.52, 0.12);
                        vec3 tip = vec3(0.4, 0.65, 0.2);

                        vec3 color;
                        if (vT < 0.5) {
                            color = mix(base, mid, vT * 2.0);
                        } else {
                            color = mix(mid, tip, (vT - 0.5) * 2.0);
                        }

                        // Per-blade variation
                        color += vColorVar;

                        // Ambient occlusion at base
                        color *= 0.6 + vT * 0.4;

                        // Fake subsurface scattering at tips
                        color += vec3(0.05, 0.08, 0.02) * vT * vT;

                        gl_FragColor = vec4(color, 1.0);
                    }
                `,
                side: THREE.DoubleSide,
            });

            const grassMesh = new THREE.Mesh(bladeGeo, grassShaderMat);
            grassMesh.frustumCulled = false;
            scene.add(grassMesh);

            // Animate
            let t=0, angle=0;
            function animate(){
                requestAnimationFrame(animate);
                const dt=0.016; t+=dt;
                grassShaderMat.uniforms.uTime.value=t;
                waterMat.uniforms.uTime.value=t;
                skyMat.uniforms.uTime.value=t;

                // Flames shader
                flameMat.uniforms.uTime.value=t;

                // Embers
                for(const fp of fireParticles){
                    fp.life+=dt*1.0;
                    if(fp.life>1){fp.life=0;fp.mesh.position.copy(firePos);fp.mesh.position.y+=0.15;fp.mesh.position.x+=(Math.random()-0.5)*0.08;fp.mesh.position.z+=(Math.random()-0.5)*0.08;fp.vx=(Math.random()-0.5)*0.1;fp.vy=0.2+Math.random()*0.5;fp.vz=(Math.random()-0.5)*0.1;}
                    fp.mesh.position.x+=fp.vx*dt;fp.mesh.position.y+=fp.vy*dt;fp.mesh.position.z+=fp.vz*dt;
                    fp.mesh.scale.setScalar(Math.max(0.01,1.0-fp.life));fp.mesh.material.opacity=(1.0-fp.life)*0.8;
                }
                // Smoke
                for(const sp of smokeParticles){
                    sp.life+=dt*0.3;
                    if(sp.life>1){sp.life=0;sp.mesh.position.copy(firePos);sp.mesh.position.y+=0.25;sp.mesh.position.x+=(Math.random()-0.5)*0.05;}
                    sp.mesh.position.y+=sp.vy*dt;sp.mesh.position.x+=sp.vx*dt+Math.sin(t+sp.life*5)*0.002;
                    const s=0.5+sp.life*1.5;sp.mesh.scale.setScalar(s);
                    sp.mesh.material.opacity=0.06*(1.0-sp.life);
                }
                fireLight.intensity=1.2+Math.sin(t*11)*0.4+Math.sin(t*17)*0.25+Math.sin(t*29)*0.1;
                fireGlow.intensity=0.5+Math.sin(t*13)*0.2;

                // Dust
                for(const dp of dustParticles){
                    dp.mesh.position.x+=Math.sin(t*dp.sp+dp.ph)*0.0015;
                    dp.mesh.position.y+=Math.sin(t*dp.sp*0.5+dp.ph)*0.0008;
                    dp.mesh.position.z+=dp.sp*dt*0.2;
                    if(dp.mesh.position.z>8)dp.mesh.position.z=-8;
                }

                // Birds
                for(const b of birds){
                    const bt=t*b.speed+b.phase;
                    b.group.position.x=b.cx+Math.sin(bt)*b.radius;
                    b.group.position.z=b.cz+Math.cos(bt*0.7)*b.radius;
                    b.group.position.y+=Math.sin(bt*0.5)*0.003;
                    b.group.rotation.y=bt+Math.PI/2;
                    // Wing flap
                    const flap=Math.sin(t*b.flapSpeed)*0.5;
                    b.wingL.rotation.z=flap;
                    b.wingR.rotation.z=-flap;
                }

                // Tree sway
                for(const tr of treeGroups){
                    tr.rotation.z=Math.sin(t*0.8+tr.position.x)*0.015;
                    tr.rotation.x=Math.sin(t*0.6+tr.position.z)*0.01;
                }

                angle+=0.0008;
                camera.position.x=Math.sin(angle)*5.5;
                camera.position.z=Math.cos(angle)*5.5;
                camera.position.y=1.6+Math.sin(angle*0.5)*0.15;
                camera.lookAt(0,0.4,0);
                renderer.render(scene,camera);
            }
            animate();

            window.addEventListener('resize',()=>{
                const w2=canvas.clientWidth||window.innerWidth, h2=canvas.clientHeight||window.innerHeight;
                camera.aspect=w2/h2; camera.updateProjectionMatrix(); renderer.setSize(w2,h2);
            });

            // ── Auth logic ──
            let mode = 'login';
            // Check if URL is /register
            if (window.location.pathname === '/register') {
                mode = 'register';
                document.getElementById('lobby-login').classList.add('hidden');
                document.getElementById('lobby-register').classList.remove('hidden');
                document.getElementById('lobby-subtitle').textContent = 'Join the Renzora community';
                document.getElementById('lobby-toggle-text').textContent = 'Already have an account? ';
                document.getElementById('lobby-toggle-btn').textContent = 'Sign In';
            }

            function toggleAuth(){
                mode = mode==='login'?'register':'login';
                document.getElementById('lobby-login').classList.toggle('hidden', mode!=='login');
                document.getElementById('lobby-register').classList.toggle('hidden', mode!=='register');
                document.getElementById('lobby-subtitle').textContent = mode==='login'?'Sign in to continue':'Join the Renzora community';
                document.getElementById('lobby-toggle-text').textContent = mode==='login'?"Don't have an account? ":'Already have an account? ';
                document.getElementById('lobby-toggle-btn').textContent = mode==='login'?'Register':'Sign In';
                document.getElementById('lobby-error').classList.add('hidden');
            }

            function setCookies(data){
                document.cookie = `token=${data.access_token};path=/;max-age=2592000;SameSite=Strict`;
                document.cookie = `refresh_token=${data.refresh_token};path=/;max-age=2592000;SameSite=Strict`;
                document.cookie = `user=${encodeURIComponent(JSON.stringify(data.user))};path=/;max-age=2592000;SameSite=Strict`;
            }

            async function doLogin(e){
                e.preventDefault();
                const form=e.target, btn=document.getElementById('lobby-login-btn'), err=document.getElementById('lobby-error');
                err.classList.add('hidden'); btn.disabled=true; btn.textContent='Signing in...';
                try{
                    const res=await fetch('/api/auth/login',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({email:form.email.value,password:form.password.value})});
                    const data=await res.json();
                    if(!res.ok) throw new Error(data.error||'Login failed');
                    setCookies(data);
                    const redirect = new URLSearchParams(window.location.search).get('redirect') || '/';
                    window.location.href = redirect;
                }catch(error){err.textContent=error.message;err.classList.remove('hidden');btn.disabled=false;btn.textContent='Sign In';}
                return false;
            }

            async function doRegister(e){
                e.preventDefault();
                const form=e.target, btn=document.getElementById('lobby-register-btn'), err=document.getElementById('lobby-error');
                err.classList.add('hidden');
                if(form.password.value!==form.confirm_password.value){err.textContent='Passwords do not match';err.classList.remove('hidden');return false;}
                btn.disabled=true; btn.textContent='Creating account...';
                try{
                    const body={username:form.username.value,email:form.email.value,password:form.password.value};
                    const refCode=new URLSearchParams(window.location.search).get('ref')||'';
                    if(refCode)body.referral_code=refCode;
                    const res=await fetch('/api/auth/register',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});
                    const data=await res.json();
                    if(!res.ok) throw new Error(data.error||'Registration failed');
                    setCookies(data);
                    // New user → go to avatar editor
                    window.location.href = '/avatar/edit';
                }catch(error){err.textContent=error.message;err.classList.remove('hidden');btn.disabled=false;btn.textContent='Create Account';}
                return false;
            }

            window._toggleAuth=toggleAuth;
            window._doLogin=doLogin;
            window._doRegister=doRegister;
            "##
        </script>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    // Same lobby page — the JS detects /register path and shows register form
    view! {
        <LoginPage />
    }
}
