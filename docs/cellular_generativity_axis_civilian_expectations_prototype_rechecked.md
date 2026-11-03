# Cellular Generativity and Civilian Evolution
## Evidence-led audit and Axis-gated expectation list for trained System-1 CUA

**Document status:** Architectural audit and proposed acceptance contract; not an implementation claim  
**Evidence basis:** supplied transcript of *Scientists Analyzed 500 Chimpanzee Drawings. The Results Were Unsettling.*; `ala-cities` (`main` and `design-package-cli`); `ala_civ_bots_prototype` (`main` and inspected feature branches); user-supplied Secure Shell v2 streamed inventory dated 2 November 2026; cited cellular-automata and primatology research  
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

### 1.1 Scope correction: one Axis star verifies a lived civilian society

The cellular-generativity target is broader than a skill-learning subsystem. One of the six Axis stars is the **civilian lived-verification star**: persistent civilians live through an authoritative cadence clock, meet needs, choose and perform work, develop skills and learning trees, form organizations, and establish institutions of governance including courts. The star verifies that this is a real, continuous simulation architecture, not a staged sequence of isolated demonstrations.

The cellular automata are part of that real architecture and are **controlled by the civilians themselves through their own agent decision process**. They are not merely a passive tool that emits a list of candidate skills for an external scheduler. Their agent-owned generative state can participate in how a civilian adapts, learns, coordinates, and carries out its life. That control remains bounded by the civilian's identity, its own decision history, the world rules, and the permissions applicable to external or institutional acts.

This does not make Axis the city's mayor, legislature, court, or task allocator. Axis remains the outer verification and authorization boundary. The in-world government, organizations, offices, laws, and courts must emerge through the citizens' own acts and institutional development. The star observes whether that society can live and develop over time; it must not manufacture the society's outcomes by assigning jobs, founding organizations, appointing judges, or scripting legal findings on the civilians' behalf.

This correction supersedes any reading of §§3, 13.5, or 13.11 in which cellular automata are only a detached candidate generator. The distinction that remains binding is narrower and more precise: **agent control of its own cellular architecture is not self-authorization to exercise another person's rights, hold public office, spend institutional resources, bypass a court, or exceed an Axis/game-adapter grant.**

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


---

# 13. Secure Shell v2 streamed-inventory integration addendum

**Addendum date:** 2 November 2026  
**Additional source:** user-supplied `secure_shell_synthesis.md`, headed *secure shell v2 — streamed inventory*.  
**Purpose:** bind the preceding civilian-evolution expectations to the v2 package/cell/ledger ecosystem without inventing schema semantics that the inventory does not show.

## 13.1 What this new source changes

The v2 inventory is a newer and more relevant architectural index for the target than the earlier remote-repository snapshot alone. It identifies active worktrees and file locations across `testport1`, `shploingus-clean`, `shploingus-game`, `shploingus`, `openlock`, `secure-shell`, `dl-enclave`, `ala_civ_bots_prototype`, `ala_package_designer`, and related trees.

The strongest change is that the civilian-evolution target is no longer adequately described as “a cellular automaton added to the civilian prototype and then displayed through Secure Shell.” The inventory shows a family of versioned cell, package, civilian, ticket, decision, admission, served, and game-action identifiers already present in the development ecosystem.

The target should be understood as:

```text
persistent civilian identity / System-1
        |
        +-- observations, practice, social exposure, memories
        |
        v
cellular-generative candidate state
        |
        +-- shploingus.cellframe/1
        +-- shploingus.cells/1
        |
        v
civilian decision / ticket / Axis-cell evidence
        |
        +-- civilian.decision/1
        +-- civilian.ticket/1
        +-- civilian.axis-cells/1
        |
        v
package ingest and admission
        |
        +-- secure-shell.ingest/1
        +-- secure-shell.admission/1
        |
        v
served / mounted package surfaces
        |
        +-- secure-shell.served/*
        +-- surface.mount/2
        +-- openlock.addons/2
        |
        v
game-specific adapter and CUA action
        |
        +-- minecraft.cua-action/1
        +-- ensouled.action/1
        +-- ensouled.frame/1
        |
        v
authoritative game result -> evidence -> persistent civilian learning
```

This is a conceptual responsibility map, not a claim that the inventory proves every arrow is already implemented or that the identifiers have precisely these field-level meanings. The actual readers, writers, schema definitions, and tests remain authoritative.

## 13.2 V2 inventory facts that must be preserved

The supplied inventory reports:

- 119,928 files inspected across the scan;
- 2,241 XML files;
- 129 distinct own numeric identifiers, with 1,061 occurrences;
- 83 qualified identifiers and 46 unqualified identifiers;
- package/container magics `V4PKG001` (10), `V4PKG002` (1), `SHPKG002` (8), `SHPKG003` (9), and `SHPKG009` (1).

The totals are scan results, not an assertion that every file is a package or that every discovered format is active in production. The scan itself records `audit-stream-truncated` for a root that reached its budget (`F:\shploingus-mod-sources`), so that part of the inventory is explicitly incomplete. It also records the refusals `audit-gap-silent`, `audit-unprovenanced`, and `audit-unstreamed`: absent roots must not be reported as zero findings; counts without file/line provenance are insufficient; and buffered output that hides truncation is not admissible.

The package-format families coexist. The inventory reports `shploingus.recipe/3` and `/4`, `secure-shell.served/1` and `/2`, as well as older/newer package magic values. It does **not** establish that the larger version number automatically supersedes the smaller one, that every old reader has been removed, or that all packages have migrated. Compatibility and ownership must be read from the owning source and test suite.

## 13.3 The most relevant identifier families

The inventory's own qualified-identifier totals include:

| Identifier | Occurrences | How this audit uses the finding |
|---|---:|---|
| `shploingus.cellframe/1` | 57 | Cell-frame family is a first-class v2 format candidate, not an informal metaphor |
| `shploingus.cells/1` | 50 | Cell records have a versioned family identity |
| `surface.mount/2` | 91 | Existing mount-v2 family remains prominent in the scanned ecosystem |
| `openlock.addons/2` | 61 | Existing addon-delivery family remains prominent |
| `secure-shell.served/1` | 48 | Served v1 is still present in the scan |
| `secure-shell.served/2` | 6 | Served v2 is also present; coexistence must be investigated rather than assumed away |
| `secure-shell.ingest/1` | 25 | An explicit ingest boundary is present |
| `secure-shell.ledger/1` | 24 | The ledger family remains part of the v2 ecosystem |
| `openlock.envelope/1` | 36 | Openlock envelope family appears in the packaging lineage |
| `shploingus.recipe/4` | 23 | Recipe v4 appears alongside recipe v3 |
| `civilian.axis-cells/1` | 9 | Civilian-specific Axis-cell records are already named |
| `civilian.decision/1` | 9 | Civilian decision records are already named |
| `civilian.ticket/1` | 6 | Civilian tickets are already named |
| `system1-agent/1` | 3 | A System-1 agent identity/record family is named |
| `minecraft.cua-action/1` | 2 | A Minecraft-specific CUA action family is named |
| `ensouled.frame/1` | 3 | An Ensouled frame family is named |
| `ensouled.autopopulation/1` | 2 | An auto-population family is named |
| `ensouled.action/1` | 1 | An Ensouled action family is named |
| `secure-shell.admission/1` | 7 | Admission has a separately named family |
| `secure-shell.material-grade/1` | 4 | Material-grade data has a separately named family |

Counts are inventory facts. The responsibility descriptions in the final column are bounded interpretations of the identifier names and their location in the named repositories; they are not schema reconstructions.

The inventory locates the core civilian formats in `C:\Users\ala\Documents\ala_civ_bots_prototype`; the Axis/civilian source lineage in `shploingus-clean`; the streamed package work and ownership/port plan in `testport1`; and the package and admission ecosystem in `secure-shell`, `openlock`, and `dl-enclave`.

## 13.4 V2 rule: the cell is not the civilian

The cell substrate must not become an alternative identity or persistence system.

A `system1-agent/1` identity, if that is the owning identity record named by the v2 implementation, must remain bound to the durable civilian identity already required by the never-stateless baseline. A cellframe, cell collection, action frame, model revision, package revision, client process, or test run is not a replacement identity.

Minimum separations:

| Thing | What it is allowed to represent | What it must not replace |
|---|---|---|
| Civilian / `AgentId` | Continuous identity, biography, needs, relationships, memory, commitments, and personal history | A cell index, model hash, username, package name, or live process |
| Cellframe | A versioned computational frame for the cell substrate, according to its owning schema | A resident record or whole-person snapshot |
| Cells | Local state and/or local computational units, according to the owning schema | Global authorization, identity, durable history, or authoritative game state |
| System-1 | Candidate generation, interpretation, arbitration, and intention continuity | Axis/Governor authority or the physical game server |
| CUA action | A bounded input attempt through an installed adapter | Proof of success merely because input was sent |
| Decision record | What was selected, rejected, held, or deferred and why | A capability grant or package admission |
| Ticket | A bounded unit of authorized work | A permanent command channel or a life identity |
| Axis-cell evidence | The required evidence/witness for a named Axis reading | A combined fitness score or self-certification |

A civilian can have many cell states, model versions, skills, and histories over one continuous identity. A model migration changes the computational implementation, not the person. When a cellframe cannot be read, the civilian must enter an explicit recovery/integrity state; it must not be re-created as a blank resident.

## 13.5 V2 rule: civilians control their cellular architecture; Axis controls authorization

The earlier candidate-only phrasing is too narrow for the intended civilian architecture. A civilian's cellular automata are an **operational, agent-controlled part of its own life and learning system**. The civilian's System-1 decision process owns the purpose and direction of its cellular processes within the declared computational, memory, and resource contract. The automata may maintain and evolve local patterns that participate in perception, affordance learning, skill acquisition, task planning, timing, coordination, and recovery. They are not restricted to writing detached proposals that some global controller later turns into behavior.

The civilian remains the decision owner. The cell system supplies structured local computation and generative adaptation; it does not become a second person, an unaccountable executive, or a global puppet master. The agent must be able to inspect and reason about its own cellular process at the declared boundary, preserve its history and lineage, select when to use it, decline or revise an unsuitable pattern, and distinguish its own learned state from another civilian's state.

The substrate may therefore update operational internal state, including: the civilian's own affordance hypotheses; practiced action motifs; coordination timing; observations and exposure traces; confidence and uncertainty; local readiness; and recovery state. These updates can influence the civilian's actual subsequent choices and work. They must be attributable to an agent, an exact cellframe/cell schema and rule revision, an authoritative tick, permitted inputs, and evidence.

It must not silently rewrite or claim ownership of:

- another civilian's identity, memory, cellular state, body, inventory, or decisions;
- the durable identity/genesis of the owning civilian or the provenance of its past;
- the world's authoritative state, conserved resources, legal record, or another actor's property;
- an organization's membership, scope, account, or authority;
- a citizen's public office, an election result, a court finding, a statute, or a treasury appropriation;
- current Axis grants, Governor authority, ticket scope, package admission, or a game's supported action vocabulary.

A learned internal pattern may be used by the civilian within its already-authorized internal and external action envelope. When it leads to an external game action or an institutional act, the ordinary adapter, ticket, world-authoritative read-back, and relevant authority checks still apply. The Axis boundary does not need to approve every lawful internal cell update; it must decide the grants and consequential boundaries that its contract owns.

Keep these records separate even when they are causally connected:

1. **Cellular state and rule evolution:** what the agent's own local generative system currently represents and how it changes.
2. **Individual demonstrated skill:** a derived claim supported by that civilian's own completed work and outcome history. C9 Q96 specifies skill as a view over the citizen's record, not a freely writable scalar.
3. **City-wide insight/progression:** discoveries accumulated by the society; C9 specifies that these do not disappear when an individual dies.
4. **Capability DAG / learning tree:** the declared, tiered and branched dependencies that make processes, tools, and forms of work available as the society develops. This is not the cellular grid itself.
5. **Organization capability:** a derived view over the skills and records of current members; C9 Q164 specifies that an organization can lose an ability when it loses its only qualified member.
6. **Authorization:** the current authority decision for a particular citizen, institution, adapter, action, scope, and revision. It is not inferred from any of the preceding five records.

A civilian may directly control its own cellular automata without thereby gaining authority over anyone else. A successful internal update, high fitness score, completed practice, or node in a learning tree is not an office, statute, permission, or grant.

## 13.6 V2 rule: package admission and presentation are different from execution

Secure Shell v2's ingest, admission, served, and mount families must remain in their own ownership boundaries. A cellular package must not be trusted merely because it is syntactically parseable, hashed, displayed, or mountable.

The expectation for a capability package is:

1. It has an explicit package identity, schema/version, declared contents, and artifact digests.
2. The owning ingest reader validates the incoming frame; unknown kinds or unsupported versions are refused by name.
3. The admission owner checks the declared capability, source, evidence, authority, compatibility, and scope.
4. Axis decisions and civilian evidence are linked to the exact candidate artifact and revision.
5. The package is served/mounted only through the already selected stream and receipt owners.
6. A failed or incompatible replacement leaves the previously admitted revision intact, where the owning replacement contract says it should.
7. The client renders the result but does not issue its own security verdict or become the writer of the authoritative ledger.
8. Every rejection, stale revision, missing witness, incompatible reader, and failed test remains inspectable as a refusal/evidence record.

The observed presence of `secure-shell.ingest/1`, `secure-shell.admission/1`, `secure-shell.served/*`, `surface.mount/2`, and `openlock.addons/2` supports using the v2 package path as the integration boundary. It does not, by itself, tell us the exact field contract or which version owns every transition. Those must be read from the named source implementations and tests before code is written.

## 13.7 V2 rule: XML and cell declarations remain data, not an execution escape hatch

The scan reports 2,241 XML files across its scanned roots, and the relevant repository inventory includes XML layout/package material. XML is therefore a substantial part of the wider ecosystem, but its mere presence does not prove that every XML file is a capability package or that all are accepted by the same reader.

For civilian evolution, declarative XML may describe:

- a user-facing view of cell state, capability candidates, evidence, tickets, or refusal reasons;
- fixed, versioned UI/control structures;
- bounded rules or cell topology only where the owning package schema explicitly allows them;
- adapter declarations and references to separately admitted implementation artifacts.

XML must not gain arbitrary script execution, reflection, filesystem/network access, direct game mutation, hidden privilege escalation, or a second command/control channel. The reader must have one owner, a closed grammar, bounded depth/size/node counts, allowed tags/attributes/styles, path confinement, and explicit handling of unsupported content. The actual v2 XML contract must be verified from its owning source; the inventory is not enough to claim those constraints are already implemented in every path.

A cell package and its XML presentation are not allowed to disagree about identity, package revision, schema, or evidence digest. The UI is a projection of admitted records, not the origin of truth.

## 13.8 V2 rule: CUA and mob APIs are adapters, not cellular actions

`minecraft.cua-action/1`, `ensouled.action/1`, and `ensouled.frame/1` are useful named integration points in the inventory. The exact payload contracts remain to be checked in their source files.

The desired responsibility split is:

```text
cellular policy / candidate
    -> System-1 selection
        -> Axis check for the proposed effect
            -> ticketed action/frame
                -> game/entity adapter
                    -> normal game input / API
                        -> authoritative game result
                            -> evidence and civilian learning
```

The cell substrate must not call a Minecraft or mob API directly. It may propose a known, typed action. The relevant adapter translates that action into the target game's supported semantics and returns observations of the actual result.

This is particularly important for mobs: a generic concept such as move, look, interact, attack, eat, follow, or rest does not imply that every entity family implements the same navigation, targeting, memory, lifecycle, or interaction semantics. A `GoalSelector` mob, a `Brain`-driven villager, an animal, a flying entity, and a modded creature need distinct capability atlases and adapters. The shared substrate can hold identity, memory, candidate patterns, and learning provenance; it cannot flatten their physical APIs into one assumed motor.

## 13.9 Additional acceptance expectations for v2

These extend the earlier CGA-01 through CGA-20 expectations. They do not claim that the inventory proves implementation.

| ID | Expectation | Acceptance evidence | Refusal condition |
|---|---|---|---|
| CGA-21 | Cellframe and cells use their owning versioned readers | Exact schema, reader/writer, fixtures, malformed-input tests, and digest identity | A second parser, guessed grammar, unqualified `cell/4`, or silent version conversion |
| CGA-22 | Cell state is bound to a durable civilian identity without becoming that identity | Same `AgentId` across cellframe revisions, client reconnect, reboot, test end, and model migration | New/blank identity after a cell or process change |
| CGA-23 | Each evolved candidate links its decision, ticket, Axis-cell witnesses, and exact artifact revision | Readable chain from observation/practice → candidate → decision/ticket → proof/evidence → admission/refusal | Candidate with no traceable source or missing link |
| CGA-24 | Learning and authorization are independently represented | Tests demonstrate a learned-but-not-admitted capability and an admitted capability with a bounded scope | Learning, fitness, certification, or practice counter grants permission |
| CGA-25 | Ingest and admission are separate, fail-closed boundaries | Unsupported schema/kind, digest mismatch, absent authority, stale evidence, and incompatible adapter tests | Parse success is treated as admission |
| CGA-26 | Served/mounted views are projections of admitted state | Surface/receipt revision and artifact digest match; client read-back and replay agree | Client creates its own security verdict or stale package replaces the admitted one |
| CGA-27 | XML is a bounded declarative surface | Parser allowlist/limits, path confinement, no arbitrary code execution, malformed XML and malicious reference tests | XML becomes an alternate executable or privileged action path |
| CGA-28 | CUA actions are adapter-mediated and server-verified | The action frame identifies actor, ticket, presenter, adapter/version, input, result, and world evidence | A sent input, API return, or local animation is represented as physical success |
| CGA-29 | Mob/API extensions preserve entity-specific semantics | Per-family capability atlas and parity tests for lifecycle, movement, perception, actions, and external hooks | One generic mob adapter claims unsupported family behavior |
| CGA-30 | Scan/audit evidence is itself complete and provenance-bearing | Streamed per-root output, file/line references, explicit absent/truncated states, reproducible digest | A partial scan is called complete; a count is used as schema proof |

## 13.10 Critical open verification work

The v2 inventory is a discovery and topology document, not a source-code audit of every identified format. Before implementation decisions are made, the following must be read from the owning repositories:

- the actual `shploingus.cellframe/1` and `shploingus.cells/1` schema definitions, readers, writers, and tests;
- the package/container implementation for `V4PKG001`, `V4PKG002`, and `SHPKG009`, and its explicit relationship (if any) to `SHPKG002`/`SHPKG003`;
- the exact v2 `secure-shell.served/2` contract and its compatibility relationship with `served/1`;
- the `secure-shell.ingest/1` and `secure-shell.admission/1` reader/writer and refusal tests;
- the precise semantics and field schemas of `civilian.axis-cells/1`, `civilian.decision/1`, `civilian.ticket/1`, and `system1-agent/1`;
- the source/tests that own `minecraft.cua-action/1`, `ensouled.action/1`, `ensouled.frame/1`, and `ensouled.autopopulation/1`;
- the XML reader/parser and its security limits for every package path that can deliver XML;
- the current ownership decisions in `testport1/docs/PACKAGE-OWNERSHIP.md`, `PORT-BASELINE.md`, `SCOPE-ACCOUNTING.md`, and `SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md`.

Until that inspection is performed, these names are integration anchors, not permission to invent a data model or implementation.

## 13.11 Revised synthesis

The transcript-derived expectations remain: observation without active teaching, learning before mastery, individual practice, help-seeking, partner-sensitive cooperation, coordination, persistent personal style, and continuity across long time spans.

The v2 inventory adds the package and evidence boundary. The `cellframe`/`cells` family belongs inside a continuous civilian's own agent-controlled architecture. Its changes can affect real choices and behavior in the lived simulation; they are not restricted to detached candidate records. At the same time, identity, personal history, civic records, authority, ticketing, package admission, served/mount, and game adapters remain separately owned.

The resulting rule is:

> **A civilian controls and develops its own cellular architecture as part of living its life. That architecture may change how the civilian learns, works, coordinates, and adapts; it cannot erase the person, control another civilian, create civic or legal authority, self-grant an Axis capability, bypass package admission, or act outside the authorized adapter path.**

This remains an expectation baseline. The streamed Secure Shell v2 inventory identifies integration families but does not prove their complete contracts or runtime acceptance. The civilian lived-verification star defined in §14 supplies the missing longitudinal context: these components must work together across real simulated life, not merely pass isolated package or action tests.

## 14. The Axis civilian lived-verification star: cadence, work, learning, and civic institutions

### 14.1 What this star is responsible for

One of the six Axis stars is the **civilian lived-verification star**. This is the user's stated top-level six-star model; the inspected `ala_civ_bots_prototype` code does not define a six-star enum or canonical six-row star registry. Its present Axis code defines nine typed ladders instead (see §14.13), which are a different level of the architecture. This document does not infer names or responsibilities for the other five stars. This star's subject is not a single candidate skill, a test agent, or a short scripted demo. Its subject is a population of continuous civilians living in a deterministic world over cadence ticks.

The star must verify that civilians actually:

- retain personal identity, memory, needs, skills, relationships, commitments, and a life history across simulation time and runtime changes;
- make their own decisions through their System-1 owner and control their own cellular-generative architecture;
- satisfy needs and respond to deprivation, illness, recovery, aging, and death according to declared rules;
- obtain and hold jobs, commute, perform useful work, form crews, and contribute measurable labour;
- found and operate organizations with real premises, members, scope, work, equipment, accounts, and claims;
- learn through observation, practice, cooperation, and verified outcomes;
- develop individual skills and collective insight, and advance a real capability DAG/learning tree that gates processes and tools;
- establish governance, offices, a chain of responsibility, elections or appointments, statutes, and appropriations;
- develop courts/benches that hear and determine cases, including historical offences that were recorded before a court existed; and
- preserve the resulting individual and civic record as a society changes, prospers, stagnates, or collapses.

A result produced by a central test script that assigns every civilian its job, organization, office, and verdict is not a pass. The verification scenario may define initial conditions, world seed, lawful starting constraints, and observation windows. The civilians must produce the consequential life and institutional activity themselves through their own decisions and the real simulation systems.

### 14.2 The relationship between System-1, CA, and the world

The intended control relationship is:

```text
One durable civilian (AgentId)
  ├── biography, memory, needs, relationships, commitments
  ├── System-1 decision owner
  │     ├── interprets current needs and world state
  │     ├── chooses / declines / defers / requests help
  │     └── controls its own cellular-generative processes
  ├── agent-owned cellular automata
  │     ├── local state and neighborhood dynamics
  │     ├── adaptation, practice motifs, affordance and timing patterns
  │     ├── individual lineage and revision history
  │     └── signals returned to the owning agent for decision and action
  └── declared capability and action envelope
            │
            ▼
  citizen's actual in-world decisions and actions
            │
            ├── personal needs, rest, shelter, care
            ├── work, jobs, tasks, commutes, crews
            ├── organizations, premises, roster, scope, claims
            ├── learning tree, tools, processes, shared insight
            └── public acts, offices, law, courts, institutions
            │
            ▼
  authoritative world outcome + civic record + evidence
            │
            └── feeds the same civilian's next life/learning cycle

Axis star: observes and verifies this longitudinal system.
Axis authority: remains outside the in-world government and grants only what its own contract owns.
```

The CA is not the person and System-1 is not displaced by the CA. The agent owns and controls its own automata as a computational part of its life. This allows the automaton to be operational rather than a passive suggestion engine while preserving a clear accountable decision owner. A CA update must never become an unreviewable direct write to the authoritative world, another person's state, or a public/legal record.

The in-world institutions are **not Axis institutions**. Axis is the outer verifier/authorization system. The society's citizens must create and staff their own organizations, offices, government, and courts under the city's rules. The technical `src/gov` code and season/ticket ledger in `ala-cities` must not be mistaken for the completed in-world government described by C9.

### 14.3 Two clocks: the physical world clock and the civilization cadence

The civilian architecture needs **two related clocks**, not one clock stretched across every subsystem:

1. **Physical world clock:** the Minecraft server tick and the ordinary chunk, block-entity, physics, entity, and connected-player processing that Minecraft actually performs. This clock is authoritative for physical movement, collisions, inventory changes, block edits, machine operation, and every claimed physical outcome.
2. **Civilization clock:** a server-owned scheduler for persistent civilian cognition, needs, affect, relationships, jobs/contracts/tickets, production/logistics/civic simulation, regional events, memory, and materialization requests. It must continue to advance a civilian's life when its player body is not currently materialized or when its region is outside ordinary entity simulation distance.

The clocks exchange explicit observations and action results. The civilization clock may progress a need, relationship, schedule, regional obligation, or clearly labelled abstract event while a body is absent. It must **not** claim that the civilian physically walked through an unloaded chunk, mined a block, opened a chest, or changed a furnace/redstone circuit while Minecraft did not simulate that physical world. Physical work requires a materialized body and a ticking physical region, or an explicitly declared abstract service whose capacity, delay, access, loss, and evidence are represented.

`ala-cities` is a useful concrete cadence reference for the civilization side: its README describes a fixed 20 Hz simulation; 40 ticks per simulated day; 30 days per month; and 60 seconds of simulation per month at 1×. Speed changes alter how quickly ticks are delivered, not the logical meaning of a tick. Those constants are reference values, not an unapproved requirement that Minecraft or every Axis environment use the same calendar. The mapping between Minecraft server ticks and civilization ticks must be versioned, deterministic, replayable, and independent of render frame rate.

C9's three simulation classes provide a useful separation:

| Update class | Examples | Verification rule |
|---|---|---|
| **Exact every tick** | positions, movement, current task progress, completed-work interpolation | no skipped logical work; deterministic tick ordering and replay |
| **Exact on change** | material mass, carried stock, holdings, ownership claims, ledger movements | update only through declared events; no free creation or silent loss |
| **Lazy on a declared boundary** | needs, health/illness, skills, condition, rot, other slow variables | derive from elapsed simulation time and declared cadence, not render frames or number of incidental calls |

Each update needs an owner, trigger/boundary, ordering rule, inputs, and persistence rule. The same seed, tick, and input history must reproduce the same logical world under the declared determinism contract. A 3× display speed must not skip meals, decay, work, legal deadlines, or governance windows. When the process is intentionally stochastic, the seed and realized event/order evidence must be persisted; do not claim deterministic replay without recording the stochastic inputs.

The per-citizen CA also needs an explicit update contract: whether its cells update synchronously or under a declared schedule; what neighborhood is visible; which world and social observations it may read; how its random seed is scoped; which agent-owned state it may change; and how a cellframe revision is replayed or migrated. A global CA scheduler must not overwrite the civilian's own direction or collapse many civilians into a shared undifferentiated automaton.

### 14.4 Lived civilian model: needs, work, and autonomy

The C9 design record explicitly replaces the old five-state `CitizenState` machine rather than merely adding flags to it. A citizen is a full agent with needs, skills, illness/health, wealth, a home, a job, and a life stage. At-home, commuting, and at-work are consequences of needs and chosen tasks, not the whole identity or a permanent state machine.

C9 Q51 defines five needs—**food, rest, warmth, health, and shelter**—with a three-rung failure ladder: work/output degrades, illness follows, and death is possible at the terminal rung. Q53 says an autonomous citizen chooses by **needs first**, then declared utility weights such as skill fit, distance, priority, and ownership. The model must preserve the declared values and the reason for a selection or refusal; it must not replace autonomy with a hidden global schedule.

Jobs and city tasks are distinct. A job is the citizen's employment/membership relationship with an organization; a task is a concrete unit of work somebody must perform. Public work remains available even when a citizen is not employed by an organization. Work has a site, a declared verb/process, inputs, a quantity or work counter, a claimant/contributor, a time basis, and a check that reads the result back.

Work must be real in the simulation: citizens travel to it, work contributes a measurable amount, and the world changes only when the declared inputs and work are present. Crews may accumulate contributions over multiple ticks or days; each contributor receives credit for their actual contribution, not for merely being present. Failed or infeasible work is refused, deferred, or left blocked with its cause named—not silently treated as completed.

A civilian must be able to rest, decline a bad or impossible deal, defer a task for a named blocker, request assistance, and return to prior goals without losing identity. The government is the owner of last resort for work nobody owns; it must not absorb all private work into one central queue.

### 14.5 Organizations, learning trees, and collective capacity

C9 Q94 defines an organization as **premises + roster + scope**. In its full form it has a place or premises, members, an owner/account relationship, tools/equipment it holds, and work it posts. A citizen founds the organization by staking premises and declaring scope. Employment is membership on the roster, not an abstract job slot detached from an institution. Public work remains separately available. A legal-person/account capacity can arrive when the money/obligation system supports it; this must not be assumed at the earliest pre-money stage.

Organizations must be able to operate, fail, gain and lose capability through their membership. Q164 makes organization skill a **derived view over the current members' records**. If the only smith leaves, dies, emigrates, or otherwise no longer belongs, the organization loses that skill. Do not store an eternal organization-level scalar that outlives the people whose work justifies it.

The learning tree is a real capability/progression structure, not just a UI visualization and not the CA cell grid. C9 specifies a tiered, branched capability DAG/lineage whose nodes represent real capabilities, tools, or processes and whose dependencies are gated by actual work, materials, skill, structures, and other declared preconditions. Citizens develop individual skills through completed work; the society accumulates a shared insight pool; the DAG gates what processes can be used or built. The city-wide unlock survives an individual's death, while individual skill is computed from that person's own record.

Keep four things distinct and linked by evidence:

- **Cellular development:** the living agent's own local generative architecture and its learned dynamics.
- **Individual proficiency:** what this person has actually practiced and demonstrated.
- **Collective insight / capability DAG:** what the society has discovered and can now build or perform.
- **Organization capacity:** what the present roster can collectively do based on member evidence.

The tree must have operational consequences. A metal axe should be a made tool that enables better/longer work; a saw blade may unlock a machine process; neither is simply a rank badge. A progression edge is accepted only when its prerequisite is met and its consumer exists. Cell evolution can help a civilian discover or improve a way to work; it must not silently mark a process as globally unlocked or make an unavailable machine legal to use.

### 14.6 Governance must develop from citizens; courts must become real institutions

C9 does not describe a preinstalled abstract government that puppeteers the population. The founding compact is a written record, and citizens establish institutions. The government is itself an organization staffed by private citizens. The distinction between a citizen's private act and a public act is the authority scope carried by the act, not a separate class of supernatural actors.

The governance system must grow in capability as the society grows. A young settlement may begin with a foreman and keeper of stores; deeper, specialized offices require prerequisites and effort. Offices have holders, declared scope, premises/seat where required, terms, appointment or election rules, revocation, and a chain of command. Offices may be collegial bodies. Organizations may nominate or fund an office-holder but cannot themselves hold public office; office authority belongs to citizens, individually or as a body.

Lawmaking is a real institutional process. Bills are records that move through declared states, consideration, enactment, and effective dates; clauses have closed declared kinds such as constituting, constraining, appropriating, or recommending. A bill does not itself grant a permission. Authority and custody are engine-enforced; legal wrongs are adjudicated. The treasury cannot be spent merely because an agent proposes it: expenditure requires a current appropriation and the authorized scope.

Courts are not a preexisting boolean field called `has_court`. C9 Q125 describes a court as a bench, which may be a building or be made up of people, and Q130 specifies collegial decision rules and recorded dissent. The institution needs a declared offence taxonomy, case/party/evidence references, jurisdiction, a decision rule, an attributable finding, and a record of dissent and appeal. The court's authority and its adjudicative finding must not be confused with a CA confidence score or an Axis package verdict.

A particularly important lifecycle test comes from C9 Q143: **before a court exists, the world records an offence but leaves the finding unresolved**. When citizens later establish a court, the new bench can hear and determine those earlier recorded events. The history is not erased or auto-convicted. The later finding is a new attributable act linked to the original event. This is a direct test of long-lived institutional development: the world must preserve facts while its institutions are still primitive, then let later institutions process the backlog under their lawful authority.

The engine/court boundary is explicit: acts requiring unheld authority or custody are refused by the engine; other acts the design allows (for example trespass, theft, violence, or fraud) are recorded and adjudicated by law. A court's finding does not retroactively turn every logged suspicion into guilt. Appeals, pardons, amnesties, supersession, and dissent are later recorded acts; they do not erase the original event.

### 14.7 The economy is part of the lived verification, not decorative simulation

The needs/work/organization loop depends on an actual resource and obligation system. In C9, materials must come from the world and move through declared gathering, processing, hauling, holding, making, repair, wear, and decay. A building cannot appear because a growth meter says demand is high. The government and organizations own or claim what nobody else owns; ownership claims remain distinct from the physical mass and location of goods.

The obligation ledger supports exchange before gold exists. Money is not conjured at the founding; in C9 it becomes possible only after gold is mined and minted, and later treasury/banking institutions are built by citizens. Per-issuer obligations can fail independently; wages, debts, work-units, queue position, insolvency, and appropriations affect whether work is accepted and whether institutions can fulfill promises. This is how the society can experience scarcity, cooperation, bargaining, and institutional failure rather than being given an unlimited invisible resource pool.

CGA's role is to help civilians adapt within this economy and its processes. It cannot make mass, money, labour, tool lineage, or institutional capacity appear without the declared world process and evidence.

### 14.8 Longitudinal verification protocol

The star passes only from evidence across a declared simulation horizon, not from one green check. The exact number of ticks, days, months, and generations belongs in the star's own accepted contract; this document does not invent it. The verifier should exercise at least the following **state transitions and causal chains**, with source world state and event records preserved:

1. **Founding and survival:** a small founding party starts without a mature city; actual needs generate work; civilians gather, process, and build what sustains them. A homeless citizen remains able to walk and work where the C9 rule requires it.
2. **Autonomous labour:** civilians choose personal needs, rest, jobs, public work, or organization work; they commute, claim tasks, contribute measurable work, decline/defer impossible tasks, and resume them after blockers change.
3. **Social learning and teamwork:** a civilian observes another's action, forms its own hypothesis, practices it, asks for help when appropriate, waits for a partner, coordinates, and records whether cooperation actually succeeded. The receiver's own evidence must remain distinct from the observed source.
4. **Organization formation:** citizens found an organization with premises and declared scope; other civilians join the roster; it posts real work, acquires/uses tools and resources, accumulates member contributions, and can lose a capability when a skilled member leaves.
5. **Learning-tree growth:** citizen work contributes to individual proficiency and collective insight; a real prerequisite becomes available; a process/tool is then built or used from actual materials and work. A learned CA motif alone does not unlock the DAG.
6. **Government formation:** citizens write the compact, establish the initial offices, develop a chain of authority, and later elect/appoint or fill additional roles according to the declared rules. Public works are undertaken by people, not by an invisible state actor.
7. **Court emergence:** before a court exists, a legally relevant event is recorded unresolved; citizens later establish a bench; the bench exercises jurisdiction and issues an attributable finding on the old event, with dissent/appeal data retained as appropriate.
8. **Continuity and loss:** an agent restarts, the runtime updates, a cellframe migrates, a job or relationship ends, a member dies, an organization loses skill, or the city collapses. Identity and prior evidence remain continuous; only the facts the rules say should change are changed.
9. **Replay and audit:** reconstruct the logical world from seed, tick, inputs, and declared records; reconcile resources, work, civic decisions, legal findings, and package/model revisions. A partial or unreadable record is unknown/refused, never silently zero or a green pass.

Evidence must be gathered longitudinally and without replacing ordinary life with a test script. The star should record the initial conditions, seed, world/ruleset revision, agent identity set, cadence schedule, test perturbations, each relevant ticket/decision, CA revision/state digests, work and material read-backs, organizational and legal records, terminal states, and any unresolved/failed checks. It must also report which life and institution phases were not reached; lack of a court because the simulated history did not reach its founding preconditions is not the same as a court test passing.

### 14.9 What the `ala-cities` source currently supports—and what it does not

`ala-cities` is the principal design reference for the civic/life portion of this star. The `design-package-cli` branch is ten commits ahead of `main` and adds a typed design package/source-scanning toolchain plus a deterministic founding-day prototype. The C9 grilling record is unusually valuable because it records the user's decisions and implementation order rather than merely sketching a generic city simulation. It must be read as a mixture of decisions, implemented slices, and explicitly open work—not as proof that the entire society is already running.

| Status | What the inspected branch supports | What must not be claimed from it |
|---|---|---|
| **Present in source** | Fixed-step deterministic world simulation; 20 Hz cadence reference; simple citizen record with home/job/task/path and work interpolation; concrete task records and work counters; ticket/evidence/retirement season records; governance work on declared tick cadences; material ledger, gathering/making/repair slices; `FoundingWorld` deterministic first-day prototype with one founder, forage/wood gathering, fire/shelter construction, day/night transition, evidence, and material-balance check. | That this equals the complete full-agent citizen model, autonomous multi-civilian society, full jobs/organizations/economy, civic government, courts, or learning-tree progression. The founding-day probe calls an explicit action sequence; it is not evidence of a mature autonomous population. |
| **Settled in C9 design** | Full citizen model replaces the five-state machine; five needs; needs-first task choice with declared utility; public/private work separation; crews and contribution records; organization premises+roster+scope; citizen-founded government; offices, election/appointment and chain; capability DAG; individual skill from work record; city-wide insight; courts/benches; unresolved offences before court founding; legal findings, dissent/appeals, obligations and appropriation. | That these are all implemented in the current Rust runtime. The design record is an authoritative target/decision trail for its project, but a design answer is not a passing runtime test. |
| **Still requiring implementation/read-back** | A persistent autonomous population living the whole loop; all five needs operating through their life/death ladder; self-directed job/work selection across cadence; organizations actually founding and operating; progression actually gating processes; government actually being constituted and staffed; offices/elections/laws and court benches actually functioning over history; the per-civilian cellular automata integrated as an agent-controlled operational subsystem; Axis star evidence across a long horizon. | Do not infer completion from document headings, type names, ticket counts, a one-day founding probe, the current `CitizenState`, or the repository's technical governor. |

Three naming cautions matter for implementation review:

- The current `src/sim/citizen.rs` still declares `AtHome`, `ToWork`, `AtWork`, `ToHome`, and `Unemployed`; C9 Q46 says that this state machine is to be **replaced**, not expanded into a larger pile of flags.
- `src/gov` and the season ticket/evidence machinery are the game's technical governance/verification layer. They are not equivalent to the in-world government, public offices, or courts described by C9. `src/agentledger.rs` is a tool-agent work ledger; its agent presence/claim records are not a citizen's job, wage, or civic life record.
- `src/founding_day.rs` proves a limited deterministic founding-day/material-survival slice. It does not prove autonomous agents have chosen those actions, that the population survives through many life stages, or that institutions have emerged.

The primary source references are [the `ala-cities` repository](https://github.com/alacripose/ala-cities), its [10-commit-ahead `design-package-cli` branch](https://github.com/alacripose/ala-cities/tree/design-package-cli), the [C9 citizen/economy/organization/governance/court design record](https://github.com/alacripose/ala-cities/blob/design-package-cli/docs/GRILLING-C9.md), the [C11 whole-game design-sweep record](https://github.com/alacripose/ala-cities/blob/design-package-cli/docs/GRILLING-C11.md), the [current citizen implementation](https://github.com/alacripose/ala-cities/blob/design-package-cli/src/sim/citizen.rs), and the [founding-day prototype](https://github.com/alacripose/ala-cities/blob/design-package-cli/src/founding_day.rs).

### 14.10 Additional Axis-star acceptance expectations

These expectations extend CGA-01 through CGA-30. They define the civilian lived-verification star; they do not rename or specify the other five Axis stars.

| ID | Expectation | Acceptance evidence | Refusal condition |
|---|---|---|---|
| CGA-31 | The civilian lived-verification star tests a persistent society over cadence, not a single skill or action | Star contract names the civilian population, world seed/ruleset, cadence horizon, required life/institution transitions, and complete evidence set | A one-shot test, temporary test identity, or scripted result is passed as lived verification |
| CGA-32 | Cadence is authoritative, deterministic, and independent of render frame rate | Tick-indexed event log, fixed update ordering, declared exact/change/boundary schedules, replay under different render speeds | Life, needs, work, or legal deadlines advance by frame time or are skipped by speed changes |
| CGA-33 | Civilian identity and personal history persist through the whole horizon | Stable AgentId, memory and relationship provenance, needs/life-stage records, explicit migrations/recovery, restart continuity | Process/cellframe/model/test end creates a blank resident or rewrites biography |
| CGA-34 | Full needs and autonomous choice drive daily life | Five needs, health/illness/life-stage effects, needs-first selection, declared utility and recorded decline/defer/help-seeking | Global script assigns actions or hides the reason for a citizen's task choice |
| CGA-35 | Civilians obtain jobs and perform real work, distinct from public tasks | Employment/member relationship, commute/path, claimed task, work counters/contributions, authoritative world read-back | A job is only a label, work is credited without action/time, or the ticket closes without reading the world |
| CGA-36 | Civilians found and operate organizations as real institutions | Premises, roster, scope, posted work, tools/resources, claims/accounts, joins/leaves, capability changes derived from members | Organization is a UI tag, auto-spawned entity, or permanent capability scalar independent of members |
| CGA-37 | The learning tree is a real capability DAG with separate personal/collective/organizational learning | Evidence-linked prerequisite graph; individual work-derived skill; collective insight; organization capability derived from live roster; gated processes/tools | CA pattern activation or visual tree node alone grants skill, unlock, or usable process |
| CGA-38 | Each civilian controls its own cellular-generative architecture | Agent-to-cellframe ownership; declared local inputs/neighbor access, update schedule, rule revision, lineage, resource budget, and state digest; observed CA changes influence the owning agent's actual behavior | Central automaton overwrites all agents; CA is passive-only despite claimed agent control; one civilian mutates another's private cells |
| CGA-39 | Civic institutions are built and staffed by citizens | Founding compact, citizen-founded organizations/government, office-holder identity and scope, appointments/elections, chain-of-command records, appropriations | Preinstalled omniscient government or test harness appoints offices and supplies all decisions |
| CGA-40 | Courts/benches develop as adjudicative bodies and process historical cases | Court founding event, jurisdiction/bench composition, closed offence taxonomy, evidence-linked findings, roll-call/dissent/appeal; old unresolved events considered after founding | Court is a boolean, CA/Axis score is used as guilt, old offences are erased or auto-convicted |
| CGA-41 | Needs, work, resources, obligations, and public finance form a conserved causal economy | Resource/mass read-back; per-issuer obligations; labour contribution and payment; declared money/treasury/appropriation prerequisites | Money/material/work appears without process; public spending exceeds authority or appropriation |
| CGA-42 | The society remains continuous through loss, change, and failure | Longitudinal replays covering job/organization changes, agent exit/death, recovery, runtime/model/cellframe revisions, collapse and later history | Test reset destroys history, organization capacity persists without members, or collapse silently discards the record |
| CGA-43 | The six-star boundary is not conflated with the city's government | Explicit ownership map: Axis verification/grant decision vs in-world offices, laws, courts, organizations, citizen decision owner, CA owner, game authority | Axis acts as the city's mayor/court, or in-world government is treated as permission to bypass Axis/package/adapters |

### 14.11 Star-level pass conditions and non-passes

The star must not pass because a population count is nonzero, jobs exist in a table, a court object has been instantiated, or the learning tree has visible nodes. It passes only when the relevant causal chains are demonstrated by actual residents over the declared horizon and their records can be read back and replayed.

At minimum, separate outcomes must be reported for:

- **life continuity:** identities, needs, health/life stage, work and memory remain coherent;
- **agent control:** each civilian actually steers its own CA and decisions rather than receiving centralized behavior assignments;
- **productive life:** work, jobs, crews, organizations, resources, tools, and obligations have observable effects;
- **learning and progression:** individual, organizational and societal capabilities are derived from evidence and gate real work;
- **institutional development:** government, offices, law, courts and historical adjudication emerge through citizen acts;
- **outer-system compliance:** Axis grants, ticket scopes, package admission, CUA/mob adapters, and world-authoritative results remain valid;
- **replay and provenance:** every pass, refusal, unknown, migration, and unresolved case retains exact tick, revision, identity, and source evidence.

A missing phase is **not reached / unverified**, not passed. A failed causal chain is a failure of the star even when the population remains alive or the renderer looks correct. A refusal is a valid, inspectable outcome only when the contract says that action should be refused; it is not a blanket substitute for a civilian's opportunity to live, work, learn, form institutions, and make consequential decisions.

### 14.12 Final rule for the civilian star

> **The civilians live the simulation; the agents control their own cellular automata; the society builds its own work, organizations, learning tree, government, and courts over authoritative cadence ticks. The Axis star verifies that this actually happens and that the record remains continuous. Axis does not govern the city, and cellular control does not grant a civilian authority over another person or over the outer game/package boundary.**

The star is successful only when the system can show a long-lived, agent-directed society—not merely a functioning CA kernel, a valid package, an action frame, or a scripted founding demonstration.

### 14.13 Recheck against `ala_civ_bots_prototype`: what exists at the agent layer

The second repository changes how the C9 design must be integrated. `ala_civ_bots_prototype` already contains a working architectural spine for **one persistent System-1 decision owner per civilian**, but it does not yet contain the full `ala-cities` civic world or an agent-owned cellular-automata runtime.

The inspected `main` tree was commit `2d04c57b5e0146c8a48915d04f4a79743dbde532` (315 tracked paths). The inspected feature branches were:

| Branch | Relation to `main` | What the branch adds in the inspected comparison | What it does not establish |
|---|---:|---|---|
| `main` | baseline | AgentMind, continuity, needs, skills, Axis gate/cells, work ledger, social/body adapters, and the V4 architecture | Full city-builder society, agent-owned CA implementation, completed headless-client/player parity, or continuous bodyless life |
| `gate/store-and-parity-refusals` | 2 commits ahead | additional build/store/parity refusals and parity/gameplay-degree tooling | in-world jobs, organizations, courts, or a CA runtime |
| `ingest/ping-wheel-language` | 3 commits ahead | Ping Wheel language/corpus ingestion and related proof/tooling | in-world institutions or a CA runtime |
| `ingest/shploingus-corpus-and-aspiration` | 4 commits ahead | richer corpus/aspiration handling in `WishDesk` and `WorkLedger`; unsupported or not-yet-targetable wishes can remain aspirations rather than opening impossible work | a job market, organization roster, government, courts, collective learning DAG, or CA implementation |

The `main` tree contains no source path for a cellular automaton/cellframe/generative morphology implementation. In particular, `AxisGate.Cells` is not that implementation. It is an Axis decision-reading accumulator. The distinction is mandatory because both systems use the word *cell* but have different owners, purposes, update rules, and evidence.

#### 14.13.1 Axis in the prototype is not the six-star aggregate

`AxisVocabulary.AXIS_COUNT` is **9**. Its typed ladders are:

- `evidence_grade`
- `certification_tier`
- `procedural_gate`
- `grade`
- `org_rank`
- `support_level`
- `ticket_evidence`
- `prose_kind`
- `establishment`

These are typed proof/classification ladders. They are not nine stars and they are not the user's six top-level stars. The six-star architecture must therefore be expressed as a separate capability/verification structure, with the civilian lived-verification star as one requirement set. Do not change `AXIS_COUNT` to six or rename one of these ladders to make the concepts appear equivalent.

`AxisGate.Cells` records qualified decision readings, refusal counts, unchecked/absent counts, coverage, the evidence sequence, and a previous-package digest. Its `civilian.axis-cells/1` output closes after a batch of 200 decisions or a 1,200-tick horizon, whichever becomes due. This is a useful compact audit trail over the decisions made by a civilian. It is **not** a CA grid, cellular genome, neighborhood automaton, or civilian learning tree. Its digest chain proves continuity of the Axis reading packages, not that the person's life or the society passed the lived-verification star.

The prototype's Axis gate also documents four concrete checks before a decision becomes a commitment: selection, task shape, reading/witness, and package arrangement. A reading without a witness is unchecked, not a pass. That is useful evidence machinery for the star, but it does not replace the star-level review of continuous life and institutions.

#### 14.13.2 The System-1 spine is real, but the current life model is narrower

`AgentMind` has a genuine fixed-tick decision path: update the body-reported needs; sense at its declared sensory interval; observe outstanding work; run urgent reflex handling; process social input and inner life; generate candidate intentions; read candidates against Axis; arbitrate one dominant intention; pass the selected action through the authority gateway; write the decision ahead of execution; execute a typed skill routine; read back the outcome; and record practice success/failure. Its senses and observations are tied to the current body view. This is the correct place for a civilian-owned cellular architecture to participate: as part of that same person's perception, learning, candidate generation, and System-1-controlled action development—not as a central system that independently pilots the body.

The existing code already makes rest a deliberate action rather than equating idleness with rest. Fatigue is tracked by the mind, a rest candidate can be selected, and the rest routine produces an observed fatigue-relief result. The `RestDecisionTest` is a focused proof of that limited behavior. This is an important foundation, but it is not yet C9's complete needs model.

The current `NeedsState` is a compact observation of player/body state: food, health, air, burning, submersion, and fall distance. It provides hungry/starving/danger predicates. It does not yet implement C9's full five-need ladder of food, rest, warmth, health, and shelter with degradation, illness, and death. The V4 roadmap keeps `NEED-01` as a separate milestone for native and extensible needs, and its status table says mod-added needs/state discovery is still outstanding.

#### 14.13.3 Skill lineage is not the C9 capability DAG

`SkillRegistry.phase1()` currently registers a bounded set of executable routines such as harvesting a tree, placing a block, eating when hungry, requesting/giving food or supplies, resting, and wandering. The `SkillLineage` view is derived from the civilian's actual practice rows and discovery tickets: a lineage node records which verb was first practiced, when it was practiced, successes/failures, and whether a discovery ticket witnessed the node.

This is valuable and should be retained. It supplies an evidence-linked **individual practice lineage**. It is not the full C9 learning/progression tree. C9's target has at least four separate learning/capability layers:

1. the individual's practiced skill and work history;
2. the society's shared insight pool and capability DAG, which gates processes/tools;
3. an organization's capability, derived from its current members' proven skills; and
4. the civilian's own CA state/rule lineage and locally learned generative behaviors.

A `SkillLineage.Node` must not be treated as a DAG unlock. A CA motif must not be treated as demonstrated skill. A demonstrated skill must not itself grant permission. An organizational capability must be recomputed when the roster changes. A society-wide unlock persists under its own rules and must not disappear merely because an individual dies. These are distinct records with explicit references and revision semantics.

#### 14.13.4 The current WorkLedger is a commitment/ticket mechanism, not a labor market

The prototype `WorkLedger` owns a civilian's long-horizon commitments, plans, objective windows, and work tickets in the world store. `observe(...)` is called by the owning mind when it has a live body view; it checks what is in sight and in hand, advances a bounded run through observed stages, replans when a step stops being workable, records satisfaction/expiry, and preserves work across process restart. The `ingest/shploingus-corpus-and-aspiration` branch improves this by retaining an unsupported wish as an aspiration without opening impossible work or spending a ticket.

That is a good personal work-commitment layer, but it does not yet mean the citizen has a job in C9's sense. The following must remain distinct:

- **standing job/employment:** membership in an organization's roster, with duties, compensation/obligations, term and scope;
- **public work availability:** tasks the government owns as last resort and any citizen may take under the declared rules;
- **personal commitment/aspiration:** something the civilian wishes to do or has agreed to pursue;
- **task:** one atomic work unit with verb, target, process, site, inputs, quantity, contributors, remaining work, and a read-back test;
- **ticket:** evidence/authorization record for a scoped unit or run of work;
- **action:** one bodily/world-side effect through the CUA adapter.

The prototype's one-open-commitment behavior and 6,000-tick work window are implementation choices for that work ledger. They must not become a global limit on a civilian's employment, relationships, public duties, private goals, or life. Likewise, an external wish is not a lawful appointment, and closing a ticket must not terminate a job or person.

#### 14.13.5 Durable identity exists; player-independent life progression does not yet

`ContinuityLedger`, `MindStorage`, `MindJournal`, and `AgentContinuity` provide the persistence/recovery spine. They preserve identity and recorded state across restart, retain unbound lives, and refuse to treat unreadable state as a blank new person. The V4 baseline explicitly requires the same AgentId and AgentMind to survive tests, client shutdown, disconnect, restart, and body replacement.

However, the current `AgentRecord.tick(...)` returns when the body is absent or not alive; it detaches the motor/hands and does not call `mind.tick(...)`. The `AgentManager` invokes its life/body processing from the Minecraft server tick, so the current live implementation does not yet advance cognition/needs/social/civic life for an unmaterialized civilian. A durable record waiting for reconnection is **continuity without off-body life simulation**.

V4 explicitly calls for `SIM-01` player-independent continuity: cognition, needs, relationships, schedules, and modeled regional activity continue when no body is materialized. Physical work remains bound to actual materialization and a ticking region; regional/abstract events must be typed as abstract and must not pretend to be physical movement. This milestone is directly required by the lived-verification star. It is not optional performance polish.

The V4 status table also marks the full city-builder substrate, production connected headless client/player parity, extensible mod needs, and tested post-test self-directed life as outstanding or unverified at the supplied status point. A future re-audit should replace that point-in-time report with exact commit, test command, output, and live-client evidence; it should not silently promote the design to implementation.

The inspected source anchors are the [V4 architecture, especially §§37 and 52–53](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/docs/minecraft_ensouled_system1_cua_architecture_v4.md), [`AxisVocabulary.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/axis/AxisVocabulary.java), [`AxisGate.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/axis/AxisGate.java), [`AgentMind.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/core/AgentMind.java), [`NeedsState.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/core/NeedsState.java), [`SkillRegistry.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/core/SkillRegistry.java), [`SkillLineage.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/core/SkillLineage.java), [`AgentRecord.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/mind/AgentRecord.java), [`ContinuityLedger.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/mind/ContinuityLedger.java), and [`WorkLedger.java` on the aspiration branch](https://github.com/alacripose/ala_civ_bots_prototype/blob/ingest/shploingus-corpus-and-aspiration/src/main/java/dev/ensouled/work/WorkLedger.java).

### 14.14 Responsibility mapping: no layer gets to impersonate another

| Layer | Owns | Does not own |
|---|---|---|
| Durable civilian identity/continuity | AgentId, genesis, autobiography, personal memory, needs history, relationships, current life state | client process ID, test identity, job/ticket identity, CA cell identity |
| System-1 mind | the civilian's one dominant intention, candidate arbitration, refusal/decline/defer/help-seeking, when to rest, whether/when to use learned patterns | world truth, another civilian's mind, public law, its own authorization |
| Civilian-owned CA | that civilian's versioned local generative state/rules, bounded update and learning dynamics, patterns it presents to its owner | direct unreviewed world mutations, other civilians' private state, Axis grants, office/court findings |
| CUA/body adapter | ordinary player input and observations for a materialized body; verified physical outcomes from Minecraft | off-body life decisions, virtual movement presented as physical, civic/legal truth |
| Civilization substrate (`ala-cities`) | world/civic/economic state: resources, jobs/tasks, organizations, public/private work, insight, institutions and their consequences | each civilian's System-1 decision ownership; Axis's outer authorization |
| Axis procedural gate and proof records | current scoped decision, declared ladder reading, evidence witness, refusals, package/read-back integrity | the city's legislature, executive, employer, judge, citizen personality, or the CA's own development |
| Sixth-star verifier | longitudinal coverage, causal-chain checks, missing phases, continuity/replay and evidence sufficiency | daily goal selection, job assignment, court decisions, CA evolution, or simulated outcomes |
| Secure Shell / package admission and serving | package identity/revision, ingest/admission, digest/provenance, served/mounted projections and declared data contracts | silently turning parsed or displayed data into trusted authority or executable action |

The System-1 agent must control its own cellular-generative architecture in a way that is visible in evidence: it receives bounded observations, its own local CA state evolves under its declared rule revision, the resulting pattern/activation is returned to that same agent's decision process, and the agent decides whether/how to act with it. An external scheduler may allocate bounded compute and tick opportunities; it must not choose the civilian's daily purpose or overwrite the civilian's local state. A CA signal is an input to the owning mind, not an independent office-holder or legal decision-maker.

### 14.15 Acceptance sequence for the lived-verification star

Do not run this as a scripted checklist in which the harness dictates each citizen's next move. The verifier sets a versioned initial world/seed and observes a bounded scenario; civilians choose and act through their ordinary System-1 loop. Tests may perturb conditions or offer scoped work, but they may not reset a life or substitute a hidden director for the agents.

The acceptance sequence should prove these linked layers in order, with successor tickets where a later layer depends on a prior one:

1. **Identity and two-clock continuity:** a resident advances its civilization state while unmaterialized; rematerialization reconciles actual body/world facts without minting a new AgentId or pretending abstract events were physical actions.
2. **Needs and personal choice:** at least two viable candidates exist (including an intentional rest/wait/decline choice where appropriate), and selection is attributable to the individual's own needs, history, beliefs, and declared weights.
3. **Real work and employment:** the civilian joins or forms an organization, takes a job/membership or public work under its scope, travels when physically materialized, contributes actual work, and has the result read back.
4. **Agent-owned learning:** a CA-derived candidate is observed evolving under a recorded per-agent revision; the owning mind chooses whether to practice; success/failure updates the personal skill lineage only from witnessed outcomes.
5. **Collective progression:** actual member work contributes to a shared insight/capability DAG; a real process/tool becomes usable only when its prerequisite and consumer conditions pass; organization capability is recomputed from its live roster.
6. **Civic institution formation:** citizens establish the founding compact, organizations, offices, and government through recorded acts; the verifier does not appoint them.
7. **Court lifecycle:** a pre-court offence remains unresolved; citizens establish a bench; the bench later hears the old case and creates a new attributable finding while preserving the original event and any dissent/appeal.
8. **Failure and continuity:** loss of a member, insolvency, scarcity, a failed plan, a runtime restart, and a civilization setback change the world under declared rules without deleting the historical record or blanking surviving residents.
9. **Audit/replay:** replay and read-back reconcile the physical world, civic graph, CA revisions, commitments/tickets, personal knowledge, organizations, and law/court records under their correct owners.

The star must report `PASS`, `FAIL`, `NOT_REACHED`, `INCONCLUSIVE`, and `REFUSED` as distinct outcomes with reasons. `NOT_REACHED` is not a pass. `INCONCLUSIVE` means the evidence is insufficient or was invalidated by intervention. `REFUSED` is valid only when the exact requested action or transition is outside the current authorized scope; it does not establish that the overall society passed. A star-level certification is a requirement-set evaluation, not an average of nine Axis ladders or the maximum evidence grade.

### 14.16 Additional expectations from the prototype cross-check

These expectations extend CGA-01 through CGA-43. They specifically prevent the six-star concept, the nine Axis ladders, the per-decision Axis cells, and the civilian-owned CA from being conflated.

| ID | Expectation | Acceptance evidence | Refusal condition |
|---|---|---|---|
| CGA-44 | Six top-level Axis stars remain distinct from the prototype's nine typed ladders | Canonical star contract lists six top-level star owners and requirement vectors; each vector may reference typed Axis ladders without renaming or reducing them | `AXIS_COUNT=9` is rewritten to six, or a ladder token is treated as a star verdict |
| CGA-45 | `civilian.axis-cells/1` remains a decision-evidence aggregate, not the CA runtime | Axis package schema records decision counts, qualified readings, refusals, evidence sequence, tick horizon, and previous digest; CA has separate per-agent cellframe/state/rule schemas | Axis `Cells` package is presented as cellular-generative state or CA evolution evidence |
| CGA-46 | Civilization cadence continues when a body is absent, while physical outcomes remain server-authoritative | SIM-01 scheduler advances defined life/needs/social/regional state; bodyless actions remain abstract; rematerialization reconciles world truth | Durable snapshot is mistaken for off-body life progression, or an abstract event claims an unperformed physical act |
| CGA-47 | Individual skill lineage, collective capability DAG, organization capability, and CA lineage remain separate | Each is independently derivable from its own evidence and revisioned source records; prerequisites are read back before unlock | A practice node, CA motif, social observation, org title, or ticket closure is used as a substitute for another layer |
| CGA-48 | Employment, public work, personal commitment, task, ticket, and action remain separate | A citizen may hold a standing job and multiple relationships while carrying bounded work/tickets; each layer has its own lifecycle and authority | The current one-commitment WorkLedger is used as the entire job/employment/economy model |
| CGA-49 | The canonical Axis authority implementation is distinguished from the prototype's provisional gateway | AuthorityGateway implementation is replaced by the canonical integration at the approved seam; every consequential action fails closed on the current exact scope | A Phase1GatePolicy stand-in or a successful practice count is claimed as canonical permission |
| CGA-50 | The sixth star observes the civilians' own decisions instead of controlling them | Evidence shows multiple autonomous candidates, self-selected goals/rest/work, citizen-founded institutions and agent-owned CA changes over the declared horizon | Test harness, central government script, or evaluator assigns daily actions or constructs court findings for residents |

### 14.17 Final prototype-reconciled rule

> **The sixth star verifies the life of the society; it does not become the society. System-1 owns each civilian's decisions, each civilian controls its own cellular-generative architecture, `ala-cities` supplies the real civic/economic substrate, the CUA adapter supplies only physical embodiment, and Axis checks the current outer permission and evidence boundaries. The civilization clock keeps persons and institutions continuous when physical bodies are absent, while Minecraft remains the authority on anything claimed to have physically happened.**

No successful Axis cell package, learned skill, completed ticket, or founding-day probe alone proves this star. It is proved only by continuous, agent-directed, causally checkable lives and institutions over the declared simulation horizon.

