class SiliconSanctum < Formula
  desc "Local AI inference control plane for Apple Silicon"
  homepage "https://github.com/nasimubd/SiliconSanctum"
  version "1.15.1"
  license "MIT"

  if Hardware::CPU.arm?
    url "https://github.com/nasimubd/SiliconSanctum/releases/download/v#{version}/silicon-sanctum-#{version}-aarch64-apple-darwin.tar.gz"
    sha256 "53dc583e7fd285d73812badff65b1d568574aa49a45f19ee56cd1f9d2e40022a"
  else
    url "https://github.com/nasimubd/SiliconSanctum/releases/download/v#{version}/silicon-sanctum-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "a2bae2d45abf524f39cea9b96ffdf36a4db80d09163d98b90b6e71f6b1dc6541"
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
