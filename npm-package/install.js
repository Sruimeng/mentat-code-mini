const { Binary } = require("binary-install");
const os = require("os");

const REPO = "Sruimeng/mentat-code";
const NAME = "mentat";

const { version } = require("./package.json");

const getPlatform = () => {
    const type = os.type();
    const arch = os.arch();

    if (type === "Windows_NT") return "win-x64.exe";
    if (type === "Linux") return "linux-x64";
    if (type === "Darwin") {
        return arch === "arm64" ? "macos-arm64" : "macos-x64";
    }
    throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
    const platform = getPlatform();
    const url = `https://github.com/${REPO}/releases/download/v${version}/${NAME}-${platform}`;
    return new Binary(NAME, url);
};

const install = () => {
    const binary = getBinary();
    binary.install();
    console.log(`${NAME} has been installed successfully!`);
};

install();
