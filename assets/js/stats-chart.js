/*
 * Views and downloads over time, as two stacked area panels sharing one x axis.
 *
 * Deliberately NOT one chart with two y axes. Views outnumber downloads by an
 * order of magnitude or more, so a shared linear axis flattens downloads onto
 * the floor, and a second axis lets any two lines be made to cross wherever the
 * scales happen to land. Two panels, each with its own scale and its own label,
 * say the same thing without inviting a comparison the geometry cannot support.
 * The shared crosshair gives back the one honest comparison: both numbers for
 * the same day, read together.
 *
 * No chart library. The page loads none, and a hand-rolled path is a few hundred
 * lines against a few hundred kilobytes.
 */
(function () {
  'use strict';

  // Validated against the dark surface for lightness band, chroma, CVD
  // separation (worst adjacent dE 29.2 under tritan, 32.2 under protan) and
  // contrast. Indigo is the site accent; amber is the only warm hue not already
  // spoken for by a status colour.
  var SERIES = [
    { key: 'views', label: 'Views', color: '#6366f1' },
    { key: 'downloads', label: 'Downloads', color: '#d97706' },
  ];

  var RANGES = [
    { key: 'days', label: 'Days' },
    { key: 'weeks', label: 'Weeks' },
    { key: 'months', label: 'Months' },
  ];

  // One panel's drawing area. `H` is its height in CSS pixels; `W` is not here,
  // because the viewBox width is the container's width in CSS pixels, measured at draw
  // time, so one viewBox unit is one pixel and the panel height below is the
  // height it actually renders at.
  //
  // A fixed viewBox scaled by the container would have made height a function of
  // width: the same chart is a full-width dashboard card at ~1550px and a
  // sidebar column at ~270px, which with a 720-unit box is a 280px-tall panel in
  // one place and a 50px one in the other, and strokes under a pixel in the
  // second. Measuring keeps both honest and keeps a 2px line 2px wide.
  var WIDE = { H: 150, L: 44, R: 12, T: 14, B: 22, dot: 4, font: 10 };
  var COMPACT = { H: 120, L: 30, R: 6, T: 12, B: 18, dot: 3.5, font: 9 };

  /* The preset plus the measured width. Floored so a container that is briefly
   * zero-width (mounted inside something still hidden) still produces a drawable
   * box rather than a divide-by-zero. */
  function geom(preset, width) {
    var g = {};
    for (var k in preset) g[k] = preset[k];
    g.W = Math.max(220, Math.round(width || 0));
    return g;
  }

  function fmt(n) {
    return n >= 1000000 ? (n / 1000000).toFixed(1).replace(/\.0$/, '') + 'M'
         : n >= 1000 ? (n / 1000).toFixed(1).replace(/\.0$/, '') + 'k'
         : String(n);
  }

  /* A bucket label for the axis and the tooltip. Months lose the day, because
   * "Mar 2026" is the bucket and "1 Mar 2026" would claim a precision the
   * rollup does not have. */
  function label(iso, range) {
    var p = iso.split('-');
    var d = new Date(Date.UTC(+p[0], +p[1] - 1, +p[2]));
    var mon = d.toLocaleDateString(undefined, { month: 'short', timeZone: 'UTC' });
    if (range === 'months') return mon + ' ' + d.getUTCFullYear();
    if (range === 'weeks') return 'w/c ' + d.getUTCDate() + ' ' + mon;
    return d.getUTCDate() + ' ' + mon;
  }

  /* A "nice" axis top: the smallest 1/2/5 x 10^n at or above the peak, so the
   * gridline lands on a number a person would have chosen. A flat-zero series
   * gets a top of 1 rather than 0, or every point would sit on the baseline and
   * the panel would look broken rather than quiet. */
  function niceMax(peak) {
    if (peak <= 0) return 1;
    var mag = Math.pow(10, Math.floor(Math.log10(peak)));
    var r = peak / mag;
    return (r <= 1 ? 1 : r <= 2 ? 2 : r <= 5 ? 5 : 10) * mag;
  }

  function x(g, i, n) {
    if (n <= 1) return g.L + (g.W - g.L - g.R) / 2;
    return g.L + (i * (g.W - g.L - g.R)) / (n - 1);
  }

  function y(g, v, max) {
    return g.T + (1 - v / max) * (g.H - g.T - g.B);
  }

  /* A smooth path through the points, as a cardinal spline with the tension that
   * keeps it from overshooting into negative territory on a spiky series. A
   * count cannot be less than zero, and a curve that dips below the baseline
   * between two points draws a day that never happened. */
  function linePath(g, vals, max) {
    var n = vals.length;
    if (!n) return '';
    if (n === 1) return 'M' + x(g, 0, n) + ',' + y(g, vals[0], max);
    var d = 'M' + x(g, 0, n).toFixed(2) + ',' + y(g, vals[0], max).toFixed(2);
    for (var i = 0; i < n - 1; i++) {
      var x0 = x(g, i, n), x1 = x(g, i + 1, n);
      var y0 = y(g, vals[i], max), y1 = y(g, vals[i + 1], max);
      var cx = (x0 + x1) / 2;
      d += 'C' + cx.toFixed(2) + ',' + y0.toFixed(2)
         + ' ' + cx.toFixed(2) + ',' + y1.toFixed(2)
         + ' ' + x1.toFixed(2) + ',' + y1.toFixed(2);
    }
    return d;
  }

  function areaPath(g, vals, max) {
    var line = linePath(g, vals, max);
    if (!line) return '';
    var floor = g.H - g.B;
    return line + 'L' + x(g, vals.length - 1, vals.length).toFixed(2) + ',' + floor
                + 'L' + x(g, 0, vals.length).toFixed(2) + ',' + floor + 'Z';
  }

  function svgEl(tag, attrs) {
    var el = document.createElementNS('http://www.w3.org/2000/svg', tag);
    for (var k in attrs) el.setAttribute(k, attrs[k]);
    return el;
  }

  /* One panel: gridlines, the area, the line, and the hover dot. */
  function panel(g, series, points, range, uid) {
    var vals = points.map(function (p) { return p[series.key]; });
    var peak = Math.max.apply(null, vals.concat([0]));
    var max = niceMax(peak);
    var total = vals.reduce(function (a, b) { return a + b; }, 0);
    var gid = 'grad-' + series.key + '-' + uid;

    var svg = svgEl('svg', {
      viewBox: '0 0 ' + g.W + ' ' + g.H,
      // width:100% with the viewBox measured from that same width means the
      // SVG renders 1:1; height comes from the viewBox, so it is exactly g.H.
      class: 'w-full block overflow-visible',
      height: String(g.H),
      role: 'img',
      'aria-label': series.label + ': ' + total + ' over the last ' + points.length + ' ' + range,
    });

    var defs = svgEl('defs', {});
    var grad = svgEl('linearGradient', { id: gid, x1: '0', y1: '0', x2: '0', y2: '1' });
    grad.appendChild(svgEl('stop', { offset: '0%', 'stop-color': series.color, 'stop-opacity': '0.38' }));
    grad.appendChild(svgEl('stop', { offset: '100%', 'stop-color': series.color, 'stop-opacity': '0.02' }));
    defs.appendChild(grad);
    svg.appendChild(defs);

    // Gridlines, recessive: they orient the eye and must never compete with the
    // data. Two in a narrow column, three when there is room: in a 270px sidebar
    // the middle label has nowhere to sit that is not on top of the line.
    (g.W > 400 ? [0, 0.5, 1] : [0, 1]).forEach(function (t) {
      var v = max * (1 - t);
      var yy = y(g, v, max);
      svg.appendChild(svgEl('line', {
        x1: g.L, x2: g.W - g.R, y1: yy, y2: yy,
        stroke: 'rgba(255,255,255,0.06)', 'stroke-width': '1',
      }));
      var t1 = svgEl('text', {
        x: g.L - 6, y: yy + 3.5, 'text-anchor': 'end',
        fill: 'rgba(161,161,170,0.75)', 'font-size': String(g.font),
      });
      t1.textContent = fmt(Math.round(v));
      svg.appendChild(t1);
    });

    svg.appendChild(svgEl('path', { d: areaPath(g, vals, max), fill: 'url(#' + gid + ')' }));
    svg.appendChild(svgEl('path', {
      d: linePath(g, vals, max), fill: 'none', stroke: series.color,
      'stroke-width': '2', 'stroke-linecap': 'round', 'stroke-linejoin': 'round',
    }));

    // Hover furniture, hidden until the pointer is over the card.
    var rule = svgEl('line', {
      y1: g.T - 4, y2: g.H - g.B, stroke: 'rgba(255,255,255,0.22)',
      'stroke-width': '1', opacity: '0',
    });
    // The 2px surface ring is what keeps the dot legible where it sits on top of
    // its own line.
    var dot = svgEl('circle', {
      r: String(g.dot), fill: series.color, stroke: '#0b0b0f', 'stroke-width': '2', opacity: '0',
    });
    svg.appendChild(rule);
    svg.appendChild(dot);

    return { svg: svg, rule: rule, dot: dot, vals: vals, max: max, total: total, peak: peak };
  }

  function render(g, root, points, range, uid) {
    root.innerHTML = '';
    var n = points.length;
    var narrow = g.W <= 400;

    var panels = SERIES.map(function (s) {
      var p = panel(g, s, points, range, uid);

      var head = document.createElement('div');
      head.className = 'flex items-baseline justify-between gap-2 px-1 mb-1';
      head.innerHTML =
        '<div class="flex items-center gap-2 min-w-0">' +
          '<span class="w-2 h-2 rounded-full shrink-0" style="background:' + s.color + '"></span>' +
          '<span class="text-[11px] uppercase tracking-wider text-zinc-400 truncate">' + s.label + '</span>' +
        '</div>' +
        '<div class="flex items-baseline gap-3 text-[11px] text-zinc-500 shrink-0">' +
          '<span><span class="text-zinc-200 font-semibold">' + p.total.toLocaleString() + '</span> total</span>' +
          (narrow ? '' : '<span class="hidden sm:inline">peak ' + p.peak.toLocaleString() + '</span>') +
        '</div>';

      var wrap = document.createElement('div');
      wrap.appendChild(head);
      wrap.appendChild(p.svg);
      root.appendChild(wrap);
      return p;
    });

    // The x axis is written once, under the lower panel: both panels share it,
    // and repeating it would say they might not.
    var axis = document.createElement('div');
    axis.className = 'flex justify-between px-1 text-[10px] text-zinc-600';
    axis.innerHTML = '<span>' + (n ? label(points[0].bucket, range) : '') + '</span>' +
                     '<span>' + (n ? label(points[n - 1].bucket, range) : '') + '</span>';
    root.appendChild(axis);

    var tip = document.createElement('div');
    tip.className = 'pointer-events-none absolute z-20 hidden rounded-lg border border-zinc-700/60 ' +
                    'bg-[#0b0b0f]/95 px-3 py-2 text-xs shadow-xl backdrop-blur';
    root.appendChild(tip);

    function hide() {
      tip.classList.add('hidden');
      panels.forEach(function (p) { p.rule.setAttribute('opacity', '0'); p.dot.setAttribute('opacity', '0'); });
    }

    function move(ev) {
      if (!n) return;
      var box = panels[0].svg.getBoundingClientRect();
      // Pointer x as a fraction of the SVG, then into viewBox units, then to the
      // nearest bucket. Going through viewBox units rather than straight to a
      // fraction of the bucket count is what keeps the crosshair on the point
      // under the cursor when the left gutter is wide.
      var vx = ((ev.clientX - box.left) / box.width) * g.W;
      var span = g.W - g.L - g.R;
      var i = Math.round(((vx - g.L) / span) * (n - 1));
      i = Math.max(0, Math.min(n - 1, i));

      panels.forEach(function (p) {
        var xx = x(g, i, n);
        p.rule.setAttribute('x1', xx); p.rule.setAttribute('x2', xx);
        p.rule.setAttribute('opacity', '1');
        p.dot.setAttribute('cx', xx);
        p.dot.setAttribute('cy', y(g, p.vals[i], p.max));
        p.dot.setAttribute('opacity', '1');
      });

      tip.innerHTML =
        '<div class="text-zinc-400 mb-1">' + label(points[i].bucket, range) + '</div>' +
        SERIES.map(function (s) {
          return '<div class="flex items-center gap-2 whitespace-nowrap">' +
            '<span class="w-2 h-2 rounded-full" style="background:' + s.color + '"></span>' +
            '<span class="text-zinc-400">' + s.label + '</span>' +
            '<span class="ml-auto text-zinc-100 font-semibold">' + points[i][s.key].toLocaleString() + '</span>' +
          '</div>';
        }).join('');
      tip.classList.remove('hidden');

      // Clamp inside the card so a tooltip near the right edge does not hang off
      // it, and flip above the pointer near the bottom.
      var rootBox = root.getBoundingClientRect();
      var tw = tip.offsetWidth, th = tip.offsetHeight;
      var lx = Math.min(Math.max(ev.clientX - rootBox.left + 14, 4), rootBox.width - tw - 4);
      var ly = ev.clientY - rootBox.top + 16;
      if (ly + th > rootBox.height) ly = ev.clientY - rootBox.top - th - 10;
      tip.style.left = lx + 'px';
      tip.style.top = ly + 'px';
    }

    root.addEventListener('mousemove', move);
    root.addEventListener('mouseleave', hide);
  }

  var uidSeq = 0;

  /**
   * Mount a chart.
   *
   * opts.el        container element (position:relative; the tooltip is absolute inside it)
   * opts.endpoint  a function (range) => url
   * opts.headers   optional object of request headers (the dashboard sends a bearer token)
   * opts.range     starting range key, default "days"
   * opts.compact   true for a narrow column (an asset page's sidebar)
   */
  function mount(opts) {
    var root = opts.el;
    if (!root) return;
    var uid = ++uidSeq;
    var range = opts.range || 'days';
    var preset = opts.compact ? COMPACT : WIDE;

    var tabs = document.createElement('div');
    tabs.className = 'flex items-center gap-0.5 p-0.5 rounded-lg bg-white/[0.03] border border-zinc-800/50 shrink-0';
    var plot = document.createElement('div');
    plot.className = 'relative mt-3 space-y-3';

    var buttons = RANGES.map(function (r) {
      var b = document.createElement('button');
      b.type = 'button';
      b.textContent = r.label;
      b.dataset.range = r.key;
      b.className = tabClass(false);
      b.addEventListener('click', function () {
        if (range === r.key) return;
        range = r.key;
        paint();
        load();
      });
      tabs.appendChild(b);
      return b;
    });

    function tabClass(on) {
      // Tighter buttons in the sidebar: three of them plus a title have to fit
      // across ~270px.
      return (opts.compact ? 'px-1.5 py-0.5 ' : 'px-2.5 py-1 ') +
        'rounded-md text-[11px] font-medium transition-colors ' +
        (on ? 'bg-white/[0.07] text-zinc-100' : 'text-zinc-500 hover:text-zinc-300');
    }

    function paint() {
      buttons.forEach(function (b) {
        b.className = tabClass(b.dataset.range === range);
      });
    }

    var head = document.createElement('div');
    head.className = 'flex items-center justify-between gap-2 flex-wrap';
    var title = document.createElement('div');
    title.className = 'text-sm font-semibold text-zinc-200';
    title.textContent = opts.title || 'Performance';
    head.appendChild(title);
    head.appendChild(tabs);

    root.innerHTML = '';
    root.appendChild(head);
    root.appendChild(plot);
    paint();

    // The placeholder's height is set inline, NOT as an `h-[...]` class: Tailwind
    // generates utilities by finding whole literal tokens in the source, so a
    // class assembled from a computed number exists in no stylesheet.
    function placeholder(inner, cls) {
      var d = document.createElement('div');
      d.className = cls;
      d.style.height = boxH + 'px';
      d.innerHTML = inner;
      plot.innerHTML = '';
      plot.appendChild(d);
    }

    function skeleton() {
      placeholder('', 'rounded-lg bg-white/[0.02] animate-pulse');
    }

    function message(text) {
      placeholder(
        '<i class="ph ph-chart-line text-2xl text-zinc-700 mb-2"></i>' +
        '<p class="text-sm text-zinc-600">' + text + '</p>',
        'flex flex-col items-center justify-center text-center');
    }

    // Reserve the height the rendered chart will take, so the card does not jump
    // when the data lands. Two panels of `H` plus their label rows, the gaps and
    // the axis line.
    var boxH = preset.H * 2 + 74;

    var lastPoints = null;
    function draw() {
      if (!lastPoints) return;
      render(geom(preset, plot.clientWidth), plot, lastPoints, range, uid);
    }

    // Redraw on resize, because the viewBox is the measured width: without this
    // a window drag would leave the old box stretched by the browser, which is
    // the distortion the measuring exists to avoid.
    var resizeTimer = null;
    function onResize() {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(draw, 120);
    }
    if (window.ResizeObserver) {
      new ResizeObserver(onResize).observe(plot);
    } else {
      window.addEventListener('resize', onResize);
    }

    var inFlight = 0;
    function load() {
      var mine = ++inFlight;
      lastPoints = null;
      skeleton();
      fetch(opts.endpoint(range), { headers: opts.headers || {} })
        .then(function (r) { return r.ok ? r.json() : Promise.reject(r.status); })
        .then(function (data) {
          // A slow first request landing after a fast second one would repaint
          // the chart with the range the user already moved off.
          if (mine !== inFlight) return;
          var points = (data && data.points) || [];
          var any = points.some(function (p) { return p.views || p.downloads; });
          if (!points.length || !any) {
            message('No activity in this period yet.');
            return;
          }
          lastPoints = points;
          draw();
        })
        .catch(function () {
          if (mine !== inFlight) return;
          message('Could not load activity.');
        });
    }

    load();
  }

  window.renzoraStatsChart = { mount: mount };
})();
