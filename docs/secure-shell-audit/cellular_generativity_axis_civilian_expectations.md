# Cellular Generativity and Civilian Evolution
## Evidence-led audit and Axis-gated expectation list for trained System-1 CUA

**Document status:** Architectural audit and proposed acceptance contract; not an implementation claim  
**Evidence basis:** supplied transcript of *Scientists Analyzed 500 Chimpanzee Drawings. The Results Were Unsettling.*; current repository source inspected 2 November 2026; cited cellular-automata and primatology research  
**Target use:** secure-shell v2, the civilian System-1 CUA architecture, declarative XML/package delivery, and future game/entity adapters including mob APIs

---

## 1. Executive finding

Cellular generativity should be adopted as a **bounded substrate for local adaptation, skill development, and distributed coordination** inside a persistent civilian—not as a replacement for the civilian's identity, memory, decision process, or authority boundary.

The transcript is useful because it presents several long-term behavioral phenomena together:

1. Individuals acquire skills by repeatedly observing other individuals, often without anyone explicitly teaching them.
2. A learner's tool choices can be influenced by what it sees before it has personally succeeded.
3. A learner can discover that a task exceeds its solo ability and solicit help.
4. Two individuals can learn to coordinate the timing of their actions through repeated attempts.
5. Individuals develop persistent, distinguishable behavioral styles, while still changing over time and with context.
6. Long-term recovery depends on continuity, opportunities, social relationships, and gradual adaptation to an altered environment.

Those are not interchangeable phenomena. The architecture must represent observational learning, individual practice, help-seeking, coordination, personal variation, and continuity as separate capabilities with separate evidence.

A cellular automaton can support part of this design: local state updates, distributed signals, pattern formation, persistence, repair, and the evolution of local rules. Neural cellular automata research demonstrates that a shared learned local update rule can produce and maintain spatial patterns and repair damaged regions. Evolutionary cellular-automata research demonstrates that local rules can be evolved for computational tasks, but also documents search pathologies and symmetry-breaking traps. Neither result establishes that a cellular automaton, by itself, supplies an embodied mind, social understanding, or lawful agency.

**Recommended architecture:** each civilian keeps one durable identity and one System-1 decision owner. A versioned cellular-generative substrate proposes or updates local skill/behavior patterns from the civilian's own observations, practice, social exposure, and verified outcomes. CUA translates an admitted skill into ordinary, bounded game inputs. The authoritative game server verifies physical results. Axis decides whether the proposed side effect and the proposed capability change are admissible. The secure-shell service records the package, ticket, proof, refusal, and test evidence. XML packages describe presentation and other closed declarative surfaces; they do not become an executable back door into the agent.

The governing rule is:

> **A civilian may learn more than it is currently authorized to do. Learning changes capability hypotheses; only a separate, current Axis decision may admit a capability or a consequential action.**

---

## 2. What the transcript actually supports

This section separates reported observations from interpretations. The transcript is the source for the narrative below; primary research is linked in §11 where available.

### 2.1 Observation is a real learning channel, not merely a tutorial overlay

In the honey-fishing experiment, the transcript describes mothers who already knew how to obtain honey using tools and infants who watched them. Twenty candidate tools were available. The adults predominantly used a small subset; infants began selecting those same kinds of tools before they had personally completed the task. The adults generally did not intervene to correct the infants' attempts, and sometimes prevented an infant from taking a tool while it still carried honey. The infants' first independent successes came after an extended period of observation and unsuccessful attempts.

The underlying Hirata and Celli study is more qualified than a simple “all infants copied their mothers” account. It involved three mother–infant dyads. Two of the three infants predominantly selected the two materials most used by adults; the third tried a wider range, although its selection was still non-random. The study describes tolerance of close observation and occasional access to tools, but reports no slow demonstration, assistance with the infant's attempt, or encouragement of the kind commonly used to define active teaching. Tool properties such as size, hardness, and flexibility may also have affected preference. [1]

**Architecture implication:** a civilian needs a durable observation-to-hypothesis path. It should be able to record an observed action sequence, identify the objects and affordances involved, form a tentative candidate, and later test that candidate itself. A social demonstration must not be reduced to a command or a copied macro.

### 2.2 Social transmission changes what an individual tries before it can perform the skill

The transcript describes a nut-cracking demonstration in which an individual named Loi was introduced to a group after learning the task. Other chimpanzees initially imitated aspects of the hand action and later worked out the role of the rock. One younger chimpanzee, Natsuki, is presented as gradually discovering the relevant use of the tool.

The important engineering point is not that imitation always copies the correct movement. It is that exposure can change attention, candidate selection, and the learner's search space before the learner has mastered the underlying action.

**Architecture implication:** social learning should be able to seed a candidate or alter an affordance belief without marking the skill mastered. The receiver must preserve the source of the observation and independently verify its own version of the skill.

### 2.3 Asking for help is distinct from copying or coordinating

The transcript describes Mizuki learning a heavy-cover task with Professor Hirata. The task was progressively made too heavy for one individual to move alone. After learning the collaborative pattern with the experimenter, Mizuki was later tested with the experimenter standing nearby but not initiating the action. Across the reported trials, she solicited help even when the experimenter was farther away or looking elsewhere, and later went directly to him without first making an independent attempt.

This part of the account concerns a human partner. It should not be generalized into a claim that chimpanzees automatically solicit every conspecific for assistance. In Hirata and Fuwa's 2007 rope-pulling study, the chimpanzees gradually learned to wait for the other individual and synchronize their pulls, but did not show the same soliciting behavior toward the conspecific partner. When Mizuki was paired with a human, she did solicit him. The behavior varied with partner identity. [2]

**Architecture implication:** “I cannot solve this alone” must be a first-class System-1 state that can generate a request to a named partner. It is not permission to command that partner, and it is not evidence that the partner accepted. Partner identity, prior interactions, availability, task structure, and the partner's own response must remain distinct inputs.

### 2.4 Coordination can be learned through repeated action and timing

The transcript describes a two-ended rope task in which two chimpanzees initially failed to coordinate. Through repeated sessions, they began attending to the partner, waiting until the other was ready, and pulling together. The primary 2007 publication reports increasing success, more frequent looking toward the partner, waiting when the partner was not holding the rope, and synchronized pulling. It also reports that they did not use eye contact or interactive behavior to synchronize the action in that task. [2]

**Architecture implication:** cooperation cannot be represented only as two agents independently selecting the same task. It needs a temporal protocol: readiness, shared preconditions, a bounded synchronization window, action start, outcome observation, retry/refusal, and closure. A partner's presence is not proof of readiness.

### 2.5 Individual style is persistent, measurable, and still capable of development

The transcript describes an archive of more than 500 drawings collected over roughly 13 years, and a later comparison with drawings made by humans. It highlights repeated individual tendencies: vertical strokes for one chimpanzee, different use of color and page coverage for another, and recognizable differences among individuals over time.

The 2026 paper by Sueur, Nicolas, Pelé, and Hirata analysed previously unseen and newly documented drawings from six chimpanzees across several years. It reported inter- and intra-individual differences and seasonal variation, and identified filling, color, and shape as three meaningful dimensions in the drawing data. It also reported variation in drawing style over years. [3]

The study supports stable differences and development in behavior. It does **not** establish that every mark was intended as human-like art, nor that a statistical style dimension proves a particular inner state.

**Architecture implication:** the system must preserve individual variation rather than forcing all civilians into one canonical policy. It should support longitudinal behavioral descriptors and measure both between-person differences and within-person change. A style signature is evidence about repeated behavior, not a diagnostic label or authority token.

### 2.6 Long-term recovery is not a reset operation

The transcript's account of Candy, Shoubou, and Musashi emphasizes the consequences of decades of isolation: physical weakness, unfamiliarity with open space, and difficulty entering social relationships. Reintroduction was gradual. Different pairings produced different responses. Over time, the chimpanzees regained strength and engaged in more of the environment, but their earlier history and health conditions did not simply disappear.

This is not a direct experiment about software learning. It is a caution against confusing exposure to a new environment with instant adaptation, or treating a period of constrained behavior as proof that the individual has no independent preferences or continuing history.

**Architecture implication:** a civilian's old skill, relationship, memory, and body history must survive a runtime or environment change. Relearning should be paced and evidenced. A disconnect, test completion, model upgrade, or temporary fallback is not a new birth and must never produce a blank replacement mind.

### 2.7 Careful observation is a scientific requirement

The transcript closes on Hirata's preference for patient relationships and close observation, and on less intrusive approaches to gathering physiological data.

**Architecture implication:** experiments should be minimally disruptive, distinguish observation from intervention, and preserve the subject's ordinary life. A test harness must not commandeer the entire civilian or erase its experience to create a cleaner benchmark.

### 2.8 Separate observed behavior from inferred intention

The opening account of Ai pushing Puchi and then quickly rejoining the group is presented with the suggestion that Ai may have been hiding what she had done. The observable sequence is the push and the return to the group. The claim that Ai understood the act as wrongdoing and deliberately concealed it is an interpretation offered by the narrator, not a directly established internal state in the transcript.

**Architecture implication:** the civilian record must distinguish raw event, inferred motive, confidence, alternative explanations, and later corroboration. A model may hypothesize deception, avoidance, fear, or ordinary social movement; it must not convert one ambiguous sequence into a settled trait or moral judgment. The same rule applies to player and mob behavior in games.

---

## 3. What cellular generativity can and cannot contribute

### 3.1 Two relevant technical families

**Neural cellular automata (NCA).** In Growing Neural Cellular Automata, each cell holds a state vector and applies a learned local update rule using neighborhood information. The shared rule can grow a target pattern, maintain it over time, and—when trained with damage—repair parts of it. This is a strong precedent for persistent local patterns and distributed repair. It is not a demonstration of autonomous embodied social cognition. [4]

**Evolutionary cellular automata.** Genetic or evolutionary search can discover local rule tables that solve computational tasks. Mitchell, Crutchfield, and Hraber's density-classification experiments documented multiple innovation strategies and impediments; the associated work discusses how symmetry breaking can produce short-term gains while blocking better solutions. Evolution therefore needs diverse evaluation cases, lineage, and resistance to proxy optimization—not just a scalar fitness reward. [5]

These two families should not be collapsed. NCA commonly learns a shared parameterized local update function; evolutionary CA searches or selects rules/structures under a fitness or task criterion. They can be combined, but their training, state, and evidence need explicit names.

### 3.2 Recommended use in a civilian

Use a local-state automaton to represent and update distributed behavioral structures such as:

- affordance hypotheses about objects and tools;
- short sensorimotor motifs and action transitions;
- task substate and local readiness signals;
- confidence and uncertainty attached to a learned pattern;
- social exposure traces and observed partner behavior;
- local coordination signals for multi-agent tasks;
- repair and recovery state for interrupted or partially learned skills.

A cell need not correspond to a biological neuron or a Minecraft block. It can be a bounded state unit in a graph, grid, or sparse neighborhood network. The representation must say what a cell means, what its neighbors are, what data it can read, and what state it may update.

A useful separation is:

```text
Durable civilian identity (AgentId)
  ├── life record, history, relationships, needs, affect
  ├── episodic and semantic memory with provenance
  ├── trained System-1 decision owner
  │     ├── candidate generation and arbitration
  │     ├── skill selection and short-horizon prediction
  │     └── request / wait / rest / decline / clarify
  ├── cellular-generative substrate
  │     ├── local behavior/affordance state
  │     ├── local update rules and versioned parameters
  │     ├── skill hypotheses and learned coordination motifs
  │     └── lineage, exposure, and update evidence
  ├── CUA skill runtime and bounded motor adapter
  └── current lawful capability envelope
          │
          ▼
  Axis gate (independent authority decision)
          │
          ▼
  ordinary client input / game API
          │
          ▼
  authoritative game-world outcome and read-back
          │
          └── evidence → memory / practice log / package ledger
```

### 3.3 Do not make the automaton the person

The durable civilian identity must remain independent of:

- a particular CA grid size or topology;
- a trained model file or neural-network checkpoint;
- one CUA client process;
- one Minecraft entity or mob object;
- a package revision;
- an experiment or test-run ID;
- a temporary level of simulation fidelity.

The automaton may be replaced, migrated, or rolled back while the civilian's identity and life history remain continuous. Any state that cannot be migrated is retained as an explicit unavailable or legacy record, not silently discarded.

### 3.5 A cell needs an explicit contract

A proposed cell record should be versioned and attributable. One workable minimum is:

```yaml
CellRecord:
  agent_id: stable AgentId
  cell_id: stable id within one agent's automaton
  state_schema: declared schema id and revision
  neighborhood_schema: declared adjacency / radius / edge policy
  rule_id: update-rule identity
  rule_revision: immutable rule revision or package digest
  local_state: bounded typed vector / discrete state
  input_refs: observations or permitted local signals consumed
  neighbor_refs: exact cells read for this update
  tick: authoritative update tick
  update_order: synchronous / deterministic schedule / declared asynchronous schedule
  evidence_refs: episodes, practice rows, test tickets, and verified outcomes
  lineage_refs: parent rule/model/skill revisions
  state_digest: digest of the resulting state and its declared framing
```

The update function should be conceptually narrow:

```text
next_state(cell) = F(
    current_state(cell),
    permitted_local_inputs(cell),
    permitted_neighbor_states(cell),
    rule_revision,
    declared_tick
)
```

The function must not read arbitrary global server state, inspect another civilian's private memory, alter an inventory, issue a packet, write an entity transform, grant a capability, or commit a ledger entry. It emits a candidate state transition. Separate owners decide whether that transition is accepted into the civilian's persistent state and whether any resulting skill may act.

The contract must also state whether updates are synchronous or asynchronous, how neighbor boundaries are handled, whether numerical operations are fixed-point or floating-point, how randomness is seeded, and what replay guarantees are claimed. A deterministic claim requires a deterministic update schedule and all random seeds and external inputs to be recorded. If the model intentionally uses stochastic or asynchronous updates, the evidence should capture the realized ordering and the permitted variance rather than falsely claiming bit-identical replay.

### 3.6 Keep local state types separate

Do not compress all learned information into one undifferentiated hidden vector. At minimum, identify distinct channels or linked records for:

- **perception:** what was sensed and from where;
- **affordance:** what an object or entity may permit;
- **skill hypothesis:** what action sequence may work;
- **practice:** what this civilian actually attempted and what happened;
- **social exposure:** what was observed or reported by another;
- **coordination:** partner readiness and shared task phase;
- **confidence/uncertainty:** the model's estimate and its calibration evidence;
- **context:** game version, body adapter, ruleset, region, environment, and relevant need state.

These channels may be encoded in a learned representation internally, but the system must retain inspectable typed provenance at the boundary. It must remain possible to answer “what did this cell update read?” and “which evidence justified this capability claim?” without reverse-engineering a latent vector.

### 3.4 Individual adaptation and population evolution are different operations

The architecture needs two separately governed loops:

**Individual learning:** a civilian updates its own hypotheses and practice state from its own experiences and legitimate observations. This may happen during ordinary life within a declared learning envelope. It does not by itself expand authority.

**Population-level evolution:** candidate update rules, shared skills, or model parameters are compared across multiple civilians or isolated training populations. This is an engineering and governance operation. It must use sandboxed copies or explicitly opted-in test actors, record lineage, preserve the old revision, and pass an independent admission process before distribution.

A successful individual should not silently rewrite the common model for every other civilian. Conversely, a shared model improvement must not overwrite each civilian's individual history or remove their distinctive parameters.

---

## 4. Required System-1 CUA behavior

The trained CUA System-1 should be a bounded, low-latency controller. It may use learned patterns to select and sequence actions, but it must not invent permissions or bypass the physical game interface.

### 4.1 Observe before acting

Every action proposal must be grounded in timestamped observations from the relevant body and game environment. The agent should distinguish:

- directly observed state;
- remembered state that may now be stale;
- a report from another agent or human;
- a prediction produced by a learned model;
- an unknown or unreadable state.

A social demonstration is an observation with provenance, not a direct write into the receiver's motor state. A learned model prediction is not a world fact. The server's authoritative state and postcondition read-back settle what physically happened.

### 4.2 Learn trajectories, not just action labels

A learned demonstration should preserve enough context to reconstruct why the action worked:

- starting observation and body state;
- visible object/entity identity and affordance;
- relevant neighborhood and line-of-sight information;
- action sequence and timing;
- partner state, where relevant;
- game version, modpack, adapter, and ruleset revision;
- attempted result and independently observed postcondition;
- failure, cancellation, refusal, and environmental changes.

The learner may infer a reusable motor motif from this record. It may not treat the whole demonstration as a universally valid macro. It must test whether the same affordance and preconditions exist in its own context.

### 4.3 Learn from failed attempts

A failure should update a specific belief or skill hypothesis, not simply lower a global reward score. Examples:

- the tool is unsuitable;
- the target is out of reach;
- a route is blocked;
- an interaction requires a different orientation;
- a partner is not ready;
- the task exceeds solo capacity;
- the object is protected or the action is unauthorized;
- the server rejected the action or the expected postcondition did not occur.

The skill system must keep “not yet learned,” “attempted and failed,” “learned but context-inapplicable,” “temporarily unavailable,” and “forbidden” distinct.

### 4.4 Preserve individual behavioral signatures

Each civilian may have individual preferences and stable tendencies in its learned policy, such as:

- preferred safe approaches to a familiar affordance;
- tool/material selection under equivalent conditions;
- exploration versus repetition tendencies;
- action timing and movement motifs;
- social partner expectations;
- preferred order of low-risk candidate skills.

These are not hard-coded personality stereotypes. They are derived from longitudinal evidence and must permit revision. Seasonal, environmental, health, and task changes can alter observed style, so the record must preserve context.

### 4.5 Rest and inactivity must be decisions

A civilian must not become stateless or inert when there is no active task. The System-1 candidate set must include self-care, rest, movement, observation, social contact, exploration, and waiting where they are supported by its actual needs and authorization. Rest must be selected as an intention based on need and context; it cannot be the default state used to hide a missing controller.

This is consistent with the current prototype's explicit fatigue/rest candidate and its V4 “never-stateless” baseline. The post-test path must restore ordinary candidate generation and arbitration, not leave the bot waiting for another test script.

### 4.6 Separate training from live inference and action

The system should have three distinct execution contexts:

1. **Training/evolution context:** consumes admitted observation and outcome records, produces a candidate rule/model artifact, and may run expensive optimization. It has no live motor authority.
2. **Evaluation context:** runs candidate revisions in replay, shadow mode, isolated fixtures, or expressly bounded test actors. It records comparisons, failure cases, resource use, and held-out performance.
3. **Resident runtime:** loads only an admitted immutable revision, updates that resident's permitted local state, selects skills, and proposes bounded CUA frames through the existing action interface.

The resident runtime must not download and execute a newly generated model merely because a trainer marked it “best.” The active revision is identified by digest and admission record. Hot replacement is an explicit migration with a rollback point. Training/evaluation artifacts must not be allowed to mutate live game state, the canonical evidence store, or another civilian's personal memory.

A training sample is valid only when its action source is identified (human demonstration, agent action, replay, or test fixture), its result is observed, and its authority/test context is known. Server fixture setup must never be mislabeled as a civilian's successful action.

---

## 5. Social learning and cooperation expectations

### 5.1 Observation and social exposure

A civilian should be able to:

- notice another agent performing a visible action;
- retain the source identity and observation time;
- identify the objects, tools, spatial context, and visible outcome;
- compare repeated demonstrations rather than trusting a single instance;
- form an unverified skill hypothesis before personal success;
- decide whether to practice, ask a question, or ignore the demonstration;
- share the evidence with another civilian through an authorized communication channel.

The system must not require the demonstrator to enter a special “teacher” mode. Passive observation and proximity can be meaningful inputs. Explicit instruction, tool handoff, coaching, and correction are additional behaviors and must be recorded distinctly.

### 5.2 Social transmission is not automatic mastery

A receiving civilian may acquire a candidate pattern from observation or a report, but the system must preserve separate states for:

- observed;
- imitated as an experiment;
- independently attempted;
- physically verified in one context;
- demonstrated across multiple contexts;
- admitted as a capability under current Axis policy.

A copied skill record without personal verification must never be presented as personally mastered. A social transmission event may make a candidate more salient, but it does not grant a capability or work authorization.

### 5.3 Help-seeking

The System-1 should be able to reason:

1. What is the goal and what are its required preconditions?
2. Which steps can I perform alone?
3. Which step exceeds my current demonstrated capability, physical capacity, or permitted scope?
4. Is a known partner visible, reachable, eligible, and plausibly capable of helping?
5. What exact assistance is requested?
6. Can the request be declined, clarified, or allowed to expire without corrupting the civilian's own intention?

A help request must be a typed social act with its own ID and expiry. It should name the subject, requested capability/role, location or bounded interaction, and expected result. A recipient decides independently whether to accept. A request does not reserve another person's body, inventory, or time.

The experiment in the transcript is specifically informative about soliciting a human partner. The architecture should support partner-specific behavior rather than assume a universal solicitation policy across all agents or species.

### 5.4 Multi-agent coordination

Any action that requires two or more bodies must have an explicit coordination protocol. Minimum phases:

```text
PROPOSED
  -> REQUESTED
  -> ACCEPTED BY EACH REQUIRED PARTICIPANT
  -> READY / PRECONDITIONS VERIFIED
  -> SYNCHRONIZED ACTION WINDOW
  -> WORLD OUTCOME OBSERVED
  -> COMMITTED OR REFUSED / FAILED / EXPIRED
```

Requirements:

- no partner is assumed ready because it is nearby;
- each participant keeps its own AgentId, intention, motor, evidence, and authority check;
- each participant's action is bounded by its own capability and ticket;
- timing windows and timeouts are explicit and deterministic where replay is required;
- a participant can decline or stop where the interaction contract permits;
- one participant's command acknowledgement cannot prove the other's action;
- shared success requires the authoritative world's postcondition, not two positive self-reports;
- retries are idempotent and cannot duplicate a transfer or spend the same resource twice.

This models the transcript's learned waiting and synchronized pulling without inventing a permanent hive-mind or central puppet controller.

### 5.5 Social memory and partner-specific expectations

A civilian should retain evidence of prior cooperation, refusals, reliability, and shared outcomes with a particular partner. These records may influence whom it asks or how it times a request. They must not become unchallengeable labels. A single failure does not prove a partner is permanently incapable or hostile. A relationship estimate is not a grant and does not override the partner's current decision.

---

## 6. Axis gating: what may evolve, and what must never self-authorize

The most important boundary in this audit is between **learning** and **permission**.

### 6.1 Evolution classes

| Change class | Example | Required treatment |
|---|---|---|
| Experience update | Add an episode, increase a practice count, revise a local belief | May occur within the declared ordinary-life learning scope; preserve source and before/after state. |
| Personal hypothesis | Infer that a flexible tool is suitable for a task | Mark as a hypothesis; no execution claim or permission follows. |
| Existing skill refinement | Improve timing or path selection within an already admitted action vocabulary | Keep the same capability envelope; compare against regression and safety cases; preserve model/rule revision. |
| New skill composition | Combine admitted `look`, `move`, `use`, and `verify` primitives into a new bounded task skill | Open a ticket; declare preconditions, actions, postconditions, budget, and evidence; require independent gate evaluation before active use. |
| New motor primitive or game API call | Add a new CUA action, mob-specific controller, or screen operation | Explicit schema/adapter decision, threat review, bounded implementation, conformance tests, and current authority. Never auto-install from a learned pattern. |
| Shared model/rule update | Replace a population-wide CA update rule or CUA policy | Separate evolution campaign; isolate training; preserve lineage and old revision; test across held-out agents, tasks, environments, and game versions; independent admission. |
| Authority or scope change | Allow container access, destructive edits, combat, property transfer, or wider world mutation | Governance/Axis decision only. Skill evidence, grade, social rank, or model performance cannot grant it. |
| Package-format or trust-boundary change | Add executable code to package ingest or expand XML/package behavior | User-owned architecture decision, source selection, security review, reader/writer ownership, baseline, and explicit proof. No implicit format extension. |

### 6.2 Gate the action and the capability change separately

Every live physical action must pass the current action gate. Every proposal to add, widen, install, or distribute a capability must pass a separate capability-evolution gate. A skill that passed last week is not blanket authorization for today's target, location, partner, or resource.

A candidate may be valuable and still be refused. A skill's success rate is not authority. A social promise is not authority. A job title is not authority. A training ticket is not a permission to change the training system's own rules.

### 6.3 Minimum evidence for a capability proposal

A proposal should carry:

- stable AgentId or explicitly named test-population identity;
- candidate skill/model/rule ID and revision;
- source lineage: observation, trial, demonstration, imported package, or population-level training;
- ticket, opener, presenter, and declared scope;
- game version, modpack, body adapter, and ruleset identity;
- exact action catalog used;
- preconditions and expected postconditions;
- training/evaluation tick span and world-state digest;
- artifact bytes and digest;
- successful and failed episodes, including refusals;
- held-out tests and comparison baseline;
- resource, damage, time, and side-effect budgets;
- reversibility and rollback plan;
- known unsupported contexts and expiry conditions.

A claimant must not choose its own evidence origin or declare its own proof current. A digest only proves which bytes were measured; it does not prove that the behavior is safe or useful.

### 6.4 Axis readings remain orthogonal

The implementation must preserve the repository's separate axes. In particular:

- `EvidenceGrade` A is strongest while unit `Grade` A is below S; same letters do not mean the same thing;
- `certification_tier` is a requirement threshold, not the maximum observed score;
- `procedural_gate` is a per-action predicate result, not a scalar;
- `ticket_evidence` records origin and is not a grade;
- `establishment` records whether a witness is active, absent, stale, unchecked, or refused;
- a proof alone is insufficient: the supporting readings/lenses are separate evidence.

No single “evolution score,” “intelligence score,” “fitness score,” or aggregate star value may collapse these ladders. Each predicate must have run; an unrun predicate is not a pass.

### 6.5 Capability grants must stay bounded

The current `secure-shell` civilian port models grants as a named grantee, capability, scope, grantor, and tick. It rejects self-grants, unknown capabilities, wildcard scope, and empty required fields. The capability set in that port currently contains `observe`, `step`, `snapshot`, and `present`. It does not currently define `learn`, `install-model`, `mutate-rules`, or `grant-authority`.

That is a useful refusal baseline. Any new capability for learning or model distribution must be introduced through the governed source and a recorded decision, not added only to a client or package. A civilian may propose its own development; it cannot issue itself the grant that authorizes it.

### 6.6 Current implementation caveats

The repositories have promising pieces, but the inspected source does **not** justify treating the entire canonical gate as production-complete:

- The Minecraft prototype contains `AxisGate` and `AxisProof` ports, but `PHASE0_CONTRACTS.md` identifies its `Phase1GatePolicy`/`AuthorityGateway` as a provisional stand-in whose canonical enforcement point must be integrated.
- The current `secure-shell` proof-of-work document explicitly says its proof window is still supplied by the caller; the repository does not yet have the real sequential `DOMAIN_TICK` chain wired into that proof. Do not claim a cryptographically anchored world clock where none has been demonstrated.
- `secure-shell`'s `secure-shell.ingest/1` reserves the `code` kind and refuses it until a reader exists. This is an important protection against executable package content being accepted merely because it can be framed or hashed.
- A proof must be read back against actual bytes, be current, name an accountable presenter, and carry an origin not self-declared by the claimant. A signature or digest proves provenance/integrity properties, not truth by itself.

### 6.7 Current repository state versus the requested target

| Area | What the inspected source establishes | What remains unproven or absent for this target |
|---|---|---|
| Civilian identity and cognition | The prototype has a stable `AgentIdentity`, one `AgentMind` decision owner, needs, beliefs, intentions, reflexes, episodes, evidence, and a practice log. Its cycle is sense/update/reflex/select/gate/act/verify. | This is not yet a trained cellular-generative CUA policy. No NCA/evolutionary-CA substrate was found in the inspected prototype. |
| Skill execution | The prototype has bounded, versioned skill routines such as harvesting, eating, walking, requests, and storage transfer, with real outcome checks for the implemented slice. | A general skill compiler that turns evolved local patterns into arbitrary new CUA primitives is not established. New verbs remain governed changes. |
| Persistent continuity | `MindPersistence`, journal/snapshot recovery, stable AgentId and practice replay exist in the current prototype; the V4 document defines a never-stateless resident and post-test return to normal life. | The V4 specification itself lists post-test self-directed life, complete continuity, and production headless-client parity as outstanding acceptance proof. Relationship/conversation/commitment continuity is not complete in the inspected implementation status. |
| CUA embodiment | The codebase has fake-player and network/body adapter work, and the implementation status reports a Node mineflayer client path for chat and logical observation. | That Node path is not the same as a full headless Fabric client; the reported complete native Fabric client control/parity path remains unproven. |
| Axis | Local `AxisGate`/`AxisProof` code records selection, task, readings, arrangements, proof inputs, and refusals. | The project's own Phase 0 contract marks its local authority gateway provisional and says canonical Axis enforcement must replace it. Do not treat the local port as final integration. |
| secure-shell backend | An append-only project ledger, typed command boundary, digest chain, fsync, idempotency, authenticated transport, package admission, and client-witness doctrine are present in the inspected source. | The client surface and proof clock have their own incomplete stages; the architecture document explicitly distinguishes backend tests from a client-visible witness. |
| Proof of work | The port checks artifact bytes, time-window shape, presenter, origin, and supporting readings. | The proof-of-work document says the actual sequential world tick chain is not yet wired; current proof windows are caller-provided. |
| XML/package delivery | `dl-openshell` has `shploingus.recipe/3`, package frames, bounded XML-to-node conversion, layout declarations, and a package reader. | It is a declarative presentation and package mechanism, not an admitted runtime for new civilian cognition or arbitrary executable CUA code. |
| Mob APIs | V4 defines mob replacement as a later, type-specific adapter family and calls for entity-family behavior atlases. | No generic mob takeover is justified by the current player slice. The active civilian project target is Minecraft 1.20.1/Fabric; historical 1.21.1/NeoForge references in the document are superseded, not implementation instructions. |
| Generative learning | None of the inspected repositories establishes cellular automata as an implemented civilian learning engine. | This document is an expectation list and integration proposal, not evidence that CA learning is already built. |

This table is important for milestone reporting: a green test of a fake-player skill routine does not prove trained CUA; a client logical snapshot does not prove GPU perception; a package mount does not prove a new skill; and a local Axis port does not prove the final canonical authority boundary.

---

## 7. secure-shell v2 and XML/package expectations

### 7.1 Preserve the existing ownership split

The current architecture documents a backend/client split:

- `secure-shell` owns the package stream, server-side ledger, receipts, admission/security verdicts, encrypted prompt vault, and event stream.
- The native shell client owns windows, lanes, display, client-side checks, and rendering of notices.
- The Minecraft client is a downstream consumer of the package stream for its declared UI surface; it is not a second ledger writer.
- The civilian runtime owns its own minds and life continuity store; the project activity ledger does not become the civilian's memory database.
- The game server owns physical world truth and postconditions.

Do not merge those planes to make a convenient “all-purpose agent server.” In particular, the shell's package feed may describe or present a capability; it does not grant that capability or move a body.

### 7.2 Use XML as a declarative surface, not an execution authority

`dl-openshell` packages use a versioned `shploingus.recipe/3` declaration and `SHPKG002`/`SHPKG003` package frames. The shell's package reader validates the frame and digest before reading the recipe. Its XML layout path is converted to a bounded node tree; the inspected `LayoutTree` parser applies allowlists for tags, attributes, and style properties, and enforces limits on XML bytes, CSS bytes, node count, depth, and text length. Script/style include blocks are not a grant to execute arbitrary package scripts inside that parsed tree.

For future civilian tools and game UIs, the same principle should hold:

- package schemas declare IDs, versions, UI structure, bounded data bindings, capability references, and action-request shapes;
- package content can present a skill, ticket, witness, refusal, or civilian state;
- a button can request a named server-side action, but the server resolves authority again;
- XML cannot call a motor directly, alter the mind store, add a new action verb, grant a capability, or certify its own package;
- renderer behavior and action semantics remain owned by their existing implementation;
- package limits, schema versions, digest framing, parser ownership, and fallback behavior are explicit.

### 7.3 No second reader, no shadow package format

A new game adapter or civilian UI should consume the existing selected package/stream contract. It should not introduce a competing parser or an unofficial sidecar format because it is convenient. Any new format requires a user decision naming the owner, source path, format/version, entry point, deliberately excluded files, rejected alternative, and evidence of contract equivalence.

The existing `secure-shell` rule—one owning reader, one store, recorded selections, explicit refusals—should apply to future game packages as well. A test page or mock XML preview can help development but cannot be reported as a live client witness.

### 7.4 Package evolution lifecycle

A model or skill package should pass this sequence:

```text
DECLARED
  -> BUILT FROM PINNED SOURCE
  -> DIGEST / SCHEMA VERIFIED
  -> SERVER ADMISSION
  -> AXIS CAPABILITY REVIEW
  -> MOUNT RECEIPT / VERSIONED REVISION
  -> CLIENT CONSUMPTION
  -> CLIENT-SIDE WITNESS
  -> OBSERVED RUNTIME VALIDATION
  -> PROMOTION, HOLD, REFUSAL, EXPIRY OR ROLLBACK
```

A successful mount is not proof the skill works. A parsed XML tree is not proof the CUA acts correctly. A training run is not a production admission. The previous admitted package should remain available until the new revision has passed its declared checks and the replacement is committed atomically.

---

## 8. Future mob APIs and other game adapters

The V4 Minecraft architecture explicitly places player-agent embodiment before mob replacement and treats mob control as a separate adapter family. A future “Mobs API” must not be interpreted as a universal behavioral contract for every entity.

### 8.1 Per-entity-family behavior atlas

Before taking over an entity type, the adapter must inventory and test:

- decision ownership (`GoalSelector`, `Brain`, custom controller, or other API);
- sensors and perception cache;
- navigation and movement controls (ground, aquatic, flying, climbing, riding, etc.);
- schedules, memory, profession, POI, target acquisition, and target-loss rules;
- interactions, pickup, inventory, breeding, work, attack, damage, and block effects;
- lifecycle: spawn, age, despawn, unload, death, conversion, breeding, dimension transfer;
- persistence fields and synchronized presentation data;
- external mod/event hooks and cancellation behavior;
- client animation, equipment, sound, and replication consequences.

A villager-like Brain entity is not interchangeable with a simple GoalSelector mob, even if both share a base entity class. A missing adapter means retain the declared vanilla owner or refuse enrollment; it does not mean attach a generic CUA and hope.

### 8.2 Explicit AI ownership modes

Use explicit ownership states such as `VANILLA`, `SHADOW`, `SYSTEM1`, and `FALLBACK`, with documented transitions and tests. `SHADOW` may observe and compare without issuing commands. `SYSTEM1` may act only after its adapter, action catalog, authority, and parity contracts pass. `FALLBACK` must be a known safe state—not an unrecorded switch to vanilla behavior that can conflict with the civilian controller.

Do not disable vanilla AI until the System-1 adapter has proved it can own the entity's complete required behavior surface. Do not let vanilla AI and System-1 issue competing motor commands.

### 8.3 Transfer learning is selective

A civilian may reuse an abstract pattern such as “wait until partner ready” across multiple entity types. It must not reuse the exact motor implementation or assume that a mob's `MoveControl`, a player's client input, and a custom game's movement API have identical semantics.

Transfer is admissible only when:

- the target adapter declares the needed semantic capability;
- the pattern's preconditions match the target body and game version;
- a target-specific test confirms the postcondition;
- unsupported behavior is reported rather than silently approximated;
- the target action passes the target game's current authority gate.

### 8.4 Cross-game adapter contract

For games beyond Minecraft, define a minimal adapter surface that exposes:

1. stable actor identity and lifecycle binding;
2. timestamped perception and epistemic status;
3. a closed, versioned action catalog;
4. bounded input generation (CUA/motor) or API operations;
5. authoritative outcome observation and postcondition read-back;
6. world/game version and active ruleset identity;
7. persistence and recovery hooks;
8. resource budgets, scope, refusals, and evidence hooks.

The System-1 and local generative substrate may be shared above this boundary. The physical motor, entity lifecycle, and action semantics remain adapter-specific.

---

## 9. Implementation expectations and acceptance tests

The following list is intended to be converted into project tickets. “Pass” means a reproduced, observable result on the named execution surface; not a class existing, a log line being printed, or a green test that exercises a different body.

### CGA-01 — Durable individual state

**Expectation:** CA state, rule lineage, skill hypotheses, and behavioral history are bound to a stable AgentId and survive disconnect, restart, body replacement, model upgrade, and test completion.

**Accept when:** an individual is stopped mid-learning, restarted, and resumes with the same identity, local state, source observations, and in-flight recovery record. No blank mind or duplicate body is created.

**Refuse when:** identity is derived from username/session alone; unreadable state is silently replaced; an experiment reset clears the resident's memory.

### CGA-02 — Explicit cellular semantics

**Expectation:** each cell/state unit has a documented meaning, neighborhood, update rule, input boundary, and permitted output. Update order, clock, numeric precision, and determinism policy are versioned.

**Accept when:** the same pinned state/rule/input produces a reproducible next state (or the system explicitly records why updates are stochastic/asynchronous and what replay guarantee is provided).

**Refuse when:** cells read arbitrary global world state, use undeclared hidden channels, or write directly to game state.

### CGA-03 — Local observation and affordance learning

**Expectation:** the civilian can observe a demonstrated object/tool interaction and form an unverified affordance or skill hypothesis before succeeding itself.

**Accept when:** the evidence trace identifies source AgentId, target/tool identity, observed trajectory, timestamps, local visibility, confidence, and the learner's later independent attempt.

**Refuse when:** the learner receives a success flag or action script without the observation context; observation is represented as personal mastery.

### CGA-04 — Observation changes candidate selection

**Expectation:** demonstrated behavior can change which candidate tools or actions the learner tries, while individual variation remains possible.

**Accept when:** a controlled test offers multiple viable and non-viable tools; observation changes pre-success selection patterns, and the test reports both influenced and non-influenced learners without forcing identical outputs.

**Refuse when:** all agents copy the same tool unconditionally or a failed copy is counted as a successful skill.

### CGA-05 — Trial-and-error refinement

**Expectation:** failed attempts change a relevant local belief or skill parameter and are preserved as episodes.

**Accept when:** each failure is classified (precondition, affordance, timing, reach, partner, authorization, engine rejection, or unverified outcome) and later attempts show a measurable, causally linked adjustment.

**Refuse when:** a global reward scalar hides the reason for failure, or the model retries the same invalid action without a changed precondition.

### CGA-06 — Passive social learning

**Expectation:** a learner can benefit from observation without a teacher action, explicit command, or reward handoff.

**Accept when:** one civilian performs a task under ordinary conditions, another is allowed to observe, and the second later demonstrates an independently verified improvement. The observer's access and exposure duration are recorded.

**Refuse when:** the demonstrator is secretly writing into the learner's skill store, or the test requires an instruction event but reports passive learning.

### CGA-07 — Individuality and longitudinal style

**Expectation:** personal tendencies remain distinguishable across repeated sessions and can evolve with experience/context.

**Accept when:** repeated comparable tasks measure within-agent stability and between-agent differences over time; the report separates context (season, environment, body state, ruleset) from persistent preference.

**Refuse when:** a style metric is converted into an identity diagnosis, intention claim, or authority rank; all residents are overwritten by one shared policy.

### CGA-08 — Non-rewarded exploration / intrinsic activity

**Expectation:** exploration, creative behavior, observation, or practice may continue without an immediate item reward when supported by the civilian's own needs and preferences.

**Accept when:** a bounded test demonstrates voluntary continuation or return to an activity without an immediate reward, records the actual candidate and resulting state, and permits the civilian to stop.

**Refuse when:** the system fabricates “interest” from a hard-coded animation or repeatedly forces the activity after refusal.

### CGA-09 — Help-seeking threshold

**Expectation:** the civilian recognizes when a task is beyond its solo ability or scope and can ask a suitable partner for a specific kind of help.

**Accept when:** solo attempts, estimated difficulty, partner selection, request contents, recipient response, and final outcome are all recorded. A request can be declined without corrupting the requester.

**Refuse when:** the requester sends a command, treats silence as acceptance, or treats a human message as permission to bypass Axis.

### CGA-10 — Partner-specific social behavior

**Expectation:** requests and coordination patterns may vary by partner history and channel, without assuming all partners behave alike.

**Accept when:** a suite includes a human, a familiar agent, an unfamiliar agent, and an unavailable/declining partner; the system distinguishes request, accept, decline, timeout, and failure.

**Refuse when:** results from a human-partner experiment are claimed to prove identical conspecific behavior.

### CGA-11 — Synchronized cooperation

**Expectation:** two or more agents can learn readiness, waiting, and coordinated timing for a task that cannot be completed by one participant alone.

**Accept when:** every participant independently accepts and passes its own gate; a declared readiness/synchronization protocol is used; server-observed postconditions prove the result; one shared transfer/work event is committed exactly once.

**Refuse when:** proximity counts as readiness; one agent's motor acknowledgement proves the other acted; the orchestrator puppets all participants through one privileged action.

### CGA-12 — Social provenance and skill transfer

**Expectation:** a learned pattern can spread through observation/report, while each receiving civilian keeps the source and its own mastery status.

**Accept when:** a third civilian receives the skill only through a documented contact/report path, independently attempts it, and can state whether the evidence is inherited, observed, or personally reproduced.

**Refuse when:** a population-wide model update erases each recipient's lineage or is treated as personal practice.

### CGA-13 — Capability evolution gate

**Expectation:** adding a skill, action primitive, adapter, or shared rule version requires a separate ticket and Axis decision.

**Accept when:** the candidate artifact, scope, source lineage, digest, presenter, evaluation window, test evidence, and rollback plan are independently checked and recorded.

**Refuse when:** the model installs or grants itself an action; a successful test automatically widens permissions; a proof alone is treated as admission.

### CGA-14 — Bounded CUA execution

**Expectation:** evolved skills compile only to the declared low-level action catalog and pass through the ordinary client/game interaction path.

**Accept when:** action inputs, ticket, presenter, adapter version, server events, and observed postconditions agree. A failed action is recorded as failed/indeterminate, not rewritten as success.

**Refuse when:** direct inventory/block/entity mutation substitutes for player action in production; a new CUA verb appears without a governed schema change.

### CGA-15 — Package and XML containment

**Expectation:** XML packages present and request approved functions; they do not execute arbitrary code or alter cognition/authority.

**Accept when:** the package frame and digest validate, the owning parser applies schema and resource limits, the surface is mounted through the existing stream, and its client witness can replay the result.

**Refuse when:** XML includes an unapproved script execution path, creates a second renderer/parser/ledger, or is credited with a working CUA skill because the UI rendered.

### CGA-16 — Mob adapter parity

**Expectation:** each entity family has a type-specific behavior atlas, lifecycle adapter, action catalog, and ownership transition before System-1 takeover.

**Accept when:** a representative entity passes perception, movement, interaction, lifecycle, save/load, replication, and external-hook tests on the exact game/API version.

**Refuse when:** an unsupported mob is enrolled under a generic adapter, vanilla AI is disabled before replacement parity, or fallback behavior is silent.

### CGA-17 — Test completion and ordinary life

**Expectation:** a test closes only its own ticket/run, releases test resources, reconciles the resident, and returns control to its ordinary System-1 schedule.

**Accept when:** after the test, the same AgentId retains memories, needs, affect, relationships, skills, possessions, position, and pending commitments; it selects its own next valid action or rest decision.

**Refuse when:** the harness teleports to spawn, resets inventory/health/needs, kills the client, clears intentions, or waits indefinitely for another test.

### CGA-18 — Model migration and rollback

**Expectation:** a new CA/CUA model revision can be installed or rolled back without changing the civilian's identity or deleting prior evidence.

**Accept when:** migration records source/target revision, schema compatibility, per-agent state transformation, before/after digest, failed fields, and recovery path. The prior approved revision remains recoverable.

**Refuse when:** migration is destructive, silently drops unknown state, or treats an unsupported model version as an empty state.

### CGA-19 — Performance and fidelity bounds

**Expectation:** per-agent cellular updates, social observation, coordination, and persistence remain bounded at population scale.

**Accept when:** tests report tick p50/p95/p99, update counts, neighborhood reads, memory writes, event fan-out, CPU/GC, network and storage costs across materialized, virtual, and mixed populations.

**Refuse when:** distant-agent simulation claims physical Minecraft actions in unloaded/non-ticking regions, or performance is inferred from a one-agent demo.

### CGA-20 — Scientific and behavioral validity

**Expectation:** all claims are proportional to measured outcomes and distinguish direct observation from interpretation.

**Accept when:** test reports include sample size, conditions, controls, variation, confidence/uncertainty, raw evidence locations, and alternative explanations.

**Refuse when:** a drawing style is called proof of artistic intent, a single successful demonstration proves general intelligence, or a simulation metric is presented as biological equivalence.

---

## 10. Recommended implementation order

1. **Freeze terminology and state ownership.** Define the CA/NCA/evolutionary-CA family actually being implemented, the meaning of a cell, the neighborhoods, update clock, and the relation to AgentMind, skill registry, memory, and CUA.
2. **Prove durable individual state first.** Bind local automaton state and revision lineage to existing durable AgentId and never-stateless records. Prove restart and test-completion continuity before population-level evolution.
3. **Build the observation-to-hypothesis path.** Capture demonstrations and affordances as evidence; produce candidate skills, not permissions.
4. **Build individual practice and failure learning.** Use the existing practice/evidence record discipline; record actual world deltas and rejected postconditions.
5. **Test individuality longitudinally.** Use multiple agents and repeated tasks, measuring stable tendencies and within-agent change without assigning personality labels from a single run.
6. **Add passive social learning.** Prove observation changes candidate selection before successful execution, without hidden teacher-side writes.
7. **Add help-seeking and partner-specific behavior.** Implement typed requests, independent acceptance, and timeout/refusal semantics.
8. **Add synchronization.** Prove two or more bodies coordinate a real shared task with independent intentions and authority checks.
9. **Introduce model evolution campaigns.** Train/test isolated candidate rules; record lineage and held-out results; require independent Axis admission and rollback.
10. **Integrate secure-shell v2/package/XML delivery.** Use one owning reader, the selected package and ledger paths, client-visible witnesses, and non-executable declarative surfaces.
11. **Port to mobs and other games only after player/body contracts are proven.** Build one entity-family atlas and adapter at a time.
12. **Scale population after complete continuity and performance tests.** Do not use a single-agent benchmark to set population claims.

---

## 11. Research and repository references

### Transcript-derived behavioral basis

- Supplied transcript: *Scientists Analyzed 500 Chimpanzee Drawings. The Results Were Unsettling.* YouTube: https://youtu.be/_Do9V1o9dNU. Transcript uploaded in this conversation. Sections used: honey-fishing observation; nut-cracking social spread; Mizuki's requests for human help; synchronized rope pulling; long-term drawings; sanctuary reintroduction and recovery; Hirata's approach to patient observation.

### Primatology research

1. Satoshi Hirata and Maura L. Celli (2003), “Role of mothers in the acquisition of tool-use behaviours by captive infant chimpanzees,” *Animal Cognition* 6, 235–244. DOI: https://doi.org/10.1007/s10071-003-0187-6. PubMed: https://pubmed.ncbi.nlm.nih.gov/13680401/. This study qualifies the transcript's honey-fishing account: three dyads, two infants showed strong selection for the adults' preferred tools, and no active teaching behaviors such as slow demonstration or assistance were observed.
2. Satoshi Hirata and Kohki Fuwa (2007), “Chimpanzees (Pan troglodytes) learn to act with other individuals in a cooperative task,” *Primates* 48, 13–21. DOI: https://doi.org/10.1007/s10329-006-0022-1. PubMed: https://pubmed.ncbi.nlm.nih.gov/17103081/. This study reports learning to wait and synchronize pulls; soliciting behavior was observed with the human partner, not the conspecific partner.
3. Cédric Sueur, Agathe Nicolas, Marie Pelé, and Satoshi Hirata (2026), “Filling, color, and shape: individuality in chimpanzee drawings,” *Primates* 67(4), 387–401. DOI: https://doi.org/10.1007/s10329-026-01263-2. PubMed: https://pubmed.ncbi.nlm.nih.gov/42096013/. Six chimpanzees; long-term inter- and intra-individual differences, seasonal variation, and filling/color/shape dimensions.

### Cellular automata research

4. Alexander Mordvintsev, Ettore Randazzo, Eyvind Niklasson, and Michael Levin (2020), “Growing Neural Cellular Automata: Differentiable Model of Morphogenesis,” *Distill*. DOI: https://doi.org/10.23915/distill.00023. Main paper: https://distill.pub/2020/growing-ca/. Supports learned local rules for growing, persistence, and regeneration of spatial patterns; does not establish autonomous embodied cognition.
5. Melanie Mitchell, James P. Crutchfield, and Peter T. Hraber (1994), “Evolving cellular automata to perform computations: mechanisms and impediments,” *Physica D* 75(1–3), 361–391. DOI: https://doi.org/10.1016/0167-2789(94)90293-3. Documents evolutionary search strategies and impediments in cellular-automata computation.
6. John P. Crutchfield and Melanie Mitchell (1995), “The evolution of emergent computation,” *Proceedings of the National Academy of Sciences* 92(23), 10742–10746. DOI: https://doi.org/10.1073/pnas.92.23.10742. Shows how sophisticated information processing can emerge in decentralized spatial systems.

### Project source repositories and inspected documents

- [ala_civ_bots_prototype](https://github.com/alacripose/ala_civ_bots_prototype): `AgentMind`, `MindLedger`, `MindPersistence`, `CandidateGenerator`, `AxisGate`, `AxisProof`, `docs/GAIA.md`, `docs/WORLD_INTERACTION.md`, `docs/PHASE0_CONTRACTS.md`, `docs/LAYER_TABLE.md`, and `docs/minecraft_ensouled_system1_cua_architecture_v4.md` (especially the player-agent, CUA, social chat, persistent resident, never-stateless, mob-adapter, implementation status, and acceptance sections).
- [secure-shell](https://github.com/alacripose/secure-shell) (private repository): `AGENTS.md`, `docs/ARCHITECTURE.md`, `docs/PROOF-OF-WORK.md`, `docs/GRADES.md`, `docs/DECISIONS.md`, `packages/axis/axis.data.json`, `server/ledger.mjs`, `server/ingest.mjs`, `tools/port/civilians.mjs`, and `tools/port/star.mjs`.
- [dl-openshell](https://github.com/alacripose/dl-openshell): `README.md`, `shell/alpha-shell/package.shspec`, `src/Openlock/Shell/ShpPackage.cs`, `src/Openlock/Shell/AddonPackages.cs`, and `src/Openlock/Shell/LayoutTree.cs`.

---

## 12. Final architectural rule

The chimpanzee studies are valuable here not because civilians should imitate chimpanzees, but because the studies force the architecture to represent development as a long-running, individual, social, and context-sensitive process.

The target system should therefore support **observation without instruction, practice without guaranteed success, individual style without isolation, help-seeking without commandeering, cooperation without centralized puppeteering, and evolution without self-authorization**.

Cellular generativity may help produce the distributed patterns that make those behaviors possible. It does not decide what a civilian is, what the civilian may do, or whether a proposed capability is admissible. Those remain the work of persistent identity, System-1, the real CUA/body adapter, authoritative world verification, and Axis.
