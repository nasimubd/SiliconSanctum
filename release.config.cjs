module.exports = {
  repositoryUrl: "https://github.com/nasimubd/SiliconSanctum.git",
  branches: ["main"],
  tagFormat: "v${version}",
  plugins: [
    ["@semantic-release/commit-analyzer", { preset: "angular" }],
    ["@semantic-release/release-notes-generator", { preset: "angular" }],
    ["@semantic-release/exec", { prepareCmd: 'node scripts/release/sync-version.cjs "${nextRelease.version}"' }],
    "@semantic-release/changelog",
    ["@semantic-release/git", {
      assets: ["CHANGELOG.md", "VERSION", "crates/sanctum-runtime/Cargo.toml", "Cargo.lock"],
      message: "chore(release): v${nextRelease.version} [skip ci]"
    }],
    ["@semantic-release/github", {
      successComment: false,
      failComment: false,
      releasedLabels: false,
      addReleases: false
    }]
  ]
};
