# Playing Your Game

The **Play** button sits at the right of the top bar, beside the Marketplace. It runs your game while the editor stays open: inside the editor's **Game** panel, or in a window of its own.

<!-- screenshot: play_controls.png - the Play button in the top bar, and the running game in the Game panel -->

## Play

**Play** (or `F5`) saves the scene and starts your game. Press it again, now reading **Stop**, to end the game. Closing the game's window ends it too.

The game runs as **its own program**, exactly the way an exported game runs, with its own frame rate, keyboard and mouse. What you play is what ships. That has three more consequences worth knowing:

- **A slow game doesn't slow the editor.** The game renders at its own frame rate, and the editor stays fully usable while it runs. You can keep working while it plays. The two still share one graphics card, so a game that keeps it fully busy makes the editor's drawing slower too.
- **A crash in the game doesn't take the editor down.** The game's window closes, the Console shows what went wrong, and you carry on.
- **Nothing the game does changes your scene.** The game loads the scene from the saved file, so moving something in a running game never touches what you are editing. That is also why Play saves first.

**The game never starts ahead of its scripts.** While Rust scripts are compiling, **Compiling** with the files being built (and, for several, a count) takes the place of **Ready** at the far left of the status bar. If you press Play then, the game starts the moment they finish. Press Stop to cancel the wait. The game also checks its scripts when it starts. Until they are loaded, its window stays black and its time stays paused, so a script always sees the game from its first frame.

The game never outlives the editor. Closing the editor ends it, and so does the editor crashing or being ended from Task Manager.

Everything the game logs appears in the editor's **Console**, under **Runtime**.

## Where the game shows

**Embed Game in Editor** (Settings > Scripting, on by default) decides where the game's window goes:

- **On: in the Game panel.** The game's real window is placed inside the editor's **Game** panel, and while the game runs that panel is the whole workspace: every other panel, the tab bars, the bottom dock and the status bar are put away, and the top bar becomes a play bar: what is playing on the left, **Restart** and **Stop** in the middle, and the companion's button on the right, so the profiler and debug tools stay a click away while you play. Restart stops the game and starts it again without the editor's layout flickering back in between. The editor window is also resized so the area under the play bar has exactly the game's shape (a 480 x 270 game is 16:9), so the game fills it with no black strips: it keeps its width and changes its height, or narrows when it would otherwise grow, and a maximized editor is un-maximized for the duration. Everything, the window's size included, goes back when the game stops. When the game ends (Stop, a crash, or the game quitting) your layout comes back exactly as it was. The layout is not saved while the game has the workspace, so closing the editor mid-play never loses it. It is still the game's own window drawing its own frames; the editor only decides where it sits, moving and resizing it with the panel and hiding it when another tab covers it. Click into the game to give it the keyboard. Menus and dialogs the editor opens over the panel are drawn on top of the game. Inside the panel the editor sets the window's size, so your project's window size and mode apply only to the separate window (VSync still applies). Windows only for now; elsewhere the game uses its own window.
- **Off: in its own window,** at the size, mode and frame rate your project's Window settings give it.

With a multiplayer Play session, only the main game goes in the panel. A dedicated server has no window, and extra test clients open in windows of their own.

**Your scene stays your scene.** The running game is a separate program with its own copy of the world, and nothing of it is copied into the editor: the Hierarchy, the Inspector and your scene are exactly as you left them while it runs, and Stop changes nothing in them. To change what the game does, edit the scene and press Play again.

**The viewport pauses while the game runs.** It goes blank, with a message in the middle saying the game is running in its own window, and draws nothing until the game stops: the game window already shows the scene, and drawing it twice would only slow both down. This holds even with Realtime on. When the game stops, the viewport carries on from the scene as Stop left it. The export dialog pauses it the same way while it is open (the dialog covers it, so there is no message). An engine crate that needs the same can hold the viewport through `renzora_viewport_api::ViewportHold` with a reason of its own; it draws again once every reason is released.

## Testing the game with an agent

An AI agent connected through the **MCP Server** plugin (from the Marketplace) can play the running game as well as edit the scene. A second or two after you press Play, the Console says **Game connected**: from then until Stop, two tools reach the game itself:

- **game_screenshot** returns a picture of the game's own window.
- **game_input** presses keys and gamepad controls in the game, each held for as long as asked. Gamepad input arrives on a virtual gamepad the game sees as a real one, so a game that reads gamepads needs nothing special. It can also move and click the mouse at a point taken from a game screenshot, type text into the game's focused text field, and send input-method composition (the syllable being built, then the committed text) to test Korean, Japanese or Chinese typing. These arrive as the window events the operating system would deliver, so the game's UI handles them exactly as it handles a person's input.

## Editing doesn't play anything

While you are arranging the scene, nothing in it moves by itself. Physics is paused, scripts don't run, animated models hold their pose, cloth hangs still, and navigation agents stay where you put them. What you see is exactly what the scene file says.

Particles are the exception: emitters keep running so you can see an effect without pressing Play.

The other exception is a script's own play button in the Inspector. Switching it on runs that one script while you edit, which is how you test a script without running the whole game.

## VR

Playing into a VR headset from the editor has been removed. A game can still run in a headset with the `--vr` flag; see [VR](/docs/r1-alpha8/editor/vr).

## Testing multiplayer

Running a server and a client at once needs two processes. See [Server Setup](/docs/r1-alpha8/multiplayer/server-setup).
