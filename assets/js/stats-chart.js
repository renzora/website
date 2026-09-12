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

  // One panel's drawing area, in viewBox units. The SVG scales uniformly to the
  // container (width:100%, height:auto), so a stroke stays a stroke rather than
  // being stretched the way preserveAspectRatio="none" would stretch it.
  var W = 720, H = 132, PAD_L = 44, PAD_R = 12, PAD_T = 14, PAD_B = 22;

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

  function x(i, n) {
    if (n <= 1) return PAD_L + (W - PAD_L - PAD_R) / 2;
    return PAD_L + (i * (W - PAD_L - PAD_R)) / (n - 1);
  }

  function y(v, max) {
    return PAD_T + (1 - v / max) * (H - PAD_T - PAD_B);
  }

  /* A smooth path through the points, as a cardinal spline with the tension that
   * keeps it from overshooting into negative territory on a spiky series. A
   * count cannot be less than zero, and a curve that dips below the baseline
   * between two points draws a day that never happened. */
  function linePath(vals, max) {
    var n = vals.length;
    if (!n) return '';
    if (n === 1) return 'M' + x(0, n) + ',' + y(vals[0], max);
    var d = 'M' + x(0, n).toFixed(2) + ',' + y(vals[0], max).toFixed(2);
    for (var i = 0; i < n - 1; i++) {
      var x0 = x(i, n), x1 = x(i + 1, n);
      var y0 = y(vals[i], max), y1 = y(vals[i + 1], max);
      var cx = (x0 + x1) / 2;
      d += 'C' + cx.toFixed(2) + ',' + y0.toFixed(2)
         + ' ' + cx.toFixed(2) + ',' + y1.toFixed(2)
         + ' ' + x1.toFixed(2) + ',' + y1.toFixed(2);
    }
    return d;
  }

  function areaPath(vals, max) {
    var line = linePath(vals, max);
    if (!line) return '';
    var floor = H - PAD_B;
    return line + 'L' + x(vals.length - 1, vals.length).toFixed(2) + ',' + floor
                + 'L' + x(0, vals.length).toFixed(2) + ',' + floor + 'Z';
  }

  function svgEl(tag, attrs) {
    var el = document.createElementNS('http://www.w3.org/2000/svg', tag);
    for (var k in attrs) el.setAttribute(k, attrs[k]);
    return el;
  }

  /* One panel: gridlines, the area, the line, and the hover dot. */
  function panel(series, points, range, uid) {
    var vals = points.map(function (p) { return p[series.key]; });
    var peak = Math.max.apply(null, vals.concat([0]));
    var max = niceMax(peak);
    var total = vals.reduce(function (a, b) { return a + b; }, 0);
    var gid = 'grad-' + series.key + '-' + uid;

    var svg = svgEl('svg', {
      viewBox: '0 0 ' + W + ' ' + H,
      class: 'w-full h-auto block overflow-visible',
      role: 'img',
      'aria-label': series.label + ': ' + total + ' over the last ' + points.length + ' ' + range,
    });

    var defs = svgEl('defs', {});
    var grad = svgEl('linearGradient', { id: gid, x1: '0', y1: '0', x2: '0', y2: '1' });
    grad.appendChild(svgEl('stop', { offset: '0%', 'stop-color': series.color, 'stop-opacity': '0.38' }));
    grad.appendChild(svgEl('stop', { offset: '100%', 'stop-color': series.color, 'stop-opacity': '0.02' }));
    defs.appendChild(grad);
    svg.appendChild(defs);

    // Three gridlines, recessive: they orient the eye and must never compete
    // with the data.
    [0, 0.5, 1].forEach(function (t) {
      var v = max * (1 - t);
      var yy = y(v, max);
      svg.appendChild(svgEl('line', {
        x1: PAD_L, x2: W - PAD_R, y1: yy, y2: yy,
        stroke: 'rgba(255,255,255,0.06)', 'stroke-width': '1',
      }));
      var t1 = svgEl('text', {
        x: PAD_L - 8, y: yy + 3.5, 'text-anchor': 'end',
        fill: 'rgba(161,161,170,0.75)', 'font-size': '10',
      });
      t1.textContent = fmt(Math.round(v));
      svg.appendChild(t1);
    });

    svg.appendChild(svgEl('path', { d: areaPath(vals, max), fill: 'url(#' + gid + ')' }));
    svg.appendChild(svgEl('path', {
      d: linePath(vals, max), fill: 'none', stroke: series.color,
      'stroke-width': '2', 'stroke-linecap': 'round', 'stroke-linejoin': 'round',
    }));

    // Hover furniture, hidden until the pointer is over the card.
    var rule = svgEl('line', {
      y1: PAD_T - 4, y2: H - PAD_B, stroke: 'rgba(255,255,255,0.22)',
      'stroke-width': '1', opacity: '0',
    });
    // The 2px surface ring is what keeps the dot legible where it sits on top of
    // its own line.
    var dot = svgEl('circle', {
      r: '4', fill: series.color, stroke: '#0b0b0f', 'stroke-width': '2', opacity: '0',
    });
    svg.appendChild(rule);
    svg.appendChild(dot);

    return { svg: svg, rule: rule, dot: dot, vals: vals, max: max, total: total, peak: peak };
  }

  function render(root, points, range, uid) {
    root.innerHTML = '';
    var n = points.length;

    var panels = SERIES.map(function (s) {
      var p = panel(s, points, range, uid);

      var head = document.createElement('div');
      head.className = 'flex items-baseline justify-between px-1 mb-1';
      head.innerHTML =
        '<div class="flex items-center gap-2">' +
          '<span class="w-2 h-2 rounded-full shrink-0" style="background:' + s.color + '"></span>' +
          '<span class="text-[11px] uppercase tracking-wider text-zinc-400">' + s.label + '</span>' +
        '</div>' +
        '<div class="flex items-baseline gap-3 text-[11px] text-zinc-500">' +
          '<span><span class="text-zinc-200 font-semibold">' + p.total.toLocaleString() + '</span> total</span>' +
          '<span class="hidden sm:inline">peak ' + p.peak.toLocaleString() + '</span>' +
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
      var t = (ev.clientX - box.left) / box.width;       // 0..1 across the plot
      var px = PAD_L + t * (W - PAD_L - PAD_R) - PAD_L;  // into plot units
      var span = (W - PAD_L - PAD_R);
      var i = Math.round((px / span) * (n - 1));
      i = Math.max(0, Math.min(n - 1, i));

      panels.forEach(function (p) {
        var xx = x(i, n);
        p.rule.setAttribute('x1', xx); p.rule.setAttribute('x2', xx);
        p.rule.setAttribute('opacity', '1');
        p.dot.setAttribute('cx', xx);
        p.dot.setAttribute('cy', y(p.vals[i], p.max));
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
   */
  function mount(opts) {
    var root = opts.el;
    if (!root) return;
    var uid = ++uidSeq;
    var range = opts.range || 'days';

    var tabs = document.createElement('div');
    tabs.className = 'flex items-center gap-1 p-0.5 rounded-lg bg-white/[0.03] border border-zinc-800/50';
    var plot = document.createElement('div');
    plot.className = 'relative mt-3 space-y-3';

    var buttons = RANGES.map(function (r) {
      var b = document.createElement('button');
      b.type = 'button';
      b.textContent = r.label;
      b.dataset.range = r.key;
      b.className = 'px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors';
      b.addEventListener('click', function () {
        if (range === r.key) return;
        range = r.key;
        paint();
        load();
      });
      tabs.appendChild(b);
      return b;
    });

    function paint() {
      buttons.forEach(function (b) {
        var on = b.dataset.range === range;
        b.className = 'px-2.5 py-1 rounded-md text-[11px] font-medium transition-colors ' +
          (on ? 'bg-white/[0.07] text-zinc-100' : 'text-zinc-500 hover:text-zinc-300');
      });
    }

    var head = document.createElement('div');
    head.className = 'flex items-center justify-between gap-3';
    var title = document.createElement('div');
    title.className = 'text-sm font-semibold text-zinc-200';
    title.textContent = opts.title || 'Performance';
    head.appendChild(title);
    head.appendChild(tabs);

    root.innerHTML = '';
    root.appendChild(head);
    root.appendChild(plot);
    paint();

    function skeleton() {
      plot.innerHTML = '<div class="h-[300px] rounded-lg bg-white/[0.02] animate-pulse"></div>';
    }

    function message(text) {
      plot.innerHTML = '<div class="h-[300px] flex flex-col items-center justify-center text-center">' +
        '<i class="ph ph-chart-line text-2xl text-zinc-700 mb-2"></i>' +
        '<p class="text-sm text-zinc-600">' + text + '</p></div>';
    }

    var inFlight = 0;
    function load() {
      var mine = ++inFlight;
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
          render(plot, points, range, uid);
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
