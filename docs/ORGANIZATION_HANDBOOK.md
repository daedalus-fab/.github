# daedalus-fab organization handbook

> Shared operating defaults for repositories maintained under **daedalus-fab**. Repository-local policy may strengthen these rules but should not silently weaken them.

## Mission

daedalus-fab maintains design, fabrication, manufacturing, and automation software. This `.github` repository is the canonical home for shared policy, reusable templates, community health files, and planning links.

## Repository contract

Each active repository must document purpose, ownership, maturity, supported hardware and environments, development and test commands, authoritative design and data formats, release and rollback procedures, compatibility policy, and GitHub Project/Linear links. Fabrication components should also document units, tolerances, material and machine assumptions, simulation limits, safety interlocks, calibration, provenance, and recovery behavior.

## Change workflow

1. Anchor work in an issue, Linear item, or documented maintenance objective.
2. Keep branches and pull requests focused.
3. Explain motivation, scope, physical and operational risk, validation, compatibility, migration, and rollback.
4. Test invalid input, tolerance boundaries, calibration, interrupted jobs, recovery, and safe-state behavior as relevant.
5. Resolve conflicts semantically by reconstructing both sides' intent.
6. Prefer squash merges for focused work unless commit structure materially improves auditability.

## Evidence, security, and documentation

Pull requests should include reproducible commands, approved fixtures or models, expected and observed results, negative-path coverage, documentation updates, and CI or local-equivalent evidence. Never commit credentials, proprietary designs, production machine data, or sensitive logs. Follow `SECURITY.md` for private reporting. Keep examples executable, units explicit, links current, and important design, safety, compatibility, and operational decisions recorded.

## Planning ownership

GitHub owns code, reviews, checks, releases, and delivery evidence. Linear owns priority, dependencies, sequencing, and cross-project planning. The organization GitHub Project is the cross-repository execution view; see `PROJECTS.md` for routing details.

## Organization health

- [ ] Profiles, descriptions, topics, and READMEs are current.
- [ ] Community health files and reusable issue/PR guidance are present.
- [ ] Units, tolerances, machine assumptions, safety boundaries, and recovery are documented.
- [ ] Required checks reflect correctness, physical safety, compatibility, and supply-chain risk.
- [ ] Stale repositories are archived or clearly marked.
- [ ] GitHub Project and Linear links resolve and reflect completed work.
