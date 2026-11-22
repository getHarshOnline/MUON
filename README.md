# MUON: Micro-Unit Operating Network

A reasoning system made of many small units needs each unit to say what it can do, what it will refuse and what it costs, so that one unit can stand in for another and the swap can be judged. MUON is the single-unit layer of Causality, the research programme that RawThoughts Enterprises Private Limited (RTEPL) runs at [causality.in](https://causality.in), with Harsh Joshi as author. MUON owns everything about one unit: the slot contract that every unit must honour, the models that fill the slot, and learning inside the unit. The plan starts with adapters for open-weight models and keeps a micro-transformer unit as the long-term implementation. [HENA](https://causality.in/hena/) studies the swarm of many units. The formal types come from [MONAD](https://causality.in/monad/).

A unit that combines temporal processing with routed experts must give the temporal output a direct path to predictions, not only a say in which expert is selected. The [research statement](paper/sections/00_problem_statement.tex) sets the comparison such a design must pass.

## The muon-kernel crate re-exports MONAD

Today the repository holds the draft slot contract in `docs/`, the [research statement](paper/sections/00_problem_statement.tex), which sets out the proposed unit, its interface and the comparisons it must pass, and the `muon-kernel` crate, which only re-exports MONAD's kernel. `protos/muon/unit.proto` imports MONAD's state proto and declares no messages. No unit coordinator or adapter exists: a conformance suite, then adapters for open-weight models, are the problems in front of it.

## The kernel builds on MONAD

The [build guide](docs/building.md) gives the pinned MONAD dependency, kernel check and paper prerequisites. The [developer overview](docs/index.md) explains the proposed slot, capabilities and resource admission. For the adaptive-unit question and learning examples, read [Computation, state and learning](https://causality.in/muon/computation-state-and-learning/).

## MUON sits between MONAD and HENA

The research layers are `MONAD (calculus) → MUON (one unit) → HENA (many units)`. Reckon is not the swarm's memory owner.

| Repository | Relationship |
| --- | --- |
| [MONAD](https://causality.in/monad/) | The calculus; MUON's only code dependency |
| [HENA](https://causality.in/hena/) | The swarm, which depends on MUON's kernel |
| [Reckon](https://causality.in/reckon/) | Measures whether a change helped, over records in stores their owners maintain; no code edge with MUON |
| [Q-MUON](https://causality.in/q-muon/) | The quantum variant, which keeps MUON's slot contract and is kept in step with MUON by merge at each stage |

## MUON is licensed AGPL-3.0-or-later

MUON's kernel, protocol header and paper use [AGPL-3.0-or-later](LICENSE). Third-party material keeps its own terms and required notices.

Commercial use is permitted under the applicable licence. Alternative terms require a separate written agreement with RawThoughts Enterprises Private Limited, which can grant them only for material it is entitled to relicense; this repository grants none. See [NOTICE](NOTICE) for ownership, [CITATION.cff](CITATION.cff) for citation and [CONTRIBUTING.md](CONTRIBUTING.md) for contribution requirements. Outside contributions remain closed pending review of the [draft agreement](CLA.md) and a way to record acceptance.

Copyright (c) 2025-2026 RawThoughts Enterprises Private Limited (RTEPL).
