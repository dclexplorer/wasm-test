pub struct TestRuntime;

impl TestRuntime {
    // Add a method to test the sandbox security
    pub fn get_test_sandbox_security() -> String {
        let test_code = r#"
            console.log(JSON.stringify(console, null, 2));
            module.exports.onStart = function() {
                const tests = {};
                const results = [];
                
                // Helper function to test and report
                function testFeature(name, testFn) {
                    try {
                        const result = testFn();
                        tests[name] = result;
                        if (result) {
                            console.warn(`⚠️  SECURITY WARNING: ${name} is ALLOWED`);
                            results.push(`⚠️  ${name}: ALLOWED (potential security risk)`);
                        } else {
                            console.log(`✅ ${name}: BLOCKED`);
                            results.push(`✅ ${name}: BLOCKED`);
                        }
                    } catch (e) {
                        tests[name] = false;
                        console.log(`✅ ${name}: BLOCKED (threw error: ${e.message})`);
                        results.push(`✅ ${name}: BLOCKED (error)`);
                    }
                }
                
                console.log("=== SANDBOX SECURITY TEST ===");
                
                // Test dangerous globals
                testFeature('Worker', () => typeof Worker !== 'undefined');
                testFeature('localStorage', () => typeof localStorage !== 'undefined');
                testFeature('sessionStorage', () => typeof sessionStorage !== 'undefined');
                testFeature('XMLHttpRequest', () => typeof XMLHttpRequest !== 'undefined');
                testFeature('document', () => typeof document !== 'undefined');
                testFeature('window', () => typeof window !== 'undefined');
                testFeature('require', () => typeof require !== 'undefined');
                testFeature('process', () => typeof process !== 'undefined');
                testFeature('global', () => typeof global !== 'undefined');
                testFeature('Buffer', () => typeof Buffer !== 'undefined');
                testFeature('eval', () => {
                    try {
                        eval('1+1');
                        return true;
                    } catch {
                        return false;
                    }
                });
                testFeature('Function constructor', () => {
                    try {
                        new Function('return 1')();
                        return true;
                    } catch {
                        return false;
                    }
                });
                
                // Test file system access
                testFeature('fs module', () => {
                    try {
                        if (typeof require !== 'undefined') {
                            require('fs');
                            return true;
                        }
                        return false;
                    } catch {
                        return false;
                    }
                });
                
                // Test network access beyond fetch
                testFeature('net module', () => {
                    try {
                        if (typeof require !== 'undefined') {
                            require('net');
                            return true;
                        }
                        return false;
                    } catch {
                        return false;
                    }
                });
                
                // Test child process
                testFeature('child_process', () => {
                    try {
                        if (typeof require !== 'undefined') {
                            require('child_process');
                            return true;
                        }
                        return false;
                    } catch {
                        return false;
                    }
                });
                
                console.log("\n=== ALLOWED FEATURES (SDK7 Runtime) ===");
                
                // Test allowed features
                const allowedFeatures = {
                    'Array': typeof Array !== 'undefined',
                    'Promise': typeof Promise !== 'undefined',
                    'JSON': typeof JSON !== 'undefined',
                    'Math': typeof Math !== 'undefined',
                    'Date': typeof Date !== 'undefined',
                    'RegExp': typeof RegExp !== 'undefined',
                    'Map': typeof Map !== 'undefined',
                    'Set': typeof Set !== 'undefined',
                    'module': typeof module !== 'undefined',
                    'exports': typeof exports !== 'undefined',
                    'fetch': typeof fetch !== 'undefined',
                    'WebSocket': typeof WebSocket !== 'undefined',
                    'setImmediate': typeof setImmediate !== 'undefined',
                    'setTimeout': typeof setTimeout !== 'undefined',
                    'setInterval': typeof setInterval !== 'undefined',
                    'console': typeof console !== 'undefined'
                };
                
                for (const [feature, available] of Object.entries(allowedFeatures)) {
                    tests[feature] = available;
                    if (available) {
                        console.log(`✅ ${feature}: Available`);
                    } else {
                        console.warn(`⚠️  WARNING: ${feature} is NOT available (should be allowed)`);
                    }
                }
                
                // Try some actual unsafe operations
                console.log("\n=== UNSAFE OPERATION TESTS ===");
                
                testFeature('Import external modules', () => {
                    try {
                        import('http');
                        return true;
                    } catch {
                        return false;
                    }
                });
                
                testFeature('Access __proto__', () => {
                    try {
                        ({}).__proto__.polluted = true;
                        return true;
                    } catch {
                        return false;
                    }
                });
                
                testFeature('Modify Object.prototype', () => {
                    try {
                        Object.prototype.polluted = true;
                        return true;
                    } catch {
                        return false;
                    }
                });
                
                console.log("\n=== SUMMARYY ===");
                console.log(JSON.stringify(tests, null, 2));
                console.log("\n=== SECURITY REPORT ===");
                results.forEach(r => console.log(r));
            };
            
            module.exports.onUpdate = function(dt) {
                console.log("Update with dt:", dt);
            };
        "#;
        
        test_code.to_string()
    }
}
