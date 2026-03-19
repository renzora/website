use leptos::prelude::*;

/// Course listing page.
#[component]
pub fn CoursesPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[1200px] mx-auto">
                <div class="flex justify-between items-center mb-6">
                    <div>
                        <h1 class="text-2xl font-bold">"Courses"</h1>
                        <p class="text-zinc-500 text-sm mt-1">"Learn game development from the community."</p>
                    </div>
                    <a href="/courses/create" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-plus-circle text-base"></i>"Create Course"
                    </a>
                </div>
                <div id="courses-list" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">"Loading..."</div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const res = await fetch('/api/courses/');
                const data = await res.json();
                const el = document.getElementById('courses-list');
                if (!data.courses?.length) { el.innerHTML = '<p class="text-zinc-500 text-sm col-span-3 text-center py-16">No courses yet. Be the first to create one!</p>'; return; }
                el.innerHTML = data.courses.map(c => {
                    const stars = c.rating > 0 ? '★'.repeat(Math.round(c.rating)) + '☆'.repeat(5 - Math.round(c.rating)) : '';
                    const diffColor = {beginner:'text-green-400 bg-green-500/10',intermediate:'text-yellow-400 bg-yellow-500/10',advanced:'text-red-400 bg-red-500/10'}[c.difficulty] || 'text-zinc-400 bg-zinc-800';
                    return `
                    <a href="/courses/${c.slug}" class="block group">
                        <div class="bg-surface-card border border-zinc-800 rounded-xl overflow-hidden hover:border-zinc-700 transition-all">
                            <div class="h-40 bg-surface flex items-center justify-center">
                                ${c.cover_image_url ? `<img src="${c.cover_image_url}" class="w-full h-full object-cover" />` : '<i class="ph ph-graduation-cap text-4xl text-zinc-700"></i>'}
                            </div>
                            <div class="p-4">
                                <div class="flex items-center gap-2 mb-2">
                                    <span class="text-[10px] px-1.5 py-0.5 rounded ${diffColor}">${c.difficulty}</span>
                                    <span class="text-[10px] text-zinc-500">${c.chapter_count} chapters</span>
                                </div>
                                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors line-clamp-2">${c.title}</h3>
                                <p class="text-xs text-zinc-500 mt-1 line-clamp-2">${c.description}</p>
                                <div class="flex items-center justify-between mt-3">
                                    <span class="text-xs text-zinc-500">${c.creator_name}</span>
                                    <div class="flex items-center gap-2">
                                        ${stars ? `<span class="text-amber-400 text-xs">${stars}</span>` : ''}
                                        <span class="text-xs font-semibold ${c.price_credits === 0 ? 'text-green-400' : 'text-zinc-50'}">${c.price_credits === 0 ? 'Free' : c.price_credits + ' credits'}</span>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2 mt-2 text-[10px] text-zinc-600">
                                    <span><i class="ph ph-users"></i> ${c.enrolled_count} enrolled</span>
                                </div>
                            </div>
                        </div>
                    </a>`;
                }).join('');
            })();
            "##
        </script>
    }
}

/// Course detail page.
#[component]
pub fn CourseDetailPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[900px] mx-auto" id="course-detail">"Loading..."</div>
        </section>
        <script>
            r##"
            (async function() {
                const parts = window.location.pathname.split('/');
                const slug = parts[parts.length - 1];
                const res = await fetch('/api/courses/' + slug);
                if (!res.ok) { document.getElementById('course-detail').textContent = 'Course not found'; return; }
                const c = await res.json();
                const el = document.getElementById('course-detail');
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const isCreator = false; // TODO: check ownership
                const stars = c.rating > 0 ? '★'.repeat(Math.round(c.rating)) + '☆'.repeat(5 - Math.round(c.rating)) : 'No ratings yet';
                const diffColor = {beginner:'text-green-400 bg-green-500/10',intermediate:'text-yellow-400 bg-yellow-500/10',advanced:'text-red-400 bg-red-500/10'}[c.difficulty] || 'text-zinc-400 bg-zinc-800';

                el.innerHTML = `
                    <a href="/courses" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-4"><i class="ph ph-arrow-left"></i>All Courses</a>

                    <div class="flex flex-col lg:flex-row gap-6">
                        <!-- Main content -->
                        <div class="flex-1">
                            <h1 class="text-2xl font-bold">${c.title}</h1>
                            <div class="flex items-center gap-3 mt-2 text-xs text-zinc-500">
                                <span class="px-1.5 py-0.5 rounded ${diffColor}">${c.difficulty}</span>
                                <span>${c.chapter_count} chapters</span>
                                <span>${c.enrolled_count} enrolled</span>
                                <span class="text-amber-400">${stars}</span>
                            </div>
                            <p class="text-sm text-zinc-400 mt-4 leading-relaxed">${c.description}</p>

                            <div class="mt-4 flex items-center gap-2">
                                <a href="/profile/${c.creator.username}" class="text-sm text-accent hover:text-accent-hover">${c.creator.username}</a>
                                <span class="text-xs px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400">${c.creator.role}</span>
                            </div>

                            <!-- Chapter list -->
                            <h2 class="text-base font-semibold mt-8 mb-3">Curriculum</h2>
                            <div class="space-y-2">
                                ${c.chapters.map((ch, i) => `
                                    <a href="/courses/${c.slug}/chapter/${ch.slug}" class="flex items-center gap-3 p-3 bg-surface border border-zinc-800 rounded-lg hover:border-zinc-700 transition-all group">
                                        <span class="w-8 h-8 rounded-lg bg-surface-card flex items-center justify-center text-xs font-semibold text-zinc-400 shrink-0">${i + 1}</span>
                                        <div class="flex-1">
                                            <span class="text-sm font-medium group-hover:text-accent transition-colors">${ch.title}</span>
                                            <div class="flex items-center gap-2 mt-0.5">
                                                ${ch.duration_minutes > 0 ? `<span class="text-[10px] text-zinc-500"><i class="ph ph-clock"></i> ${ch.duration_minutes}min</span>` : ''}
                                                ${ch.is_free_preview ? '<span class="text-[10px] text-green-400">Free preview</span>' : ''}
                                            </div>
                                        </div>
                                        <i class="ph ph-play-circle text-lg text-zinc-600 group-hover:text-accent transition-colors"></i>
                                    </a>
                                `).join('')}
                            </div>
                        </div>

                        <!-- Sidebar -->
                        <div class="w-full lg:w-72 shrink-0">
                            <div class="bg-surface-card border border-zinc-800 rounded-xl p-5 sticky top-20">
                                <div class="text-2xl font-bold mb-1">${c.price_credits === 0 ? 'Free' : c.price_credits + ' credits'}</div>
                                ${token ? `<button onclick="enrollCourse('${c.slug}')" class="w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">${c.price_credits === 0 ? 'Enroll for Free' : 'Enroll Now'}</button>` : `<a href="/login" class="block w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors text-center">Sign in to Enroll</a>`}
                                <div class="mt-4 space-y-2 text-xs text-zinc-500">
                                    <div class="flex justify-between"><span>Chapters</span><span class="text-zinc-300">${c.chapter_count}</span></div>
                                    <div class="flex justify-between"><span>Students</span><span class="text-zinc-300">${c.enrolled_count}</span></div>
                                    <div class="flex justify-between"><span>Difficulty</span><span class="text-zinc-300">${c.difficulty}</span></div>
                                </div>
                            </div>
                        </div>
                    </div>
                `;
            })();

            async function enrollCourse(slug) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const res = await fetch('/api/courses/' + slug + '/enroll', { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                const data = await res.json();
                if (res.ok) { window.location.reload(); } else { alert(data.error || 'Failed'); }
            }
            "##
        </script>
    }
}

/// Chapter viewer with content.
#[component]
pub fn ChapterViewPage() -> impl IntoView {
    view! {
        <div class="flex min-h-[calc(100vh-56px)]" id="chapter-layout">"Loading..."</div>
        <script>
            r##"
            (async function() {
                const parts = window.location.pathname.split('/');
                const courseSlug = parts[2];
                const chapterSlug = parts[4];

                const [courseRes, chapterRes] = await Promise.all([
                    fetch('/api/courses/' + courseSlug),
                    fetch('/api/courses/' + courseSlug + '/chapters/' + chapterSlug + '/view'),
                ]);
                if (!courseRes.ok || !chapterRes.ok) { document.getElementById('chapter-layout').textContent = 'Not found'; return; }
                const course = await courseRes.json();
                const chapter = await chapterRes.json();

                document.getElementById('chapter-layout').innerHTML = `
                    <!-- Sidebar -->
                    <aside class="w-64 shrink-0 border-r border-zinc-800 bg-surface sticky top-14 h-[calc(100vh-56px)] overflow-y-auto hidden lg:block">
                        <div class="p-4">
                            <a href="/courses/${course.slug}" class="text-xs text-accent hover:text-accent-hover flex items-center gap-1 mb-3"><i class="ph ph-arrow-left"></i>Back to course</a>
                            <h3 class="text-sm font-semibold mb-3">${course.title}</h3>
                            <div class="space-y-1">
                                ${course.chapters.map((ch, i) => `
                                    <a href="/courses/${course.slug}/chapter/${ch.slug}" class="flex items-center gap-2 px-2 py-1.5 rounded text-[13px] ${ch.slug === chapterSlug ? 'bg-accent/10 text-accent' : 'text-zinc-400 hover:text-zinc-50 hover:bg-white/5'} transition-all">
                                        <span class="text-[10px] w-5 text-center text-zinc-500">${i + 1}</span>
                                        <span class="truncate">${ch.title}</span>
                                    </a>
                                `).join('')}
                            </div>
                        </div>
                    </aside>
                    <!-- Content -->
                    <div class="flex-1 min-w-0 px-8 py-10 lg:px-16 max-w-[860px]">
                        ${chapter.locked ? `
                            <div class="text-center py-20">
                                <i class="ph ph-lock-simple text-4xl text-zinc-600 mb-3"></i>
                                <h2 class="text-lg font-semibold">Locked Chapter</h2>
                                <p class="text-sm text-zinc-500 mt-2">Enroll in the course to access this chapter.</p>
                                <a href="/courses/${course.slug}" class="inline-flex items-center gap-2 mt-4 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">Enroll Now</a>
                            </div>
                        ` : `
                            <h1 class="text-2xl font-bold mb-2">${chapter.title}</h1>
                            ${chapter.duration_minutes > 0 ? `<span class="text-xs text-zinc-500"><i class="ph ph-clock"></i> ${chapter.duration_minutes} min</span>` : ''}
                            ${chapter.video_url ? `<div class="mt-4 mb-6 rounded-xl overflow-hidden border border-zinc-800"><video src="${chapter.video_url}" controls class="w-full"></video></div>` : ''}
                            <div class="mt-6 prose-content text-sm text-zinc-300 leading-relaxed">${chapter.content}</div>
                        `}
                    </div>
                `;
            })();
            "##
        </script>
    }
}

/// Course builder (creator) with WYSIWYG.
#[component]
pub fn CreateCoursePage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[900px] mx-auto">
                <a href="/courses" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-4"><i class="ph ph-arrow-left"></i>"Back to Courses"</a>
                <h1 class="text-xl font-bold mb-6">"Create a Course"</h1>

                <div id="course-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>

                <div class="bg-surface-card border border-zinc-800 rounded-xl p-6 space-y-4">
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Title"</label>
                        <input type="text" id="c-title" placeholder="e.g. Building a 3D Platformer from Scratch" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Description"</label>
                        <textarea id="c-desc" rows="4" placeholder="What will students learn?" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent resize-y"></textarea>
                    </div>
                    <div class="grid grid-cols-3 gap-4">
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Category"</label>
                            <select id="c-cat" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm">
                                <option value="game-design">"Game Design"</option>
                                <option value="programming">"Programming"</option>
                                <option value="3d-art">"3D Art"</option>
                                <option value="2d-art">"2D Art"</option>
                                <option value="audio">"Audio"</option>
                                <option value="vfx">"VFX & Shaders"</option>
                                <option value="renzora">"Renzora Engine"</option>
                                <option value="unreal">"Unreal Engine"</option>
                                <option value="unity">"Unity"</option>
                                <option value="godot">"Godot"</option>
                                <option value="general">"General"</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Difficulty"</label>
                            <select id="c-diff" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm">
                                <option value="beginner">"Beginner"</option>
                                <option value="intermediate">"Intermediate"</option>
                                <option value="advanced">"Advanced"</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Price (credits, 0=free)"</label>
                            <input type="number" id="c-price" min="0" value="0" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm" />
                        </div>
                    </div>
                    <button onclick="createCourse()" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-plus-circle text-base"></i>"Create & Add Chapters"
                    </button>
                </div>
            </div>
        </section>
        <script>
            r##"
            async function createCourse() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const err = document.getElementById('course-error');
                err.classList.add('hidden');
                try {
                    const res = await fetch('/api/courses/create', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            title: document.getElementById('c-title').value,
                            description: document.getElementById('c-desc').value,
                            category: document.getElementById('c-cat').value,
                            difficulty: document.getElementById('c-diff').value,
                            price_credits: parseInt(document.getElementById('c-price').value) || 0,
                        })
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Failed');
                    window.location.href = '/courses/' + data.slug + '/edit';
                } catch(e) { err.textContent = e.message; err.classList.remove('hidden'); }
            }
            "##
        </script>
    }
}

/// Course editor with chapter management and WYSIWYG.
#[component]
pub fn EditCoursePage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[900px] mx-auto">
                <div id="editor-content">"Loading..."</div>
            </div>
        </section>
        // Load Quill WYSIWYG editor
        <link rel="stylesheet" href="https://cdn.quilljs.com/1.3.7/quill.snow.css" />
        <script src="https://cdn.quilljs.com/1.3.7/quill.min.js"></script>
        <script>
            r##"
            let courseSlug = '';
            let quill = null;

            (async function() {
                courseSlug = window.location.pathname.split('/')[2];
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }

                const res = await fetch('/api/courses/' + courseSlug, { headers: { 'Authorization': 'Bearer ' + token } });
                if (!res.ok) { document.getElementById('editor-content').textContent = 'Course not found'; return; }
                const c = await res.json();

                document.getElementById('editor-content').innerHTML = `
                    <a href="/courses/${c.slug}" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-4"><i class="ph ph-arrow-left"></i>View Course</a>
                    <div class="flex justify-between items-center mb-6">
                        <h1 class="text-xl font-bold">${c.title}</h1>
                        <div class="flex gap-2">
                            <button onclick="togglePublish('${c.slug}', ${!c.published})" class="px-4 py-2 rounded-lg text-sm font-medium ${c.published ? 'bg-green-500/10 text-green-400 border border-green-500/20' : 'bg-zinc-800 text-zinc-300'}">${c.published ? 'Published ✓' : 'Publish'}</button>
                        </div>
                    </div>

                    <!-- Chapters -->
                    <h2 class="text-base font-semibold mb-3">Chapters</h2>
                    <div id="chapter-list" class="space-y-2 mb-6">
                        ${c.chapters.map((ch, i) => `
                            <div class="flex items-center gap-3 p-3 bg-surface border border-zinc-800 rounded-lg">
                                <span class="text-xs text-zinc-500 w-6 text-center">${i + 1}</span>
                                <span class="text-sm font-medium flex-1">${ch.title}</span>
                                <span class="text-[10px] text-zinc-500">${ch.duration_minutes}min</span>
                                ${ch.is_free_preview ? '<span class="text-[10px] text-green-400">Free</span>' : ''}
                                <button onclick="editChapter('${ch.slug}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-pencil-simple"></i></button>
                                <button onclick="if(confirm('Delete chapter?')) delChapter('${ch.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20"><i class="ph ph-trash"></i></button>
                            </div>
                        `).join('') || '<p class="text-zinc-500 text-sm">No chapters yet.</p>'}
                    </div>

                    <!-- Add chapter form -->
                    <div class="bg-surface-card border border-zinc-800 rounded-xl p-6">
                        <h3 class="text-sm font-semibold mb-4">Add Chapter</h3>
                        <div class="space-y-4">
                            <div><label class="block text-xs text-zinc-500 mb-1.5">Title</label>
                            <input type="text" id="ch-title" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" /></div>
                            <div class="grid grid-cols-3 gap-3">
                                <div><label class="block text-xs text-zinc-500 mb-1.5">Duration (min)</label>
                                <input type="number" id="ch-duration" value="10" min="0" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm" /></div>
                                <div><label class="block text-xs text-zinc-500 mb-1.5">Video URL (optional)</label>
                                <input type="url" id="ch-video" placeholder="https://..." class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm" /></div>
                                <div class="flex items-end"><label class="flex items-center gap-2 pb-2.5"><input type="checkbox" id="ch-free" class="rounded" /><span class="text-sm text-zinc-300">Free preview</span></label></div>
                            </div>
                            <div><label class="block text-xs text-zinc-500 mb-1.5">Content</label>
                            <div id="wysiwyg-editor" class="bg-surface border border-zinc-800 rounded-lg" style="min-height:200px"></div></div>
                            <button onclick="addChapter()" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors"><i class="ph ph-plus-circle"></i>Add Chapter</button>
                        </div>
                    </div>
                `;

                // Init Quill WYSIWYG
                quill = new Quill('#wysiwyg-editor', {
                    theme: 'snow',
                    placeholder: 'Write your chapter content...',
                    modules: {
                        toolbar: [
                            [{ header: [1, 2, 3, false] }],
                            ['bold', 'italic', 'underline', 'strike'],
                            [{ list: 'ordered' }, { list: 'bullet' }],
                            ['blockquote', 'code-block'],
                            ['link', 'image'],
                            ['clean']
                        ]
                    }
                });
            })();

            async function addChapter() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const content = quill.root.innerHTML;
                const res = await fetch('/api/courses/' + courseSlug + '/chapters', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        title: document.getElementById('ch-title').value,
                        content: content,
                        duration_minutes: parseInt(document.getElementById('ch-duration').value) || 0,
                        video_url: document.getElementById('ch-video').value || null,
                        is_free_preview: document.getElementById('ch-free').checked,
                    })
                });
                if (res.ok) { window.location.reload(); } else { const d = await res.json(); alert(d.error || 'Failed'); }
            }

            async function togglePublish(slug, publish) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                await fetch('/api/courses/' + slug + '/update', {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ published: publish })
                });
                window.location.reload();
            }

            async function delChapter(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                await fetch('/api/courses/' + courseSlug + '/chapters/' + id + '/delete', {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                window.location.reload();
            }

            function editChapter(slug) {
                // TODO: open edit modal with WYSIWYG
                alert('Edit chapter: ' + slug + ' (coming soon)');
            }
            "##
        </script>
    }
}
