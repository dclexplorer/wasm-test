console.log("Test scene runtime loaded");
module.exports.onStart = function() {
    console.log("Scene runtime started");
    const engine = require("~system/EngineApi")
    console.log(JSON.stringify(engine, null, 2));
};

module.exports.onUpdate = function(dt) {
    console.log("Update with dt:", dt);
};