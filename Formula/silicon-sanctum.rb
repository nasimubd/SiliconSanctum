class SiliconSanctum < Formula
  desc "Local AI inference control plane for Apple Silicon"
  homepage "https://github.com/nasimubd/SiliconSanctum"
  version "1.15.0"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/nasimubd/SiliconSanctum/releases/download/v#{version}/silicon-sanctum-#{version}-aarch64-apple-darwin.tar.gz"
    sha256 "7177f3db04f424872470bcd407193a66dc282da0ecdc6d3e388c27bd48aef6d9"
  else
    url "https://github.com/nasimubd/SiliconSanctum/releases/download/v#{version}/silicon-sanctum-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "REPLACE_WITH_RELEASE_SHA256"
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
