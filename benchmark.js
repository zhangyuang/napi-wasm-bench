const b = require('benny');
const fs = require('fs');

async function run() {
    const wasm = await import('./pkg-wasm/wasm_demo.js');
    // await wasm.default();
    fs.renameSync('./target/release/libwasm_demo.dylib', './target/release/libwasm_demo_old.node');
    const napi = require('./target/release/libwasm_demo.node');
    
    const testString = "Hello, World! ".repeat(1000); // 创建一个较长的测试字符串

    await b.suite(
        'String Processing Benchmark',

        b.add('WASM Implementation', () => {
            wasm.process_string_wasm(testString);
        }),

        b.add('N-API Implementation', () => {
            napi.processStringNapi(testString);
        }),

        b.cycle(),
        b.complete(),
    );
}

run().catch((e) => {
    console.error(e);
}); 