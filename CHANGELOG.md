# [1.2.0](https://github.com/nasimubd/SiliconSanctum/compare/v1.1.0...v1.2.0) (2026-09-22)


### Bug Fixes

* **io:** bound retained chunk memory before spawning workers ([0a7dfb4](https://github.com/nasimubd/SiliconSanctum/commit/0a7dfb47e7fedc2b70733973b51316473c47d7de))
* **io:** configure uncached reads when opening models ([81b2f12](https://github.com/nasimubd/SiliconSanctum/commit/81b2f120f15f20db1f95695e551ef47cb7de88da))
* **io:** reject allocations beyond slice address limits ([e4f3c92](https://github.com/nasimubd/SiliconSanctum/commit/e4f3c92605997efd3b9b94c90e95bf25621ee98b))
* **io:** reject incomplete model chunks in read workers ([95d0820](https://github.com/nasimubd/SiliconSanctum/commit/95d08206d84fa05a94e0e4fc51a010275bfb83ac))
* **io:** require complete shared buffer transfers ([8862cb3](https://github.com/nasimubd/SiliconSanctum/commit/8862cb38a6edda0987f5853776f5c427304f890a))
* **mach:** reject incomplete VM counter snapshots ([0f74d7f](https://github.com/nasimubd/SiliconSanctum/commit/0f74d7f6289dce76d651b5776813a9e05f09315e))
* **mach:** release host port after every query ([1fccaf5](https://github.com/nasimubd/SiliconSanctum/commit/1fccaf52f5d6f92ec2818edb977c59219a2b7d7e))
* **pressure:** contain handler panics at dispatch boundaries ([4e9c8ba](https://github.com/nasimubd/SiliconSanctum/commit/4e9c8ba3499e1a889d25a0a9890a446a20159c87))
* **release:** exempt legacy rename boundary ([f53ef2a](https://github.com/nasimubd/SiliconSanctum/commit/f53ef2a945c6e8388c82806cfe53d9ca85d0d6fc))
* **release:** lint commits since latest release ([e90470a](https://github.com/nasimubd/SiliconSanctum/commit/e90470a134d02c1df78585902affa00bd41423cc))
* **sysctl:** mirror kernel integer width on write ([710ef94](https://github.com/nasimubd/SiliconSanctum/commit/710ef94a1307f006e88e329db3b43505cbb3b1cf))
* **sysctl:** satisfy strict lint contracts ([3a3b412](https://github.com/nasimubd/SiliconSanctum/commit/3a3b41248a431d8793b9034828deb3523e8197c9))
* **sysctl:** support native 32-bit wired limit ([8646480](https://github.com/nasimubd/SiliconSanctum/commit/8646480fed3fc38f0c1b6c49b38fb9585181909a))


### Features

* **darwin:** add Darwin module boundary ([a1481a2](https://github.com/nasimubd/SiliconSanctum/commit/a1481a2cef74a73a6e24788a24a8894fb8212aab))
* **darwin:** expose Apple Silicon page size ([aac5d1d](https://github.com/nasimubd/SiliconSanctum/commit/aac5d1dbca84e17259f2bc45b8bfe942849429de))
* **io:** adapt aligned buffers as Metal sinks ([8f7f89e](https://github.com/nasimubd/SiliconSanctum/commit/8f7f89e68ccdc23433902ee4bc765a5510c7aba2))
* **io:** add aligned allocation error ([39cdc41](https://github.com/nasimubd/SiliconSanctum/commit/39cdc41bc09f47eceed2b1e12f4fcc2d3db21bad))
* **io:** add bounded direct-read workers ([dca4151](https://github.com/nasimubd/SiliconSanctum/commit/dca415192a8dc555be9daf60216e26455f9e7c04))
* **io:** add direct-read worker errors ([abe5b7e](https://github.com/nasimubd/SiliconSanctum/commit/abe5b7e4e08caede4518dc38b7d29560d3a6ddb4))
* **io:** add shared buffer alignment error ([b62bb30](https://github.com/nasimubd/SiliconSanctum/commit/b62bb308803048e732d02a9aa4540250560add83))
* **io:** add shared buffer capacity error ([c8432ca](https://github.com/nasimubd/SiliconSanctum/commit/c8432ca6742044ff32b7edf251acfa69c006d58f))
* **io:** allocate aligned model buffers ([a4a1f50](https://github.com/nasimubd/SiliconSanctum/commit/a4a1f509aed9ccd79ff866ca7a0ba175855af477))
* **io:** allow aligned buffer ownership transfer ([06cb735](https://github.com/nasimubd/SiliconSanctum/commit/06cb735f8f4cf619ff9912a7c110c6ffe67ae08a))
* **io:** define chunk stream range ([c630f93](https://github.com/nasimubd/SiliconSanctum/commit/c630f932050b6d5455820c48d2c9e9511b5fc76f))
* **io:** define direct I/O errors ([08670dd](https://github.com/nasimubd/SiliconSanctum/commit/08670dd0b07d44845581d409e9016ff560bdc773))
* **io:** define direct I/O module boundary ([71ec502](https://github.com/nasimubd/SiliconSanctum/commit/71ec502ebabbfbf2a384cafffb6062fc95de5f4c))
* **io:** define shared Metal buffer sink ([0edd4d1](https://github.com/nasimubd/SiliconSanctum/commit/0edd4d17ac7d3d0ec43cacad04437cf29e007a20))
* **io:** enable F_NOCACHE on model files ([efb8d4a](https://github.com/nasimubd/SiliconSanctum/commit/efb8d4ab401d1cb90c07f0c01f8f93fa64e4fe6a))
* **io:** expose aligned buffer slices ([ce014e6](https://github.com/nasimubd/SiliconSanctum/commit/ce014e6bb4ab371dcf526807517ec8fa46c5a7f9))
* **io:** implement EINTR-safe positional reads ([93a2c8f](https://github.com/nasimubd/SiliconSanctum/commit/93a2c8fe9b4b03ecc3896a48e94958b4c42f07f6))
* **io:** open model files read-only ([746cedb](https://github.com/nasimubd/SiliconSanctum/commit/746cedbe6f9e9be3ec4f2a6ce3e07f64ba5ed5a1))
* **io:** stream chunks into shared Metal buffers ([74b412b](https://github.com/nasimubd/SiliconSanctum/commit/74b412b003e05f7476c7ffe359a9facba872259d))
* **io:** validate aligned buffer layouts ([dbc0184](https://github.com/nasimubd/SiliconSanctum/commit/dbc018497d6fb8c5b8d7959ef99e7d8ca02099e6))
* **mach:** connect native host provider ([d86afd8](https://github.com/nasimubd/SiliconSanctum/commit/d86afd873f6705c9ef07f07ac779f4b0115aae95))
* **mach:** convert VM pages to bytes ([1d92ae0](https://github.com/nasimubd/SiliconSanctum/commit/1d92ae064ee73bc13e60ce5fb0cdd6a2d6f520c1))
* **mach:** define byte telemetry snapshot ([bf71f34](https://github.com/nasimubd/SiliconSanctum/commit/bf71f349a0e199c95d322b4d8e734cfc11e3dbec))
* **mach:** define host telemetry contract ([518b902](https://github.com/nasimubd/SiliconSanctum/commit/518b9027aea07a31e4f3e84d48d1c9ee62b26eaf))
* **mach:** define native host provider ([2337309](https://github.com/nasimubd/SiliconSanctum/commit/2337309ae13172cf3f5c01eee4661e67967d4e8c))
* **mach:** define telemetry errors ([8ec7f91](https://github.com/nasimubd/SiliconSanctum/commit/8ec7f913b63a7e19f1f4220028b54df1da8c70e0))
* **mach:** define telemetry module boundary ([5634534](https://github.com/nasimubd/SiliconSanctum/commit/563453447a842d13e988f0ae02c37e8f04858742))
* **mach:** define VM page counter snapshot ([8171aaf](https://github.com/nasimubd/SiliconSanctum/commit/8171aaf53838df61ee3b1e6ed8233d7e2155957d))
* **mach:** expose aggregate memory telemetry ([281686c](https://github.com/nasimubd/SiliconSanctum/commit/281686c54f150ceac505c2aa62633e526b4602bc))
* **mach:** read native VM page size ([b7b9be9](https://github.com/nasimubd/SiliconSanctum/commit/b7b9be99ca86046292bee67577ca386b68d3ea9d))
* **mach:** represent truncated host statistics replies ([c0ddff8](https://github.com/nasimubd/SiliconSanctum/commit/c0ddff815f99a933ed1335d2f4a6c3d9d13dba20))
* **mach:** wrap host statistics query ([0aa49df](https://github.com/nasimubd/SiliconSanctum/commit/0aa49df6b71c44378a17de097885c09e44863735))
* **metal:** own page-aligned shared Metal resources ([60c0014](https://github.com/nasimubd/SiliconSanctum/commit/60c0014d0f58fab6fba380723e17a5b9f2bb106c))
* **metal:** read model ranges directly into shared worker buffers ([a2ed37e](https://github.com/nasimubd/SiliconSanctum/commit/a2ed37ea44589e884bd71674983c8dac9d860aba))
* **platform:** define platform module boundary ([663852f](https://github.com/nasimubd/SiliconSanctum/commit/663852f5c54a99becf41c83a70110e5f0f656327))
* **platform:** define unsupported platform error ([77395c3](https://github.com/nasimubd/SiliconSanctum/commit/77395c303ebd626ffc500de103a3d0f12ecb641d))
* **pressure:** decode dispatch event flags ([c50d8b9](https://github.com/nasimubd/SiliconSanctum/commit/c50d8b92606cb5b87261801610a4d6bcfe9fc10c))
* **pressure:** define eviction callback contract ([4281347](https://github.com/nasimubd/SiliconSanctum/commit/4281347ffc1ad0844a686a28a0820e1530ae1c70))
* **pressure:** define pressure levels ([7e819d5](https://github.com/nasimubd/SiliconSanctum/commit/7e819d531eb8e928e668215763e8979b691d0b4a))
* **pressure:** define pressure module boundary ([b8eb3c4](https://github.com/nasimubd/SiliconSanctum/commit/b8eb3c4887ca0d726c740260998789d188933e3d))
* **pressure:** dispatch decoded pressure events ([c05fe4a](https://github.com/nasimubd/SiliconSanctum/commit/c05fe4aa06e79238308ba7586cb208a81462726a))
* **pressure:** manage native dispatch source lifecycle ([5380dc3](https://github.com/nasimubd/SiliconSanctum/commit/5380dc3bf8cda11196aab33747396e8fba0195c4))
* **qos:** define interactive QoS class ([85ff833](https://github.com/nasimubd/SiliconSanctum/commit/85ff8332057bf7114a3e251be2bc1bed865a4f24))
* **qos:** define scheduler setter contract ([546834b](https://github.com/nasimubd/SiliconSanctum/commit/546834b6cb5abe0f8ed4eab438c512c8514e7216))
* **qos:** define thread binding error ([1c15708](https://github.com/nasimubd/SiliconSanctum/commit/1c1570851352063afc43b80754f5be8f47d6201f))
* **qos:** define thread QoS module boundary ([9f4dbd1](https://github.com/nasimubd/SiliconSanctum/commit/9f4dbd1947d02d7196b07752b57cddc85f5caebf))
* **qos:** expose interactive inference binding ([9233b17](https://github.com/nasimubd/SiliconSanctum/commit/9233b171d684c48554a4fc3e2fef8a52733dd6e1))
* **qos:** wrap native pthread QoS setter ([8ed1708](https://github.com/nasimubd/SiliconSanctum/commit/8ed1708daadf2b7e1179858743f5b73fc0a03fcb))
* **sysctl:** add typed wired limit reader ([1ba6d4c](https://github.com/nasimubd/SiliconSanctum/commit/1ba6d4cd6d1fc35aedbb38e17c3549c78ec01fbc))
* **sysctl:** add typed wired limit writer ([4195015](https://github.com/nasimubd/SiliconSanctum/commit/4195015150f835d19acac5038e41eeacd4ec09f1))
* **sysctl:** decode unsigned kernel values ([9b650c5](https://github.com/nasimubd/SiliconSanctum/commit/9b650c524bfbfbd6d82de17e96f722b81683372f))
* **sysctl:** define native backend type ([4c60f72](https://github.com/nasimubd/SiliconSanctum/commit/4c60f726a7a65697f64749774e7df1aa9fe194cf))
* **sysctl:** define read backend contract ([00da69f](https://github.com/nasimubd/SiliconSanctum/commit/00da69f7fc3e7cf3f1805216b4b750189f8e979e))
* **sysctl:** define sysctl errors ([404db7d](https://github.com/nasimubd/SiliconSanctum/commit/404db7d2fea04d7fd4483be67d8d646ca3347ac1))
* **sysctl:** define sysctl module boundary ([a0f1941](https://github.com/nasimubd/SiliconSanctum/commit/a0f194171a8a8c82545323f7dca333d2f520cb43))
* **sysctl:** define wired limit restoration guard ([6e620cd](https://github.com/nasimubd/SiliconSanctum/commit/6e620cd51ad4918a30705b7677d8971853e5579c))
* **sysctl:** define write backend contract ([bdec6bd](https://github.com/nasimubd/SiliconSanctum/commit/bdec6bd9449b1e9df9048480543a30c10eb282fb))
* **sysctl:** expose fallible wired limit restoration ([b980d14](https://github.com/nasimubd/SiliconSanctum/commit/b980d14b7ca2801ba75be3fa698d6cec920ffd1e))
* **sysctl:** implement native size query ([899f0a0](https://github.com/nasimubd/SiliconSanctum/commit/899f0a0adcf15cdbc5742b55c19423212763b696))
* **sysctl:** implement native value read ([8bc31c0](https://github.com/nasimubd/SiliconSanctum/commit/8bc31c073122b9863874a7fe23157314181e7286))
* **sysctl:** implement native value write ([2dd2576](https://github.com/nasimubd/SiliconSanctum/commit/2dd25766f8fddd0ce5dcfeea366c11f06991dbbe))
* **sysctl:** preserve previous wired limit ([b706ff9](https://github.com/nasimubd/SiliconSanctum/commit/b706ff9356ee4348686ce83872d190a27c2d5c6d))
* **sysctl:** restore wired limit on guard drop ([781c78f](https://github.com/nasimubd/SiliconSanctum/commit/781c78f9700d709509091789bc73df95e1ad1bc2))
* **sysctl:** validate native key encoding ([0806938](https://github.com/nasimubd/SiliconSanctum/commit/0806938205871f0d344d35d1f9cb5abc91ef15d5))

# [1.1.0](https://github.com/nasimubd/SiliconSanctum/compare/v1.0.1...v1.1.0) (2026-09-16)


### Features

* add one-command alpha-forge fallback ([#23](https://github.com/nasimubd/SiliconSanctum/issues/23)) ([5517923](https://github.com/nasimubd/SiliconSanctum/commit/55179236c0fcac6d236c866a5e0d0c8ff78953c9))
* automate TickArchive migration checks ([#22](https://github.com/nasimubd/SiliconSanctum/issues/22)) ([b58486d](https://github.com/nasimubd/SiliconSanctum/commit/b58486d8762f7c85494fb2f442174423921429ca))

## [1.0.1](https://github.com/nasimubd/SiliconSanctum/compare/v1.0.0...v1.0.1) (2026-09-15)


### Bug Fixes

* parameterize workstation paths ([#14](https://github.com/nasimubd/SiliconSanctum/issues/14)) ([ca9b950](https://github.com/nasimubd/SiliconSanctum/commit/ca9b950315a7785bdd73385743978aae256bf637))
* preserve release commit boundary ([#21](https://github.com/nasimubd/SiliconSanctum/issues/21)) ([2690b0a](https://github.com/nasimubd/SiliconSanctum/commit/2690b0a29cb3559907edbf6b94d7babe7075f2ac))
* remove machine-specific paths ([#20](https://github.com/nasimubd/SiliconSanctum/issues/20)) ([21c96c8](https://github.com/nasimubd/SiliconSanctum/commit/21c96c88da196d6d78dcfae282f0a833c5bdd1d2))

# 1.0.0 (2026-09-15)


### Bug Fixes

* **cli:** support commands from any directory ([70ae05b](https://github.com/nasimubd/SiliconSanctum/commit/70ae05b5bfd5fec18dbb88c22fbfc310bee2d78d))
* configure Kaggle account and dataset defaults ([471c7d4](https://github.com/nasimubd/SiliconSanctum/commit/471c7d4c9443bfa7181a3a0606cf04baff256f10))
* declare semantic release repository URL ([64ffe7f](https://github.com/nasimubd/SiliconSanctum/commit/64ffe7f70399949c6cade2d4632c852dcb145f2e))
* embed Kaggle job manifest in kernel ([72ca0a0](https://github.com/nasimubd/SiliconSanctum/commit/72ca0a0425536c01d95713abf28558e36cd187c3))
* exempt pre-release legacy commit history ([4a147ab](https://github.com/nasimubd/SiliconSanctum/commit/4a147ab64df36f183cd750a29e6693ab84495d2f))
* install and verify Kaggle CLI ([eeaedb2](https://github.com/nasimubd/SiliconSanctum/commit/eeaedb2ca19464a7d841b47081c432d3834536ce))
* **server:** manage Ollama profiles idempotently ([64ab678](https://github.com/nasimubd/SiliconSanctum/commit/64ab678a9942392d6b2434c9173320343c2a2f4c))


### Features

* add quota-aware Kaggle burst worker ([a9fa251](https://github.com/nasimubd/SiliconSanctum/commit/a9fa251e48f690165115b6b455a511f96b752e98))
* expose explicit long-context Claude sessions ([c5fccc5](https://github.com/nasimubd/SiliconSanctum/commit/c5fccc5445e3cdf25a6f7ea305e4e35d84a986cf))


### Performance Improvements

* optimize local inference and benchmark backends ([2cc6152](https://github.com/nasimubd/SiliconSanctum/commit/2cc6152c8d2aaa7d3f908c235f014bc1c46825e9))

# Changelog

All notable changes to this project will be documented in this file.

Releases are generated from Conventional Commits by `mise run release:full`.
