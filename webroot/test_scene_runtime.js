console.log(`Test scene runtime loaded ${JSON.stringify(Deno)}`);
module.exports.onStart = async function() {
    console.log(`testing enabled: ${await Deno.core.ops.testing_enabled()}`);
    console.log("Scene runtime started");
    console.log("Scene runtime started with worker version:", JSON.stringify(Deno));
    //const engine = require("~system/EngineApi")
    console.log(JSON.stringify(engine, null, 2));
};

module.exports.onUpdate = async function(dt) {
    console.log("Update with dt:", dt);
};