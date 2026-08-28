# TODO

Working list for `nocomponents`. Status reflects the tree as of 2026-08-25; check items off as they
land and add new ones rather than keeping a parallel list elsewhere. Closed items are one line —
the reasoning is in the commit that closed them, and anything still to do about one is its own open
item rather than a sentence buried in a closed one.

## Bugs

- [x] Dialog content resolved a *different* dialog's context: leptos' `Portal` mounts under an
      owner of its own, which a bare `provide_context` does not reach. Scoped with `<Provider>`.
- [x] `animate.css` fill-mode fallbacks were dead code — a registered `@property` resolves to its
      initial value, not to the `var()` fallback.
- [x] The dialog plays its exit animation: the portal's `Show` gates on `is_mounted`, not `is_open`.
- [x] Closing floating content swallowed clicks for `unmount_delay`; the positioning shells are
      permanently `pointer-events-none` and the styled content re-enables itself.
- [x] Selecting a submenu item left the parent menu open — `MenuRoot` publishes the outermost
      context alongside the one the submenu's own `FloatingRoot` shadows.
- [x] A delayed unmount could fire into a disposed owner and panic; both roots clear the
      `TimeoutHandle` in `on_cleanup`.
- [x] Colour picker rails took the press and went dead for the rest of the drag: `set_rgba` read
      the colour *tracked*, so a lossy round trip undid every move as a stale write. Reads
      untracked now.
- [x] One `crop_image` over bytes rather than two functions over two inputs — every caller was
      sniffing the type and branching, and the avatar block silently flattened picked GIFs.
- [x] Cropper drag modifiers: shift holds the grabbed shape from the far corner, alt holds it about
      the centre, and shift on a move snaps to eight directions. `DragPoint` carries the modifiers,
      so they can be taken up or let go mid-drag.
- [x] `utils::use_media_query` — one `match_media`, re-read on `resize`, so what it says and what
      CSS is doing cannot drift.
- [x] `Select` wears the `Field` around it like every other control; the field is handed the
      trigger's own id rather than minting a second one no element wears.
- [x] `SidebarMenuButton` takes a `node_ref`, so a dropdown can anchor to a sidebar row.
- [x] A toast raised from a view that disposes itself in the same handler was a panic:
      `show()` minted `is_open` and `height` under the caller's owner. `ToastContext` carries the
      provider's `Owner` and `show()` runs inside it.
- [x] A `FieldLabel` beside a `Combobox` pointed `for` at an id nothing wore. The wiring the select
      already had is now `field_control_for_trigger`, shared by both.

- [x] The ⌘K palette navigated to `/base/base/docs/…` on the Pages build: `use_navigate` resolves
      against the router's base and was handed a path `href()` had already prefixed. `A` is the
      other way round, which is why only the palette broke — and why `trunk serve` never showed it.
- [x] Switching pages reset the sidebar's scroll to the top. Every page rendered its own
      `DocLayout`, so navigating rebuilt the whole sidebar; the chrome is now `DocShell` on a
      `ParentRoute` at `/docs` and only the `Outlet` swaps.
- [x] The sidebar is on `utils::use_media_query`, and `SidebarContext::is_mobile` is a `Signal`
      rather than an `RwSignal` — derived from the viewport, so nothing but the window decides it.
      Not the drop-in it looked: the old `resize` handler did two things, and closing the mobile
      overlay when the breakpoint is crossed is now its own effect, keyed on the *change* rather
      than the value so it cannot shut an overlay the reader just opened.
- [x] A floating trigger overwrote a caller's own `id`. It now adopts an id already on the node as
      `trigger_id` instead, so a `<label for>` still names the trigger and everything downstream —
      the field's `control_id`, the content's `aria-labelledby` — follows the caller's id rather
      than a minted one. A trigger with no id of its own still gets one.
- [x] A menu item can be disabled. `DropdownMenuItemRoot` takes `disabled`, writes `data-disabled`
      and `aria-disabled`, and refuses the click in the handler rather than trusting the styled
      layer's `pointer-events-none` — so an item styled by someone else refuses it too. One root, so
      the dropdown, the context menu and the menubar all got it; `DropdownMenuItem` and
      `MenubarItem` pass it through.
- [x] The `<select>` is `NativeSelect`'s own root, so `attr:id`, `attr:name` and any event land on
      the control instead of on a wrapper. The chevron is a background image, since a pseudo-element
      does not render on a `<select>` and there is no way to mask one to a token: two baked greys,
      light and dark, which are the values `--muted-foreground` has in both default palettes. A
      custom palette's own muted-foreground will not reach it — the price of the wrapper going.
      `on_blur` stays as a convenience, but its reason for existing is gone and its doc says so.
- [x] `DropdownMenuTrigger` and `PopoverTrigger` build their `Button` as the `render` the root
      already takes, instead of around `ctx.trigger_ref` behind the root's back. They were doing
      what `ComboboxTrigger` does — it was never the only one, and `SelectTrigger` was the only one
      doing it right. Found by the `data-slot` work: a trigger that skips its root gets nothing the
      root writes onto the node.
- [x] `ComboboxTrigger` goes through its root too, and the duplicated `field_control_for_trigger`
      is gone with it — the root does the field wiring once and hands the resolved `disabled` back.
      What made that possible: the trigger callback is a **`TriggerRender` struct** now, not a
      tuple. `(AnyNodeRef, Callback<MouseEvent>)` had to grow a third element, and a tuple has to
      be re-broken every time it does; a struct takes a new field and leaves every caller alone.
      Verified where it matters — a combobox inside a `<Field disabled=true>` still comes out
      disabled, which is exactly what routing it through the root would otherwise have dropped.
- [x] `FloatingTrigger` takes a `data_slot`, written onto the node rather than into the markup —
      the trigger is its own `<button>` in one place and whatever `render` returned in another, and
      the node ref is all the shapes have in common, so one path covers both. `dropdown_menu` and
      `select`, which had no `data-slot` anywhere, now carry one on every part they own: the
      trigger, the content, the item, and the submenu's trigger and panel.
- [x] Every component carries a `data-slot` now. The audit found twelve with none at all — badge,
      button, checkbox, data_table, date_picker, input, label, progress, spinner, switch, textarea,
      toast — which was most of the ones people reach for first. The rule: the primitive writes it
      where there is one, the styled layer writes it for the style-only components.
- [x] The colour picker's alpha thumb is clipped top and bottom. The `overflow-hidden` was never
      doing anything: a background is already clipped to its own element's border radius, so the
      chequer rounds itself and the rail inside it is `rounded-full` too. Dropping it lets the 2px
      of thumb that stands proud of the `h-3` rail render, and the alpha thumb is now the same full
      circle as the hue thumb above it.

- [x] The carousel's Next stayed enabled past the last reachable slide when more than one was on
      screen: `can_scroll_next` was `index + 1 < count`, which counts *slides* rather than scroll
      positions. `CarouselContext::last` is the bound now — the track's own scroll extent, and the
      last slide whose offset still fits inside it — read in the same pass as the current index,
      since both come from one walk of the slides and that walk happens on every scroll event.

      Two things it needs beyond the arithmetic. The bound is a **measurement**, so it is `None`
      until one has been taken and falls back to counting slides in the meantime: a bound of zero
      means "nowhere to go", and a carousel measured before its track has a box would wear that as
      one — disabling the Next that is the only thing that would have scrolled it and measured it
      again. And a `ResizeObserver` on the track is what re-measures, because leaving a docs page
      and coming back mounts one exactly that way; that was the reported symptom, a Sizes demo
      stuck showing its first three slides with a dead Next.

- [x] A dragged kanban card drifted from the cursor if anything scrolled that the board did not
      scroll itself. `KanbanContext::scrolled` accumulated only the board's *own* auto-scroll
      steps, so it was blind to the page scrolling, a wheel flick, or a column scrolling. Replaced
      rather than added to: the card's own container is measured at the press, and every move takes
      the difference — its rect answers for everything outside it, its scroll offsets for the one
      thing that moves the card without moving it. Recomputed on the edge-scroll timer, which is
      already the gesture's heartbeat, so a pointer held still is covered too. Verified in the
      browser: an 80px page scroll used to cost 80px of drift and now costs about two.

- [x] `Button` renders `ButtonRoot`, which was rendered by nothing. Filling it rather than
      deleting it, because there turned out to be behaviour for it to hold and every one of these
      was missing: `Button` had **no `disabled` prop at all**, so every caller wrote
      `attr:disabled` and a button inside a disabled `<Field>` stayed live — the bypass again, one
      layer down. The root resolves `disabled` through the new `field::field_disabled` (the field
      *and* the fieldset, since a `<fieldset disabled>` does not reach a row that renders as an
      `<a>`), refuses the click in the handler rather than trusting `disabled:pointer-events-none`,
      and takes `type` as a prop defaulting to `ButtonType::Button` — HTML's default is `submit`,
      which is why three demos were carrying `attr:r#type="button"` to undo it.

      `render` moved onto the root too, so both shapes go through one place, and `StyledRender`
      grew a third field: the resolved `disabled`. That is what the struct was for — every caller
      written against the two it had kept compiling. It matters because `disabled` is a
      `<button>`'s attribute and means nothing on an `<a>`: the root writes `aria-disabled` and
      `data-disabled` onto the node for that shape, and the styled layer grew the matching
      `aria-disabled:` classes, which `SidebarMenuButton`'s were already written against.
- [x] All three render callbacks are named structs: `TriggerRender` for the floating triggers,
      `StyledRender` (class + node ref) for `Button` and `SidebarMenuButton`, `AnchorRender` (node
      ref alone) for the tooltip and hover card. Kept separate rather than merged into one: a
      tooltip trigger has no click to run and no classes to wear, and handing it fields that mean
      nothing where it stands is worse than three honest shapes. Each can now grow on its own.
- [x] The canvas took the primary button and left the caller nothing. `pan_on_drag` is off by
      default now: a press is a click, and panning is space and drag, the middle button or two
      fingers — which is where every editor since Figma has put it. The cost of the old default was
      invisible until you tried to build on the thing: a board that pans on press has no gesture
      left to select a node with, none to drag one by, and nowhere to put a click on empty space
      that means "deselect". `on_surface_click` is the other half — a click that landed on the
      board rather than on anything placed on it, reported in *world* coordinates, which is the
      part a caller cannot work out for itself since it takes the surface's own box and the current
      transform. It stays quiet for a click on a node and for the click that ends a pan. The three
      pan states are one `data-pan` attribute (`ready`/`active`/absent) rather than a flag each:
      they overlap while space is held through a drag, and two attributes styled separately leave
      which cursor wins to stylesheet order.
- [x] Canvas nodes were something you could look at and nothing else. `CanvasNodeRoot` reports its
      own gestures now — `on_move` for a drag on the body, `on_resize` for one of the eight grips,
      and it still applies neither: the caller keeps the list, as with the kanban board. What the
      node owns is the arithmetic nobody should write twice, since a pointer travels in screen
      pixels and a node lives in world units, and the zoom to divide by is in the canvas' context
      that only something inside the canvas can reach. The resize itself is `WorldBox::resized` in
      `utils::viewport`, unit-tested off the DOM like the rest of that module, and measured from
      the **press** rather than from the last move: clamped incrementally at the minimum the
      remainder is thrown away, and the box then lags the pointer by however far it was dragged
      past the limit. `aspect` keeps a ratio and draws only the corners, a side handle having no
      opposite edge to hold still on the axis it does not move.
- [x] The grips are sized in screen pixels and divided by the zoom before they are drawn — a 10px
      square is 1px at 0.1× and covers its own node at 4×. Everything else on a board is meant to
      scale and a control is not, which is the same argument the background dots already won. The
      surface publishes `--canvas-zoom` for the CSS that has to make the same division: there is no
      class for "one pixel however far in we are", but `calc(1px / var(--canvas-zoom))` is one.
- [x] `CanvasNodeLabel` — a node's label that becomes an input on double-click. Two rules pointing
      opposite ways: the text lets a press through, so a sticky is still dragged by its words; the
      input stops one, that press being the caret's. The input takes the node's own font and colour
      rather than a browser's defaults, and its background is a tint of `currentColor`, so it
      blends on an amber sticky and on a bordered shape alike. Escape restores — which needed a
      flag, because Chrome fires `blur` at an element it removes from the document and the blur
      would otherwise commit the text Escape had just thrown away.
- [x] `Textarea` takes a `node_ref`. The selection is the one piece of a textarea's state that
      lives nowhere but the DOM, so a toolbar that formats what the reader has highlighted cannot
      be written without reaching the element.
- [x] The canvas node's label was an `<input>`, and the text it stands in for wraps. So a sticky's
      words reflowed onto one line the moment you began editing them and jumped back when you
      stopped — the one moment an editor should be still. A `<textarea>` now, `rows="1"` and sized
      by its content, wearing the node's own font, colour and alignment through `[font:inherit]`
      and `text-inherit`: a browser gives a form control none of that, which is why it looked like
      somebody else's element sitting in the note. Measured at 126×77 idle and 126×77 editing.
- [x] Editing wore a tinted background and a ring. Both gone — the caret is the only thing that
      says an edit is happening, which is the only thing that needs to.
- [x] Enter left the note instead of breaking the line. It now does what it does in every other
      multi-line field, and everything else ends the edit **keeping** the text: Escape,
      ⌘/Ctrl+Enter, and clicking away. No cancel, on purpose — the words under the caret are the
      note, so a key that silently threw them away would be a key nobody presses twice. The two
      keys stop propagation as well as preventing the default, or the key that ended a rename goes
      on to close the dialog the board is sitting in.
- [x] Escape committed the very text it was meant to discard. Chrome fires `blur` at an element it
      takes out of the document, so the blur spoke after the key had. A `settled` flag, cleared
      when an edit *begins* rather than by the blur, so a browser that does not send that blur
      cannot leave it set for the next edit. Escape commits by design now, but the flag still
      earns its place: without it a caller that **rejects** an edit sees the same one arrive twice.

- [x] **A multi-line label read as one line when it was not being edited.** A span collapses `\n`
      to a space, so a note typed over three lines came back as one — which Enter breaking the
      line had just made easy to hit. `whitespace-pre-wrap` on the styled label fixes it, and this
      is now **verified in a browser** rather than only compiled: the span computes `pre-wrap`,
      and a three-line note committed with Escape comes back 60px tall at a 20px line-height —
      three lines, with the newlines still in the text.
- [x] `VideoPlayer`'s subtitles sat under the control bar, and the lift meant to clear them was
      dead code twice over. `[&::-webkit-media-text-track-container]:-translate-y-20` generated
      the rule it was supposed to, and Chrome ignored it: Tailwind v4 writes the independent
      `translate` property, and that pseudo-element honours only `transform`. Firefox has no such
      pseudo-element at all, so there was never a version of this that worked in both.

      **So the player draws the cues itself.** The active track is set `Hidden` rather than
      `Showing` — which keeps `activeCues` current and `cuechange` firing while stopping the
      browser painting — and `VideoPlayerContext::cues` becomes ordinary elements that can be
      positioned, themed and animated. They step above the bar on the same `data-active` the bar
      reads and drop back when it goes.

      Three things fell out of it. Cue text is the source as authored, so `cue_text` drops WebVTT
      markup (`<i>`, `<v Roger>`, the karaoke timestamps) and decodes the six character references
      the format defines — unit-tested, since it is string arithmetic. Sizes are a reader
      preference (`VideoPlayerCaptionSize`, a submenu) written in `cqw` against a container on the
      caption box, so one choice holds from a thumbnail to fullscreen. And both demo `.vtt` files
      carried a bare `<track>`, which a browser reads as a tag and drops — the cue really did say
      "passed in as a  child".

      **The cost, written down because it is invisible until someone hits it**: the browser's VTT
      layout goes with its rendering, so cue settings that place a line somewhere other than the
      bottom (`line`, `position`, `align`, `size`, `vertical`) are not honoured, and the box is
      positioned against the player rather than the video frame — so on a letterboxed video the
      caption sits in the black bar rather than over the picture.
- [x] The pointer now goes away with the bar: the player wears `cursor-none` and takes it back on
      `data-[active=true]`, which is the same fact the bar fades on, so the two cannot disagree.
      Gated on `mode.has_controls()` — a mode that draws no bar has no bar going down, and a
      cursor vanishing over a video with no controls is a cursor the reader has lost. Child rules
      still win over the inherited value, so the scrubber keeps its `cursor-pointer`.
- [x] Menus were misplaced by a constant offset, and only after this session's own change: the
      caption sizes needed a container to measure against, and `@container/player` on the root
      compiles to `container-type: inline-size`, which applies layout containment — which makes
      the element **a containing block for `position: fixed` descendants**. The floating shell is
      fixed, so floating-ui's viewport coordinates were read against the player's origin: every
      menu on the page was off by exactly the player's top-left, right in fullscreen (origin 0,0)
      and wrong on the way out. The container belongs on the caption box, which contains no
      floating layer.

      **Anything that establishes a containing block — `transform`, `filter`, `contain`,
      `container-type`, `will-change` — breaks a fixed-positioned floating layer under it.** The
      symptom is a constant offset equal to that ancestor's origin, which reads like a
      positioning bug in the menu rather than a property on something else entirely.
- [x] A submenu vanished in fullscreen. `DropdownMenuSubContent` hard-wired a `<Portal>` to
      `document.body` while the top-level content leaves portalling to the caller — and only the
      fullscreen element's own subtree is rendered. Deleting the portal was not the fix: the
      parent content is `overflow-hidden` and is between the submenu and its containing block, so
      an un-portalled submenu is clipped instead. `FloatingPortal` (`primitives/floating.rs`)
      mounts into `document.fullscreenElement` when there is one and the body otherwise,
      re-reading it on `fullscreenchange` since that changes under a layer already open. Used by
      `DropdownMenuPortalRoot` too, so every portalled menu gains it.

      With submenus working, the player's own menu folds: `submenu=true` on the three provided
      groups turns fourteen rows into three, which at 106px instead of 528px is the difference
      between a menu and a wall.

## Accessibility & keyboard

All closed bar the canvas, which arrived after this list was written and brought its own hole with
it. This no longer blocks a "stable" tag. What exists, and where to reuse it:

- [x] `primitives/dismiss.rs` — one layer stack ordered by opening time. Escape dismisses the
      topmost; an outside `pointerdown` walks down until a layer contains the target or is modal.
- [x] `primitives/roving_focus.rs` — a group is one tab stop and the arrows move inside it. Items
      are found by `data-roving-item`, so it does not care how they are rendered. Used by the
      dropdown menu, select, tabs, toggle group, radio group and accordion.
- [x] `primitives/typeahead.rs` — keystrokes accumulate for a second, a repeated letter cycles. In
      the select and, via `handle_menu_keys`, in all three menu surfaces.
- [x] Arrow keys open a closed trigger; the intent reaches the content as
      `FloatingContext.open_focus`.
- [x] Dialog focus: the Tab trap, the focus-in, and focus returned to whatever had it on close.
- [x] Floating triggers carry `aria-haspopup` / `aria-expanded` / `aria-controls`, written onto
      `trigger_ref` from an effect — the trigger comes in three shapes and the node ref is all they
      share.
- [ ] Not all closed after all: a canvas node cannot be reached by the keyboard at all. Filed with
      the rest of the canvas work rather than here, since it shares a design with the selection
      model — see *Canvas: what a collaboration tool needs*.

## Canvas: what a collaboration tool needs

The canvas exists because of `novoice`, a collaborative editor that will be built on it, and this
is the gap between what is in the tree and what that needs. **Ordered, not a bag**: each group is
what has to be true before the next one is worth starting, and the reason is written down for
each — a list of features with no argument for their order is a list somebody will reorder by
whichever is most fun.

What is already right, and should not be redone: the canvas **never owns what is on it**. It
reports where a drag would put a node and the caller decides, so a CRDT can be the single source
of truth without competing with local state, and snapping, grids and constraints are arithmetic
applied to the reported point rather than features the component has to grow. The viewport is a
plain signal, which makes follow-the-presenter three lines. The arithmetic is in `utils::viewport`,
off the DOM and unit-tested. And a resize is measured from the press rather than accumulated,
which matters more under network lag than it ever did locally.

### 1. Blocking — a document cannot be edited by two people without it

- [x] **`on_gesture_end` on `CanvasNodeRoot`**, carrying a `CanvasGesture { from, to, handle }`.
      `on_move` and `on_resize` still fire on every pointer move; this fires once, when the
      gesture is over, which is the distinction a shared document is built on — ephemeral presence
      while the pointer is down, one committed operation when it comes up, and one undo entry per
      drag rather than forty.

      **One callback rather than the `on_move_end` / `on_resize_end` this asked for.** What a
      caller does with either is the same job — write an operation somebody can replay — so two
      hooks meant handing the same body to both, and `handle` (`None` for a move, `Some` for the
      grip) is a field that says which it was and reads better than the callback's name. The same
      argument the render callbacks settled: a struct grows a field and leaves callers alone.

      Two things that only showed up in the building. `use_drag` ends a gesture on release whether
      or not the pointer ever moved, so a plain click on a node was an operation on the wire; it
      is silent now when nothing changed, because selecting is not editing. And *where it ended*
      is read back out of the caller's own signals rather than recomputed from the pointer — so a
      caller that snapped to a grid, clamped to a lane or refused the move outright reports what
      it did rather than what it was asked to do.

### 2. Settle before there is data, because it is free now and expensive later

- [x] **Rotation: the field is reserved, the gesture is deferred.** `WorldBox` carries an `angle`
      in radians about its own centre; `CanvasNodeRoot` takes it, renders it as a `transform` and
      folds it into every `CanvasGesture`. So a board's saved shape and its operations are already
      the ones rotation needs, and the part that was actually expensive — migrating boards made in
      the meantime — cannot arise.

      `new` still takes four numbers and `rotated()` adds the angle, so nothing that built a box
      before had to change. The transform is written **only when the angle is not zero**, so the
      upright node that every board is made of is the same element it always was rather than one
      the browser has to composite.

      What is deferred, and where it says so in the source rather than only here: `resized` takes
      its delta in world axes, which stop being the box's axes once it is turned, so it carries the
      angle through untouched and is documented as correct only for an upright box — and a node
      with an angle **draws no grips at all**, because eight grips that grow a box in the wrong
      direction are worse than none. `WorldBox::around` has the matching gap: it takes each box as
      upright, so it under-covers a turned one by however far its corners swing out.

- [ ] **The rotation gesture itself**, when novoice wants it. Three pieces, and the field above is
      not one of them: rotate the pointer delta into the box's own frame before `resized` and back
      after; project the four corners in `around` so fit and cull stop under-covering; and a ninth
      grip to turn it by, which is the only new thing on screen.

### 3. Before it is a whiteboard

- [ ] **Pinch on touch.** `use_drag` follows one pointer and nothing anywhere tracks a
      `pointerId`, so there is no two-finger anything. Zoom works on a trackpad only because a
      browser disguises a pinch as `ctrl+wheel`, and on a tablet there is no zoom at all — the
      styled surface sets `touch-none`, so the browser will not do it either. Two pointers on the
      surface should scale about the midpoint between them and pan by its travel, which is one
      gesture rather than two. Belongs in `drag.rs` if a second primitive wants it, and in the
      canvas if none does.
- [ ] **A drag on the surface, not only a click.** `pan_on_drag=false` freed the primary button
      and `on_surface_click` gives back a single point — which is enough to place something and
      nothing else. A marquee, a lasso, and drawing a shape by dragging it out all need the press,
      the live rectangle and the release. `on_surface_drag` reporting a `WorldBox` per move and at
      the end, with the same press-relative arithmetic the node's resize already uses.

### 4. Before it is big

- [ ] **Z-order as a prop.** Depth is DOM order today, so "bring to front" is a move within the
      caller's list, which changes the keys and rebuilds the markup. Fine at four nodes, wrong at
      four hundred, and a collaborative board will reorder from a peer's operation rather than
      from a button. A `z` prop written into `z-index` costs nothing and makes depth a number two
      clients can agree on.
- [ ] **Cull what is off screen.** Every node is mounted, always. A board is the one component
      here where the count is the reader's to decide, and thousands of absolutely-positioned
      elements is where this stops being usable. The viewport already knows the visible world
      rectangle, so the test is arithmetic the caller could do — but it needs the rectangle, which
      means publishing it: `CanvasContext::visible()`.
- [ ] **`content_bounds` is coupled to that.** It reads every `[data-slot=canvas-node]` out of the
      DOM and takes each one's rect — a forced layout per call, which is fine for a button press
      and not for a loop, and *silently wrong* the moment culling lands, since it would fit only
      what happens to be mounted. Whichever of the two is done first has to answer for the other:
      either fit takes the caller's own bounds, or culling keeps a way to measure what it hid.

### 5. Before it is a product

- [ ] **Multi-select gestures.** `selected` is a `Signal<bool>` per node, so the *state* is already
      the right shape — what is missing is shift-click, marquee (see the surface drag above), and
      a move or resize that applies to a set. A group resize is one `WorldBox` around the
      selection and each member scaled inside it, which is arithmetic for `utils::viewport` and a
      test rather than anything new in the DOM.
- [ ] **Nodes are not reachable by keyboard.** The surface is a tab stop and the things on it are
      not: nothing on a board can be selected, moved or renamed without a pointer. Wants a roving
      tab stop over the nodes (`primitives/roving_focus.rs` already does the hard part), arrows to
      nudge, Enter to edit the label, and the same `data-selected` the pointer path writes.
- [ ] **World coordinates from outside the canvas.** `CanvasContext::world_at` is only reachable
      by something inside the provider, and the board block hit that immediately: its palette
      lives in the sidebar, so the drop had to re-derive the conversion from the viewport signal
      and the box the canvas was wrapped in. Paste-at-pointer and a file dropped from the desktop
      are the same problem. `Viewport` has the arithmetic; what is missing is the surface's own
      rectangle, published rather than measured again by hand.

## Components — gap vs the shadcn/ui catalogue

Shipped (59): accordion, alert, alert-dialog, aspect-ratio, avatar, badge, breadcrumb, button,
button-group, calendar, card, carousel, chart, checkbox, code, collapsible, color-picker,
combobox, command, context-menu, data-table, date-picker, dialog, drawer, dropdown-menu, empty,
field, hover-card, image-cropper, input, input-group, input-otp, item, kbd, label, menubar,
native-select, navigation-menu, pagination, popover, progress, radio-group, resizable, scroll-area,
select, separator, sheet, sidebar, skeleton, slider, spinner, switch, table, tabs, textarea, toast,
toggle, toggle-group, tooltip. `code` is not in the shadcn catalogue — it is ours, for documenting
the rest.

**P3 — substantial state machines.** All closed, and three of them are what the docs site itself
runs on: the sidebar is the nav, the command palette is ⌘K, and the data table is the only honest
way to find out what living with a generic row type is like.

- [x] Command — unmatched items are not rendered, so document order *is* arrow order. The highlight
      moves through `aria-activedescendant`, which is why it could not reuse `roving_focus`.
- [x] Combobox — the command palette inside a floating layer. `ComboboxItemRoot` is the whole
      bridge: it writes the value and closes the layer.
- [x] Navigation Menu — links rather than commands, so leaving is decided at the menu, not per
      trigger.
- [x] Menubar — a row of dropdown menus with one bar over them, reusing the dropdown's surface,
      items and submenus whole.
- [x] Resizable — percentages, never pixels, with `flex-grow` *being* the percentage.
- [x] Carousel — scroll snapping rather than a transform, so a trackpad and a touch drag come free.
- [x] Calendar — over `utils::date`, so there is no `chrono` for consumers to agree with. Focus
      follows a *date*, not a cell, which is what steps off the end of a month page.
- [x] Date Picker — a popover holding a calendar, plus the one rule that makes it a picker: the
      layer closes when the selection is finished.
- [x] Table sort/selection handles, then Data Table — plain `Copy` handles rather than contexts, so
      they work over a hand-laid-out table too.
- [x] Chart — line, area and bar as one `<svg>`, geometry in viewBox units, so nothing is measured
      and there is no resize listener.

**P4 — app-specific pieces.** Useful only once the core set is solid; several are chat/AI surfaces
rather than general UI, so they are the easiest to defer or skip.

- [ ] Direction (RTL provider; cheap, but only pays off once enough components read it)
- [ ] Marker
- [ ] Attachment
- [ ] Bubble
- [ ] Message
- [ ] Message Scroller
- [ ] Questionnaire

**Beyond the shadcn catalogue.** Ours, and none of them took a dependency.

- [x] `primitives/drag.rs` — `use_drag`, with `.against()`, `.on_start` and `.on_end`. `slider`,
      `scroll_area`, `drawer` and `resizable` are folded onto it: 83 lines of listener bookkeeping
      gone, and all four now end on `pointercancel`, which every hand-rolled copy missed.
- [x] `utils::color` — `Rgba` / `Hsva` / `Hsla` / `Oklch`, a parser for every CSS form this library
      writes, a formatter, 9 tests. HSV↔HSL is exact where anything through 8-bit RGB is not, which
      is why a picker stores `Hsva`; and `--chart-1` does not fit in sRGB, so reading one of the
      theme's own tokens back clips it a few degrees round the hue circle.
- [x] `utils::image` — a normalised crop rectangle, `moved_by`, `resized` holding an aspect ratio,
      and a canvas for the output. 12 tests.
- [x] `utils::gif` — cropping an animated GIF at the LZW level, so palettes, delays, disposal,
      transparency and the loop count survive by never being decoded. 10 tests plus a browser check.
- [x] Color Picker — holds HSVA, so the hue survives the black corner. The square is two gradients
      over a pure hue; the two rails are one component twice.
- [x] Image Cropper — a still through the canvas, an animated GIF through `crop_gif` and still
      animating afterwards: 16 frames in, 16 frames out, checked with Chrome's own decoder.
- [x] `kanban` — a board of columns of cards, on `use_drag` like every other gesture here, so it
      ends on `pointercancel` and no card is left stuck to the cursor when the browser takes the
      pointer away.

      **The board does not own the cards.** It reports `on_move` — the card, the column it left,
      the column and index it landed at — and the caller applies that to their own list. Same
      stance as `DataTable` sorting what it is handed: a board that reordered a list it did not own
      would disagree with the caller's the moment anything else touched it.

      Where a card would land is read out of the DOM (`data-kanban-column`, `data-kanban-card`)
      rather than from a registry the parts keep in step, so columns and cards can come and go
      without telling anybody. The card in hand is skipped while counting, or the index it came
      from reads one too far down. A card let go where it started reports nothing.

      A press is not a drag: the gesture only announces itself once the pointer has travelled four
      pixels, and a press that never does is a click, reported as `on_select`. That has to be a
      prop rather than the caller's own `on:click`, because a card following the pointer is
      `pointer-events: none` and the release lands on the column underneath — a card could not
      receive a click at all until this. Only the gesture knows whether the pointer moved.

      The card lifts and follows the pointer, by `transform` rather than by leaving the list: the
      gap it came from stays open, so no column reflows mid-gesture and the slot under the pointer
      does not shift as the card moves. It goes `pointer-events: none` while it travels, so what
      is under the cursor is whatever it is over rather than itself. The transform is never
      transitioned — easing a thing that is following a cursor makes it lag by the duration. The
      line showing where it will land is a `KanbanSlot` the caller renders between its own cards,
      because only the caller knows where "between" is.
- [x] The board scrolls itself while a card is held near an edge, ramped from nothing at the outer
      edge of a 72px zone to 16px a tick at the board's own edge. On a timer rather than on pointer
      moves: a pointer held still at the edge sends no more events, and that is exactly when the
      reader is waiting for the board to come to them.

      Two things it needs that are not obvious. The card rides inside the scroller, so the board
      scrolling out from under it slides it away from the cursor — the distance scrolled is added
      back into its offset. And a card translated to the right *grows* the board's scrollable
      width, so scrolling to the live maximum runs past the real end and snaps back the moment the
      card is let go; the range is measured once at the press, while nothing is translated, and
      the scroll clamped to that.
- [x] `floating_menu` — a panel the reader can pick up and put somewhere else. Fixed to the
      viewport rather than placed in the flow, because that is what "move it out of the way" means:
      it stays where it was put while the page scrolls under it. Everything else follows — the
      position is client coordinates, and the clamp is against the window.

      **Dragged by a handle, not by the whole panel.** A panel with buttons in it that also moves
      when you press a button is a panel you cannot use; a caller who does want the whole thing to
      move puts the handle around everything, which is their decision and not this one's.

      The clamp keeps 32px reachable off either side and off the bottom, but never lets the panel
      above the top: there is nothing above the top edge to grab it back by. It is a pure function
      taking the viewport as an argument — `window()` *panics* off-wasm rather than returning zero,
      so a clamp that read it directly could not be tested at all. Five tests on the rules, and the
      window read in one place beside them.

      The position is a plain `RwSignal` the caller may pass in, which is as useful for reading
      where the reader left it as for driving it.

      Two shapes and two densities. **Horizontal by default** — a strip of actions floating over
      the thing they act on is what this kind of menu usually is; vertical is the variation and
      turns it into a panel. The handle moves to the start of whichever it is, a grip along the top
      of a strip being a grip nobody can hit, and the separator turns across the flow with it.
      `compact` drops the labels for the icons and gives each item a tooltip and an `aria-label`
      instead, which is what the sidebar does when it collapses to a rail — a label taken away
      still has to be readable somehow.

      `FloatingMenuItem` renders its own `<button>` rather than a `Button`: `ButtonSize` is a value
      and not a signal, so a menu that changes density while it is on screen could not change the
      size of its items with it.
- [x] `canvas` — an infinite surface you can pan and zoom, with things placed on it in world
      coordinates. The FigJam shape rather than the drawing-app shape.

      **The canvas owns where you are looking, and nothing else.** Nodes take their world position
      as props and the caller keeps the list — the same stance as the kanban board reporting a move
      rather than applying it, and for the same reason: a canvas keeping its own copy would
      disagree with the caller's the moment anything else edited a node.

      The arithmetic is `utils::viewport`, which knows nothing about the DOM and has 10 tests.
      That split is not tidiness: `window()` *panics* off-wasm, so a viewport that read its own
      size could not be tested at all — the same argument as `floating_menu`'s clamp. One
      convention holds it together, **screen = world x zoom + offset**, which makes panning
      addition that never has to know the zoom.

      The test that earns its keep is zooming about the pointer. Zooming toward the middle of the
      surface is one line and wrong — whatever is under the cursor has to stay under it — and the
      anchor is taken *before* the zoom changes with the offset re-derived after, so clamping
      cannot break the promise: a zoom refused at the limit leaves the surface exactly where it
      was rather than sliding it by the difference.

      Two things about the gestures. A browser reports a **trackpad pinch as ctrl+wheel** — there
      is no pinch event on the desktop — so that flag rather than a modifier anybody pressed is
      what separates zooming from two-finger panning. And `DragPoint` reports distance from the
      *press*, so panning by it on every move accelerates the canvas away from the pointer; the
      previous delta is remembered and the difference applied. Verified in a browser: ten pointer
      steps totalling 80x40 pan the canvas exactly 80x40.

      The dotted background sits **outside** the transformed layer. Inside it the dots would scale
      with the zoom and blur out at 4x; a repeating background whose spacing follows the zoom and
      whose dots do not is what paper actually looks like.

      Fit reads the nodes out of the DOM (`[data-slot=canvas-node]`) rather than from a registry,
      so nodes can come and go without telling anybody, and an empty board is left alone rather
      than fitted to nothing.

      Styled: `Canvas`, `CanvasNode`, `CanvasSticky` (four colours), `CanvasShape`
      (rectangle/ellipse), `CanvasText`, `CanvasControls`, `CanvasBackground`. Cost two icons,
      `plus` and `minus`.

- [ ] Canvas, pass two: selection and moving things. Click, shift-click and a marquee over empty
      space; dragging a selection by transform and reporting once on release, the way a kanban
      card does. `pan_on_drag=false` exists for exactly this — a canvas whose empty space means a
      marquee rather than a pan.

- [ ] Canvas connectors: edges between nodes, with anchors, arrowheads and re-routing as nodes
      move. The largest of the three and the one that most wants its own primitive, since routing
      is arithmetic and belongs beside `utils::viewport` rather than in a component.

- [x] `timeline` — a sequence of events along an axis, vertical for now.

      **A list, not a scaled axis.** Events sit evenly spaced in document order and are never
      positioned by their dates: that is what almost every "timeline" UI actually is — an activity
      feed, a changelog, an order's progress. A timeline where March and December sit
      proportionally apart is arithmetic over a domain and belongs beside `chart`, and conflating
      the two would hand every caller a layout that lies about half its inputs. The module header
      says so, the way `calendar` says it is not a date-time library.

      An `<ol>` of `<li>`, because that is what a sequence of events is — a screen reader is told
      how many there are before any styling matters, and `TimelineState::Current` is the one state
      that writes `aria-current="step"`.

      **Items count themselves**, like carousel slides, so each knows whether it is last. Not a
      nicety: a connector drawn after the final event points at nothing, and `:last-child` stops
      being true the moment a caller wraps an item in anything.

      **The connector is a part the caller places**, so a pending stretch can be dashed where a
      finished one is solid; the styled item includes one by default and hides it on `data-last`.
      **Markers are content** — a dot, a tick, a number, an avatar. An empty marker is
      `aria-hidden`, since the item already carries the state and a screen reader announcing
      "bullet" before every event helps nobody.

      That last rule caught a real bug on the way in: the styled `TimelineMarker` forwarded
      `{children.map(…)}` into the root, and the view macro passes a children closure whether or
      not there are any — so every marker looked non-empty and none was ever marked decorative.
      The styled layer now branches on the `Option` instead. Verified both ways round in a
      browser: bare dots hidden, icons and numbers not.

      One component rather than a Timeline and a Stepper, since the state enum is the whole
      difference and a stepper is a timeline you have not finished walking.

- [x] Timeline, horizontal and alternating. The primitive already wrote `data-orientation` on
      every part, so both are the styled layer plus one new fact: which side of the axis an event
      sits on. `TimelineSide` is derived from the item's own index rather than set per item —
      a zig-zag a caller keeps in step by hand stops zig-zagging the moment one event is inserted.

      The item is a two-axis grid that transposes: `[auto_1fr]` columns vertically,
      `[1fr_auto_1fr]` when alternating so the markers hold a centre column, and rows rather than
      columns when horizontal. Alternating is deliberately vertical-only — horizontally it would
      put half the events above the line and half below, which reads as two timelines.

      **The bug worth remembering is how it failed.** Three of the four parts took their new
      classes and the item did not, because `cargo fmt` had collapsed its `cn!` onto one line
      between writing the edit and applying it, so the patch matched nothing — and the edit was
      the one without an assertion on it. The item stayed a two-column grid while content was
      being sent to `col-start-3`, which grid quietly honours by inventing an implicit third
      column per row, of whatever width that row's content happens to be. Every marker landed
      somewhere different and the axis was gone. Nothing errored, nothing warned, and the computed
      `grid-column-start` values all read exactly as intended — only `grid-template-columns`
      showed two tracks where there should have been three. **Patch against the class literal
      rather than the surrounding block, and assert every replacement.**

      Two things the browser makes you know: `dragover` must be prevented or the drop is refused
      *silently*, and `dragleave` fires when the pointer crosses onto a child — so leaving is a
      counter reaching zero rather than one event, or a zone with a label in it flickers.

      It also turned up the only `use web_sys::` in the styled layer, which would have broken an
      install: `src/components/` may name leptos and `deps`, nothing else. `leptos::web_sys::File`
      instead.
- [x] `tree` — its own element rather than a mode on the sidebar, because a tree is not a sidebar
      feature: it is a thing a sidebar can contain, and a file picker or a settings pane wants the
      same one. `role="tree"`, one tab stop, `aria-level` per depth, and `aria-expanded` only on
      the items that have children — which is not knowable until the children have rendered, so
      `TreeGroupRoot` is what tells its item it is a branch. A closed group renders nothing rather
      than hiding, so "the next visible item" is just the next one in the DOM and the roving focus
      needs no notion of visibility. Enter and Space are routed through the row's own click, so the
      pointer and the keyboard take one path.

      Four things it shipped wrong first, all invisible to the compiler: the keys were on the row,
      which is a *child* of the focused item and so never saw them; `attr:aria-level` on a native
      element renders nothing, so every item was depthless; nothing called `sync_tab_stop`, so the
      tree could not be tabbed into at all; and — the one a reader spotted before I did — **a
      named Tailwind group matches any ancestor that carries it**, so `group/tree-item` on an item
      that nests inside another item lit up every descendant. The chevron of a closed folder inside
      an open one was permanently rotated, and a focused branch ringed every row beneath it.
      Both now key off the row, which does not nest, and the focus ring is a direct-child variant
      (`[[data-slot=tree-item]:focus-visible>&]`) rather than a group at all.

      **Nesting a component inside itself makes `group/name` unusable for its own state.** Worth
      remembering for the kanban board and anything else recursive.
- [x] The sidebar half: `SidebarTree` / `SidebarTreeItem` / `SidebarTreeRow` / `SidebarTreeGroup`,
      the same primitives with the sidebar's classes rather than the tree's own — one behaviour,
      two styled layers, which is the split the library is built on. The rows take
      `SidebarMenuButton`'s look and the group takes `SidebarMenuSub`'s rule, and the whole thing
      folds away with `group-data-[collapsible=icon]:hidden`, a tree of labels having nothing to
      show at that width. `sidebar` gained `tree` as a feature edge.
- [x] `video_player` — the `<video>` element and the controls that drive it, in five view modes.

      **The element is the state, and the player mirrors it.** Nothing decides that the video is
      playing: the element says so and one `sync` reads the whole lot back, behind every media
      event rather than a handler per fact. That is not tidiness — pressing play calls `play()`,
      whose promise a browser is entitled to reject, and a player that had set `playing` on the
      press would sit showing a pause button over a still frame. Seeks read back where they
      landed too, since the element clamps them.

      **The modes are behaviour, not decoration**, which is why they live in the primitive. Full,
      Reduced and Limited are bars of decreasing size and the styled layer draws them; the last
      two draw nothing and differ in the one thing only this layer can express — `None` is a video
      you can press, `Uncontrolled` is a video you cannot, so its surface is inert and it is not a
      tab stop.

      **The player does not own a playlist.** Previous and Next report a `VideoPlayerStep` and the
      caller moves their own list on, the same stance as the board reporting a move. Nobody
      listening means no playlist, so both controls disable rather than swallow the press.

      The scrubber and the volume slider are one component twice, differing only in what a
      fraction along means — and in their keys, since a keypress on a time bar means five seconds
      rather than five percent of however long the video is. The seek rail disables itself until
      the metadata arrives: a duration that is NaN or infinite is no length to scrub along, and
      zero is how the rest of the file spells that. `format_timestamp` says `--:--` for those
      rather than a confident `0:00`, and has four tests.

      Two demo clips, because they show different things. The default one is generated with
      ffmpeg — a timecode burned into the frame, which is the one thing a real video cannot show
      you: that a press at the middle of the rail lands where the rail says. 243kB. The second is
      a 40-second excerpt of *Tears of Steel* (CC BY 3.0, Blender Foundation, credited under the
      demo), 3.4MB at 960x400, and it is what shows the buffered bar filling and the `waiting`
      state — the generated clip arrives before you can press play, so it can never show either.
      Both are served by trunk's `copy-file` rather than `include_bytes!`d into the wasm, which is
      the mistake `docs/assets/avatar.jpg` is still an example of.

      Self-hosted rather than put on a CDN: this is a plain `<video>`, so it cannot play HLS, and
      HLS is most of what a video host sells. The docs page says so, since it is a real limit of
      the component and somebody will look for it under "several formats".

      Cost ten Lucide icons (`play`, `pause`, `skip-back`, `skip-forward`, `volume`,
      `volume-off`, `maximize`, `minimize`, `settings`, `captions`) in a new `icons/media.rs`.

- [x] The video player's second pass: a collapsing volume, menus, and subtitles.

      **Volume collapses to its icon** until the group is hovered or something inside it takes
      focus. Focus as well as hover, or it is a control the keyboard can reach and never see —
      and a `[@media(hover:none)]` fallback leaves it open where there is no hover at all, since
      Tailwind gates every `group-hover:` rule behind `@media (hover: hover)` and a phone would
      otherwise get a volume slider it could never open. That one is invisible from a desktop:
      headless Chrome reports `hover: none` by default, which is why the first probe of it
      measured the harness rather than the component and needed
      `--blink-settings=availableHoverTypes=2` to say anything at all.

      **`VideoPlayerMenu` takes whatever you put in it.** Speed and subtitle groups ship because
      every player has them, but quality levels or an audio-track picker are the same shape.
      Deliberately **not portalled**: a menu portalled to `document.body` vanishes the moment the
      player goes fullscreen, since only the fullscreen element's own subtree is drawn. And the
      idle timer had to learn about it — `menu_open` joins `scrubbing` in `controls_visible`, or
      the bar fades out from under an open menu and leaves it hanging over nothing.

      **Subtitles already worked**, because a `<track>` is a child of the `<video>` and the
      player's children are the video's children. What was missing was everything around them: the
      track list is now read back off the element rather than taken as a prop — the browser is
      what turns `<track>` elements into tracks, so a list passed in beside them would be a second
      copy to keep in step. `set_text_track` disables every other track explicitly, since a
      `<track default>` comes up showing and two showing at once paints one caption over another.
      Cues are nudged up off the control bar with `::-webkit-media-text-track-container`, which is
      Chromium and WebKit only; Firefox still overlaps, and drawing cues ourselves is the only
      real fix.

      **A caller can replace the control bar entirely** with the `controls` prop and keep the
      shell — the video, the surface, the keys, fullscreen. That is what makes "add your own
      settings" true rather than a claim: the context is public, so any control can be built from
      `use_video_player()`.

**Unfinished corners of what is shipped.**

- [x] `ToastBuilder::action(label, on_press)` — "deleted · undo". The callback is a plain
      `Arc<dyn Fn>` rather than a `Callback`, because what raises such a toast is on its way out.
- [x] A `Field` knows when it has been left: `FieldContext.touched`, `data-touched` on the field,
      `touched` on `FieldControl`, set from a `focusout` with the related target deciding — `blur`
      does not bubble and a styled control may wrap itself in a box. `NativeSelect` also takes an
      `on_blur` for a caller who is not using a field.
- [x] `ComboboxItem` takes `selected` and `close_on_select`: a multi-select is a list the caller
      keeps, not a mode the combobox has.
- [x] `utils::theme` is three axes rather than one: light/dark stays a `dark` class, the palette
      is a name on `data-theme`, so a theme has a light and a dark of its own, and an accent is a
      name on `data-accent` — one hue picked *inside* whichever palette is in force, which is
      fourteen names in a stylesheet rather than fourteen more palettes. `ThemeContext` gained
      `color`, `accent` and a `resolved` that is never `System` — which had to become live, so the
      media query is listened to rather than read: `use_media_query` re-reads on `resize`, the
      one event a colour-scheme change never fires.
- [x] A fourth axis, and the one that is not a name: a custom corner radius. There is nothing to
      enumerate, so `ThemeContext::radius` is a CSS length written into `--radius` on `<html>`'s
      own `style` — the one place a `[data-theme]` rule cannot outrank — and the switcher offers
      it as a slider over the palette's own. Clearing it hands the radius back to the palette.
- [x] An accent moves the *tinted surface* too — `--accent` / `--sidebar-accent`, which is hover,
      a focused item and the active sidebar row, and is unfortunately not the accent hue. Derived
      from the accent at a fixed lightness and a low chroma, which is what Grove and Ember were
      writing out by hand. Without it a palette with a green hover kept it while the brand went
      orange, which is the half-changed look the axis exists to avoid.
- [x] The palette rows depend on the mode: Latte is the light half of all three Catppuccin
      flavours, so light offers one row saying Latte and dark offers three. A row that stands for
      several names keeps whichever is in force rather than silently choosing a dark half.
- [x] Which way the text on an accent goes is the accent's own lightness, not the mode's — Latte's
      yellow is lighter than Mocha's blue. `--primary-foreground` reads the L back out with
      relative colour syntax and clamps it to a black or a white, so it is one line rather than
      fourteen judgements per flavour.
- [x] The docs' theme switcher is one popover with the four axes in it — three columns and a
      slider — and the choice in force actually marked, rather than a menu you reopen twice. Writing it turned up that
      `attr:data-theme` on a *native* `<span>` renders nothing — the palette dots had all been
      showing the same colour.
- [x] `FieldSet` / `FieldLegend`, and `FieldSetControl` beside `FieldControl`: a field labels one
      control with `for`, a set is labelled the other way round with the group pointing
      `aria-labelledby` at the legend. A real `<fieldset>` for the one thing a `role="group"`
      could not borrow — `disabled` on it disables everything inside. `RadioGroup` reads it, and
      a description or an error under a fieldset registers on the set rather than on a field.
- [x] `RadioGroupItem` was the only control in its family that never built a `FieldControl`, so a
      `FieldLabel` beside one pointed `for` at an id nothing wore. Found writing the fieldset.
- [x] `highlight` reads the three shapes it was missing: a call is `TokenKind::Function`, a JS
      regex is a literal rather than punctuation plus contents — decided by whether a value could
      begin there, which is why the loop remembers its last token — and `${…}` is cut out of a
      template literal and tokenized as the code it is. Capitalisation still makes a type, which
      in both languages is the convention rather than a guess.
- [x] `utils::date::DateNames` reads a locale's names out of `Intl.DateTimeFormat` — `js_sys`,
      which `Date::today` was already reaching into. Resolved once per calendar, since a grid
      reads a weekday name 42 times, and the captions are *formatted* rather than assembled,
      because the order of the parts belongs to the locale too (`2026年8月`). No `Intl` and an
      unreadable tag both fall back to English. `Calendar` and `DatePicker` take a `locale`.
- [x] `utils::color` reads all 148 colour names, and `Rgba::name` reads one back for a swatch that
      would rather say `rebeccapurple`. A sorted table of packed `u32`s, searched only once the
      shapes with punctuation in them are ruled out; `transparent` is matched ahead of it, being
      the one keyword with an alpha.
- [ ] `ColorPicker` in more shapes than the square-and-rail. A **triangular** picker (saturation
      and value on the corners of a triangle, hue on the ring around it) and a **wheel** are the two
      people ask for; both are the same `Hsva` underneath, so what is new is the geometry — a point
      inside a triangle or a disc mapped to two channels, and back. That is arithmetic with a unit
      test, and the picker already keeps HSVA rather than RGB precisely so a shape like this cannot
      lose the hue in a corner. The rails stay: a shape is a way to pick, not the only way, and the
      text field is what makes any of them exact.

- [ ] `ColorPicker` should use a `Select` instead of a button to switch colors, and instead of
      a unified input, using a divided input (with `InputGroup`) with a `Select` to switch formats
  - 3 for rgb
  - 4 for rgba
  - 3 for hsl
  - 4 for hsla
  - 4 for hsva
  - 4 for oklch
- [ ] `ButtonSize` cannot size the icons inside the button, and the rules that were meant to are
      dead code. Every size writes `[&_svg:not([class*='size-'])]:size-3` or similar, and the base
      writes `size-4` — but **every icon in this library renders with `size-4` already**
      (`IconRoot` is `cn!("size-4", class.get())`), so `:not([class*='size-'])` never matches and
      an icon in an `Xs` button stays 16px in a 24px button. The rules are inherited from shadcn,
      where a lucide icon carries no default size and the guard does bite.

      Dropping the guard is not the fix: `[&_svg]:size-3` is a descendant rule and would outrank a
      caller's own `size-6` on the icon. Dropping the icons' default leaves a bare icon with no
      size at all. The shape that works is a variable — the button sets `--icon-size` per size,
      `IconRoot` defaults to `size-(--icon-size,1rem)` — so the button decides, and a caller who
      writes a size on the icon still wins on the icon's own class.

- [ ] `highlight` is still a lexer per language: a new one is a new scanner beside `tokenize_rust`,
      and the six that exist share only their helpers. Fine at six; worth a second look at ten.
- [ ] `navigation_menu` has no shared viewport: each panel is absolutely positioned against the
      menu, so they all open in the same place, but the box does not slide or resize between them.
      That needs the active panel measured and the size animated.
- [ ] `chart` needs more of each mode, not only more modes. How a series gets from one point to the
      next is its own choice and is currently always a straight line: an area or a line should be
      able to be **curved** (a monotone spline, which does not overshoot into values the data never
      had) or **stepped** (blocky — which is the honest shape for anything that holds a value until
      it changes, a price or a setting), as well as the sharp one it draws now. That is an
      interpolation prop over the existing path arithmetic rather than a new kind of chart.

- [ ] `chart` needs the shapes that are not a cartesian plot at all: **pie**, **radial** (a bar
      chart bent round a circle) and **radar**. They share an angular scale and a centre instead of
      an x and a y, so they want their own primitive beside `primitives/chart.rs`'s rather than a
      mode inside it — and all three are arithmetic, which means they can be unit-tested the way
      `utils::color` is rather than only eyeballed.

- [ ] `chart` is line, area and bar over a categorical x axis. No stacking, no second axis, no time
      scale, no pie — each is more arithmetic in `primitives/chart.rs`, not new machinery.
- [ ] `chart` draws every label it is handed, and the tooltip reads that same list, so a long series
      cannot be thinned by blanking labels without leaving the tooltip with nothing to say. The
      caller's way out is fewer, wider buckets (the chart block does exactly that). A `labels` /
      `tick_labels` split, or drawing every nth, would make the chart's own answer possible.
- [ ] `data_table` has no pagination of its own; it shows what it is handed. `Pagination` is
      shipped and the two compose — the data-table block demonstrates it, sorting the whole set
      before slicing a page out of it.
- [ ] `utils::gif` re-emits every frame at its clipped size rather than diffing consecutive frames
      for a minimal changed rectangle, so a cropped GIF is a little larger than it needs to be. The
      sample goes 17.5kB → 7.3kB on a crop to a third of the area, which is fine but not optimal.
- [ ] `docs/assets/avatar.jpg` is a 6.9MB PNG with a `.jpg` name. It is `include_bytes!`d, so all
      of it lands in the wasm binary — worth re-encoding to a few hundred kilobytes and renaming.
- [ ] The colour picker's square announces itself as `role="application"` with a description of the
      keys. There is no honest ARIA role for a two-axis drag; the rails and the text field are the
      paths that announce themselves properly.

## Docs site

- [x] Docs pages for alert-dialog, label and spinner; a `code` snippet under every demo (89 blocks
      across 26 pages), extracted from the demo markup so the two cannot drift.
- [x] Prop tables on all 47 component pages — 199 parts, one table each, under one API reference
      heading.
- [x] Blocks. `docs/src/pages/docs/blocks/` is a module per component behind one parameterised route
      and a `REGISTRY` keyed by slug, so a block is a file and a line.
- [x] Twelve blocks: a colour picker in a popover, a palette handing over to an alert dialog, a
      table that sorts before it pages, an avatar cropper that keeps a GIF moving, an application
      shell, a settings dialog with the sidebar inside it, a form that waits before it complains, a
      dialog that becomes a drawer, a range over one set of numbers, a multi-select of people, a
      delete you can take back, and right-click actions on a table row. Three of the last four
      needed library work first, and two of those turned up bugs — which is the argument for
      writing them.

- [x] Two more blocks, both of them arguments for a change in the library rather than dressing on
      one. **A board you drag things onto**: a palette down the side of a canvas, and you make a
      thing by dragging it off and letting go over the board — which is the block `pan_on_drag`
      was turned off for. The drop crosses a border no context reaches, since the palette sits
      outside the canvas, so it is done with the viewport signal and the box the canvas was wrapped
      in; `use_drag` listens on the window, so one gesture spans both panes. **A toolbar that reads
      the selection**: a markdown editor whose toggle group tells the truth, because nothing in it
      remembers what was pressed — the marks are read back out of the text under the caret every
      time either the text or the selection moves. That is what `Textarea`'s `node_ref` is for, and
      why every crossing between the two coordinate systems is converted: a browser counts the
      selection in UTF-16 code units and Rust indexes bytes.

      The board block is also the argument for `on_move` / `on_resize` / `CanvasNodeLabel`: it had
      its own `use_drag` and its own division by the zoom, and every consumer would have written
      the same twenty lines. It writes no pointer code for the things on the board now.

- [x] `/docs/utility`, filling the second dead nav link: `cn!`, the common types, `next_id`,
      `use_media_query`, `get_placement`, and the arithmetic modules — dates, colour, crops, GIFs,
      the tokenizer — each with the reasoning the module's own `//!` header carries.
- [x] An "On this page" rail (`docs/src/components/page_nav.rs`) on every page wide enough for it.
      It reads headings marked `data-toc` out of its own `<main>`, so `DemoSection` and
      `ApiReference` opt in once and no page has to list anything.
- [x] A second kanban block, the Trello-shaped card: a priority bar with a tooltip, the counts as
      icons rather than three more lines of text, and everything else behind a dialog the card
      opens with `on_select`. All of it is the caller's markup — the component owes it `children`
      and the guarantee that a control inside a card does not start a drag. The bar is a `Tooltip`
      trigger rendered `as_child`, so the tooltip hangs off the bar and not off a wrapper.

      Cost two icons (`paperclip`, `message`) and one lesson: deriving the dialog's `open` from
      "is a ticket selected" and clearing the ticket when it closes **hangs the page**. `set`
      notifies whether or not the value changed, so the two effects write to each other for ever.
      Two plain signals — the dialog owns whether it shows, the block owns what it shows — and
      nothing needs clearing, because nothing reads the ticket while the dialog is closed.
- [x] A kanban block: columns you add, cards you add, and two ways to move one — drag it, or
      right-click and pick a column. Both end in the same closure, `on_move` handing back a
      `KanbanMove` and the menu building one by hand, so the two cannot come to disagree about what
      moving means. The card is the context menu's trigger, which works only because the drag
      guards on the primary button. Cards are keyed by an id rather than by their title: two cards
      called the same thing would be one card that teleports.
- [ ] Blocks for the rest, in rough order of what would teach the most: **select** — a dependent
      pair where the second is filtered by the first; **resizable** — a two-pane layout that
      remembers its split; **menubar** — an application menu with the keyboard doing the work;
      **carousel** — slides that are links, so tab order and scroll snapping have to agree;
      **date-picker** — a range across two months, which is the one the calendar's API has not been
      asked for yet.
- [ ] Snippets are extracted, not curated. A few still carry the demo's layout wrapper (`<div
      class="flex gap-3">`). Worth a pass to trim the ones where it adds nothing.
- [ ] Dead nav link in `docs/src/layout/doc_layout.rs`: `/docs/introduction` has no route and falls
      through to "Not Found." `/docs/installation` is a page now.
- [x] The form-state page is back, on a **git** dependency. It came off main because `docs` named
      `noform` by path (`../../noform`, a sibling clone that exists on one machine): cargo will not
      load a workspace whose member has an unreadable manifest, so every CI job died inside
      `cargo metadata` — `cargo fmt --all` first among them — before a compiler ran. Marking the
      dependency optional does not help; the manifest is read either way. Nothing local catches it
      either, since the path resolves where it was written; the check that does is a copy of the
      tree somewhere without the sibling.

      **A path dependency outside this repo must not land on main.** To work against a local
      `noform`, override it in an untracked `.cargo/config.toml`, not by editing `docs/Cargo.toml`.
- [x] `/docs/theme`: the three axes, what each lands on `<html>` read back live, the tokens and
      the radius scale in a row of its own, and the CSS a consumer writes to define a palette or
      an accent.
      Catppuccin is three palettes rather than four — Latte is the light half of all three, which
      is how the flavours ship everywhere else and what keeps the mode axis meaning one thing.
      Values generated from `catppuccin/palette`, in oklch, rather than eyeballed.

- [ ] The header link between a component and its blocks is a stopgap. A component page should open
      onto a *switch*: **Component**, **Blocks**, and later **Primitives** — one page, three views
      of it. The routes are already shaped for it, so it is a `Tabs` in `DocLayout` driven by the
      path rather than a restructure.
- [ ] Adding a component means editing five files, two of which (`NAV`, `COMPONENTS`) are
      hand-maintained copies of the same list — which is how `/docs` came to be missing progress,
      skeleton and textarea. Drive routes, nav and the index from one const.
- [x] Blocks moved out of `/docs/<component>/blocks` and into their own `/blocks` section. Filing a
      block under one component was always a fiction — each composes three to ten of them — so the
      owner was whichever file it lived in and the rest could not reach it. Each block now carries
      the components it uses, the index searches over those as well as the prose, and a component
      page hands the reader `/blocks?uses=<slug>`: `dialog` surfaces eight where it used to show
      one. The palette lists blocks by name, and the home page has a way in to each section.

      One cost, taken deliberately: `/docs` and `/blocks` are two `ParentRoute`s sharing `DocShell`,
      so crossing between the sections remounts the sidebar and loses its scroll. Within a section
      it is preserved, which is the common case; one shell over both would mean a parent route at
      the root, and the home page does not want a sidebar.

- [x] `/primitives`, a section of its own with one line in the sidebar — fifty nav entries would
      double a sidebar nobody would then read. The five with **no styled layer** are done first
      because they had nowhere else they could ever have gone: a scheme that hung primitives off
      their component's page could not house `use_drag`, which is what seven components are built
      on. Each page carries what a prop table cannot — the context, the `data-*` it writes, the
      keys it binds — and a demo built from plain classes rather than the styled layer, which is
      the claim the layer exists to make.

      The index searches over `used_by` as well as the prose, so "which primitive does the slider
      use" is answerable from either end.
- [ ] The other 45 primitives, the ones that do have a styled layer. 157 parts, and the component
      page should gain a switch — Overview · Primitives · Blocks — since nobody looks up
      `DialogContentRoot` except while reading about `Dialog`.

      **Generate these rather than writing them.** 199 styled prop tables are already hand-pasted
      and drift; 157 more doubles a debt that is already the largest in the docs. An `xtask` over
      the `#[component]` signatures would emit both sides from one source and close the five-files
      item too. It does not help the five above — four of them contain no `#[component]` at all —
      which is why they went first.
- [ ] The Leptos-shaped types the prop tables keep naming — `Signal<String>`, `ChildrenFn`, the
      `render` callback shape — still have nowhere to be described once. `Side` / `Align` /
      `Orientation` / `SideOffset` are on `/docs/utility` now, so this is the other half of that
      page rather than a `/docs/types` of its own.
- [ ] Generate the prop-table rows at build time rather than keeping 199 hand-pasted tables in step:
      an `xtask` or a build script over the `#[component]` signatures, emitting the same `ApiEntry`
      shape. It could drive `NAV` and `COMPONENTS` too, closing the five-files item above.

## Publishing

- [x] Consumers have no way to get the CSS. Answered by not shipping the styled layer as a crate at
      all: a Tailwind build cannot see classes inside `~/.cargo/registry`, so `cargo nocli` installs
      `src/components/<name>.rs` into the project's own `src/` and `init` installs the stylesheet
      beside it. The crate keeps everything with no classes in it.
- [x] One feature per element (64 of them), gating both layers under one name, each listing what its
      own source imports so cargo works the closure out. `tests/features.rs` re-derives the table
      from the imports and fails if `Cargo.toml` has drifted — the failure it prevents is silent,
      since `--features full` builds either way.

- [x] Both manifests carry `description`, `license`, `repository`, `homepage`, `keywords` and
      `categories`; `cargo publish --dry-run` is clean for each.
- [x] Apache-2.0, with the shadcn/ui and Radix MIT notices reproduced in `NOTICE` as their licence
      requires. `LICENSE` is also copied into `cli/`, which is packaged separately.
- [x] `leptos` no longer carries `features = ["csr"]`, so the consumer picks the mode. Checked
      against `ssr`, `hydrate` and `csr` from a scratch crate — all three compile. Compiling is not
      running: nothing here has been *exercised* under hydration, and the primitives call
      `document()` directly, so an SSR consumer is unexplored rather than supported.
- [ ] The CLI is not published, and `cargo add nocomponents` inside it assumes the library is. Until
      both are on crates.io the only working path is `--from <checkout>`, which is also how the CLI
      is tested.
- [x] `nocomponents.lock` beside the config records a hash of each file as installed, so `add` now
      says *which* kind of file it left alone — "edited here" against "unchanged, --force refreshes
      it" — and `--force` names what of yours it overwrote. FNV-1a, not a cryptographic hash: the
      question is only whether a file is byte-for-byte what we wrote, and `\r` is skipped so a
      checkout on Windows does not read as edited.
- [x] `components remove`, which refuses twice — once for a component another installed one is
      built on (found by searching the installed files for the rewritten import, so there is no
      list to keep in step), and once for a file the project has edited. `--force` overrides both,
      and doing so is what broke the scratch project during testing, which is the argument for the
      refusal.
- [x] The installed `mod.rs` is written between `// nocli:begin` and `// nocli:end`; anything
      outside them survives. A module the project declares itself out there is skipped inside the
      block rather than declared twice — the block lists every `.rs` in the directory, their own
      files included.
- [ ] `init` installs the whole of `globals.css`, which is four palettes and fourteen accents —
      most of a consumer's stylesheet, for tokens they mostly will not use. Worth splitting the
      base tokens from the palettes so a project takes one and opts into the rest.
- [ ] `cargo doc` over the styled layer is a list of bare signatures. Its prose is the docs site's
      prop tables now, and a description written in both places drifts, so `src/components` carries
      no `///` on purpose — docs.rs stays empty until it can be generated from the same source as
      the tables. `src/primitives` and `src/utils` keep theirs, being undocumented elsewhere.
- [ ] README is a disclaimer — needs install steps, a minimal example and the feature-flag table.

## Repo hygiene

- [x] CI in `.github/workflows/ci.yml`: `fmt --check`, `clippy --features full -D warnings`,
      `check -p docs`, `test --features full`. `pages.yml` deploys the docs site on every push to
      main — needs Settings → Pages → Source: GitHub Actions once.

- [ ] Nothing installs a panic hook, so a panic in the docs app surfaces as
      `RuntimeError: unreachable` with no message and no line. It cost an afternoon on the toast
      disposal bug, where the panic was real and the only way to find it was bisecting the call.
      Three lines in `docs/src/main.rs`.
- [ ] Almost no tests over what renders. `src/utils` is covered, `tests/features.rs` checks the
      feature table, and `cli/` has unit tests over the import rewriting; nothing exercises a
      component. `wasm-bindgen-test` smoke tests over the primitives' `data-state` transitions
      would catch regressions in the floating and toast state machines — the toast one has now been
      wrong twice.
