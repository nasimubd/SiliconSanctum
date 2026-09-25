class SiliconSanctum < Formula
  desc "Local AI inference control plane for Apple Silicon"
  homepage "https://github.com/nasimubd/SiliconSanctum"
  version "1.15.2"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/nasimubd/SiliconSanctum/releases/download/v#{version}/silicon-sanctum-#{version}-aarch64-apple-darwin.tar.gz"
    sha256 "f3241362fa7cda3a780914a6d4bd013f1ff972fc566bb9d0915df5736c6a9066"
  else
    url "https://github.com/nasimubd/SiliconSanctum/releases/download/v#{version}/silicon-sanctum-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "3aaf8d8c76119451e7c9a3074cfe1590154e180d24fc2167cec9c66775a10c74"
  end

  def install
    bin.install "sanctum"
    bin.install_symlink bin/"sanctum" => "sanctum-serve"
    bin.install_symlink bin/"sanctum" => "sanctum-claude"
    bin.install_symlink bin/"sanctum" => "sanctum-codex"
    bin.install_symlink bin/"sanctum" => "sanctum-opencode"
    bin.install_symlink bin/"sanctum" => "sanctum-aider"
    bin.install_symlink bin/"sanctum" => "sanctum-doctor"
    bin.install_symlink bin/"sanctum" => "sanctum-benchmark"
  end

  test do
    assert_match "sanctum-serve", shell_output("#{bin}/sanctum --help")
  end
end
