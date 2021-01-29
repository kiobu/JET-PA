/*

    < ------------------ >
    JET Profile Archiver
    by kiobu#0011
    < ------------------ >

*/

console.log("\n [ JET Profile Archiver ] by kiobu (https://github.com/kiobu/JET-PA)\n")

let go = () => {
    console.log("Launching Server...")
    const Process = require("child_process")
    try {
        Process.spawn("Server.exe", [], {shell: true, detached: true})
    } catch (err) { 
        console.error("Could not spawn Server.exe process. Is this executable in the same folder as Server.exe?")
    }
}

const fs = require('fs')
const ppath = `${__dirname}/user/profiles`

// Read user/profiles.
let dir = undefined;
try {
    dir = fs.readdirSync(ppath, 'utf-8')
} catch (err) {
    console.log(`Could not read ${ppath}. Is this executable in the same folder as Server.exe?`)
    go();
    return;
}

const _cloneProfileDirAsync = (src, dest) => {
    if (!fs.lstatSync(src).isDirectory()) { return; }
    const srcDir = fs.readdirSync(src)
    for (let k in srcDir) {
        try {
            let file = fs.readFileSync(`${src}/${srcDir[k]}`)
            fs.writeFileSync(`${dest}/${srcDir[k]}`, file)
            console.log(`Wrote ${`${srcDir[k]}`}`)
        } catch (e) {
            console.error(`There was an error backing up a profile:\n > ${e}`)
        }
    }
}

let profiles = []

for (let k in dir) { profiles.push(dir[k]) }

// Set up backup directory.
const apath = `${__dirname}/__profileBackups`
if (!fs.existsSync(apath)) { fs.mkdirSync(apath) }

const timestamp = ((new Date()).toString()).substr(0, (new Date()).toString().indexOf('(')).split(" ").join("-").split(":").join("_");
try { fs.mkdirSync(`${apath}/${timestamp}`) } catch (e) { console.error(`Couldn't create folder ${apath}/${timestamp}`); }

for (let k in profiles) {
    console.log(`Archiving ${profiles[k]} ...`)
    fs.mkdirSync(`${apath}/${timestamp}/${profiles[k]}`)
    _cloneProfileDirAsync(`${ppath}/${profiles[k]}`, `${apath}/${timestamp}/${profiles[k]}`)
}

go();