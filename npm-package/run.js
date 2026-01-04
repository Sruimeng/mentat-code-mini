#!/usr/bin/env node

const { Binary } = require("binary-install");
const os = require("os");
const { version } = require("./package.json");

const REPO = "Sruimeng/mentat-code";
const NAME = "mentat";

const getPlatform = () => {
    const type = os.type();
    const arch = os.arch();

    if (type === "Windows_NT") return "win-x64.exe";
    if (type === "Linux") return "linux-x64";
    if (type === "Darwin") {
        return arch === "arm64" ? "macos-arm64" : "macos-x64";
    }
    return "linux-x64";
};

const binary = new Binary(
    NAME,
    `https://github.com/${REPO}/releases/download/v${version}/${NAME}-${getPlatform()}`
);
binary.run();
