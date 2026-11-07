# Icon gallery

Browse the icons registered by `MDCIcons`. Technology logos use the bundled Devicon font; generic interface symbols use the lightweight SVG registry. Both use the active mdBook foreground colour. Change the mdBook theme to check contrast, or filter the grid below.

Use a registered name such as `icon=rust` on a Tab, or include an appropriately classified filename in a File Tree. Unknown brand names are not guaranteed to have a Devicon font glyph.

<div id="mdc-icon-gallery" class="mdc-icon-gallery">
  <input id="mdc-icon-search" type="search" placeholder="Filter icons" aria-label="Filter icons" />
  <div id="mdc-icon-grid" class="mdc-icon-gallery__grid"></div>
</div>

<style>
.mdc-icon-gallery { margin: 1.25rem 0; }
.mdc-icon-gallery input { box-sizing: border-box; width: 100%; max-width: 26rem; padding: .5rem; margin-bottom: 1rem; }
.mdc-icon-gallery__grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(135px, 1fr)); gap: .5rem; }
.mdc-icon-gallery__cell { border: 1px solid var(--table-border-color); border-radius: .35rem; padding: .75rem; display: flex; gap: .65rem; align-items: center; min-width: 0; }
.mdc-icon-gallery__cell > :first-child { font-size: 1.65rem; flex: 0 0 1.65rem; }
.mdc-icon-gallery__cell code { font-size: .8rem; overflow-wrap: anywhere; background: transparent; }
</style>
<script>
(() => {
  function start() {
    const grid = document.getElementById('mdc-icon-grid');
    const search = document.getElementById('mdc-icon-search');
    if (!grid || !search || !window.MDCIcons) return;
    const names = [...window.MDCIcons.names].sort();
    function render() {
      const query = search.value.trim().toLowerCase();
      grid.replaceChildren();
      for (const name of names.filter(n => n.includes(query))) {
        const icon = window.MDCIcons.create(name);
        if (!icon) continue;
        const cell = document.createElement('div');
        cell.className = 'mdc-icon-gallery__cell';
        const label = document.createElement('code');
        label.textContent = name;
        cell.append(icon, label);
        grid.append(cell);
      }
    }
    search.addEventListener('input', render);
    render();
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, {once:true});
  else start();
})();
</script>
