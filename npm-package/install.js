const { Binary } = require("binary-install");
const os = require("os");
const fs = require("fs");
const path = require("path");
const crypto = require("crypto");
const https = require("https");
const http = require("http");

const REPO = "Sruimeng/mentat-code";
const NAME = "mentat";

const { version } = require("./package.json");

// 尝试加载校验和文件（如果存在）
let checksums = {};
try {
    const checksumsPath = path.join(__dirname, "checksums.json");
    if (fs.existsSync(checksumsPath)) {
        checksums = require("./checksums.json");
    }
} catch (e) {
    // 校验和文件不存在或无法解析，继续但不验证
    console.warn("⚠️  checksums.json not found, skipping integrity verification");
}

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

const getBinaryUrl = () => {
    const platform = getPlatform();
    return `https://github.com/${REPO}/releases/download/v${version}/${NAME}-${platform}`;
};

/**
 * 计算文件的 SHA256 哈希值
 * @param {string} filePath - 文件路径
 * @returns {Promise<string>} - 十六进制格式的哈希值
 */
const calculateFileHash = (filePath) => {
    return new Promise((resolve, reject) => {
        const hash = crypto.createHash("sha256");
        const stream = fs.createReadStream(filePath);
        
        stream.on("data", (data) => hash.update(data));
        stream.on("end", () => resolve(hash.digest("hex")));
        stream.on("error", reject);
    });
};

/**
 * 验证文件完整性
 * @param {string} filePath - 文件路径
 * @param {string} expectedHash - 预期的哈希值
 * @returns {Promise<boolean>} - 验证是否通过
 */
const verifyIntegrity = async (filePath, expectedHash) => {
    if (!expectedHash) {
        console.warn("⚠️  No checksum available for verification");
        return true; // 没有校验和时跳过验证
    }
    
    const actualHash = await calculateFileHash(filePath);
    
    if (actualHash !== expectedHash) {
        console.error("❌ Checksum verification failed!");
        console.error(`   Expected: ${expectedHash}`);
        console.error(`   Actual:   ${actualHash}`);
        return false;
    }
    
    console.log("✅ Checksum verification passed");
    return true;
};

/**
 * 获取二进制文件的安装路径
 * @returns {string} - 二进制文件路径
 */
const getBinaryPath = () => {
    const platform = getPlatform();
    const binaryName = platform.endsWith(".exe") ? `${NAME}.exe` : NAME;
    
    // binary-install 默认安装到 node_modules/.bin 或包目录下
    const possiblePaths = [
        path.join(__dirname, "node_modules", ".bin", binaryName),
        path.join(__dirname, binaryName),
        path.join(__dirname, "bin", binaryName),
    ];
    
    for (const p of possiblePaths) {
        if (fs.existsSync(p)) {
            return p;
        }
    }
    
    // 返回默认路径
    return path.join(__dirname, binaryName);
};

const install = async () => {
    try {
        const platform = getPlatform();
        const url = getBinaryUrl();
        
        console.log(`📦 Installing ${NAME} v${version} for ${platform}...`);
        console.log(`   URL: ${url}`);
        
        // 使用 binary-install 下载
        const binary = new Binary(NAME, url);
        binary.install();
        
        // 获取预期的校验和
        const checksumKey = `${NAME}-${platform}`;
        const expectedChecksum = checksums[checksumKey];
        
        // 验证完整性（如果有校验和）
        if (expectedChecksum) {
            const binaryPath = getBinaryPath();
            
            if (fs.existsSync(binaryPath)) {
                const isValid = await verifyIntegrity(binaryPath, expectedChecksum);
                
                if (!isValid) {
                    // 验证失败，删除文件并退出
                    fs.unlinkSync(binaryPath);
                    console.error("❌ Installation aborted due to checksum mismatch");
                    console.error("   This could indicate a corrupted download or a security issue.");
                    console.error("   Please try again or report this issue.");
                    process.exit(1);
                }
            } else {
                console.warn(`⚠️  Binary not found at expected path: ${binaryPath}`);
                console.warn("   Skipping integrity verification");
            }
        }
        
        console.log(`✅ ${NAME} has been installed successfully!`);
        
    } catch (error) {
        console.error(`❌ Installation failed: ${error.message}`);
        process.exit(1);
    }
};

install();
