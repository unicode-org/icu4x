import fs from 'fs';
import crypto from 'crypto';
import { spawn } from 'child_process';

const genShFile = 'gen.sh';
const hashFile = 'gen_hash.txt';

function calculateHash(filePath) {
    const hash = crypto.createHash('sha256');
    const fileData = fs.readFileSync(filePath);
    hash.update(fileData);
    return hash.digest('hex');
}

function runGenSh() {
    console.log('Running gen.sh...');
    const genShProcess = spawn('sh', [genShFile]);

    genShProcess.stdout.on('data', (data) => {
        console.log(data.toString());
    });

    genShProcess.stderr.on('data', (data) => {
        console.error(data.toString());
    });

    genShProcess.on('close', (code) => {
        if (code !== 0) {
            console.error(`gen.sh exited with code ${code}`);
        }
    });
}

try {
    const currentHash = calculateHash(genShFile);
    const previousHash = fs.existsSync(hashFile) ? fs.readFileSync(hashFile, 'utf-8') : null;

    if (currentHash !== previousHash) {
        runGenSh();
        fs.writeFileSync(hashFile, currentHash);
        console.log('gen.sh has been updated. Output regenerated.');
    } else {
        console.log('gen.sh has not changed. Skipping execution.');
    }
} catch (error) {
    console.error('An error occurred:', error);
}
