# 220 questions — mission frontier, 2026-11-04

Answer in place: append `-> answer` on the line, or answer by number in your next message. Nothing here
is a decision until it is answered and a ticket lands it. Questions are grouped by the axis they
gate; the range in each heading is the group's question numbers.

## A. What the client shows — rendering and package content (1–30)

1. Does paint order follow a frame's declared depth, or the display list's insertion order?
2. When a cell has no art, does the client draw paper or state absence?
3. Is the stage rect carried once per movie or once per frame?
4. How are partially transparent fills composited over paper?
5. Are bitmap fills sampled at cell resolution with the existing 4-point Q113 rule, or a new one?
6. What colour states a bitmap fill that failed to decode — and is grey acceptable?
7. Which wins when the package declares both a movie rate and a display refresh?
8. Is `visible 0` distinguishable from "the character is absent" in the grammar today?
9. What does the client print when the palette refuses a frame?
10. Should the loader plan the next frame while drawing the current one?
11. What is the largest quad batch the device should receive per frame?
12. On resize, are quads re-projected or rebuilt?
13. Should the window letterbox or stretch a 640×480 stage into 960×640?
14. What evidence proves a frame drew imported content rather than a stand-in?
15. Should the importer emit a per-frame digest the client can print?
16. How do we detect a frame that is one flat colour by accident rather than by design?
17. Does the client keep the last good frame when planning fails mid-run?
18. What is the refusal code for a frame that cannot be planned?
19. Does the package carry a poster frame for a preview before the first tick?
20. Are glyph-scale details lost at cell resolution, and is that accepted?
21. Should every drawn frame print its painted-cell count?
22. Can two frames share one planned batch by digest?
23. What is the real cost difference between planning all frames at startup and lazily?
24. Should a frame's plan be cached across ticks, and evicted when?
25. What does the window do for a package with zero frames — refuse or show absence?
26. Is 960×640 the right default window for a 640×480 stage?
27. Should the client print the declared rate and its fixed-point units at start?
28. What does "streaming the real game" mean when the package is a local file?
29. Should the client read frames lazily by byte offset rather than holding them all?
30. What single observation counts as a client witness under AGENTS §7?

## B. Sprites and timelines (31–50)

31. Is a sprite's frame index always the movie frame index, or can it be offset?
32. How many nested sprite levels must the importer lower?
33. What happens to a sprite frame beyond the movie's last frame?
34. Should a sprite timeline wrap independently of the movie timeline?
35. Are sprite placements' matrices composed with the stage matrix or replaced?
36. Which depth wins when a child sprite and a parent placement collide?
37. Should the package carry the sprite's own frame count per sprite?
38. Can a sprite be re-parented mid-timeline, and does the package express that?
39. What is the refusal for a sprite that references itself?
40. How are sprite masks represented, and are they cells or absent?
41. Do mouse-following sprites exist in this movie's model, and if so who animates them?
42. Should the loader advance all sprite timelines at the movie tick, or only visible ones?
43. What is the cost of lowering every sprite frame at import time?
44. Can the importer skip unreachable sprite frames and still claim completeness?
45. How does the client know a frame's sprite set is complete?
46. Are sprite frame labels carried anywhere, and do they need to be?
47. Should a sprite's timeline be a separate `[timeline]` block?
48. How are sprite-local sounds scheduled relative to movie ticks?
49. What is the evidence that a sprite frame is drawn and not the movie's frame?
50. Does the movie's frame 1 sprite state equal the initial placement list?

## C. Scripts and the code lane (51–80)

51. Does the shell compile AVM1 per frame, per script, or per movie?
52. Does one program describe the whole movie or one tick?
53. If per-tick, how does the client know which program applies to which tick?
54. Should `code` frames be admitted at mount now, or wait for the shell's ruling?
55. What is the admission rule — one program per movie, per frame, or a keyed table?
56. Does a program need to address a sprite's timeline, not just the movie's?
57. What operations does the movie's own scripts need that the current set lacks?
58. Do scripts need variables and arithmetic, or only timeline control?
59. Do scripts need conditionals, and if so what bounds their execution?
60. Should the executor keep a bounded interpreter state between ticks?
61. What is the total step budget for one movie tick, and who enforces it?
62. How are script-triggered sounds distinguished from timeline sounds?
63. Should a stopped timeline still execute per-tick scripts?
64. What happens when a script goto targets a frame beyond the carried set?
65. Is `gotoAndPlay` expressible by `goto` + `play`, or does it need its own operation?
66. How do we refuse a program that references a sound not in the package?
67. Should a program be able to remove a character from the display list?
68. Should a program be able to load external content — and if so, is that refused by name?
69. What is the maximum program document size?
70. Should programs be digested and printed in the run log?
71. How does a scripted run differ observably from an unscripted one in the log?
72. What test proves the executor is deterministic across processes?
73. Should a program be able to stop the clock, or only the timeline?
74. How are script errors surfaced: refusal, skip, or halt?
75. What is the smallest compiled program that would remove the loading cover?
76. Would that program be proof of anything, or an invented stand-in?
77. Who decides that the movie's scripts have been covered — the shell or the user?
78. What documentation must exist before a code frame is admitted?
79. Does admission require a schema bump or an addendum?
80. What happens to packages carrying code frames under an older reader?

## D. Sounds (81–100)

81. Which clip format do the nine package sounds decode through?
82. Is the audio channel device-backed or a render-to-buffer proof first?
83. What is the refusal when no audio device exists — and does the window still run?
84. Should a sound start at the tick boundary or at the sub-tick offset?
85. How are overlapping sounds of the same character handled?
86. What is the maximum simultaneous channels?
87. Does the package carry per-sound loop counts and volume?
88. Should sounds pause when the timeline stops?
89. How is a sound's duration expressed — samples, ticks, or both?
90. What happens when a clip outlives the movie's last tick?
91. Should the client print `sound <character> started at tick N`?
92. Is the decoded clip cached across ticks?
93. Are sounds allowed to be resampled, and at what rate?
94. What is the evidence that a sound is the package's own and not synthesized?
95. Should a silent run state that it was silent by package content?
96. Can a package carry a sound with no character name?
97. How does the audio lane share the Script horizon without becoming a second scheduler?
98. What is the test for "a tick that announces a sound starts it exactly once"?
99. Do sounds need to be routed through the same refusal surface as frames?
100. What does "sounds play" mean for a headless proof?

## E. Client axis (101–130)

101. What is the "client axis" in one sentence — the schedule, the surface, or both?
102. Is the axis the three horizons (render/script/logic) or a fourth dimension?
103. Does an axis coordinate name a tick, a frame, a cell, or all three?
104. Should the axis be addressable by external tools (a bot, a reviewer)?
105. Does the axis need to be serialisable so a bot can point at "tick 40, cell 3,2"?
106. Can the axis express a sprite instance, or only stage coordinates?
107. Should axis coordinates be stable across windows of different sizes?
108. Is the axis the same thing as the router's address space?
109. Does a bot need to read the axis without running the window?
110. Should the axis expose refusal states, or only drawn content?
111. How does the axis stay one surface rather than a second package format?
112. Is the axis a log, an API, or a protocol?
113. Should the axis be queryable per tick for a range of ticks?
114. Does the axis need a name for "the frame the script moved us to"?
115. What is the client's answer to "where am I" for a reviewer bot?
116. Should the axis carry the movie tick and the wall clock separately?
117. Is the axis allowed to be lossy for speeds a human cannot see?
118. How does a reviewer quote an axis position in a report?
119. What is the refusal when an axis address does not exist?
120. Should axis addresses be printable in a URL-like form?
121. Does the axis subsume the input router, or coexist?
122. How does the axis handle a resize mid-timeline?
123. Are axis coordinates per package or per mount?
124. What does an axis snapshot contain — frame id, tick, cells, sounds?
125. How large may a snapshot be before it needs a digest?
126. Should the axis be the mechanism for the "wishing" a bot performs?
127. Does the axis need a stable version number?
128. What is the smallest axis implementation that a reviewer bot could use today?
129. What would make the axis a second runtime (and therefore refused)?
130. Which document owns the axis definition?

## F. Civilians and the wishes format (131–155)

131. What is a "civilian" — a spectator, a bot body, or an agent persona?
132. What is a "wish" — a request, a task, or a goal tree?
133. Who may issue a wish, and to whom?
134. Is a wish durable state or an ephemeral message?
135. What identifies a civilian uniquely, and is it source-safe?
136. Does a civilian carry permissions, and who grants them?
137. Is the wishes format a package, a frame kind, or its own document?
138. If it is a document, where does it live and what reads it?
139. How does a wish name a target: an axis address, a game, a bot?
140. Can a wish be refused, and is the refusal recorded?
141. Does a wish carry an expiry?
142. How are wishes audited without exposing source content?
143. Is "upgraded civilians" a version bump of that format?
144. What is the schema versioning rule for wishes — like the code lane?
145. Can a wish reference a package by digest?
146. Do wishes carry budgets (ticks, steps, wall time)?
147. How does a wish express "review everything" without enumerating everything?
148. Is a wish allowed to change the game state, or only observe?
149. What is the difference between a wish and an input event?
150. Who owns the wishes format: this repository, the shell, or the civilian system?
151. What is the refusal for a wish naming an unknown axis address?
152. Do wishes need to be replayable, and what makes them so?
153. How does a wish report completion — evidence, digest, or claim?
154. Can a wish result be a refusal, and does that count as success?
155. What must exist before a civilian may enter a game?

## G. CUA driver and node bot control proof (156–180)

156. Is the CUA driver the node-controlled bot, or does it drive a node-controlled bot?
157. What is the transport between node and the driver — process, socket, or stdio?
158. Does the bot read the client axis or the screen pixels?
159. If pixels, what is the proof that the bot saw imported content?
160. If the axis, what is the proof the axis matched the drawn frame?
161. What is the minimal node program that proves control?
162. Does the proof need to be committed, run, or only documented?
163. What are the bot's allowed actions for review: read-only?
164. How does a bot "review everything" without becoming a second gate?
165. Should bot findings enter the ledger?
166. What is the refusal when a bot tries to act on a package it did not mount?
167. Does the driver need to be headless, and what does headless mean here?
168. How is a headless run witnessed if no window opens?
169. What is the health signal for a stuck bot?
170. Should the bot's control loop be ticked by the movie or by wall time?
171. How does the bot handle the debug-console refusal?
172. What is the bot's evidence artifact — screenshots, axis dumps, or both?
173. Do screenshots belong in `artifacts/` and under what redaction rule?
174. Can a bot witness be reported as a client witness? (No — but state why.)
175. What is the refusal for a bot that claims gameplay from a mount?
176. Does the driver need a capability list per civilian?
177. How does a bot pause or resume the movie?
178. What is the bot's rate limit against the scheduler's catch-up?
179. How do two bots share one client without becoming a second scheduler?
180. What test proves node control end to end?

## H. Secure shell v2 and the civilian system model (181–192)

181. Which parts of secure shell v2 are upstream of this repository?
182. Is the civilian system 1 model documented anywhere this repo may read?
183. What does "headlessly be wished to handle code" require of the client?
184. Does handling code mean editing this repository, or a target package?
185. What is the sandbox story for a wished bot that edits files?
186. Is the shell the authority for states (`held`/`absent`/`refused`) still?
187. How does a civilian report state without emitting gate states?
188. What is the interface between a wish and a shell invocation?
189. Does the shell own the wish log, or does this repo?
190. What is the refusal for a civilian that tries to judge source content?
191. What does the upstream model need from this example project to be proven?
192. What must not be copied from upstream into this repo?

## I. Packages, legacy formats, dependencies (193–206)

193. Which legacy formats are in scope for the importer?
194. Does "legacy" mean older SWF versions, or another authoring format entirely?
195. What is the rule for a second package magic? (One only, per AGENTS §2.)
196. Should each legacy reader emit the same `V4PKG001` frames?
197. What dependency policy applies to decoders (codecs, archives, fonts)?
198. Are dependencies vendored, pinned, or feature-gated?
199. What is the refusal when a dependency is missing at runtime?
200. How does the importer report a source it cannot measure?
201. Should the importer accept a directory of movies?
202. What is the drag-and-drop contract for the importer?
203. Does a dropped file get copied into `inputs/`, and is that allowed?
204. What is the refusal for a movie whose stage size differs from the window?
205. How are dependencies checked in CI without network access?
206. What is the maximum package size the client will mount?

## J. MCP surface, streaming, other games (207–216)

207. What operations must the MCP surface expose?
208. Which operations must it refuse by name?
209. Does the MCP surface read the client axis or the gate?
210. Should the MCP surface be able to start a window?
211. Is the MCP surface allowed to write packages?
212. What is the refusal for an MCP caller without a ticket?
213. What does "testing the importer on other games" require as fixtures?
214. Are other games' movies allowed under AGENTS §1 at all?
215. What is the proof that a package streams rather than loads whole?
216. Which of these answers requires the shell's ruling before code?

## K. Health, training formats, and the research refusal (217–220)

217. Which modern training formats are in scope for civilians and bots?
218. What research record must exist before a format is adopted?
219. What is the ratchet that refuses adopting a format with no research recorded?
220. Who owns the research record — this repository or the shell?

---

Count: 220 questions. Answers become tickets; none become rules by being asked.
