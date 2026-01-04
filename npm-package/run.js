#!/usr/bin/env node

const { Binary } = require("binary-install");
const os = require("os");
const { version } = require("./package.json");

const REPO = "Sruimeng/mentat-code-mini";
const NAME = "mentat";

/**
 * 获取当前平台的二进制文件后缀
 * @returns {string} 平台标识符
 * @throws {Error} 如果平台不支持
 */
const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT") {
    if (arch === "x64") return "win-x64.exe";
    throw new Error(
      `Unsupported Windows architecture: ${arch}. Only x64 is supported.`
    );
  }

  if (type === "Linux") {
    if (arch === "x64") return "linux-x64";
    throw new Error(
      `Unsupported Linux architecture: ${arch}. Only x64 is supported.`
    );
  }

  if (type === "Darwin") {
    if (arch === "arm64") return "macos-arm64";
    if (arch === "x64") return "macos-x64";
    throw new Error(
      `Unsupported macOS architecture: ${arch}. Only arm64 and x64 are supported.`
    );
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

try {
  const platform = getPlatform();
  const url = `https://github.com/${REPO}/releases/download/v${version}/${NAME}-${platform}`;

  const binary = new Binary(NAME, url);
  binary.run();
} catch (error) {
  console.error(`❌ ${error.message}`);
  console.error("\nSupported platforms:");
  console.error("  - Windows x64");
  console.error("  - Linux x64");
  console.error("  - macOS x64");
  console.error("  - macOS arm64 (Apple Silicon)");
  process.exit(1);
}
