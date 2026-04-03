use leptos::prelude::*;

#[component]
pub fn AvatarEditorPage() -> impl IntoView {
    view! {
        <script type="importmap">
            r#"{"imports":{"three":"https://unpkg.com/three@0.170.0/build/three.module.js","three/addons/":"https://unpkg.com/three@0.170.0/examples/jsm/"}}"#
        </script>

        <section class="h-screen bg-[#08080c] flex flex-col overflow-hidden select-none" id="av-root">

            // 3D canvas — always visible as background
            <canvas id="av-canvas" class="absolute inset-0 w-full h-full block z-0"></canvas>

            // ── Editor UI ──
            <div id="av-editor-ui" class="hidden absolute inset-0 z-10 flex pointer-events-none">

                // Left side — top bar floats over viewport
                <div class="flex-1 flex flex-col">
                    <div class="pointer-events-auto shrink-0 flex items-center justify-between px-5 py-2">
                        <a href="/" class="text-zinc-400 hover:text-zinc-200 transition-colors text-sm flex items-center gap-1.5">
                            <i class="ph ph-arrow-left"></i>"Back"
                        </a>
                        <div class="flex items-center gap-3">
                            <span class="text-xs text-zinc-500 flex items-center gap-1"><i class="ph ph-coins"></i><span id="av-bal" class="text-zinc-300">"0"</span></span>
                            <button onclick="window._save()" id="av-save" class="pointer-events-auto px-5 py-1.5 rounded-xl text-xs font-semibold bg-accent text-white hover:bg-accent-hover transition-all shadow-lg shadow-accent/20">
                                "Save Avatar"
                            </button>
                        </div>
                    </div>
                </div>

                // ── Right panel ──
                <div class="pointer-events-auto w-[320px] shrink-0 h-full flex flex-col bg-[rgba(8,8,14,0.88)] backdrop-blur-2xl border-l border-white/[0.06] shadow-2xl shadow-black/50">

                    // Category tabs — vertical icon strip at top, scrollable horizontally
                    <div class="flex items-center border-b border-white/[0.05] px-1.5 shrink-0 overflow-x-auto av-scroll-h">
                        <button onclick="window._cat('class')" data-cat="class" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-200 border-b-2 border-accent transition-all">
                            <i class="ph ph-person text-base"></i><span class="text-[9px]">"Class"</span>
                        </button>
                        <button onclick="window._cat('weapon')" data-cat="weapon" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-sword text-base"></i><span class="text-[9px]">"Weapon"</span>
                        </button>
                        <button onclick="window._cat('shield')" data-cat="shield" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-shield text-base"></i><span class="text-[9px]">"Off-hand"</span>
                        </button>
                        <button onclick="window._cat('potion')" data-cat="potion" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-flask text-base"></i><span class="text-[9px]">"Potion"</span>
                        </button>
                        <button onclick="window._cat('color')" data-cat="color" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-palette text-base"></i><span class="text-[9px]">"Color"</span>
                        </button>
                        <div class="w-px h-5 bg-white/[0.06] mx-0.5 shrink-0"></div>
                        <button onclick="window._cat('idle')" data-cat="idle" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-hand-waving text-base"></i><span class="text-[9px]">"Social"</span>
                        </button>
                        <button onclick="window._cat('combat')" data-cat="combat" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-lightning text-base"></i><span class="text-[9px]">"Combat"</span>
                        </button>
                        <button onclick="window._cat('move')" data-cat="move" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-person-simple-run text-base"></i><span class="text-[9px]">"Move"</span>
                        </button>
                        <button onclick="window._cat('tools')" data-cat="tools" class="av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all">
                            <i class="ph ph-hammer text-base"></i><span class="text-[9px]">"Tools"</span>
                        </button>
                    </div>

                    // Items area — fills remaining height
                    <div id="av-items" class="flex-1 overflow-y-auto p-3 av-scroll"></div>
                </div>
            </div>

            // Toast
            <div id="av-toast" class="hidden fixed top-16 left-1/2 -translate-x-1/2 px-4 py-2 rounded-xl text-xs font-medium z-40"></div>
        </section>

        <script type="module">
            r##"
            import * as THREE from 'three';
            import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
            import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

            let scene, camera, renderer, controls, clock, mixer;
            let currentModel = null, rightSlot = null, leftSlot = null;
            let equippedR = null, equippedL = null;
            let anims = {};
            let token, curAnim = 'Idle_A';
            let thumbRenderer, thumbScene, thumbCamera;
            const thumbCache = {};

            const loader = new GLTFLoader();
            const load = u => new Promise((r,e) => loader.load(u, r, undefined, e));

            const CHARS = [
                { id:'Knight', name:'Knight', file:'/assets/avatar/characters/Knight.glb' },
                { id:'Barbarian', name:'Barbarian', file:'/assets/avatar/characters/Barbarian.glb' },
                { id:'Mage', name:'Mage', file:'/assets/avatar/characters/Mage.glb' },
                { id:'Ranger', name:'Ranger', file:'/assets/avatar/characters/Ranger.glb' },
                { id:'Rogue', name:'Rogue', file:'/assets/avatar/characters/Rogue.glb' },
                { id:'Rogue_Hooded', name:'Hooded Rogue', file:'/assets/avatar/characters/Rogue_Hooded.glb' },
                { id:'Druid', name:'Druid', file:'/assets/avatar/characters/Druid.glb' },
                { id:'Engineer', name:'Engineer', file:'/assets/avatar/characters/Engineer.glb' },
            ];
            const WEAPONS = [
                { id:'none', name:'None' },
                { id:'sword_1handed', name:'Sword', file:'/assets/avatar/items/sword_1handed.gltf' },
                { id:'sword_2handed', name:'Greatsword', file:'/assets/avatar/items/sword_2handed.gltf' },
                { id:'sword_2handed_color', name:'Royal Sword', file:'/assets/avatar/items/sword_2handed_color.gltf' },
                { id:'axe_1handed', name:'Axe', file:'/assets/avatar/items/axe_1handed.gltf' },
                { id:'axe_2handed', name:'Battleaxe', file:'/assets/avatar/items/axe_2handed.gltf' },
                { id:'dagger', name:'Dagger', file:'/assets/avatar/items/dagger.gltf' },
                { id:'staff', name:'Staff', file:'/assets/avatar/items/staff.gltf' },
                { id:'druid_staff', name:'Druid Staff', file:'/assets/avatar/items/druid_staff.gltf' },
                { id:'wand', name:'Wand', file:'/assets/avatar/items/wand.gltf' },
                { id:'bow', name:'Bow', file:'/assets/avatar/items/bow_withString.gltf' },
                { id:'crossbow_1handed', name:'Crossbow', file:'/assets/avatar/items/crossbow_1handed.gltf' },
                { id:'crossbow_2handed', name:'Heavy Crossbow', file:'/assets/avatar/items/crossbow_2handed.gltf' },
                { id:'shotgun', name:'Shotgun', file:'/assets/avatar/items/shotgun.gltf' },
                { id:'engineer_Wrench', name:'Wrench', file:'/assets/avatar/items/engineer_Wrench.gltf' },
            ];
            const SHIELDS = [
                { id:'none', name:'None' },
                { id:'shield_round', name:'Round Shield', file:'/assets/avatar/items/shield_round.gltf' },
                { id:'shield_round_color', name:'Painted Round', file:'/assets/avatar/items/shield_round_color.gltf' },
                { id:'shield_round_barbarian', name:'War Shield', file:'/assets/avatar/items/shield_round_barbarian.gltf' },
                { id:'shield_badge', name:'Badge Shield', file:'/assets/avatar/items/shield_badge.gltf' },
                { id:'shield_badge_color', name:'Painted Badge', file:'/assets/avatar/items/shield_badge_color.gltf' },
                { id:'shield_square', name:'Tower Shield', file:'/assets/avatar/items/shield_square.gltf' },
                { id:'shield_spikes', name:'Spiked Shield', file:'/assets/avatar/items/shield_spikes.gltf' },
                { id:'spellbook_open', name:'Spellbook', file:'/assets/avatar/items/spellbook_open.gltf' },
                { id:'smokebomb', name:'Smokebomb', file:'/assets/avatar/items/smokebomb.gltf' },
                { id:'mug_full', name:'Ale Mug', file:'/assets/avatar/items/mug_full.gltf' },
                { id:'quiver', name:'Quiver', file:'/assets/avatar/items/quiver.gltf' },
            ];
            const POTIONS = [
                { id:'none', name:'None' },
                ...['red','blue','green','orange'].flatMap(c => ['small','medium','large','huge'].map(s => ({
                    id:`potion_${s}_${c}`, name:`${s[0].toUpperCase()+s.slice(1)} ${c[0].toUpperCase()+c.slice(1)}`,
                    file:`/assets/avatar/items/potion_${s}_${c}.gltf`
                }))),
            ];
            const ANIM_GROUPS = {
                idle:{title:'Social',anims:['Idle_A','Idle_B','Waving','Cheering','Interact','PickUp','Throw','Use_Item','Spawn_Ground','Sit_Chair_Idle','Sit_Floor_Idle','Lie_Idle','Push_Ups','Sit_Ups']},
                combat:{title:'Combat',anims:['Melee_1H_Attack_Chop','Melee_1H_Attack_Slice_Diagonal','Melee_1H_Attack_Stab','Melee_2H_Attack_Chop','Melee_2H_Attack_Spin','Melee_2H_Attack_Spinning','Melee_Block','Melee_Blocking','Melee_Dualwield_Attack_Chop','Melee_Unarmed_Attack_Kick','Melee_Unarmed_Attack_Punch_A','Ranged_Bow_Draw','Ranged_Bow_Release','Ranged_Magic_Shoot','Ranged_Magic_Spellcasting','Ranged_Magic_Summon','Hit_A','Hit_B','Death_A','Death_B']},
                move:{title:'Movement',anims:['Walking_A','Walking_B','Walking_Backwards','Running_A','Running_B','Jump_Full_Short','Jump_Full_Long','Dodge_Forward','Dodge_Backward','Dodge_Left','Dodge_Right','Crawling','Crouching','Sneaking','Running_Strafe_Left','Running_Strafe_Right']},
                tools:{title:'Tools',anims:['Chopping','Digging','Hammering','Pickaxing','Sawing','Lockpicking','Fishing_Cast','Fishing_Idle','Fishing_Reeling','Fishing_Catch','Working_A','Working_B','Holding_A']},
            };
            const PALETTE = ['#E53E3E','#DD6B20','#D69E2E','#38A169','#319795','#3182CE','#5A67D8','#805AD5','#D53F8C','#1A202C','#2D3748','#4A5568','#718096','#A0AEC0','#E2E8F0','#FFFAF0','#8B4513','#CD853F','#C0392B','#2C3E50'];
            const CFG = { character:'Knight', weapon:'none', shield:'none', potion:'none', anim:'Idle_A', colors:{} };
            const trees = [];
            const butterflies = [];
            const clouds = [];
            let worldTime = 0;

            // ── Boot ──
            initScene();
            buildTerrain();
            animate();

            token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) {
                window.location.href = '/login?redirect=/avatar/edit';
            } else {
                loadEditor();
            }

            // ── Load editor after auth ──
            async function loadEditor() {
                try { const r=await fetch('/api/credits/balance',{headers:{'Authorization':'Bearer '+token}}); if(r.ok){const d=await r.json();document.getElementById('av-bal').textContent=(d.credit_balance??0).toLocaleString();} } catch(e){}
                try { const r=await fetch('/api/avatar/me',{headers:{'Authorization':'Bearer '+token}}); if(r.ok){const d=await r.json();const ep=d.equipped_parts||{};Object.assign(CFG,{character:ep.character||'Knight',weapon:ep.weapon||'none',shield:ep.shield||'none',potion:ep.potion||'none',anim:ep.anim||'Idle_A',colors:ep.colors||{}});curAnim=CFG.anim;} } catch(e){}
                initThumbRenderer();
                const results = await Promise.all(['General','CombatMelee','CombatRanged','MovementBasic','MovementAdvanced','Simulation','Special','Tools'].map(f => load('/assets/avatar/animations/Rig_Medium_'+f+'.glb').catch(()=>null)));
                for (const g of results) if(g) for(const c of g.animations) if(c.name!=='T-Pose') anims[c.name]=c;
                await loadChar(CFG.character);
                await attach('weapon',CFG.weapon); await attach('shield',CFG.shield); await attach('potion',CFG.potion);
                document.getElementById('av-editor-ui').classList.remove('hidden');
                window._cat('class');
            }

            // ── Scene ──
            function initScene() {
                const canvas = document.getElementById('av-canvas');
                const w = canvas.clientWidth || window.innerWidth, h = canvas.clientHeight || window.innerHeight;
                scene = new THREE.Scene();
                scene.background = new THREE.Color(0x6a9fca);
                scene.fog = new THREE.FogExp2(0x7aafca, 0.035);

                camera = new THREE.PerspectiveCamera(28, w/h, 0.1, 80);
                camera.position.set(0, 1.2, 4.5);

                renderer = new THREE.WebGLRenderer({canvas, antialias:true});
                renderer.setSize(w, h);
                renderer.setPixelRatio(Math.min(window.devicePixelRatio,2));
                renderer.shadowMap.enabled = true;
                renderer.shadowMap.type = THREE.PCFSoftShadowMap;
                renderer.outputColorSpace = THREE.SRGBColorSpace;

                scene.add(new THREE.HemisphereLight(0x87CEEB, 0x3a6b35, 0.6));
                scene.add(new THREE.AmbientLight(0xffffff, 0.45));
                const sun = new THREE.DirectionalLight(0xfff4e0, 1.5);
                sun.position.set(5,8,4); sun.castShadow=true;
                sun.shadow.mapSize.set(2048,2048); sun.shadow.bias=-0.0005;
                sun.shadow.camera.left=-12;sun.shadow.camera.right=12;sun.shadow.camera.top=12;sun.shadow.camera.bottom=-12;sun.shadow.camera.far=30;
                scene.add(sun);

                controls = new OrbitControls(camera, renderer.domElement);
                controls.target.set(0,0.85,0); controls.enablePan=false;
                controls.minDistance=2.5; controls.maxDistance=6;
                controls.maxPolarAngle=Math.PI*0.75; controls.minPolarAngle=Math.PI*0.2;
                controls.enableDamping=true; controls.dampingFactor=0.06; controls.update();
                clock = new THREE.Clock();

                window.addEventListener('resize',()=>{
                    const w2=canvas.clientWidth||window.innerWidth, h2=canvas.clientHeight||window.innerHeight;
                    camera.aspect=w2/h2; camera.updateProjectionMatrix(); renderer.setSize(w2,h2);
                });
            }

            function buildTerrain() {
                const size=40, seg=80;
                const geo = new THREE.PlaneGeometry(size,size,seg,seg);
                const pos = geo.attributes.position;
                for (let i=0;i<pos.count;i++){
                    const x=pos.getX(i),y=pos.getY(i);
                    let h = Math.sin(x*0.3)*Math.cos(y*0.3)*0.6 + Math.sin(x*0.7+1.3)*Math.cos(y*0.5+0.8)*0.3 + Math.sin(x*1.5+3)*Math.cos(y*1.2+2.1)*0.1;
                    const d=Math.sqrt(x*x+y*y); if(d<3)h*=Math.max(0,(d-1.2)/1.8);
                    pos.setZ(i,h);
                }
                geo.computeVertexNormals();
                const ground = new THREE.Mesh(geo, new THREE.MeshStandardMaterial({color:0x4a7a3b,roughness:0.95,flatShading:true}));
                ground.rotation.x=-Math.PI/2; ground.receiveShadow=true; scene.add(ground);

                const plat = new THREE.Mesh(new THREE.CylinderGeometry(0.9,1.1,0.12,12), new THREE.MeshStandardMaterial({color:0x777777,roughness:0.85,flatShading:true}));
                plat.position.y=0.01; plat.receiveShadow=plat.castShadow=true; scene.add(plat);

                const treeMat = new THREE.MeshStandardMaterial({color:0x2d5a1e,roughness:0.9,flatShading:true});
                const treeMat2 = new THREE.MeshStandardMaterial({color:0x1e4a15,roughness:0.9,flatShading:true});
                const trunkMat = new THREE.MeshStandardMaterial({color:0x5a3a1e,roughness:0.9,flatShading:true});
                const rockMat = new THREE.MeshStandardMaterial({color:0x666666,roughness:0.9,flatShading:true});
                const grassMat = new THREE.MeshStandardMaterial({color:0x5a9a3a,roughness:0.9,side:THREE.DoubleSide});

                function th(x,z){let h=Math.sin(x*0.3)*Math.cos(z*0.3)*0.6+Math.sin(x*0.7+1.3)*Math.cos(z*0.5+0.8)*0.3+Math.sin(x*1.5+3)*Math.cos(z*1.2+2.1)*0.1;const d=Math.sqrt(x*x+z*z);if(d<3)h*=Math.max(0,(d-1.2)/1.8);return h;}

                // Trees — min distance 5 from center
                for(let i=0;i<25;i++){
                    const a=Math.random()*Math.PI*2, d=5+Math.random()*13;
                    const x=Math.cos(a)*d, z=Math.sin(a)*d, y=th(x,z);
                    const t=new THREE.Group();
                    const trH=0.4+Math.random()*0.5;
                    const trunk=new THREE.Mesh(new THREE.CylinderGeometry(0.06,0.1,trH,5),trunkMat);trunk.position.y=trH/2;trunk.castShadow=true;t.add(trunk);
                    const foliageGroup = new THREE.Group(); foliageGroup.name='foliage';
                    for(let l=0;l<2+Math.floor(Math.random()*2);l++){
                        const c=new THREE.Mesh(new THREE.ConeGeometry(0.35-l*0.07,0.4+Math.random()*0.2,6),Math.random()>0.4?treeMat:treeMat2);
                        c.position.y=trH+l*0.25+0.15;c.castShadow=true;foliageGroup.add(c);
                    }
                    t.add(foliageGroup);
                    const sc = 0.8+Math.random()*0.8;
                    t.position.set(x,y,z); t.scale.setScalar(sc);
                    scene.add(t);
                    trees.push({ group:t, foliage:foliageGroup, phase:Math.random()*Math.PI*2, speed:0.5+Math.random()*0.8 });
                }

                // Rocks — min distance 2.5
                for(let i=0;i<15;i++){
                    const a=Math.random()*Math.PI*2, d=2.5+Math.random()*11;
                    const x=Math.cos(a)*d, z=Math.sin(a)*d;
                    const r=new THREE.Mesh(new THREE.DodecahedronGeometry(0.1+Math.random()*0.2,0),rockMat);
                    r.position.set(x,th(x,z)+0.05,z);r.rotation.set(Math.random()*2,Math.random()*2,0);r.scale.y=0.5+Math.random()*0.5;
                    r.castShadow=r.receiveShadow=true;scene.add(r);
                }

                // Grass — min distance 1.8
                for(let i=0;i<60;i++){
                    const a=Math.random()*Math.PI*2, d=1.8+Math.random()*10;
                    const x=Math.cos(a)*d, z=Math.sin(a)*d;
                    const g=new THREE.Group();
                    for(let b=0;b<2+Math.floor(Math.random()*3);b++){
                        const bl=new THREE.Mesh(new THREE.ConeGeometry(0.02,0.12+Math.random()*0.12,3),grassMat);
                        bl.position.set((Math.random()-0.5)*0.08,0.06,0);bl.rotation.z=(Math.random()-0.5)*0.3;g.add(bl);
                    }
                    g.position.set(x,th(x,z),z);scene.add(g);
                }

                // Flowers
                const fc=[0xe85d75,0xf0c040,0xd070d0,0x70a0f0];
                for(let i=0;i<20;i++){
                    const a=Math.random()*Math.PI*2, d=2.5+Math.random()*8;
                    const x=Math.cos(a)*d, z=Math.sin(a)*d;
                    const f=new THREE.Group();
                    const fStem=new THREE.Mesh(new THREE.CylinderGeometry(0.008,0.008,0.12,3),grassMat);fStem.position.y=0.06;f.add(fStem);
                    const fHead=new THREE.Mesh(new THREE.SphereGeometry(0.025,5,5),new THREE.MeshStandardMaterial({color:fc[Math.floor(Math.random()*fc.length)],roughness:0.7}));fHead.position.y=0.13;f.add(fHead);
                    f.position.set(x,th(x,z),z);scene.add(f);
                }

                // ── Clouds ──
                const cloudMat = new THREE.MeshStandardMaterial({color:0xffffff,roughness:1,transparent:true,opacity:0.35});
                for(let i=0;i<8;i++){
                    const cloud = new THREE.Group();
                    const puffs = 3+Math.floor(Math.random()*3);
                    for(let p=0;p<puffs;p++){
                        const puff = new THREE.Mesh(new THREE.SphereGeometry(0.4+Math.random()*0.5,7,7),cloudMat);
                        puff.position.set((Math.random()-0.5)*1.2, Math.random()*0.2, (Math.random()-0.5)*0.5);
                        puff.scale.y = 0.4+Math.random()*0.3;
                        cloud.add(puff);
                    }
                    cloud.position.set(-20+Math.random()*40, 6+Math.random()*4, -15+Math.random()*30);
                    cloud.scale.setScalar(1+Math.random()*1.5);
                    scene.add(cloud);
                    clouds.push({ group:cloud, speed:0.008+Math.random()*0.015 });
                }

                // ── Butterflies ──
                const bfColors = [0xff6b9d,0xffd93d,0x6bcbff,0xc084fc,0xff8c42];
                for(let i=0;i<12;i++){
                    const bf = new THREE.Group();
                    const wingMat = new THREE.MeshStandardMaterial({color:bfColors[Math.floor(Math.random()*bfColors.length)],roughness:0.5,side:THREE.DoubleSide,transparent:true,opacity:0.8});
                    // Two triangle wings
                    const wingGeo = new THREE.BufferGeometry();
                    const verts = new Float32Array([0,0,0, 0.06,0.04,0.03, 0.06,-0.02,0.06]);
                    wingGeo.setAttribute('position',new THREE.BufferAttribute(verts,3));
                    wingGeo.computeVertexNormals();
                    const lw = new THREE.Mesh(wingGeo,wingMat); lw.name='lwing';
                    const rw = new THREE.Mesh(wingGeo.clone(),wingMat); rw.name='rwing'; rw.scale.x=-1;
                    // Body
                    const body = new THREE.Mesh(new THREE.CapsuleGeometry(0.005,0.03,3,4),new THREE.MeshStandardMaterial({color:0x333333}));
                    bf.add(body); bf.add(lw); bf.add(rw);

                    const cx=(-5+Math.random()*10), cz=(-5+Math.random()*10);
                    bf.position.set(cx, 0.8+Math.random()*1.5, cz);
                    bf.scale.setScalar(0.6+Math.random()*0.5);
                    scene.add(bf);
                    butterflies.push({ group:bf, cx, cz, radius:0.5+Math.random()*2, phase:Math.random()*Math.PI*2, speed:1+Math.random()*2, wingSpeed:8+Math.random()*6, baseY:bf.position.y });
                }
            }

            // ── Thumbs ──
            function initThumbRenderer(){
                thumbScene=new THREE.Scene();thumbScene.background=new THREE.Color(0x14141a);
                thumbCamera=new THREE.PerspectiveCamera(30,1,0.01,20);thumbCamera.position.set(0,0.3,2);
                thumbScene.add(new THREE.AmbientLight(0xffffff,1.2));const tDir=new THREE.DirectionalLight(0xffffff,1);tDir.position.set(1,2,2);thumbScene.add(tDir);
                thumbRenderer=new THREE.WebGLRenderer({antialias:true});thumbRenderer.setSize(72,72);thumbRenderer.outputColorSpace=THREE.SRGBColorSpace;
            }
            async function getThumb(file){
                if(thumbCache[file])return thumbCache[file];
                try{const g=await load(file);const o=g.scene.clone();const b=new THREE.Box3().setFromObject(o);const c=b.getCenter(new THREE.Vector3());const s=b.getSize(new THREE.Vector3());o.position.sub(c);o.scale.multiplyScalar(1.2/Math.max(s.x,s.y,s.z));thumbScene.add(o);thumbRenderer.render(thumbScene,thumbCamera);thumbScene.remove(o);const u=thumbRenderer.domElement.toDataURL();thumbCache[file]=u;return u;}catch(e){return null;}
            }

            // ── Character ──
            async function loadChar(id){
                const c=CHARS.find(x=>x.id===id);if(!c)return;
                if(currentModel){scene.remove(currentModel);dispose(currentModel);}
                if(mixer)mixer.stopAllAction();equippedR=equippedL=rightSlot=leftSlot=null;
                const g=await load(c.file);currentModel=g.scene;
                currentModel.traverse(ch=>{if(ch.isMesh){ch.castShadow=ch.receiveShadow=true;if(ch.material){ch.material.side=THREE.FrontSide;ch.material.depthWrite=true;ch.material.transparent=false;}}if(ch.isBone&&ch.name==='handslotr')rightSlot=ch;if(ch.isBone&&ch.name==='handslotl')leftSlot=ch;});
                applyColors();scene.add(currentModel);CFG.character=id;
                mixer=new THREE.AnimationMixer(currentModel);playAnim(curAnim);
            }
            function applyColors(){if(!currentModel||!CFG.colors)return;currentModel.traverse(ch=>{if(ch.isMesh&&ch.material&&CFG.colors[ch.name]){ch.material=ch.material.clone();ch.material.color.set(CFG.colors[ch.name]);}});}
            function playAnim(name){if(!mixer)return;mixer.stopAllAction();const c=anims[name];if(!c)return;const a=mixer.clipAction(c);const once=['Death','Hit','Spawn','Jump_Full','Dodge','Throw','Interact','PickUp','Use_Item','Fishing_Cast','Fishing_Catch'];if(once.some(p=>name.includes(p))){a.setLoop(THREE.LoopOnce);a.clampWhenFinished=true;}a.play();curAnim=name;CFG.anim=name;}
            async function attach(slot,id){CFG[slot]=id;const isR=slot==='weapon';if(isR&&equippedR){equippedR.parent?.remove(equippedR);dispose(equippedR);equippedR=null;}if(!isR&&equippedL){equippedL.parent?.remove(equippedL);dispose(equippedL);equippedL=null;}if(id==='none'||!id)return;const list=slot==='weapon'?WEAPONS:slot==='shield'?SHIELDS:POTIONS;const item=list.find(x=>x.id===id);if(!item?.file)return;try{const g=await load(item.file);const m=g.scene;m.traverse(ch=>{if(ch.isMesh){ch.castShadow=true;if(ch.material){ch.material.side=THREE.FrontSide;ch.material.depthWrite=true;ch.material.transparent=false;}}});const bone=isR?rightSlot:leftSlot;if(bone)bone.add(m);if(isR)equippedR=m;else equippedL=m;}catch(e){}}
            function dispose(o){o?.traverse(c=>{c.geometry?.dispose();if(c.material){if(Array.isArray(c.material))c.material.forEach(m=>m.dispose());else c.material.dispose();}})}
            function animate(){
                requestAnimationFrame(animate);
                const delta = clock.getDelta();
                worldTime += delta;
                if(mixer) mixer.update(delta);

                // Tree sway
                for(const t of trees){
                    const sway = Math.sin(worldTime*t.speed+t.phase)*0.03;
                    t.foliage.rotation.z = sway;
                    t.foliage.rotation.x = sway*0.5;
                }

                // Butterflies
                for(const bf of butterflies){
                    const t2 = worldTime*bf.speed+bf.phase;
                    bf.group.position.x = bf.cx + Math.sin(t2)*bf.radius;
                    bf.group.position.z = bf.cz + Math.cos(t2*0.7)*bf.radius;
                    bf.group.position.y = bf.baseY + Math.sin(t2*1.3)*0.3;
                    bf.group.rotation.y = t2;
                    // Wing flap
                    const wingAngle = Math.sin(worldTime*bf.wingSpeed)*0.7;
                    const lw = bf.group.getObjectByName('lwing');
                    const rw = bf.group.getObjectByName('rwing');
                    if(lw) lw.rotation.y = wingAngle;
                    if(rw) rw.rotation.y = -wingAngle;
                }

                // Clouds drift
                for(const c of clouds){
                    c.group.position.x += c.speed;
                    if(c.group.position.x > 25) c.group.position.x = -25;
                }

                controls.update();
                renderer.render(scene,camera);
            }

            // ── Color groups ──
            function getColorGroups(){
                if(!currentModel)return[];const map={};currentModel.traverse(ch=>{if(ch.isMesh&&ch.name&&!ch.name.includes('Mannequin'))map[ch.name]=ch;});
                const groups=[],used=new Set();
                for(const[name,mesh]of Object.entries(map)){if(used.has(name))continue;
                    let pair=null;if(name.includes('Left'))pair=name.replace('Left','Right');else if(name.includes('Right'))pair=name.replace('Right','Left');
                    if(pair&&map[pair]&&!used.has(pair)){groups.push({label:name.replace(/.*_/,'').replace('Left','').replace('Right','')+'s',names:[name,pair],mesh});used.add(name);used.add(pair);}
                    else{groups.push({label:name.replace(/.*_/,''),names:[name],mesh});used.add(name);}
                }return groups;
            }

            // ── UI ──
            let selectedMesh = null;

            function selectCat(cat){
                document.querySelectorAll('.av-cat').forEach(b=>{const a=b.dataset.cat===cat;b.className='av-cat flex flex-col items-center gap-0.5 px-2.5 py-2 border-b-2 transition-all '+(a?'text-zinc-200 border-accent':'text-zinc-500 border-transparent hover:text-zinc-300');});
                const el=document.getElementById('av-items');

                if(cat==='class') renderCharGrid(el);
                else if(cat==='weapon') renderItemGrid(el,WEAPONS,'weapon',CFG.weapon);
                else if(cat==='shield') renderItemGrid(el,SHIELDS,'shield',CFG.shield);
                else if(cat==='potion') renderItemGrid(el,POTIONS,'potion',CFG.potion);
                else if(cat==='color') renderColorPanel(el);
                else if(ANIM_GROUPS[cat]) renderAnimGrid(el,ANIM_GROUPS[cat].anims);
            }

            function renderCharGrid(el){
                el.innerHTML=`<div class="grid grid-cols-4 gap-1.5">${CHARS.map(c=>{
                    const a=CFG.character===c.id;
                    return `<button onclick="window._char('${c.id}')" class="p-1.5 rounded-xl border transition-all ${a?'border-accent/40 bg-accent/[0.08]':'border-white/[0.04] hover:border-white/[0.1]'}">
                        <div id="tc-${c.id}" class="w-full aspect-square rounded-lg bg-[#14141a] mb-1 flex items-center justify-center overflow-hidden"><i class="ph ph-circle-dashed text-sm text-zinc-800 animate-pulse"></i></div>
                        <div class="text-[10px] font-medium ${a?'text-accent':'text-zinc-400'} truncate text-center">${c.name}</div>
                    </button>`;
                }).join('')}</div>`;
                CHARS.forEach(async c=>{const u=await getThumb(c.file);const t=document.getElementById('tc-'+c.id);if(t&&u)t.innerHTML=`<img src="${u}" class="w-full h-full object-contain"/>`;});
            }

            function renderItemGrid(el,items,slot,activeId){
                el.innerHTML=`<div class="grid grid-cols-5 gap-1.5">${items.map(item=>{
                    const a=activeId===item.id;const n=item.id==='none';
                    return `<button onclick="window._equip('${slot}','${item.id}')" class="p-1.5 rounded-xl border transition-all ${a?'border-accent/40 bg-accent/[0.08]':'border-white/[0.04] hover:border-white/[0.1]'}">
                        <div id="ti-${slot}-${item.id}" class="w-full aspect-square rounded-lg bg-[#14141a] mb-1 flex items-center justify-center overflow-hidden">
                            ${n?'<i class="ph ph-prohibit text-base text-zinc-700"></i>':'<i class="ph ph-circle-dashed text-xs text-zinc-800 animate-pulse"></i>'}
                        </div>
                        <div class="text-[9px] font-medium ${a?'text-accent':'text-zinc-500'} truncate text-center">${item.name}</div>
                    </button>`;
                }).join('')}</div>`;
                items.forEach(async item=>{if(!item.file)return;const u=await getThumb(item.file);const t=document.getElementById('ti-'+slot+'-'+item.id);if(t&&u)t.innerHTML=`<img src="${u}" class="w-full h-full object-contain"/>`;});
            }

            function renderAnimGrid(el,animNames){
                const avail=animNames.filter(n=>anims[n]);
                el.innerHTML=`<div class="grid grid-cols-4 gap-1.5">${avail.map(name=>{
                    const a=curAnim===name;const label=name.replace(/_/g,' ').replace(/Melee /,'').replace(/Ranged /,'');
                    return `<button onclick="window._anim('${name}')" class="px-2 py-2 rounded-xl border transition-all ${a?'border-accent/40 bg-accent/[0.08]':'border-white/[0.04] hover:border-white/[0.1]'}">
                        <div class="text-[9px] font-medium ${a?'text-accent':'text-zinc-500'} truncate text-center">${label}</div>
                    </button>`;
                }).join('')}</div>`;
            }

            function renderColorPanel(el){
                if(!currentModel){el.innerHTML='<p class="text-zinc-600 text-xs py-4">Load a character first</p>';return;}
                const groups=getColorGroups();
                el.innerHTML=`<div class="space-y-3">${groups.map(g=>{
                    const key=g.names[0];const cur=CFG.colors[key]||'#'+g.mesh.material.color.getHexString();
                    return `<div>
                        <div class="text-[10px] text-zinc-500 font-medium mb-1.5">${g.label}</div>
                        <div class="flex flex-wrap gap-1">${PALETTE.map(c=>
                            `<button onclick="window._setColor('${key}','${c}')" class="w-5 h-5 rounded-full border transition-all hover:scale-125 ${cur.toLowerCase()===c.toLowerCase()?'border-accent ring-1 ring-accent/40 scale-110':'border-white/[0.08]'}" style="background:${c}"></button>`
                        ).join('')}
                        <button onclick="window._resetColor('${key}')" class="w-5 h-5 rounded-full border border-white/[0.08] flex items-center justify-center hover:border-white/20" title="Reset"><i class="ph ph-arrow-counter-clockwise text-[8px] text-zinc-600"></i></button></div>
                    </div>`;
                }).join('')}</div>`;
            }

            // ── Actions ──
            async function pickChar(id){await loadChar(id);await attach('weapon',CFG.weapon);await attach('shield',CFG.shield);await attach('potion',CFG.potion);selectCat('class');}
            async function equip(slot,id){await attach(slot,id);selectCat(slot);}
            function setAnim(name){playAnim(name);const b=document.querySelector('.av-cat.text-zinc-200');if(b)selectCat(b.dataset.cat);}

            function setColor(meshName,color){
                if(!currentModel)return;const groups=getColorGroups();const g=groups.find(g=>g.names.includes(meshName));const names=g?g.names:[meshName];
                currentModel.traverse(ch=>{if(ch.isMesh&&names.includes(ch.name)){ch.material=ch.material.clone();ch.material.color.set(color);}});
                names.forEach(n=>CFG.colors[n]=color);selectCat('color');
            }
            function resetColor(meshName){
                const groups=getColorGroups();const g=groups.find(g=>g.names.includes(meshName));const names=g?g.names:[meshName];
                names.forEach(n=>delete CFG.colors[n]);
                loadChar(CFG.character).then(()=>{attach('weapon',CFG.weapon);attach('shield',CFG.shield);attach('potion',CFG.potion);selectCat('color');});
            }

            async function save(){
                const btn=document.getElementById('av-save');const orig=btn.innerHTML;
                btn.innerHTML='Saving...';btn.disabled=true;
                try{const body={skin_color:'#C68642',eye_color:'#4A90D9',hair_color:'#3B2F2F',equipped_parts:{character:CFG.character,weapon:CFG.weapon,shield:CFG.shield,potion:CFG.potion,anim:CFG.anim,colors:CFG.colors}};
                const res=await fetch('/api/avatar/me',{method:'PUT',headers:{'Authorization':'Bearer '+token,'Content-Type':'application/json'},body:JSON.stringify(body)});
                toast(res.ok?'Avatar saved!':((await res.json()).error||'Failed'),!res.ok);}catch(e){toast('Failed',true);}
                btn.innerHTML=orig;btn.disabled=false;
            }

            function toast(msg,err){const t=document.getElementById('av-toast');t.textContent=msg;t.className='fixed top-16 left-1/2 -translate-x-1/2 px-4 py-2 rounded-xl text-xs font-medium z-40 '+(err?'bg-red-500/20 border border-red-500/30 text-red-400':'bg-green-500/20 border border-green-500/30 text-green-400');setTimeout(()=>t.classList.add('hidden'),3000);}

            window._cat=selectCat;window._char=pickChar;window._equip=equip;window._anim=setAnim;
            window._setColor=setColor;window._resetColor=resetColor;window._save=save;
            "##
        </script>

        <style>
            r#"
            .av-scroll::-webkit-scrollbar { width: 3px; }
            .av-scroll::-webkit-scrollbar-track { background: transparent; }
            .av-scroll::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.05); border-radius: 2px; }
            .av-scroll { scrollbar-width: thin; scrollbar-color: rgba(255,255,255,0.05) transparent; }
            .av-scroll-h::-webkit-scrollbar { height: 0; }
            .av-scroll-h { scrollbar-width: none; }
            "#
        </style>
    }
}
