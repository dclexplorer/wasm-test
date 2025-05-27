pub struct TestRuntime;

impl TestRuntime {
    // Add a method to test the sandbox security
    pub fn get_test_sandbox_security() -> String {
        let test_code = r#"
            exports.onStart = function() {
                const tests = {};
                
                // Test blocked features
                tests.Worker = typeof Worker !== 'undefined';
                tests.localStorage = typeof localStorage !== 'undefined';
                tests.XMLHttpRequest = typeof XMLHttpRequest !== 'undefined';
                tests.document = typeof document !== 'undefined';
                tests.window = typeof window !== 'undefined';
                tests.require = typeof require !== 'undefined';
                tests.process = typeof process !== 'undefined';
                
                // Test allowed features
                tests.Array = typeof Array !== 'undefined';
                tests.Promise = typeof Promise !== 'undefined';
                tests.JSON = typeof JSON !== 'undefined';
                tests.Math = typeof Math !== 'undefined';
                
                // Test SDK7 runtime features
                tests.module = typeof module !== 'undefined';
                tests.exports = typeof exports !== 'undefined';
                tests.fetch = typeof fetch !== 'undefined';
                tests.WebSocket = typeof WebSocket !== 'undefined';
                tests.setImmediate = typeof setImmediate !== 'undefined';
                
                console.log(JSON.stringify(tests, null, 2));
            };
            
            exports.onUpdate = function(dt) {
                console.log("Update with dt:", dt);
            };
        "#;
        
        test_code.to_string()
    }
}
