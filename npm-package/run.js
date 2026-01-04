#!/usr/bin/env node

const os = require("os");
const path = require("path");
const { spawn } = require("child_process");
const fs = require("fs");

// 必须与 release.yml 中生成的文件名前缀一致
const NAME = "mentat";

/**
 * 根据系统架构获取对应的二进制文件名
 * 这些文件名必须完全匹配 release.yml 中 Artifacts 的命名
 */
const getBinaryName = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT") {
    if (arch === "x64") return `${NAME}-win-x64.exe`;
    throw new Error(`Unsupported Windows architecture: ${arch}`);
  }

  if (type === "Linux") {
    if (arch === "x64") return `${NAME}-linux-x64`;
    throw new Error(`Unsupported Linux architecture: ${arch}`);
  }

  if (type === "Darwin") {
    if (arch === "arm64") return `${NAME}-macos-arm64`;
    if (arch === "x64") return `${NAME}-macos-x64`;
    throw new Error(`Unsupported macOS architecture: ${arch}`);
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const main = () => {
  try {
    const binaryName = getBinaryName();
    // 指向当前包内的 bin 目录
    const binaryPath = path.join(__dirname, "bin", binaryName);

    // 检查文件是否存在（防御性编程）
    if (!fs.existsSync(binaryPath)) {
      console.error(`❌ Critical Error: Binary not found at ${binaryPath}`);
      console.error(
        "   This npm package might be corrupted or built incorrectly."
      );
      console.error(
        "   Please try reinstalling: npm install -g mentat-code-mini --force"
      );
      process.exit(1);
    }

    // 🔥 核心：启动子进程运行二进制文件
    // stdio: 'inherit' 让子进程直接使用当前终端的输入输出（支持颜色、交互）
    const proc = spawn(binaryPath, process.argv.slice(2), { stdio: "inherit" });

    // 监听子进程结束，传递退出码
    proc.on("close", (code) => {
      process.exit(code);
    });

    // 监听错误（比如没有执行权限）
    proc.on("error", (err) => {
      console.error(`❌ Failed to start subprocess: ${err.message}`);
      process.exit(1);
    });
  } catch (error) {
    console.error(`❌ ${error.message}`);
    process.exit(1);
  }
};

main();
